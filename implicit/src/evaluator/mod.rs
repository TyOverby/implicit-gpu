use compiler::*;

use itertools::Itertools;
use nodes::{Node, NodeRef, PolyGroup};
use opencl::FieldBuffer;

use opencl::OpenClContext;
use polygon::run_poly;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Evaluator {
    finished: Mutex<HashMap<GroupId, FieldBuffer>>,
    width: usize,
    height: usize,
    nest: Nest,
}


impl Evaluator {
    pub fn new(nest: Nest, width: usize, height: usize, _prev: Option<Evaluator>) -> Evaluator {
        Evaluator {
            finished: Mutex::new(HashMap::new()),
            width: width,
            height: height,
            nest: nest,
        }
    }

    pub fn evaluate(&self, which: GroupId, ctx: &OpenClContext) -> FieldBuffer {
        let _guard = ::flame::start_guard(format!("evaluate {:?}", which));
        {
            let finished = self.finished.lock().unwrap();
            if let Some(buff) = finished.get(&which) {
                return buff.clone();
            }
        }

        let eval_basic_group = |root: &Node| -> FieldBuffer {
            let _guard = ::flame::start_guard(format!("eval_basic_group"));
            let (program, compilation_info) = ::compiler::compile(root);
            let deps: Vec<FieldBuffer> = compilation_info
                .dependencies
                .iter()
                .map(|&g| self.evaluate(g, ctx))
                .collect();

            let out = ctx.field_buffer(self.width, self.height, None);
            let kernel = ctx.compile("apply", program);

            let mut kc = kernel.gws([self.width, self.height]).arg_buf(out.buffer()).arg_scl(
                self.width as
                    u64,
            );

            for dep in &deps {
                kc = kc.arg_buf(dep.buffer());
            }

            ::flame::span_of("eval", || kc.enq().unwrap());

            out
        };

        let eval_polygon = |poly: &PolyGroup, dx: f32, dy: f32| -> FieldBuffer {
            let _guard = ::flame::start_guard(format!("eval_poylgon"));
            let additive_field = {
                let _guard = ::flame::start_guard("additive field");
                let points_all = poly.additive.iter().flat_map(|a| a.points.iter().cloned()).collect::<Vec<_>>();
                run_poly(&points_all, self.width, self.height, Some((dx, dy)), ctx)
            };

            let subtractive_field = {
                let _guard = ::flame::start_guard("subtractive field");
                let points_all = poly.subtractive.iter().flat_map(|a| a.points.iter().cloned()).collect::<Vec<_>>();
                if points_all.len() != 0 {
                    Some(run_poly(&points_all, self.width, self.height, Some((dx, dy)), ctx))
                } else {
                    None
                }
            };

            if let Some(subtractive_field) = subtractive_field {
                let program = Node::And {
                    children: vec![
                        NodeRef::new(Node::OtherGroup { group_id: GroupId(0) }),
                        NodeRef::new(Node::Not { target: NodeRef::new(Node::OtherGroup { group_id: GroupId(1) }) }),
                    ],
                };

                let (program, _) = ::compiler::compile(&program);
                let kernel = ctx.compile("apply", program);

                let out = ctx.field_buffer(self.width, self.height, None);

                let kc = kernel
                    .gws([self.width, self.height])
                    .arg_buf(out.buffer())
                    .arg_scl(self.width as u64)
                    .arg_buf(additive_field.buffer())
                    .arg_buf(subtractive_field.buffer());

                ::flame::span_of("eval", || kc.enq().unwrap());

                out
            } else {
                additive_field
            }
        };

        let group = self.nest.get(which);
        let out = match group {
            &NodeGroup::Basic(ref root) => eval_basic_group(root),
            &NodeGroup::Freeze(ref root) => {
                let field_buf = eval_basic_group(root);
                let (width, height) = field_buf.size();
                let lines = ::marching::run_marching(&field_buf, ctx);
                let res = ::polygon::run_poly_raw(lines, width, height, None, ctx);
                res
            }
            &NodeGroup::Polygon { ref group, dx, dy } => eval_polygon(group, dx, dy),
        };

        {
            let mut finished = self.finished.lock().unwrap();
            finished.insert(which, out.clone());
        }

        out
    }

    pub fn get_polylines(&self, buffer: &FieldBuffer, ctx: &OpenClContext) -> Vec<((f32, f32), (f32, f32))> {
        let lines = ::marching::run_marching(buffer, ctx);
        let lines = lines.values().into_iter().tuples::<(_, _, _, _)>();
        lines
            .map(|(a, b, c, d)| ((a, b), (c, d)))
            .filter(|&((x1, y1), (x2, y2))| {
                !(x1.is_nan() || x2.is_nan() || y1.is_nan() || y2.is_nan())
            })
            .collect()
    }
}
