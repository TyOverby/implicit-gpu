use super::Paths;
use super::formats;
use {flame, implicit, implicit_language, latin};
use implicit::opencl::OpenClContext;

pub fn run_test(paths: &Paths, ctx: &OpenClContext) -> Result<(), String> {
    let _guard = flame::start_guard(format!("running {:?}", paths.script));
    use implicit::debug::image;

    let script_name = paths.script.to_str().unwrap_or("<unknown source file>");
    let source = latin::file::read_string_utf8(&paths.script).unwrap();
    let tree = implicit_language::parse(&source[..], script_name).unwrap();

    let mut nest = implicit::compiler::Nest::new();
    let target = nest.group(tree.node());
    let evaluator = implicit::evaluator::Evaluator::new(nest, 500, 500, None);

    let output = implicit::run_scene(&implicit::scene::Scene{
        x: 0,
        y: 0,
        width: 500,
        height: 500,

        unit: "px".into(),
        simplify: false,

        figures: vec![
            implicit::scene::Figure {
                shapes: vec![implicit::scene::Shape {
                    color: (0, 0, 0),
                    draw_mode: implicit::scene::DrawMode::Line(implicit::scene::LineMode::Solid),
                    node: tree,
                }]
            }
        ],
    });

    let result = evaluator.evaluate(target, &ctx);
    let lines = evaluator
        .get_polylines(&result, &ctx)
        .into_iter()
        .map(|((x1, y1), (x2, y2))| formats::lines::Line(x1, y1, x2, y2))
        .collect::<Vec<_>>();
    ctx.empty_queue();

    image::save_field_buffer(&result, &paths.actual_image, image::ColorMode::Debug);
    implicit::export::svg::write_out(&paths.actual_svg, output);
    latin::file::write(&paths.actual_values, formats::field::field_to_text(&result)).unwrap();
    latin::file::write(&paths.actual_lines, formats::lines::lines_to_text(lines.iter().cloned())).unwrap();

    if latin::file::exists(&paths.expected_svg) {
        let expected = latin::file::read(&paths.expected_svg).unwrap();
        let actual = latin::file::read(&paths.actual_svg).unwrap();
        if expected != actual {
            return Err(format!("svg files not the same\n  {}\n  {}",
                paths.expected_svg.to_str().unwrap(),
                paths.actual_svg.to_str().unwrap(),
            ));
        }
    } else {
        return Err(
            format!("could not find expected svg file at {}", paths.expected_svg.to_str().unwrap()))
    }

    if latin::file::exists(&paths.expected_values) {
        formats::field::compare(
            &latin::file::read_string_utf8(&paths.expected_values).unwrap(),
            &paths.expected_values.to_str().unwrap(),
            (result.size(), result.values()),
        )?;
    } else {
        return Err(
            format!(
                "could not find expected values file at {}",
                paths.expected_values.to_str().unwrap(),
            )
        );
    }

    if latin::file::exists(&paths.expected_lines) {
        formats::lines::compare(
            &latin::file::read_string_utf8(&paths.expected_lines).unwrap(),
            &paths.expected_lines.to_str().unwrap(),
            &lines,
        )?;
    } else {
        return Err(
            format!(
                "could not find expected lines file at {}",
                paths.expected_lines.to_str().unwrap(),
            )
        );
    }

    Ok(())
}
