extern crate buffer_dump;
extern crate gpu_interp;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::BufReader;

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

fn update_draw(buffer: &mut gpu_interp::Buffer, draw: &mut Vec<u32>, depth: u32) {
    let mut layer = vec![];
    buffer_dump::util::slice(buffer, depth, &mut layer);
    for (&data, pixel) in layer.iter().zip(draw.iter_mut()) {
        if data == 0.0 {
            *pixel = 0x0000FF;
        } else if data <= 0.0 {
            *pixel = 0xFF0000;
        } else {
            *pixel = 0x00FF00;
        }
    }
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

    let mut layer = vec![];
    buffer_dump::util::slice(&mut buffer, 0, &mut layer);
    let mut draw = layer.iter().map(|_| 0x000000).collect::<Vec<_>>();
    update_draw(&mut buffer, &mut draw, buffer_depth / 2);

    let mut window = Window::new(
        "Test - ESC to exit",
        buffer.width as usize,
        buffer.height as usize,
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
            update_draw(&mut buffer, &mut draw, depth_view)
        }
        window.update_with_buffer(&draw).unwrap();
    }
}
