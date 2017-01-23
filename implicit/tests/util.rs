#![allow(unused_imports)]
extern crate implicit;
extern crate itertools;
extern crate latin;

use self::itertools::Itertools;
use self::implicit::opencl::FieldBuffer;
use self::implicit::nodes::Node;
use self::implicit::debug::image::{save_field_buffer, ColorMode};

pub fn run_test(name: &str, node: &Node, width: usize, height: usize) {
    let actual_img_loc = format!("./target/{}-actual.png", name);
    let actual_text_loc = format!("./target/{}-actual.txt", name);
    let actual_frozen_loc = format!("./target/{}-frozen.png", name);

    let actual_buf = implicit::run_single(node, width, height);
    {
        let frozen = implicit::run_single(&Node::Freeze(node), width, height);
        if !pretty_close(&actual_buf, &frozen) {
            save_field_buffer(&actual_buf, &actual_img_loc, ColorMode::Debug);
            save_field_buffer(&frozen, &actual_frozen_loc, ColorMode::Debug);
            panic!("frozen copy not similar \n open {} {}",
                   actual_img_loc, actual_frozen_loc);
        }
    }

    let actual = field_to_text(&actual_buf);

    let expected = match latin::file::read(format!("./tests/{}.txt", name)) {
        Ok(bytes) => String::from_utf8(bytes).unwrap(),
        Err(_) => {
            latin::file::write(&actual_text_loc, actual).unwrap();
            panic!("could not find expected text, saved to {}", actual_text_loc)
        }
    };

    if expected != actual {
        save_field_buffer(&actual_buf, &actual_img_loc, ColorMode::Debug);
        panic!("expected field is not the actual field. saved to {}", actual_img_loc);
    }
}

fn pretty_close(a: &FieldBuffer, b: &FieldBuffer) -> bool {
    let av = a.values();
    let bv = b.values();
    let width = a.width();

    assert_eq!(av.len(), bv.len());
    assert_eq!(a.width(), b.width());
    assert_eq!(a.height(), b.height());

    for (i, (a, b)) in av.into_iter().zip(bv.into_iter()).enumerate() {
        let asig = a.signum();
        let bsig = b.signum();

        if (asig != bsig) &&
            // if a or b is 0, then any error would throw this off.
            !(a == 0.0 || b == 0.0) &&
            // a could be +0.0 and b could be -0.0 so signum would lie.
            !(a == b)
        {
            return false
        }
    }

    return true
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
