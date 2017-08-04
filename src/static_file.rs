use mime::Mime;
use mime_guess::guess_mime_type;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, BufReader};

pub fn serve_statically<'a, I>(roots: I, mut path: &str) -> Option<(Mime, Vec<u8>)> 
where I: Iterator<Item=&'a str>{
    if path.starts_with("./") {
        path = &path[2..];
    }

    if path.contains("../") || path.contains("./") {
        return None;
    }

    // Remove trailing slashes so that the path isn't considered 
    // "absolute"  This way PathBuf::push doesn't overwrite the 
    // whole path.
    while path.starts_with("/") {
        path = &path[1..];
    }

    for search_dir in roots {
        let mut local_path = PathBuf::new();
        local_path.push(search_dir);
        local_path.push(path);

        if local_path.exists() && local_path.is_file() {
            let mime = guess_mime_type(&local_path);
            let file = match File::open(local_path) {
                Ok(f) => f,
                Err(_) => continue,
            };

            let mut file = BufReader::new(file);
            let mut out = Vec::new();
            if let Err(_) = file.read_to_end(&mut out) {
                continue;
            }

            return Some((mime, out));
        }
    }

    return None; 
}
