use crate::opencl::{FieldBuffer, LineBuffer, OpenClContext};

const PROGRAM: &'static str = include_str!("../shaders/marching.c");

pub fn run_marching(input: &mut FieldBuffer, ctx: &OpenClContext) -> (LineBuffer, u32) {
    let _guard = ::flame::start_guard("opencl marching [run_marching]");

    let (width, height) = (input.width as usize, input.height as usize);
    let mut kernel = ctx.compile("apply", PROGRAM, |register| {
        register.buffer("buffer");
        register.long("width");
        register.long("height");
        register.buffer("out");
        register.buffer("atomic");
    });

    let line_buffer = ctx.line_buffer_uninit(width * height * 4);
    let sync_buffer = ctx.sync_buffer();

    ::flame::start("setup kernel");
    kernel.set_default_global_work_size(::ocl::SpatialDims::Two(width, height));
    kernel
        .set_arg("buffer", input.to_opencl(ctx.queue()))
        .unwrap();
    kernel.set_arg("width", width as u64).unwrap();
    kernel.set_arg("height", height as u64).unwrap();
    kernel.set_arg("out", line_buffer.buffer()).unwrap();
    kernel.set_arg("atomic", sync_buffer.buffer()).unwrap();
    ::flame::end("setup kernel");

    unsafe {
        ::flame::span_of("opencl marching [execution]", || kernel.enq().unwrap());
    }

    let count = sync_buffer.value();
    (line_buffer, count)
}

#[test]
fn basic() {
    fn test_this(a: f32, b: f32, c: f32, d: f32, ctx: &OpenClContext) -> ((f32, f32), (f32, f32)) {
        let mut buf = ctx.field_buffer(2, 2, 1, Some(&[a, b, d, c]));
        let lines = run_marching(&mut buf, &ctx).0.values(None);

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

    assert_close(
        test_this(0.5, -0.5, -0.5, 0.5, &ctx),
        ((0.5, 1.0), (0.5, 0.0)),
    );
    assert_close(
        test_this(-0.5, -0.5, -0.5, 0.5, &ctx),
        ((0.5, 1.0), (0.0, 0.5)),
    );
    assert_close(
        test_this(-0.5, 0.5, 0.5, -0.5, &ctx),
        ((0.5, 0.0), (0.5, 1.0)),
    );
    assert_close(
        test_this(-0.75, 0.25, 0.25, -0.75, &ctx),
        ((0.75, 0.0), (0.75, 1.0)),
    );
    assert_close(
        test_this(0.75, -0.25, -0.75, -0.25, &ctx),
        ((0.0, 0.75), (0.75, 0.0)),
    );
    assert_close(
        test_this(0.75, 0.25, -0.25, 0.25, &ctx),
        ((0.5, 1.0), (1.0, 0.5)),
    );
    assert_close(
        test_this(-0.75, 0.35, 0.45, 0.55, &ctx),
        ((0.6818182, 0.0), (0.0, 0.5769231)),
    );
    assert_close(
        test_this(-0.75, -0.35, 0.45, -0.55, &ctx),
        ((1.0, 0.43750003), (0.55, 1.0)),
    );
    assert_close(
        test_this(0.75, -0.35, -0.45, -0.55, &ctx),
        ((0.0, 0.5769231), (0.6818182, 0.0)),
    );
    assert_close(
        test_this(0.75, 0.35, -0.45, 0.55, &ctx),
        ((0.55, 1.0), (1.0, 0.4375)),
    );
}
