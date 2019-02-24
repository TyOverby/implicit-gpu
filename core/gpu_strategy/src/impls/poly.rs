use crate::opencl::{FieldBuffer, LineBuffer, OpenClContext};
#[cfg(test)]
use debug_helpers::*;
#[cfg(test)]
use expectation::{extensions::*, Provider};
use expectation_plugin::expectation_test;
use extern_api::Matrix;
use extern_api::Polygon;
use ocl::Kernel;

pub fn exec_poly(ctx: &OpenClContext, poly: Polygon, width: u32, height: u32) -> FieldBuffer {
    run_poly(poly.points, None, width, height, poly.matrix, ctx).unwrap()
}

type Point = euclid::Point2D<f32>;

const PROGRAM: &'static str = concat!(
    include_str!("../shaders/dist_to_line.c"),
    include_str!("../shaders/polygon.c")
);

pub fn run_poly<I>(
    points: I,
    signfield: Option<&mut FieldBuffer>,
    width: u32,
    height: u32,
    matrix: Matrix,
    ctx: &OpenClContext,
) -> Option<FieldBuffer>
where
    I: IntoIterator<Item = Point>,
{
    if let Some(signfield) = &signfield {
        assert_eq!(signfield.height, height);
        assert_eq!(signfield.width, width);
    }

    let _guard = ::flame::start_guard("run_poly");

    let mut buffer = vec![];
    for Point { x, y, .. } in points {
        buffer.push(x);
        buffer.push(y);
    }

    if buffer.len() == 0 {
        return None;
    }

    let buffer = ctx.line_buffer(&buffer[..]);
    let buffer_len = buffer.size();

    match signfield {
        Some(sf) => Some(run_poly_raw_with_sign(
            buffer, sf, width, height, buffer_len, matrix, ctx,
        )),
        None => Some(run_poly_raw_no_sign(buffer, width, height, matrix, ctx)),
    }
}

#[inline(always)]
pub fn add_matrix(kernel: Kernel, matrix: Matrix) -> Kernel {
    let matrix = matrix.inverse().unwrap();
    kernel.set_arg("m11", matrix.m11).unwrap();
    kernel.set_arg("m12", matrix.m12).unwrap();
    kernel.set_arg("m21", matrix.m21).unwrap();
    kernel.set_arg("m22", matrix.m22).unwrap();
    kernel.set_arg("m31", matrix.m31).unwrap();
    kernel.set_arg("m32", matrix.m32).unwrap();
    kernel
}

pub fn run_poly_raw_no_sign(
    lines: LineBuffer,
    width: u32,
    height: u32,
    matrix: Matrix,
    ctx: &OpenClContext,
) -> FieldBuffer {
    let _guard = ::flame::start_guard("run_poly_raw");
    let mut out = ctx.field_buffer(width, height, 1, None);
    let mut kernel = ctx.compile("apply_no_sign", PROGRAM, |register| {
        register.buffer("buffer");
        register.long("width");
        register.buffer("lines");
        register.long("count");
        register.matrix();
    });

    kernel.set_default_global_work_size(::ocl::SpatialDims::Two(width as usize, height as usize));
    kernel
        .set_arg("buffer", out.to_opencl(ctx.queue()))
        .unwrap();
    kernel.set_arg("width", width as u64).unwrap();
    kernel.set_arg("lines", lines.buffer()).unwrap();
    kernel.set_arg("count", lines.size()).unwrap();
    let kernel = add_matrix(kernel, matrix);
    unsafe {
        kernel.enq().unwrap();
    }
    out
}

pub fn run_poly_raw_with_sign(
    lines: LineBuffer,
    signfield: &mut FieldBuffer,
    width: u32,
    height: u32,
    count: usize,
    matrix: Matrix,
    ctx: &OpenClContext,
) -> FieldBuffer {
    let _guard = ::flame::start_guard("run_poly_raw");
    let mut out = ctx.field_buffer(width, height, 1, None);
    let mut kernel = ctx.compile("apply_with_sign", PROGRAM, |register| {
        register.buffer("buffer");
        register.buffer("signbuffer");
        register.long("width");
        register.buffer("lines");
        register.long("count");
        register.matrix();
    });

    kernel.set_default_global_work_size(::ocl::SpatialDims::Two(width as usize, height as usize));
    kernel
        .set_arg("buffer", out.to_opencl(ctx.queue()))
        .unwrap();
    kernel
        .set_arg("signbuffer", signfield.to_opencl(ctx.queue()))
        .unwrap();
    kernel.set_arg("width", width as u64).unwrap();
    kernel.set_arg("lines", lines.buffer()).unwrap();
    kernel.set_arg("count", count).unwrap();
    let kernel = add_matrix(kernel, matrix);
    unsafe {
        kernel.enq().unwrap();
    }
    out
}

#[expectation_test]
fn exec_triangle(provider: Provider) {
    use euclid::*;
    use extern_api::*;

    let ctx = OpenClContext::default();
    let polygon = Polygon {
        points: vec![
            point2(1.0, 1.0),
            point2(15.0, 1.0),
            point2(15.0, 1.0),
            point2(15.0, 15.0),
            point2(15.0, 15.0),
            point2(1.0, 1.0),
        ],
        matrix: Matrix::identity(),
    };
    let mut buffer = exec_poly(&ctx, polygon, 20, 20);

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&mut buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&mut buffer, w_bw, ColorMode::BlackAndWhite);
}

#[expectation_test]
fn exec_line_bad(provider: Provider) {
    use euclid::*;
    use extern_api::*;

    let ctx = OpenClContext::default();
    let polygon = Polygon {
        points: vec![
            point2(1.0, 1.0),
            point2(15.0, 15.0),
            point2(15.0, 15.0),
            point2(1.0, 1.0),
        ],
        matrix: Matrix::identity(),
    };
    let mut buffer = exec_poly(&ctx, polygon, 20, 20);

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&mut buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&mut buffer, w_bw, ColorMode::BlackAndWhite);
}

#[expectation_test]
fn exec_triangle_scaled(provider: Provider) {
    use euclid::*;
    use extern_api::*;

    let ctx = OpenClContext::default();
    let polygon = Polygon {
        points: vec![
            point2(1.0, 1.0),
            point2(15.0, 1.0),
            point2(15.0, 1.0),
            point2(15.0, 15.0),
            point2(15.0, 15.0),
            point2(1.0, 1.0),
        ],
        matrix: Matrix::create_scale(2.0, 1.0),
    };
    let mut buffer = exec_poly(&ctx, polygon, 40, 20);

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&mut buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&mut buffer, w_bw, ColorMode::BlackAndWhite);
}
