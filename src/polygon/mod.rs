use ::opencl::{OpenClContext, FieldBuffer};

const PROGRAM: &'static str = include_str!("./polygon.c");

pub fn run_poly(xs: &[f32], ys: &[f32], width: usize, height: usize, ctx: &OpenClContext) -> FieldBuffer {
    let _guard = ::flame::start_guard("run_poly");
    assert_eq!(xs.len(), ys.len());

    let len = xs.len();

    let out = ctx.field_buffer(width, height, None);
    let kernel = ctx.compile("apply", PROGRAM);

    let xs_buf = ctx.line_buffer(xs);
    let ys_buf = ctx.line_buffer(ys);

    kernel
        .arg_buf(out.buffer())
        .arg_scl(width)
        .arg_buf(xs_buf.buffer())
        .arg_buf(ys_buf.buffer())
        .arg_scl(len)
        .enq().unwrap();

    out
}
