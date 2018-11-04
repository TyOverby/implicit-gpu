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

    let mut buffer_count = 0;
    let mut i_sub = 0;

    for (i, op) in opcodes.enumerate() {
        let i = i - i_sub;
        match op.trim().chars().next() {
            None | Some('#') => {
                i_sub += 1;
                continue;
            }
            Some(_) => {}
        }

        if op.starts_with("buffer_") {
            buffer_count += 1;
        }

        writeln!(
            &mut rust_file,
            "pub const {}:u8 = {};",
            op.to_uppercase(),
            i
        );
        writeln!(&mut opencl_file, "#define OP_{} {}", op.to_uppercase(), i);
    }

    writeln!(
        &mut rust_file,
        "pub const BUFFER_COUNT:usize = {};",
        buffer_count
    );
    {
        write!(&mut opencl_file, "#define INPUT_BUFFERS ");
        let mut args = (0..buffer_count)
            .map(|i| format!("__global float* BUFFER_{}", i))
            .flat_map(|a| vec![a, ",".to_string()])
            .collect::<Vec<_>>();
        args.pop();
        let args = args.concat();
        writeln!(&mut opencl_file, "{}", args);
    }
    {
        write!(&mut opencl_file, "#define IMPLEMENT_INPUT_BUFFERS ");
        for i in 0..buffer_count {
            let pusher = "stack[stack_ptr++] = v;";
            write!(
                &mut opencl_file,
                "case {}: {{float v = BUFFER_{}[pos]; {} break;}}",
                i, i, pusher
            );
        }
        writeln!(&mut opencl_file);
    }

    Ok(())
}
