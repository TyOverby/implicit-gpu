extern crate buffer_dump;
extern crate gpu_interp;
extern crate minifb;

mod util;

use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::BufReader;
use util::*;

enum Event {
    Up,
    Down,
    None,
}

fn down_event(window: &Window) -> bool {
    window.is_key_pressed(Key::Comma, minifb::KeyRepeat::Yes)
        && (window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift))
}
fn up_event(window: &Window) -> bool {
    window.is_key_pressed(Key::Period, minifb::KeyRepeat::Yes)
        && (window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift))
}

fn get_event(window: &Window) -> Event {
    if down_event(window) {
        Event::Down
    } else if up_event(window) {
        Event::Up
    } else {
        Event::None
    }
}

fn update_draw(buffer: &mut gpu_interp::Buffer, depth: u32) -> Image {
    let mut layer = vec![];
    buffer_dump::util::slice(buffer, depth, &mut layer);
    let mut image = Image::new(buffer.width as usize, buffer.height as usize, 0xFF00FF);
    for (&data, pixel) in layer.iter().zip(image.data.iter_mut()) {
        if data == 0.0 {
            *pixel = 0x0000FF;
        } else if data <= 0.0 {
            *pixel = 0xFF0000;
        } else {
            *pixel = 0x00FF00;
        }
    }
    image
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
    let buffer_depth = buffer.depth;

    let mut image = update_draw(&mut buffer, buffer_depth / 2)
        .pixelize()
        .pixelize()
        .pixelize();

    let mut window = Window::new(
        "Test - ESC to exit",
        image.width,
        image.height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut depth_view = buffer_depth / 2;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let should_redraw = match get_event(&window) {
            Event::Down if depth_view != 0 => {
                depth_view -= 1;
                true
            }
            Event::Up if depth_view < (buffer.depth - 1) => {
                depth_view += 1;
                true
            }
            _ => false,
        };
        if should_redraw {
            image = update_draw(&mut buffer, depth_view)
                .pixelize()
                .pixelize()
                .pixelize();
        }

        let (w, h) = window.get_size();
        let image_to_draw = image.pad(w, h, 0xFFFF00);

        window.update_with_buffer(&image_to_draw.data).unwrap();
    }
}
