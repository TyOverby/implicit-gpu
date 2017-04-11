extern crate implicit;
extern crate latin;
extern crate implicit_language;
extern crate walkdir;
extern crate flame;

use std::path::PathBuf;

use walkdir::{WalkDir, DirEntry};

fn run_test(script: PathBuf) {
    let _guard = flame::start_guard(format!("running {:?}", script));
    use implicit::debug::image;
    let mut image = script.clone();
    image.set_extension("png");

    let source = latin::file::read(&script).unwrap();
    let source = String::from_utf8(source).unwrap();

    let script_name = script.to_str().unwrap_or("<unknown>");

    let tree = implicit_language::parse(&source[..], script_name).unwrap();
    println!("{:?}", tree);

    let result = implicit::run_single(tree.node(), 500, 500);

    image::save_field_buffer(&result, image.to_str().unwrap(), image::ColorMode::Debug);

}

fn main() {
    fn ends_with_impl(e: &DirEntry) -> bool {
        e.path()
            .extension()
            .map(|e| e == "impl")
            .unwrap_or(false)
    }

    let iter = WalkDir::new("./")
        .into_iter()
        .filter_map(Result::ok)
        .filter(ends_with_impl)
        .map(|e| e.path().to_path_buf());

    for entry in iter {
        if entry.ends_with("frozen_poly.impl") {
            run_test(entry);
        }
    }
}

