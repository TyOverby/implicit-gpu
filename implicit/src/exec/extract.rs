use geometry::PathSegment;
use inspector::*;
use opencl::{FieldBuffer, OpenClContext};

#[cfg(test)]
use expectation::{extensions::TextDiffExtension, Provider};
use expectation_plugin::expectation_test;
#[cfg(test)]
use ocaml::Shape;

pub fn extract_lines(
    ctx: &OpenClContext,
    inspector: BoxedInspector,
    field: &FieldBuffer,
) -> Vec<PathSegment> {
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

    connect_lines(lines, inspector)
}

#[cfg(test)]
fn run_shape_paths(shape: Shape, width: usize, height: usize, provider: Provider) {
    use debug::print_path_segments;
    use exec::exec_shape;
    use opencl::OpenClContext;

    let ctx = OpenClContext::default();
    let buffer = exec_shape(&ctx, provider.duplicate(), shape, width, height, |_| unimplemented!());
    let mut extracted = extract_lines(&ctx, provider.duplicate(), &buffer);
    extracted.sort();

    let out = provider.text_writer("out.lines.txt");
    print_path_segments(out, &extracted);
}

#[expectation_test]
fn extract_circle(provider: Provider) {
    use ocaml::*;

    let shape = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));

    run_shape_paths(shape, 22, 22, provider);
}

#[expectation_test]
fn extract_subtraction(provider: Provider) {
    use ocaml::*;

    let a = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));

    let b = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 5.0,
    }));

    let c = Shape::Intersection(vec![a, Shape::Not(Box::new(b))]);

    run_shape_paths(c, 22, 22, provider);
}
