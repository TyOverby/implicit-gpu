extern crate buffer_dump;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::BufReader;

fn down_event(window: &Window) -> bool {
    window.is_key_pressed(Key::Comma, minifb::KeyRepeat::No)
        && (window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift))
}
fn up_event(window: &Window) -> bool {
    window.is_key_pressed(Key::Period, minifb::KeyRepeat::No)
        && (window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift))
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let filename = match args.next() {
        Some(f) => f,
        None => {
            println!("this program accepts a file name parameter");
            std::process::exit(1)
        }
    };
    let file = File::open(filename).unwrap();
    let mut file = BufReader::new(file);
    let mut buffer = buffer_dump::read(&mut file).unwrap();

    let mut layer = vec![];
    buffer_dump::util::slice(&mut buffer, 0, &mut layer);

    let mut draw = layer.iter().map(|_| 0xFF0000).collect::<Vec<_>>();

    let mut window = Window::new(
        "Test - ESC to exit",
        buffer.width as usize,
        buffer.height as usize,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Comma, minifb::KeyRepeat::No)
            && (window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift))
        {
            println!("hi");
        }

        for (k, p) in draw.iter_mut().enumerate() {
            if k % 2 == 0 {
                *p = 0x000000;
            } else {
                *p = 0xFF0000;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&draw).unwrap();
    }
}
