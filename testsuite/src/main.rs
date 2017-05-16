extern crate implicit;
extern crate colored;
extern crate latin;
extern crate implicit_language;
extern crate walkdir;
extern crate flame;
#[macro_use]
extern crate snoot;
#[macro_use]
extern crate serde_derive;

use colored::Colorize;
use implicit::opencl::OpenClContext;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub mod formats;

struct Paths {
    script: PathBuf,

    actual_image: PathBuf,
    actual_values: PathBuf,
    actual_lines: PathBuf,

    expected_values: PathBuf,
    expected_lines: PathBuf,
}

fn run_test(paths: &Paths, ctx: &OpenClContext) -> Result<(), String> {
    let _guard = flame::start_guard(format!("running {:?}", paths.script));
    use implicit::debug::image;

    let script_name = paths.script.to_str().unwrap_or("<unknown source file>");
    let source = latin::file::read_string_utf8(&paths.script).unwrap();
    let tree = implicit_language::parse(&source[..], script_name).unwrap();

    let mut nest = implicit::compiler::Nest::new();
    let target = nest.group(tree.node());
    let evaluator = implicit::evaluator::Evaluator::new(nest, 500, 500, None);

    let result = evaluator.evaluate(target, &ctx);
    let lines = evaluator
        .get_polylines(&result, &ctx)
        .into_iter()
        .map(|((x1, y1), (x2, y2))| formats::lines::Line(x1, y1, x2, y2))
        .collect::<Vec<_>>();
    ctx.empty_queue();

    image::save_field_buffer(&result, &paths.actual_image, image::ColorMode::Debug);
    latin::file::write(&paths.actual_values, formats::field::field_to_text(&result)).unwrap();
    latin::file::write(&paths.actual_lines, formats::lines::lines_to_text(lines.iter().cloned())).unwrap();

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
            ),
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
            ),
        );
    }

    Ok(())
}

fn main() {
    use std::io::{Write, stdout};
    fn ends_with_impl(e: &DirEntry) -> bool { e.path().extension().map(|e| e == "impl").unwrap_or(false) }
    fn clear(size: usize) {
        print!(
            "{}{}{}",
            ::std::iter::repeat(8 as char).take(size).collect::<String>(),
            ::std::iter::repeat(' ').take(size).collect::<String>(),
            ::std::iter::repeat(8 as char).take(size).collect::<String>(),
        );
    }

    let root_dir = ::std::env::current_dir().unwrap();
    let mut test_dir = root_dir.clone();
    test_dir.push("tests");

    let test_files = WalkDir::new(&test_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(ends_with_impl)
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    let max_path_size = test_files
        .iter()
        .map(|p| p.strip_prefix(&test_dir))
        .filter_map(Result::ok)
        .filter_map(|p| p.to_str().map(str::len))
        .max()
        .unwrap_or(0);

    let mut ctx = implicit::opencl::OpenClContext::default();

    let mut any_failures = false;
    for entry in test_files {
        let script = entry;
        let script_name: PathBuf = script.strip_prefix(&test_dir).unwrap().into();

        let paths = Paths {
            script: script,
            actual_image: root_dir.join("actual").join(script_name.with_extension("png")),
            actual_values: root_dir.join("actual").join(script_name.with_extension("values")),
            actual_lines: root_dir.join("actual").join(script_name.with_extension("lines")),

            expected_values: root_dir.join("expected").join(script_name.with_extension("values")),
            expected_lines: root_dir.join("expected").join(script_name.with_extension("lines")),
        };

        let running = "running".yellow();
        print!(
            "{}:{} {}",
            script_name.to_str().unwrap(),
            std::iter::repeat(' ')
                .take(max_path_size - script_name.to_str().unwrap().len())
                .collect::<String>(),
            running,
        );
        stdout().flush().unwrap();
        clear(running.len());

        let old_hook = ::std::panic::take_hook();
        ::std::panic::set_hook(Box::new(|_| ()));
        let result = ::std::panic::catch_unwind(|| run_test(&paths, &ctx)).map_err(|e| e.downcast::<String>());
        ::std::panic::set_hook(old_hook);

        match result {
            Ok(Ok(())) => println!("{}", "OK!".green()),
            Ok(Err(e)) => {
                any_failures = true;
                println!("{}", "ERROR!".red());
                println!("  {}", e.red());
            }
            Err(Ok(panic_string)) => {
                any_failures = true;
                ctx = implicit::opencl::OpenClContext::default();

                println!("{}", "PANIC!".red());
                println!("  {}", panic_string.trim().blue());
            }
            Err(Err(_)) => {
                ctx = implicit::opencl::OpenClContext::default();
            }
        }
    }

    if any_failures {
        std::process::exit(1);
    }
}
