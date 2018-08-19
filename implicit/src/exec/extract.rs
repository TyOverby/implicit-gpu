use geometry::PathSegment;
use opencl::{FieldBuffer, OpenClContext};

#[cfg(test)]
use expectation::{extensions::TextDiffExtension, Provider};
#[cfg(test)]
use ocaml::Shape;

pub fn extract_lines(ctx: &OpenClContext, field: &FieldBuffer) -> Vec<PathSegment> {
    use euclid::point2;
    use itertools::Itertools;
    use lines::connect_lines;

    let (lines, count) = ::marching::run_marching(&field, ctx);
    let lines = lines.values(Some(count));
    let lines = lines
        .into_iter()
        .tuples::<(_, _, _, _)>()
        .take_while(|&(a, b, c, d)| !(a.is_nan() || b.is_nan() || c.is_nan() || d.is_nan()))
        .map(|(a, b, c, d)| (point2(a, b), point2(c, d)))
        .collect::<Vec<_>>();

    connect_lines(lines)
}

#[cfg(test)]
fn run_shape_paths(shape: Shape, width: usize, height: usize, mut provider: Provider) {
    use euclid::TypedPoint2D;
    use exec::exec_shape;
    use opencl::OpenClContext;
    use std::io::Write;

    pub fn is_clockwise<K>(pts: &[TypedPoint2D<f32, K>]) -> bool {
        assert!(pts.len() > 0);
        let mut total = 0.0f32;
        for slice in pts.windows(2) {
            let a = slice[0];
            let b = slice[1];
            total += (b.x - a.x) * (b.y + a.y);
        }
        {
            let a = pts[0];
            let b = pts[pts.len() - 1];
            total += (b.x - a.x) * (b.y + a.y);
        }
        total > 0.0
    }

    let ctx = OpenClContext::default();
    let buffer = exec_shape(&ctx, shape, width, height, |_| unimplemented!());
    let mut extracted = extract_lines(&ctx, &buffer);
    extracted.sort();

    let mut out = provider.text_writer("out.lines.txt");
    writeln!(out, "{} line segments", extracted.len());
    for (i, segment) in extracted.into_iter().enumerate() {
        writeln!(out);
        writeln!(out, "Line Segment {} ", i);
        writeln!(out, "{} points", segment.path.len());
        writeln!(out, "Clockwise? {}", is_clockwise(&segment.path[..]));
        for point in segment {
            writeln!(out, "{:?}", point);
        }
    }
}

expectation_test!{
    fn expectation_test_extract_circle(provider: Provider) {
        use euclid::*;
        use ocaml::*;

        let shape = Shape::Terminal(BasicTerminals::Circle(Circle {
            x: 11.0,
            y: 11.0,
            r: 10.0,
            mat: Transform2D::identity(),
        }));

        run_shape_paths(shape, 22, 22, provider);
    }
}

expectation_test!{
    fn expectation_test_extract_subtraction(provider: Provider) {
        use euclid::*;
        use ocaml::*;

        let a = Shape::Terminal(BasicTerminals::Circle(Circle {
            x: 11.0,
            y: 11.0,
            r: 10.0,
            mat: Transform2D::identity(),
        }));

        let b = Shape::Terminal(BasicTerminals::Circle(Circle {
            x: 11.0,
            y: 11.0,
            r: 5.0,
            mat: Transform2D::identity(),
        }));

        let c = Shape::Intersection(vec![a, Shape::Not(Box::new(b))]);

        run_shape_paths(c, 22, 22, provider);
    }
}
