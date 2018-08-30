use geometry::Point;
use ocaml::Matrix;
use ocl::Kernel;
use opencl::{FieldBuffer, LineBuffer, OpenClContext};

const PROGRAM: &'static str = concat!(
    include_str!("shaders/dist_to_line.c"),
    include_str!("shaders/polygon.c")
);

// TODO: rewrite this function so that it just takes &[f32]
pub fn run_poly<I>(
    points: I,
    signfield: Option<&FieldBuffer>,
    width: usize,
    height: usize,
    matrix: Matrix,
    ctx: &OpenClContext,
) -> Option<FieldBuffer>
where
    I: IntoIterator<Item = Point>,
{
    if let Some(signfield) = &signfield {
        assert_eq!(signfield.height(), height);
        assert_eq!(signfield.width(), width);
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
fn add_matrix(kernel: Kernel, matrix: Matrix) -> Kernel {
    println!("matrix: {:?}", matrix);
    let matrix = matrix.inverse().unwrap();
    kernel
        .arg_scl(matrix.m11)
        .arg_scl(matrix.m12)
        .arg_scl(matrix.m21)
        .arg_scl(matrix.m22)
        .arg_scl(matrix.m31)
        .arg_scl(matrix.m32)
}

pub fn run_poly_raw_no_sign(
    lines: LineBuffer,
    width: usize,
    height: usize,
    matrix: Matrix,
    ctx: &OpenClContext,
) -> FieldBuffer {
    let _guard = ::flame::start_guard("run_poly_raw");
    let out = ctx.field_buffer(width, height, None);
    let kernel = ctx.compile("apply_no_sign", PROGRAM);

    let exec = kernel
        .queue(ctx.queue().clone())
        .gws([width, height])
        .arg_buf(out.buffer())
        .arg_scl(width as u64)
        .arg_buf(lines.buffer())
        .arg_scl(lines.size());
    let exec = add_matrix(exec, matrix);
    unsafe {
        exec.enq().unwrap();
    }
    out
}

pub fn run_poly_raw_with_sign(
    lines: LineBuffer,
    signfield: &FieldBuffer,
    width: usize,
    height: usize,
    count: usize,
    matrix: Matrix,
    ctx: &OpenClContext,
) -> FieldBuffer {
    let _guard = ::flame::start_guard("run_poly_raw");
    let out = ctx.field_buffer(width, height, None);
    let kernel = ctx.compile("apply_with_sign", PROGRAM);

    let exec = kernel
        .queue(ctx.queue().clone())
        .gws([width, height])
        .arg_buf(out.buffer())
        .arg_buf(signfield.buffer())
        .arg_scl(width as u64)
        .arg_buf(lines.buffer())
        .arg_scl(count);
    let exec = add_matrix(exec, matrix);
    unsafe {
        exec.enq().unwrap();
    }
    out
}
