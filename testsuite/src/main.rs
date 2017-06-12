extern crate implicit;
extern crate regex;
extern crate colored;
extern crate latin;
extern crate walkdir;
extern crate flame;
#[macro_use]
extern crate snoot;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use colored::Colorize;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub mod formats;
mod run_test;

pub struct Paths {
    json: PathBuf,

    actual_image: PathBuf,
    actual_values: PathBuf,
    actual_lines: PathBuf,
    actual_svg: PathBuf,

    expected_values: PathBuf,
    expected_lines: PathBuf,
    expected_svg: PathBuf,
}


fn main() {
    use std::io::{Write, stdout};
    fn ends_with_json(e: &DirEntry) -> bool { e.path().extension().map(|e| e == "json").unwrap_or(false) }
    fn clear(size: usize) {
        print!(
            "{}{}{}",
            ::std::iter::repeat(8 as char).take(size).collect::<String>(),
            ::std::iter::repeat(' ').take(size).collect::<String>(),
            ::std::iter::repeat(8 as char).take(size).collect::<String>(),
        );
    }

    let args = std::env::args().collect::<Vec<_>>();
    let test_matcher = if args.len() == 1 {
        ::regex::RegexSet::new(&["."]).unwrap()
    } else {
        match ::regex::RegexSet::new(&args) {
            Ok(set) => set,
            Err(e) => {
                println!("{:?}", e);
                ::std::process::exit(2);
            }
        }
    };

    let root_dir = ::std::env::current_dir().unwrap();
    let mut test_dir = root_dir.clone();
    test_dir.push("tests");

    // Walk the tests directory
    let test_files = WalkDir::new(&test_dir)
        .into_iter()
        // Taking only the ones that actually have paths
        .filter_map(Result::ok)
        // With a filename that ends in ".impl"
        .filter(ends_with_json)
        // Converted to a PathBuf
        .map(|e| e.path().to_path_buf())
        // Where the path can be converted to a string
        .filter(|p| p.to_str().is_some())
        // And the path is accepted by the matcher
        .filter(|p| test_matcher.is_match(p.to_str().unwrap()))
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
        let json = entry;
        let script_name: PathBuf = json.strip_prefix(&test_dir).unwrap().into();

        let paths = Paths {
            json,
            actual_image: root_dir.join("actual").join(script_name.with_extension("png")),
            actual_svg: root_dir.join("actual").join(script_name.with_extension("svg")),
            actual_values: root_dir.join("actual").join(script_name.with_extension("values")),
            actual_lines: root_dir.join("actual").join(script_name.with_extension("lines")),

            expected_values: root_dir.join("expected").join(script_name.with_extension("values")),
            expected_lines: root_dir.join("expected").join(script_name.with_extension("lines")),
            expected_svg: root_dir.join("expected").join(script_name.with_extension("svg")),
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
        let result =
            ::std::panic::catch_unwind(|| run_test::run_test(&paths, &ctx))
            .map_err(|e| e.downcast::<String>())
            .map_err(|e| e.or_else(|e| e.downcast::<&'static str>().map(|s| Box::new(s.to_string()))));
        ::std::panic::set_hook(old_hook);

        match result {
            Ok(Ok(())) => println!("{}", "OK!".green()),
            Ok(Err(errors)) => {
                any_failures = true;
                println!("{}", "ERROR!".red());
                for  e in errors {
                    print!("{}", e.to_string().red());
                }
           }
            Err(Ok(panic_string)) => {
                any_failures = true;
                ctx = implicit::opencl::OpenClContext::default();

                println!("{}", "PANIC!".red());
                println!("  {}", panic_string.trim().blue());
            }
            Err(Err(_)) => {
                any_failures = true;
                ctx = implicit::opencl::OpenClContext::default();

                println!("{}", "PANIC!".red());
            }
        }
    }

    if any_failures {
        std::process::exit(1);
    }
}
