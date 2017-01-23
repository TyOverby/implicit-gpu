#![allow(unused_imports)]
extern crate implicit_gpu;
extern crate itertools;
extern crate latin;

use self::itertools::Itertools;
use self::implicit_gpu::opencl::FieldBuffer;
use self::implicit_gpu::nodes::Node;
use self::implicit_gpu::image::{save_field_buffer, ColorMode};

pub fn run_test(name: &str, node: &Node, width: usize, height: usize) {
    let actual_buf = implicit_gpu::run_single(node, width, height);
    let actual = field_to_text(&actual_buf);

    let expected = match latin::file::read(format!("./tests/{}.txt", name)) {
        Ok(bytes) => String::from_utf8(bytes).unwrap(),
        Err(_) => {
            let actual_text_loc = format!("./target/{}-actual.txt", name);
            latin::file::write(&actual_text_loc, actual).unwrap();
            panic!("could not find expected text, saved to {}", actual_text_loc)
        }
    };

    if expected != actual {
        let actual_img_loc = format!("./target/{}-actual.png", name);
        implicit_gpu::image::save_field_buffer(&actual_buf, &actual_img_loc, ColorMode::Debug);
        panic!("expected field is not the actual field. saved to {}", actual_img_loc);
    }
}

fn field_to_text(buffer: &FieldBuffer) -> String {
    let mut buff = String::new();
    let values = buffer.values();

    for row in values.chunks(buffer.width()) {
        let r = row.iter().map(|e| format!("{}", e)).join(", ");

        buff.push_str(&r);
        buff.push('\n');
    }

    buff
}
