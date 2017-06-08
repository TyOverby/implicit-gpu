extern crate rand;
extern crate lazy_static;
extern crate latin;
extern crate typed_arena;
extern crate vecmath;
extern crate vectordraw;
extern crate ocl;
extern crate flame;
extern crate fnv;
extern crate itertools;
extern crate image as image_crate;

#[macro_export]
macro_rules! create_node {
    ($alloc: ident, $code: block) => {
        {
            let anchor = $crate::nodes::Anchor::new();
            let arena = ::typed_arena::Arena::new();
            let result: &'static $crate::nodes::Node<'static> = {
                let $alloc = |a| {
                    let r: &'static $crate::nodes::Node<'static> = unsafe {
                        ::std::mem::transmute(arena.alloc(a))
                    };

                    anchor.hold(r)
                };

                let result = $code;

                unsafe { ::std::mem::transmute(result) }
            };

            unsafe {
                $crate::nodes::StaticNode::new(arena, result)
            }
        }
    };
}

pub mod nodes;
pub mod compiler;
pub mod opencl;
pub mod debug;
pub mod polygon;
pub mod marching;
pub mod evaluator;
pub mod nan_filter;
pub mod export;
pub mod scene;
pub mod output;
pub mod lines;

pub fn run_single(node: &nodes::Node, width: usize, height: usize) -> ::opencl::FieldBuffer {
    use compiler::Nest;
    use evaluator::Evaluator;

    let ctx = opencl::OpenClContext::default();

    let mut nest = Nest::new();
    let target = nest.group(node);

    // Create a new Execution Context
    let evaluator = Evaluator::new(nest, width, height, None);
    let result = evaluator.evaluate(target, &ctx);
    ctx.empty_queue();
    result
}

pub fn run_scene(scene: &scene::Scene) -> output::OutputScene {
    use output::*;
    use scene::*;
    use itertools::Itertools;
    use compiler::Nest;
    use evaluator::Evaluator;

    if scene.x != 0 || scene.y != 0{
        panic!("scenes not centered at the origin are unsupported");
    }

    // Setup
    let ctx = opencl::OpenClContext::default();
    let mut nest = Nest::new();

    let mut output = OutputScene {
        figures: vec![],
    };

    // Build all the shapes first, then collect them later.
    // This will allow for further optimization in the future.
    let mut treemap = ::std::collections::BTreeMap::new();
    for figure in &scene.figures {
        for shape in &figure.shapes {
            let id = nest.group(shape.node.node());
            treemap.insert(shape, id);
        }
    }

    let evaluator = Evaluator::new(nest, scene.width as usize, scene.height as usize, None);

    for figure in &scene.figures {
        let mut output_shapes = vec![];
        for shape in &figure.shapes {
            let id = treemap.get(&shape).unwrap();
            let result = evaluator.evaluate(*id, &ctx);
            let (pts_xs, pts_ys) = ::marching::run_marching(&result, &ctx);

            let pts_xs = pts_xs.values().into_iter();
            let pts_ys = pts_ys.values().into_iter();
            let pts = pts_xs.zip(pts_ys);
            let pts = pts.filter(|&(x, y)| !(x.is_nan() || y.is_nan()));
            let pts = pts.tuples::<(_, _)>();

            let (lines, _) = lines::connect_lines(pts, scene.simplify);
            let (additive, subtractive) = lines::separate_polygons(lines);
            let output_shape = match shape.draw_mode {
                DrawMode::Filled => OutputShape {
                    color: shape.color,
                    lines: LineGroup::Polygon {
                        filled: true,
                        additive, subtractive
                    }
                },
                DrawMode::Line(LineMode::Solid) => OutputShape {
                    color: shape.color,
                    lines: LineGroup::Polygon {
                        filled: false,
                        additive: additive,
                        subtractive: subtractive,
                    }
                }
            };

            output_shapes.push(output_shape);

            ctx.empty_queue();
        }

        output.figures.push(OutputFigure{ shapes: output_shapes});
    }

    output
}
