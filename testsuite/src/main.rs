extern crate implicit;
extern crate implicit_language;
extern crate walkdir;

use walkdir::WalkDir;

fn main() {
    let iter = WalkDir::new("./")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map(|e| e == "impl").unwrap_or(false));

    for entry in iter {
        println!("{:?}", entry);
    }
}
