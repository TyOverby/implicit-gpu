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

    let out = implicit_gpu::marching::run_marching(buff, DIM, DIM, &ctx);

    let mut out_vec = vec![::std::f32::NAN; out.len()];
    out.read(&mut out_vec).enq().unwrap();

    println!("nan {}", out_vec.iter().filter(|x| x.is_nan()).count());
    println!("zero {}", out_vec.iter().filter(|&&x| x == 0.0).count());
    println!("positive {}", out_vec.iter().filter(|&&x| x > 0.0).count());
    println!("negative {}", out_vec.iter().filter(|&&x| x < 0.0).count());

    let lines = out_vec.into_iter().tuples().enumerate().filter(|&(_, (a, b, c, d))|
        !(a.is_nan() && b.is_nan() && c.is_nan() && d.is_nan())
    ).collect::<Vec<_>>();

    println!("{:?}", lines);

    let pixels = lines.iter().cloned().flat_map(|(_, (x1, y1, x2, y2))| {
        vec![
            ColorVertex {
                pos: [x1, y1],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            ColorVertex {
                pos: [x2, y2],
                color: [1.0, 0.0, 1.0, 1.0],
            },
        ]
    }).collect::<Vec<_>>();

    let mut window = Window::new_with_defaults().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame([0.0, 0.0, 0.0]);
        frame.scale(10.0, 10.0);


        /*
        frame.draw(Pixels {
            pixels: &pixels,
            ..Default::default()
        }).unwrap();
        */

        frame.color((1.0, 0.0, 0.0));
        for &(_, (x1, y1, x2, y2)) in lines.iter() {
            /*
            frame.rect(x1, y1, 0.2, 0.2).color((0.0, 1.0, 0.0)).fill();
            frame.rect(x2, y2, 0.2, 0.2).color((1.0, 0.0, 0.0)).fill();
            */
            frame.draw_line(x1, y1, x2, y2, 1.0);
            /*
            frame.draw(Line {
                start: (x1, y1),
                end: (x2, y2),
                color: [1.0, 0.0, 0.0, 1.0],
                ..Default::default()
            }).unwrap();*/
        }
        //frame.draw_points(&pixels);
    }

    /*
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let r = 20.0;
    let pi = 3.14159;
    let n = 100;

    for x in 0 .. n {
        let x = x as f32;
        let n = n as f32;
        xs.push(f32::cos(2.0 * pi / n * x) * r + 50.0);
        ys.push(f32::sin(2.0 * pi / n * x) * (r * 1.5) + 50.0);
    }

    let buffer = implicit_gpu::polygon::run_poly(&xs, &ys, DIM, DIM);

    let mut vec = vec![0.0f32; buffer.len()];
    buffer.read(&mut vec).enq().unwrap();

    save_image(&vec, DIM, "out-poly.png", ColorMode::Debug);
    */
}
