#[cfg(test)]
use expectation::{extensions::*, Provider};
#[cfg(test)]
use ocaml::Shape;

use expectation_plugin::expectation_test;
use opencl::{FieldBuffer, OpenClContext};

const PROGRAM: &'static str = include_str!("../shaders/drag.c");

pub fn exec_drag(ctx: &OpenClContext, input: &FieldBuffer, dx: f32, dy: f32) -> FieldBuffer {
    let out = ctx.field_buffer(input.width(), input.height(), None);
    let mut kernel = ctx.compile("apply", PROGRAM, |register| {
        register.buffer("buffer");
        register.buffer("input");
        register.float("dx");
        register.float("dy");
        register.long("width");
        register.long("height");
    });

    kernel.set_default_global_work_size(::ocl::SpatialDims::Two(input.width(), input.height()));
    kernel.set_arg("buffer", out.buffer()).unwrap();
    kernel.set_arg("input", input.buffer()).unwrap();
    kernel.set_arg("dx", dx).unwrap();
    kernel.set_arg("dy", dy).unwrap();
    kernel.set_arg("width", input.width() as u64).unwrap();
    kernel.set_arg("height", input.height() as u64).unwrap();

    unsafe {
        kernel.enq().unwrap();
    }
    out
}

#[cfg(test)]
fn drag_shape_helper(
    shape: Shape,
    width: usize,
    height: usize,
    dx: f32,
    dy: f32,
    provider: Provider,
) {
    use debug::*;
    use exec::exec_shape;

    let ctx = OpenClContext::default();

    let before_buffer = exec_shape(&ctx, shape, width, height, |_| unimplemented!());
    let after_buffer = exec_drag(&ctx, &before_buffer, dx, dy);

    let w_color = provider.png_writer("before.color.png");
    save_field_buffer(&before_buffer, w_color, ColorMode::Debug);
    let w_bw = provider.png_writer("before.bw.png");
    save_field_buffer(&before_buffer, w_bw, ColorMode::BlackAndWhite);

    let w_color = provider.png_writer("after.color.png");
    save_field_buffer(&after_buffer, w_color, ColorMode::Debug);
    let w_bw = provider.png_writer("after.bw.png");
    save_field_buffer(&after_buffer, w_bw, ColorMode::BlackAndWhite);
}

#[expectation_test]
fn drag_circle(provider: Provider) {
    use ocaml::*;

    let shape = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));

    drag_shape_helper(shape, 30, 30, 5.0, 7.0, provider);
}
