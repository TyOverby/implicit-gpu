extern crate implicit;
extern crate latin;
extern crate implicit_language;
extern crate walkdir;
extern crate flame;
#[macro_use]
extern crate snoot;
#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;
use walkdir::{WalkDir, DirEntry};
use implicit::opencl::OpenClContext;

pub mod formats;

struct Paths {
    script: PathBuf,

    actual_image: PathBuf,
    actual_values: PathBuf,
    actual_lines: PathBuf,

    _expected_image: PathBuf,
    _expected_values: PathBuf,
    _expected_lines: PathBuf,
}

fn run_test(paths: &Paths, ctx: &OpenClContext) {
    let _guard = flame::start_guard(format!("running {:?}", paths.script));
    use implicit::debug::image;

    let source = latin::file::read(&paths.script).unwrap();
    let source = String::from_utf8(source).unwrap();

    let script_name = paths.script.to_str().unwrap_or("<unknown source file>");
    let tree = implicit_language::parse(&source[..], script_name).unwrap();
    println!("{:?}", tree);

    let mut nest = implicit::compiler::Nest::new();
    let target = nest.group(tree.node());
    let evaluator = implicit::evaluator::Evaluator::new(nest, 500, 500, None);
    let result = evaluator.evaluate(target, &ctx);
    let lines = evaluator.get_polylines(&result, &ctx).into_iter().map(|((x1, y1), (x2, y2))| formats::lines::Line(x1, y1, x2, y2));
    ctx.empty_queue();

    image::save_field_buffer(&result, &paths.actual_image, image::ColorMode::Debug);
    latin::file::write(&paths.actual_values, formats::field::field_to_text(&result)).unwrap();
    latin::file::write(&paths.actual_lines, formats::lines::lines_to_text(lines)).unwrap();
}

fn main() {
    fn ends_with_impl(e: &DirEntry) -> bool {
        e.path()
            .extension()
            .map(|e| e == "impl")
            .unwrap_or(false)
    }

    let root_dir = ::std::env::current_dir().unwrap();
    let mut test_dir = root_dir.clone();
    test_dir.push("tests");

    let iter = WalkDir::new(&test_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(ends_with_impl)
        .map(|e| e.path().to_path_buf());

    let ctx = implicit::opencl::OpenClContext::default();

    for entry in iter {
        let script = entry;
        let script_name: PathBuf = script.strip_prefix(&test_dir).unwrap().into();

        let paths = Paths {
            script: script,
            actual_image: root_dir.join("actual").join(script_name.with_extension("png")),
            actual_values: root_dir.join("actual").join(script_name.with_extension("values")),
            actual_lines: root_dir.join("actual").join(script_name.with_extension("lines")),

            _expected_image: root_dir.join("expected").join(script_name.with_extension("png")),
            _expected_values: root_dir.join("expected").join(script_name.with_extension("values")),
            _expected_lines: root_dir.join("expected").join(script_name.with_extension("lines")),
        };

        if !paths.script.ends_with("frozen_poly.impl") {
            run_test(&paths, &ctx);
        }
    }
}

