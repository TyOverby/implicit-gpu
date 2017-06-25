use super::Paths;
use super::formats;
use {flame, implicit, latin};
use implicit::nodes::NodeRef;
use implicit::opencl::OpenClContext;

pub enum Error {
    CouldNotFind { file: String },
    SvgMismatch { expected: String, actual: String },
    LineMismatch {
        expected: String,
        actual: String,
        message: String,
    },
    FieldMismatch {
        expected: String,
        actual: String,
        message: String,
    },
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Error::CouldNotFind { ref file } => writeln!(formatter, "  • Could not find file {}", file)?,
            Error::SvgMismatch { ref expected, ref actual } => {
                writeln!(formatter, "  • svg files are not the same")?;
                writeln!(formatter, "    expected file : {}", expected)?;
                writeln!(formatter, "    actual file   : {}", actual)?;
            }
            Error::LineMismatch {
                ref expected,
                ref actual,
                ref message,
            } => {
                writeln!(formatter, "  • line files are not the same ({})", message)?;
                writeln!(formatter, "    expected file : {}", expected)?;
                writeln!(formatter, "    actual file   : {}", actual)?;
            }
            Error::FieldMismatch {
                ref expected,
                ref actual,
                ref message,
            } => {
                writeln!(formatter, "  • field files are not the same ({})", message)?;
                writeln!(formatter, "    expected file : {}", expected)?;
                writeln!(formatter, "    actual file   : {}", actual)?;
            }
        }

        Ok(())
    }
}

pub fn run_test(paths: &Paths, ctx: &OpenClContext) -> Result<(), Vec<Error>> {
    let _guard = flame::start_guard(format!("running {:?}", paths.json));
    use implicit::debug::image;

    let source = latin::file::read_string_utf8(&paths.json).unwrap();

    let tree = NodeRef::new(::serde_json::from_str(&source).unwrap());
    /*
    let tree = NodeRef::new(implicit_language::parse(&source[..], script_name).unwrap());
    let as_json = ::serde_json::to_string_pretty(&tree).unwrap();
    latin::file::write(paths.script.with_extension("json"), as_json).unwrap();
    */

    let mut nest = implicit::compiler::Nest::new();
    let target = nest.group(tree.clone());
    let evaluator = implicit::evaluator::Evaluator::new(nest, 500, 500, None);

    let mut errors = vec![];

    let output = implicit::run_scene(&implicit::scene::Scene {
        unit: "px".into(),
        simplify: true,

        figures: vec![
            implicit::scene::Figure {
                shapes: vec![
                    implicit::scene::Shape {
                        color: (0, 0, 0),
                        draw_mode: implicit::scene::DrawMode::Line(implicit::scene::LineMode::Solid),
                        implicit: tree.clone(),
                    },
                ],
            },
        ],
    });

    let result = evaluator.evaluate(target, &ctx);
    let mut lines = evaluator
        .get_polylines(&result, &ctx)
        .into_iter()
        .map(|((x1, y1), (x2, y2))| formats::lines::Line(x1, y1, x2, y2))
        .collect::<Vec<_>>();
    lines.sort();
    ctx.empty_queue();

    image::save_field_buffer(&result, &paths.actual_image, image::ColorMode::Debug);
    implicit::export::svg::write_out(&paths.actual_svg, output).unwrap();
    latin::file::write(&paths.actual_values, formats::field::field_to_text(&result)).unwrap();
    latin::file::write(&paths.actual_lines, formats::lines::lines_to_text(lines.iter().cloned())).unwrap();

    if latin::file::exists(&paths.expected_svg) {
        let expected = latin::file::read(&paths.expected_svg).unwrap();
        let actual = latin::file::read(&paths.actual_svg).unwrap();
        if expected != actual {
            errors.push(Error::SvgMismatch {
                expected: paths.expected_svg.to_str().unwrap().into(),
                actual: paths.actual_svg.to_str().unwrap().into(),
            });
        }
    } else {
        errors.push(Error::CouldNotFind { file: paths.expected_svg.to_str().unwrap().into() });
    }

    if latin::file::exists(&paths.expected_values) {
        if let Err(message) = formats::field::compare(
            &latin::file::read_string_utf8(&paths.expected_values).unwrap(),
            &paths.expected_values.to_str().unwrap(),
            (result.size(), result.values()),
        ) {
            errors.push(Error::FieldMismatch{
                expected: paths.expected_values.to_str().unwrap().into(),
                actual: paths.actual_values.to_str().unwrap().into(),
                message: message
            });
        }
    } else {
        errors.push(Error::CouldNotFind { file: paths.expected_values.to_str().unwrap().into() });
    }

    if latin::file::exists(&paths.expected_lines) {
        if let Err(message) = formats::lines::compare(
            &latin::file::read_string_utf8(&paths.expected_lines).unwrap(),
            &paths.expected_lines.to_str().unwrap(),
            &lines,
        )
        {
            errors.push(Error::LineMismatch {
                expected: paths.expected_lines.to_str().unwrap().into(),
                actual: paths.actual_lines.to_str().unwrap().into(),
                message: message,
            });
        }
    } else {
        errors.push(Error::CouldNotFind { file: paths.expected_lines.to_str().unwrap().into() });
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(())
    }
}
