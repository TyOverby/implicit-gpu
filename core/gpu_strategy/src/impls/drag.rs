#[cfg(test)]
use expectation::{extensions::*, Provider};
#[cfg(test)]
use extern_api::Shape;

use crate::opencl::{FieldBuffer, OpenClContext};
use expectation_plugin::expectation_test;

const PROGRAM: &'static str = include_str!("../shaders/drag.c");

pub fn exec_drag(ctx: &OpenClContext, input: &mut FieldBuffer, dx: f32, dy: f32) -> FieldBuffer {
    let mut out = ctx.field_buffer(input.width, input.height, 1, None);
    let mut kernel = ctx.compile("apply", PROGRAM, |register| {
        register.buffer("buffer");
        register.buffer("input");
        register.float("dx");
        register.float("dy");
        register.long("width");
        register.long("height");
    });

    kernel.set_default_global_work_size(::ocl::SpatialDims::Two(
        input.width as usize,
        input.height as usize,
    ));
    kernel
        .set_arg("buffer", out.to_opencl(ctx.queue()))
        .unwrap();
    kernel
        .set_arg("input", input.to_opencl(ctx.queue()))
        .unwrap();
    kernel.set_arg("dx", dx).unwrap();
    kernel.set_arg("dy", dy).unwrap();
    kernel.set_arg("width", input.width as u64).unwrap();
    kernel.set_arg("height", input.height as u64).unwrap();

    unsafe {
        kernel.enq().unwrap();
    }
    out
}

#[cfg(test)]
fn drag_shape_helper(shape: Shape, width: u32, height: u32, dx: f32, dy: f32, provider: Provider) {
    use crate::impls::exec_shape;
    use debug_helpers::*;

    let ctx = OpenClContext::default();

    let mut before_buffer = exec_shape(
        &ctx,
        provider.duplicate(),
        shape,
        width,
        height,
        |_| unimplemented!(),
    );
    let mut after_buffer = exec_drag(&ctx, &mut before_buffer, dx, dy);

    let w_color = provider.png_writer("before.color.png");
    save_field_buffer(&mut before_buffer, w_color, ColorMode::Debug);
    let w_bw = provider.png_writer("before.bw.png");
    save_field_buffer(&mut before_buffer, w_bw, ColorMode::BlackAndWhite);

    let w_color = provider.png_writer("after.color.png");
    save_field_buffer(&mut after_buffer, w_color, ColorMode::Debug);
    let w_bw = provider.png_writer("after.bw.png");
    save_field_buffer(&mut after_buffer, w_bw, ColorMode::BlackAndWhite);
}

#[expectation_test]
fn drag_circle(provider: Provider) {
    use extern_api::*;

    let shape = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));

    drag_shape_helper(shape, 30, 30, 5.0, 7.0, provider);
}

#[expectation_test]
fn drag_rect(provider: Provider) {
    use extern_api::*;

    let shape = Shape::Terminal(Terminal::Rect(Rect {
        x: 1.0,
        y: 1.0,
        w: 10.0,
        h: 10.0,
    }));

    drag_shape_helper(shape, 20, 20, 5.0, 7.0, provider);
}
