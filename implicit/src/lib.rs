extern crate flame;
extern crate fnv;
extern crate image as image_crate;
extern crate itertools;
extern crate latin;
extern crate lazy_static;
extern crate ocl;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate serde_json;
extern crate typed_arena;
extern crate vecmath;
extern crate vectorphile;

pub mod nodes;
pub mod compiler;
pub mod opencl;
pub mod debug;
pub mod polygon;
pub mod marching;
pub mod evaluator;
pub mod export;
pub mod scene;
pub mod output;
pub mod lines;
pub mod telemetry;

use lines::util::geom::Rect;

pub fn run_single(node: nodes::NodeRef, width: usize, height: usize) -> ::opencl::FieldBuffer {
    use compiler::Nest;
    use evaluator::Evaluator;

    let ctx = opencl::OpenClContext::default();

    let mut nest = Nest::new();
    let target = nest.group(node);

    // Create a new Execution Context
    let evaluator = Evaluator::new(nest, width, height, None);
    let tloc = telemetry::TelemetryLocation::new();
    let result = evaluator.evaluate(target, &ctx, &mut telemetry::NullTelemetry, tloc);
    ctx.empty_queue();
    result
}

pub fn run_scene(mut scene: scene::Scene, telemetry: &mut telemetry::Telemetry) -> output::OutputScene {
    use compiler::Nest;
    use evaluator::Evaluator;
    use output::*;
    use scene::*;

    fn compute_figure_size(figure: &scene::Figure) -> Option<lines::util::geom::Rect> {
        let mut rect: Option<Option<Rect>> = None;
        for shape in &figure.shapes {
            rect = match (rect, shape.implicit.bounding_box()) {
                (None, (a, _)) => Some(a),
                (Some(None), _) | (_, (None, _)) => Some(None),
                (Some(Some(a)), (Some(b), Some(c))) => Some(Some(a.union_with(&b.union_with(&c)))),
                (Some(Some(a)), (Some(b), None)) => Some(Some(a.union_with(&b))),
            }
        }

        rect.and_then(|a| a)
    }
    fn compute_scene_size(scene: &scene::Scene) -> Option<lines::util::geom::Rect> {
        let mut rect: Option<Option<Rect>> = None;

        for figure in &scene.figures {
            rect = match (rect, compute_figure_size(figure)) {
                (None, a) => Some(a),
                (Some(None), _) | (_, None) => Some(None),
                (Some(Some(a)), Some(b)) => Some(Some(a.union_with(&b))),
            }
        }

        rect.and_then(|a| a)
    }
    telemetry.scene_started();
    // Setup
    let ctx = opencl::OpenClContext::default();
    let mut nest = Nest::new();

    let mut output = OutputScene { figures: vec![] };

    // Build all the shapes first, then collect them later.
    // This will allow for further optimization in the future.
    for figure in &mut scene.figures {
        let figure_bounds = if let Some(figure) = compute_figure_size(figure) {
            figure
        } else {
            continue;
        };

        for shape in &mut figure.shapes {
            *(&mut shape.implicit) = nodes::NodeRef::new(nodes::Node::Translate {
                dx: -figure_bounds.left() + 2f32,
                dy: -figure_bounds.top() + 2f32,
                target: shape.implicit.clone(),
            });
        }
    }
    let mut treemap = ::std::collections::BTreeMap::new();
    for figure in &scene.figures {
        for shape in &figure.shapes {
            let id = nest.group(shape.implicit.clone());
            treemap.insert(shape, id);
        }
    }

    let bb = match compute_scene_size(&scene) {
        Some(bb) => bb,
        None => return OutputScene { figures: vec![] },
    };

    let mut tloc = telemetry::TelemetryLocation::new();

    telemetry.scene_bounding_box(tloc, bb.top_left.x, bb.top_left.y, bb.width(), bb.height());
    let evaluator = Evaluator::new(nest, bb.width().ceil() as usize + 4, bb.height().ceil() as usize + 4, None);

    for figure in &scene.figures {
        tloc.new_figure();
        let mut figure_telemetry = tloc.clone();

        let mut output_shapes = vec![];
        for shape in &figure.shapes {
            figure_telemetry.new_shape();
            let shape_telemetry = figure_telemetry.clone();

            let id = treemap.get(&shape).unwrap();
            let result = evaluator.evaluate(*id, &ctx, telemetry, shape_telemetry);

            let line_buffer = ::marching::run_marching(&result, &ctx);
            let (additive, subtractive) = evaluator::line_buffer_to_poly(&line_buffer, telemetry, tloc, scene.simplify);

            let output_shape = match shape.draw_mode {
                DrawMode::Filled => OutputShape {
                    color: shape.color,
                    lines: LineGroup::Polygon {
                        filled: true,
                        additive,
                        subtractive,
                    },
                },
                DrawMode::Line(LineMode::Solid) => OutputShape {
                    color: shape.color,
                    lines: LineGroup::Polygon {
                        filled: false,
                        additive: additive,
                        subtractive: subtractive,
                    },
                },
            };

            output_shapes.push(output_shape);

            ctx.empty_queue();
        }

        telemetry.figure_finished(figure_telemetry, &output_shapes);
        let fbb = compute_figure_size(figure).unwrap();
        output.figures.push(OutputFigure {
            shapes: output_shapes,
            left: fbb.left() as f32,
            top: fbb.top() as f32,
            width: fbb.width() as f32,
            height: fbb.height() as f32,
        });
    }
    telemetry.scene_finished(tloc, &output);
    output
}
