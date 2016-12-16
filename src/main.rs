extern crate ocl;
extern crate implicit_gpu;

use ocl::ProQue;
use implicit_gpu::image::{ColorMode, save_image};
use implicit_gpu::*;

const DIM: usize = 100;
const SIZE: f32 = 20.0;

fn run(program: &str) {
    let proq = ProQue::builder().src(program).dims([DIM, DIM]).build().unwrap();
    let buf = proq.create_buffer::<f32>().unwrap();
    let kernel = proq.create_kernel("apply").unwrap();

    kernel.arg_buf(&buf).arg_scl(DIM).enq().unwrap();
    let mut vec = vec![5.0f32; buf.len()];
    buf.read(&mut vec).enq().unwrap();

    save_image(&vec, DIM, "out.png", ColorMode::Debug);
}

fn main() {
    let scene = circle(50.0, 50.0, 20.0).and(&circle(60.0, 60.0, 20.0));

    let program  = compile(&scene);
    println!("{}", program);
    run(&program);
}
