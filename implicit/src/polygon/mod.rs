use opencl::{FieldBuffer, LineBuffer, OpenClContext};
use std::f32::INFINITY;

const PROGRAM: &'static str = include_str!("./polygon.c");

// TODO: rewrite this function so that it just takes &[f32]
pub fn run_poly(points: &[(f32, f32)], width: usize, height: usize, pos_mod: Option<(f32, f32)>, ctx: &OpenClContext) -> FieldBuffer {
    let _guard = ::flame::start_guard("run_poly");

    if points.len() == 0 {
        return ctx.field_buffer(width, height, Some(&vec![INFINITY; width * height]));
    }

    let mut buffer = Vec::with_capacity(points.len());
    for &(xs, ys) in points {
        buffer.push(xs);
        buffer.push(ys);
    }

    let buffer = ctx.line_buffer(&buffer[..]);

    run_poly_raw(buffer, width, height, pos_mod, ctx)
}

pub fn run_poly_raw(lines: LineBuffer, width: usize, height: usize, pos_mod: Option<(f32, f32)>, ctx: &OpenClContext) -> FieldBuffer {
    let _guard = ::flame::start_guard("run_poly_raw");
    let out = ctx.field_buffer(width, height, None);
    let kernel = ctx.compile("apply", PROGRAM);

    let pos_mod = pos_mod.unwrap_or((0.0, 0.0));

    kernel
        .gws([width, height])
        .arg_buf(out.buffer())
        .arg_scl(width as u64)
        .arg_buf(lines.buffer())
        .arg_scl(lines.size())
        .arg_scl(pos_mod.0)
        .arg_scl(pos_mod.1)
        .enq()
        .unwrap();
    out
}
