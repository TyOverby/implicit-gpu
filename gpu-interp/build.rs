use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() -> Result<(), Box<Error>> {
    let mut in_file = File::open("./opcodes.txt")?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;
    let opcodes = in_contents.split("\n");

    let out_env = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_env);
    let dest_path = Path::new(&out_dir);

    let mut rust_file = File::create(&Path::join(dest_path, "opcodes.rs"))?;
    let mut opencl_file = File::create(&Path::join(dest_path, "opcodes.c"))?;

    for (i, op) in opcodes.enumerate() {
        match op.trim().chars().next() {
            None | Some('#') => continue,
            Some(_) => {}
        }
        writeln!(&mut rust_file, "pub const {}:u8 = {};", op.to_uppercase(), i);
        writeln!(&mut opencl_file, "#define OP_{} {}", op.to_uppercase(), i);
    }

    Ok(())
}
