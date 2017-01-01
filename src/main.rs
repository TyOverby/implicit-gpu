#[macro_use]
extern crate implicit_gpu;
extern crate typed_arena;
extern crate ocl;
extern crate flame;

use implicit_gpu::nodes::*;
use implicit_gpu::compiler::*;

use implicit_gpu::image::{ColorMode, save_image};
use implicit_gpu::opencl::OpenClContext;

/*
const DIM: usize = 1000;

fn run(program: &str, dims: [usize; 2], ctx: &OpenClContext) -> Buffer<f32> {
    ::flame::start("prep");
    let buf = ctx.output_buffer(dims);
    ::flame::start("compiling");
    let kernel = ctx.compile("apply", program);
    ::flame::end("compiling");
    ::flame::end("prep");

    kernel.gws(dims).arg_buf(&buf).arg_scl(DIM).enq().unwrap();

    ::flame::start("teardown");
    let mut vec = vec![0.0f32; buf.len()];
    buf.read(&mut vec).enq().unwrap();
    ::flame::end("teardown");

    save_image(&vec, DIM, "out.png", ColorMode::Debug);

    buf
}*/

fn main() {
    let stat = create_node!(a, {
        a(Node::Modulate(-20.0,
            a(Node::And(vec![
                a(Node::Circle{ x: 50.0, y: 50.0, r: 50.0 }),
                a(Node::Not(a(Node::Circle{ x: 100.0, y: 100.0, r: 50.0 }))),
                a(Node::Polygon(PolyGroup::single_additive(vec![0.0, 1.0], vec![0.0, 1.0]))),
                a(Node::Polygon(PolyGroup::single_additive(vec![0.0, 1.0], vec![0.0, 1.0]))),
            ]))
        ))
    });

    let mut nest = Nest::new();
    nest.group(stat.node());
    println!("{:#?}", nest);
}
