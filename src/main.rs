extern crate ocl;
extern crate itertools;
extern crate implicit_gpu;
extern crate lux;

use itertools::Itertools;
use ocl::Buffer;
use implicit_gpu::image::{ColorMode, save_image};
use implicit_gpu::*;
use implicit_gpu::opencl::OpenClContext;

use lux::prelude::*;
use lux::graphics::ColorVertex;

const DIM: usize = 1000;

fn run(program: &str, dims: [usize; 2], ctx: &OpenClContext) -> Buffer<f32> {
    let buf = ctx.output_buffer(dims);
    let kernel = ctx.compile("apply", program);

    kernel.gws(dims).arg_buf(&buf).arg_scl(DIM).enq().unwrap();

    let mut vec = vec![0.0f32; buf.len()];
    buf.read(&mut vec).enq().unwrap();

    save_image(&vec, DIM, "out.png", ColorMode::Debug);

    buf
}

fn main() {
    let ctx = OpenClContext::default();

    let scene = circle(50.0, 50.0, 20.0).and(&circle(60.0, 60.0, 20.0));
    let _scene = circle(50.0, 50.0, 0.0);

    let program  = compile(&scene);
    let buff = run(&program, [DIM, DIM], &ctx);

    let lines = implicit_gpu::marching::march(buff, DIM, DIM, &ctx);

    let mut window = Window::new_with_defaults().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame([0.0, 0.0, 0.0]);
        frame.scale(10.0, 10.0);

        frame.color((1.0, 0.0, 0.0));
        for line in &lines {
            for segment in line.windows(2) {
                let (p1x, p1y) = segment[0];
                let (p2x, p2y) = segment[1];
                frame.draw_line(p1x, p1y, p2x, p2y, 1.0);
            }

            let (p1x, p1y) = line[0];
            let (p2x, p2y) = line[line.len() - 1];
            frame.draw_line(p1x, p1y, p2x, p2y, 1.0);
        }
    }
}
