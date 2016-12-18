extern crate ocl;
extern crate implicit_gpu;
extern crate lux;
extern crate flame;

use ocl::Buffer;
use implicit_gpu::image::{ColorMode, save_image};
use implicit_gpu::*;
use implicit_gpu::opencl::OpenClContext;

use lux::prelude::*;

const DIM: usize = 100;

fn run(program: &str, dims: [usize; 2], ctx: &OpenClContext) -> Buffer<f32> {
    ::flame::start("prep");
    let buf = ctx.output_buffer(dims);
    let kernel = ctx.compile("apply", program);
    ::flame::end("prep");

    kernel.gws(dims).arg_buf(&buf).arg_scl(DIM).enq().unwrap();

    /*
    ::flame::start("teardown");
    let mut vec = vec![0.0f32; buf.len()];
    buf.read(&mut vec).enq().unwrap();
    ::flame::start("teardown");

    save_image(&vec, DIM, "out.png", ColorMode::Debug);
    */

    buf
}

fn main() {
    let ctx = OpenClContext::default();

    let scene = circle(50.0, 50.0, 20.0).and(&circle(60.0, 60.0, 20.0));

    let program  = compile(&scene);
    ::flame::start("entire");
        ::flame::start("first program");
        let buff = run(&program, [DIM, DIM], &ctx);
        ::flame::end("first program");

        ::flame::start("marching");
        let lines = implicit_gpu::marching::march(buff, DIM, DIM, true, &ctx);
        ::flame::end("marching");
    ::flame::end("entire");

    ::flame::dump_stdout();

    /*

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
    */
}
