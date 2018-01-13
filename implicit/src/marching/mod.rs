use opencl::{FieldBuffer, LineBuffer, OpenClContext};

const PROGRAM: &'static str = include_str!("marching.c");

pub fn run_marching(input: &FieldBuffer, ctx: &OpenClContext) -> LineBuffer {
    let _guard = ::flame::start_guard("opencl marching [run_marching]");

    let (width, height) = (input.width(), input.height());
    let kernel = ctx.compile("apply", PROGRAM);
    let from = ::flame::span_of("opencl marching [build vec]", || vec![::std::f32::NAN; width * height * 4]);

    let line_buffer = ctx.line_buffer(&from);
    let sync_buffer = ctx.sync_buffer();

    ::flame::start("setup kernel");
    let exec = kernel
        .queue(ctx.queue().clone())
        .gws([width, height])
        .arg_buf(input.buffer())
        .arg_scl(width as u64)
        .arg_scl(height as u64)
        .arg_buf(line_buffer.buffer())
        .arg_buf(sync_buffer.buffer());
    ::flame::end("setup kernel");

    unsafe {
        ::flame::span_of("opencl marching [execution]", || exec.enq().unwrap());
    }

    line_buffer
}

#[test]
fn basic() {
    fn test_this(a: f32, b: f32, c: f32, d: f32, ctx: &OpenClContext) -> ((f32, f32), (f32, f32)) {
        let buf = ctx.field_buffer(2, 2, Some(&[a, b, d, c]));
        let lines = run_marching(&buf, &ctx).values();

        return ((lines[0], lines[1]), (lines[2], lines[3]));
    }

    fn assert_close(a: ((f32, f32), (f32, f32)), b: ((f32, f32), (f32, f32))) {
        let ((a1, a2), (a3, a4)) = a;
        let ((b1, b2), (b3, b4)) = b;

        assert!((a1 - b1).abs() < 0.001, "for {:?}", b);
        assert!((a2 - b2).abs() < 0.001, "for {:?}", b);
        assert!((a3 - b3).abs() < 0.001, "for {:?}", b);
        assert!((a4 - b4).abs() < 0.001, "for {:?}", b);
    }

    let ctx = OpenClContext::default();

    assert_close(test_this(0.5, -0.5, -0.5, 0.5, &ctx), ((0.5, 1.0), (0.5, 0.0)));
    assert_close(test_this(-0.5, -0.5, -0.5, 0.5, &ctx), ((0.5, 1.0), (0.0, 0.5)));
    assert_close(test_this(-0.5, 0.5, 0.5, -0.5, &ctx), ((0.5, 0.0), (0.5, 1.0)));
    assert_close(test_this(-0.75, 0.25, 0.25, -0.75, &ctx), ((0.75, 0.0), (0.75, 1.0)));
    assert_close(test_this(0.75, -0.25, -0.75, -0.25, &ctx), ((0.0, 0.75), (0.75, 0.0)));
    assert_close(test_this(0.75, 0.25, -0.25, 0.25, &ctx), ((0.5, 1.0), (1.0, 0.5)));
    assert_close(test_this(-0.75, 0.35, 0.45, 0.55, &ctx), ((0.6818182, 0.0), (0.0, 0.5769231)));
    assert_close(test_this(-0.75, -0.35, 0.45, -0.55, &ctx), ((1.0, 0.43750003), (0.55, 1.0)));
    assert_close(test_this(0.75, -0.35, -0.45, -0.55, &ctx), ((0.0, 0.5769231), (0.6818182, 0.0)));
    assert_close(test_this(0.75, 0.35, -0.45, 0.55, &ctx), ((0.55, 1.0), (1.0, 0.4375)));
}
