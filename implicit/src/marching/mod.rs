mod polygonize;
pub mod util;

use self::util::geom::{Line, Point};
use itertools::Itertools;
use nodes::Polygon;
use opencl::{FieldBuffer, LineBuffer, OpenClContext};

const PROGRAM: &'static str = include_str!("marching.c");

pub fn run_marching(input: &FieldBuffer, ctx: &OpenClContext) -> (LineBuffer, LineBuffer) {
    let _guard = ::flame::start_guard("opencl marching [run_marching]");

    let (width, height) = (input.width(), input.height());
    let kernel = ctx.compile("apply", PROGRAM);
    let from = vec![::std::f32::NAN; width * height * 2];
    let out_xs = ctx.line_buffer(&from);
    let out_ys = ctx.line_buffer(&from);

    kernel
        .gws([width, height])
        .arg_buf(input.buffer())
        .arg_scl(width as u64)
        .arg_scl(height as u64)
        .arg_buf(out_xs.buffer())
        .arg_buf(out_ys.buffer())
        .enq()
        .unwrap();

    (out_xs, out_ys)
}

pub fn march(input: &FieldBuffer, simplify: bool, ctx: &OpenClContext) -> Vec<Polygon> {
    let _guard = ::flame::start_guard("march");

    let (out_xs, out_ys) = run_marching(input, ctx);

    let lines = ::flame::span_of(
        "point filtering", || {
            Iterator::zip(out_xs.values().into_iter(), out_ys.values().into_iter())
                .tuples()
                .filter(|&((a, b), (c, d))| !(a.is_nan() && b.is_nan() && c.is_nan() && d.is_nan()))
                .map(|((a, b), (c, d))| Line(Point { x: a, y: b }, Point { x: c, y: d }))
                .collect::<Vec<_>>()
        }
    );

    ::flame::start("line connecting");
    let (lns, _) = polygonize::connect_lines(lines);
    let mut polygons = vec![];
    for polygon in lns.into_iter() {
        let polygon = if simplify {
            polygonize::simplify_line(polygon)
        } else {
            polygon
        };

        let mut xs = Vec::with_capacity(polygon.len());
        let mut ys = Vec::with_capacity(polygon.len());

        for pt in polygon {
            xs.push(pt.x);
            ys.push(pt.y);
        }

        polygons.push(Polygon { xs: xs, ys: ys });
    }
    ::flame::end("line connecting");

    polygons
}

#[test]
fn basic() {
    fn test_this(a: f32, b: f32, c: f32, d: f32, ctx: &OpenClContext) -> ((f32, f32), (f32, f32)) {
        let buf = ctx.field_buffer(2, 2, Some(&[a, b, d, c]));

        let (out_xs, out_ys) = run_marching(&buf, &ctx);
        let (out_xs, out_ys) = (out_xs.values(), out_ys.values());

        return ((out_xs[0], out_ys[0]), (out_xs[1], out_ys[1]));
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
