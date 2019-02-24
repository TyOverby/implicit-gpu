#[cfg(test)]
use debug_helpers::*;
#[cfg(test)]
use expectation::{extensions::*, Provider};

use crate::opencl::{FieldBuffer, OpenClContext};
use expectation_plugin::expectation_test;
use extern_api::Matrix;

const PROGRAM: &'static str = include_str!("../shaders/simplex.c");

pub fn get_noise(
    ctx: &OpenClContext,
    width: u32,
    height: u32,
    cutoff: f32,
    matrix: Matrix,
) -> FieldBuffer {
    let mut out = ctx.field_buffer(width, height, 1, None);
    let mut kernel = ctx.compile("apply", PROGRAM, |register| {
        register.buffer("buffer");
        register.long("width");
        register.float("cutoff");
        register.matrix();
    });

    kernel.set_default_global_work_size(::ocl::SpatialDims::Two(width as usize, height as usize));
    kernel
        .set_arg("buffer", out.to_opencl(ctx.queue()))
        .unwrap();
    kernel.set_arg("width", width as u64).unwrap();
    kernel.set_arg("cutoff", cutoff).unwrap();
    let kernel = crate::impls::poly::add_matrix(kernel, matrix);

    unsafe {
        kernel.enq().unwrap();
    }
    out
}

#[expectation_test]
fn exec_noise(provider: Provider) {
    use crate::opencl::*;
    use extern_api::Matrix;

    let ctx = OpenClContext::default();
    let mut buffer = get_noise(&ctx, 20, 20, 0.5, Matrix::identity());

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&mut buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&mut buffer, w_bw, ColorMode::BlackAndWhite);
}
