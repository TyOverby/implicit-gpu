#[cfg(test)]
use debug::*;
#[cfg(test)]
use expectation::{extensions::*, Provider};

use expectation_plugin::expectation_test;
use ocaml::Matrix;
use opencl::{FieldBuffer, OpenClContext};

const PROGRAM: &'static str = include_str!("../shaders/simplex.c");

pub fn get_noise(
    ctx: &OpenClContext,
    width: usize,
    height: usize,
    cutoff: f32,
    matrix: Matrix,
) -> FieldBuffer {
    let out = ctx.field_buffer(width, height, None);
    let mut kernel = ctx.compile("apply", PROGRAM, |register| {
        register.buffer("buffer");
        register.long("width");
        register.float("cutoff");
        register.matrix();
    });

    kernel.set_default_global_work_size(::ocl::SpatialDims::Two(width, height));
    kernel.set_arg("buffer", out.buffer()).unwrap();
    kernel.set_arg("width", width as u64).unwrap();
    kernel.set_arg("cutoff", cutoff).unwrap();
    let kernel = ::polygon::add_matrix(kernel, matrix);

    unsafe {
        kernel.enq().unwrap();
    }
    out
}

#[expectation_test]
fn exec_noise(provider: Provider) {
    use ocaml::Matrix;
    use opencl::*;

    let ctx = OpenClContext::default();
    let buffer = get_noise(&ctx, 20, 20, 0.5, Matrix::identity());

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&buffer, w_bw, ColorMode::BlackAndWhite);
}
