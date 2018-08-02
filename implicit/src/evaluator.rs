use compiler::*;
use euclid::point2;
use geometry::Point;
use itertools::Itertools;
use lines::connect_lines;
use nodes::poly::separate_polygons;
use nodes::{Node, PolyGroup};
use opencl::OpenClContext;
use opencl::{FieldBuffer, LineBuffer};
use polygon::run_poly;
use std::collections::HashMap;
use std::sync::Mutex;
use telemetry::{Telemetry, TelemetryLocation};

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

    pub fn evaluate(&self, which: GroupId, ctx: &OpenClContext, telemetry: &mut Telemetry, tloc: TelemetryLocation) -> FieldBuffer {
        let _guard = ::flame::start_guard(format!("evaluate {:?}", which));
        {
            let finished = self.finished.lock().unwrap();
            if let Some(buff) = finished.get(&which) {
                return buff.clone();
            }
        }

        let eval_basic_group = |root: &Node, telemetry: &mut Telemetry, mut tloc: TelemetryLocation| -> FieldBuffer {
            let _guard = ::flame::start_guard(format!("eval_basic_group"));
            let (program, compilation_info) = ::compiler::compile(root);
            let deps: Vec<FieldBuffer> = compilation_info
                .dependencies
                .iter()
                .map(|&g| {
                    tloc.new_intermediate();
                    self.evaluate(g, ctx, telemetry, tloc)
                })
                .collect();

            let out = ctx.field_buffer(self.width, self.height, None);
            let kernel = ctx.compile("apply", program.clone());

            let mut kc = kernel
                .queue(ctx.queue().clone())
                .gws([self.width, self.height])
                .arg_buf(out.buffer())
                .arg_scl(self.width as u64);

            for dep in &deps {
                kc = kc.arg_buf(dep.buffer());
            }

            ::flame::span_of("eval", || unsafe { kc.enq().unwrap() });

            telemetry.intermediate_eval_basic(tloc, &out, &program, root);
            out
        };

        let eval_polygon = |poly: &PolyGroup, dx: f32, dy: f32, telemetry: &mut Telemetry| -> FieldBuffer {
            let _guard = ::flame::start_guard(format!("eval_poylgon"));
            let additive_field = {
                let _guard = ::flame::start_guard("additive field");
                let points_all = poly.additive.iter().flat_map(|a| a.points.iter().cloned());
                run_poly(points_all, None, self.width, self.height, Some((dx, dy)), ctx).unwrap()
            };

            telemetry.intermediate_eval_poly(tloc, &additive_field);
            additive_field
        };

        let group = self.nest.get(which);
        let out = match group {
            &NodeGroup::Basic(ref root) => eval_basic_group(root, telemetry, tloc),
            &NodeGroup::Freeze(ref root) => {
                let field_buf = eval_basic_group(root, telemetry, tloc);
                let (width, height) = field_buf.size();
                let (lines, count) = ::marching::run_marching(&field_buf, ctx);
                let (lines, _) = line_buffer_to_poly(&lines, count, telemetry, tloc, true);
                let lines = lines.into_iter().flat_map(grouping_to_segments);
                let res = ::polygon::run_poly(lines, Some(field_buf), width, height, None, ctx);
                match res {
                    Some(res) => {
                        telemetry.intermediate_eval_poly(tloc, &res);
                        res
                    }
                    None => ctx.field_buffer_inf(self.width, self.height),
                }
            }
            &NodeGroup::Polygon { ref group, dx, dy } => eval_polygon(group, dx, dy, telemetry),
        };

        {
            let mut finished = self.finished.lock().unwrap();
            finished.insert(which, out.clone());
        }

        out
    }

    pub fn get_polylines(&self, buffer: &FieldBuffer, ctx: &OpenClContext) -> Vec<((f32, f32), (f32, f32))> {
        let (lines, lines_count) = ::marching::run_marching(buffer, ctx);
        let lines = lines.values(Some(lines_count)).into_iter().tuples::<(_, _, _, _)>();
        lines.map(|(a, b, c, d)| ((a, b), (c, d))).collect()
    }
}

pub fn line_buffer_to_poly(
    buffer: &LineBuffer, count: u32, telemetry: &mut Telemetry, tloc: TelemetryLocation, simplify: bool,
) -> (Vec<Vec<Point>>, Vec<Vec<Point>>) {
    let lines = buffer.values(Some(count));

    let lines = lines
        .into_iter()
        .tuples::<(_, _, _, _)>()
        .take_while(|&(a, b, c, d)| !(a.is_nan() || b.is_nan() || c.is_nan() || d.is_nan()))
        .map(|(a, b, c, d)| (point2(a, b), point2(c, d)))
        .collect::<Vec<_>>();

    let lines = connect_lines(lines, simplify, telemetry, tloc);
    let (additive, subtractive) = separate_polygons(lines);
    (
        additive.into_iter().map(|a| a.path.to_vec()).collect(),
        subtractive.into_iter().map(|a| a.path.to_vec()).collect(),
    )
}

// TODO: replace with impl trait
fn grouping_to_segments<A, I>(iter: I) -> Vec<A>
where
    I: IntoIterator<Item = A>,
    A: Clone,
{
    let mut iter = iter.into_iter();
    let mut out = vec![];
    let first = if let Some(first) = iter.next() {
        first
    } else {
        return out;
    };

    out.push(first.clone());

    for item in iter {
        out.push(item.clone());
        out.push(item);
    }

    out.push(first);

    out
}
