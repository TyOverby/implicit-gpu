use ocl::{Buffer};
use opencl::OpenClContext;

const PROGRAM: &'static str = include_str!("marching.c");

pub fn run_marching(input: Buffer<f32>, width: usize, height: usize, ctx: &OpenClContext) -> Buffer<f32> {
    let kernel = ctx.compile("apply", PROGRAM);

    let from = vec![::std::f32::NAN; width * height * 4];
    let out = ctx.input_buffer([from.len()], &from);

    kernel
        .gws([width, height])
        .arg_buf(&input)
        .arg_scl(width)
        .arg_scl(height)
        .arg_buf(&out)
        .enq().unwrap();

    out
}

#[test]
fn basic() {
    fn test_this(a: f32, b: f32, c: f32, d: f32, ctx: &OpenClContext) -> ((f32, f32), (f32, f32)) {
        let buf = ctx.input_buffer([2, 2], &[a, b, d, c]);

        let result = run_marching(buf, 2, 2, &ctx);
        let mut out = vec![0.0 / 0.0; result.len()];
        result.read(&mut out).enq().unwrap();

        return ((out[0], out[1]), (out[2], out[3]))
    }

    fn assert_close(a: ((f32, f32), (f32, f32)), b: ((f32, f32), (f32, f32))) {
        let ((a1, a2), (a3, a4)) = a;
        let ((b1, b2), (b3, b4)) = b;

        assert!((a1 - b1).abs() < 0.001);
        assert!((a2 - b2).abs() < 0.001);
        assert!((a3 - b3).abs() < 0.001);
        assert!((a4 - b4).abs() < 0.001);
    }

    let ctx = OpenClContext::default();

    assert_close(test_this(1.0, 0.0, 1.0, 1.0, &ctx), ((1.0, 0.0), (1.0, 0.0)));
    assert_close(test_this(0.5, -0.5, -0.5, 0.5, &ctx), ((0.5, 0.0), (0.5, 1.0)));
    assert_close(test_this(-0.5, -0.5, -0.5, 0.5, &ctx), ((0.0, 0.5), (0.5, 1.0)));
    assert_close(test_this(-0.5, 0.5, 0.5, -0.5, &ctx), ((0.5, 0.0), (0.5, 1.0)));
    assert_close(test_this(-0.75, 0.25, 0.25, -0.75, &ctx), ((0.75, 0.0), (0.75, 1.0)));
    assert_close(test_this(0.75, -0.25, -0.75, -0.25, &ctx), ((0.0, 0.75), (0.75, 0.0)));
    assert_close(test_this(0.75, 0.25, -0.25, 0.25, &ctx), ((0.5, 1.0), (1.0, 0.5)));
    assert_close(test_this(-0.75, 0.35, 0.45, 0.55, &ctx), ((0.0, 0.5769231), (0.6818182, 0.0)));
    assert_close(test_this(-0.75, -0.35, 0.45, -0.55, &ctx), ((0.55, 1.0), (1.0, 0.43750003)));
    assert_close(test_this(0.75, -0.35, -0.45, -0.55, &ctx), ((0.0, 0.5769231), (0.6818182, 0.0)));

    println!("{:?}", test_this(0.75, 0.35, -0.45, 0.55, &ctx));
    assert_close(test_this(0.75, 0.35, -0.45, 0.55, &ctx), ((0.55, 1.0), (1.0, 0.4375)));

}
