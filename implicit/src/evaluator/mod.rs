use compiler::*;

use itertools::Itertools;
use nan_filter::filter_nans;
use nodes::{Node, PolyGroup, StaticNode};
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

        let eval_basic_group = |root: &StaticNode| -> FieldBuffer {
            let _guard = ::flame::start_guard(format!("eval_basic_group"));
            let (program, compilation) = ::compiler::compile(root.node());
            let deps: Vec<FieldBuffer> = compilation.deps().iter().map(|&g| self.evaluate(g, ctx)).collect();

            let out = ctx.field_buffer(self.width, self.height, None);
            let kernel = ctx.compile("apply", program);

            let mut kc = kernel.gws([self.width, self.height]).arg_buf(out.buffer()).arg_scl(self.width as u64);

            for dep in &deps {
                kc = kc.arg_buf(dep.buffer());
            }

            ::flame::span_of("eval", || kc.enq().unwrap());

            out
        };

        let eval_polygon = |poly: &PolyGroup| -> FieldBuffer {
            let _guard = ::flame::start_guard(format!("eval_poylgon"));
            let additive_field = {
                let _guard = ::flame::start_guard("additive field");
                let xs_all: Vec<_> = poly.additive.iter().flat_map(|a| a.xs.iter().cloned()).collect();
                let ys_all: Vec<_> = poly.additive.iter().flat_map(|a| a.ys.iter().cloned()).collect();
                run_poly(&xs_all, &ys_all, self.width, self.height, ctx)
            };

            let subtractive_field = {
                let _guard = ::flame::start_guard("subtractive field");
                let xs_all: Vec<_> = poly.subtractive.iter().flat_map(|a| a.xs.iter().cloned()).collect();
                let ys_all: Vec<_> = poly.subtractive.iter().flat_map(|a| a.ys.iter().cloned()).collect();
                if xs_all.len() != 0 {
                    Some(run_poly(&xs_all, &ys_all, self.width, self.height, ctx))
                } else {
                    None
                }
            };

            if let Some(subtractive_field) = subtractive_field {
                let program = create_node!(
                    a, {
                        a(
                            Node::And(
                                vec![
                                    a(Node::OtherGroup(GroupId(0))),
                                    a(Node::Not(a(Node::OtherGroup(GroupId(1))))),
                                ]
                            )
                        )
                    }
                );

                let (program, _) = ::compiler::compile(program.node());
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
                let (xs, ys) = ::marching::run_marching(&field_buf, ctx);
                let xs = filter_nans(&xs, ctx);
                let ys = filter_nans(&ys, ctx);
                let res = ::polygon::run_poly_raw(xs, ys, width, height, ctx);
                res
            }
            &NodeGroup::Polygon(ref poly) => eval_polygon(poly),
        };

        {
            let mut finished = self.finished.lock().unwrap();
            finished.insert(which, out.clone());
        }

        out
    }

    pub fn get_polylines(&self, buffer: &FieldBuffer, ctx: &OpenClContext) -> Vec<((f32, f32), (f32, f32))> {
        let (xs, ys) = ::marching::run_marching(buffer, ctx);
        let points = xs.values().into_iter().zip(ys.values().into_iter());
        let lines = points.tuples();
        lines
            .filter(|&((x1, y1), (x2, y2))| !(x1.is_nan() || x2.is_nan() || y1.is_nan() || y2.is_nan()))
            .collect()
    }
}
