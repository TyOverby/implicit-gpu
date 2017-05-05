use ::opencl::{OpenClContext, FieldBuffer, LineBuffer};
use std::f32::INFINITY;

const PROGRAM: &'static str = include_str!("./polygon.c");

pub fn run_poly(xs: &[f32], ys: &[f32], width: usize, height: usize, ctx: &OpenClContext) -> FieldBuffer {
    let _guard = ::flame::start_guard("run_poly");
    assert_eq!(xs.len(), ys.len());

    if xs.len() == 0 {
        return ctx.field_buffer(width, height, Some(&vec![INFINITY; width * height]));
    }

    let xs_buf = ctx.line_buffer(xs);
    let ys_buf = ctx.line_buffer(ys);

    run_poly_raw(xs_buf, ys_buf, width, height, ctx)
}

pub fn run_poly_raw(xs: LineBuffer, ys: LineBuffer, width: usize, height: usize, ctx: &OpenClContext) -> FieldBuffer {
    debug_assert!(xs.non_nans_at_front());
    debug_assert!(ys.non_nans_at_front());

    let _guard = ::flame::start_guard("run_poly_raw");
    let out = ctx.field_buffer(width, height, None);
    let kernel = ctx.compile("apply", PROGRAM);

    kernel
        .gws([width, height])
        .arg_buf(out.buffer())
        .arg_scl(width as u64)
        .arg_buf(xs.buffer())
        .arg_buf(ys.buffer())
        .arg_scl(xs.size())
        .enq().unwrap();
    out
}
