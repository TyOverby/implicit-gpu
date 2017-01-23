extern crate latin;
extern crate implicit;
extern crate lux;

use std::str::FromStr;
use lux::prelude::*;
use lux::color;
use std::io::Write;

fn main() {
    let mut args = std::env::args();
    let name = args.nth(1).unwrap();
    let lines = read_text(&name);
    let mut window = Window::new_with_defaults().unwrap();

    let mut last_down = false;
    let (mut last_x, mut last_y) = (0.0, 0.0);
    let (mut dx, mut dy) = (0.0, 0.0);
    let io = ::std::io::stdout();
    let mut io = io.lock();

    while window.is_open() {
        let mut frame = window.cleared_frame(color::WHITE);

        for event in window.events() {
            if let lux::interactive::Event::MouseMoved((mx, my)) = event {
                let mx = mx as f32;
                let my = my as f32;

                write!(io, "\r{}, {}            ", mx - dx, my - dy).unwrap();
                io.flush().unwrap();

                 match (last_down, window.is_mouse_down()) {
                    (true, true) => {
                        dx += mx - last_x;
                        dy += my - last_y;
                        last_x = mx;
                        last_y = my;
                    }
                    (false, true) => {
                        last_x = mx;
                        last_y = my;
                    }
                    _ => {}
                }

                last_down = window.is_mouse_down();
            }
        }

        frame.translate(dx, dy);
        for &((x1, y1), (x2, y2)) in &lines {
            frame.draw_line(x1, y1, x2, y2, 2.0);
        }
    }
}

fn read_text(name: &str) -> Vec<((f32, f32), (f32, f32))> {
    latin::file::read_lines(name).unwrap().map(|line| {
        let line = line.unwrap();
        let mut chunks =
            line.split(", ")
            .map(f32::from_str)
            .map(|r| r.expect("could not parse float"));
        let x1 = chunks.next().unwrap();
        let y1 = chunks.next().unwrap();
        let x2 = chunks.next().unwrap();
        let y2 = chunks.next().unwrap();
        ((x1, y1), (x2, y2))
    }).collect()
}
