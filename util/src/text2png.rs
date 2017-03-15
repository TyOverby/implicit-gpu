extern crate latin;
extern crate implicit;

use std::str::FromStr;
use implicit::debug::image::*;

fn main() {
    let mut args = std::env::args();
    let name = args.nth(1).unwrap();
    let floats = read_text(&name);
    write_image(&name, floats);
}

fn read_text(name: &str) -> Vec<Vec<f32>> {
    latin::file::read_lines(name).unwrap().map(|line| {
        let line = line.unwrap();
        let chunks = line.split(", ");
        chunks.map(|chunk| {
            f32::from_str(chunk).unwrap()
        }).collect()
    }).collect()
}

fn write_image(name: &str, floats: Vec<Vec<f32>>) {
    let out_name = format!("{}.png", name);
    let width = floats[0].len();
    let buff = floats.into_iter().flat_map(|line| line.into_iter()).collect::<Vec<_>>();

    save_image(&buff, width, &out_name, ColorMode::BlackAndWhite);
}
