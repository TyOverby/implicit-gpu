/*
use crate::opencl::{FieldBuffer, OpenClContext};
use debug_helpers::*;

#[cfg(test)]
use expectation::{extensions::TextDiffExtension, Provider};
use expectation_plugin::expectation_test;
#[cfg(test)]
use extern_api::Shape;

type PathSegment = line_stitch::PathSegment<euclid::UnknownUnit>;
type Point = euclid::Point2D<f32>;

pub fn extract_lines(
    ctx: &OpenClContext,
    inspector: BoxedInspector,
    field: &mut FieldBuffer,
) -> Vec<(Point, Point)> {
    use euclid::point2;
    use itertools::Itertools;

    let (lines, count) = crate::impls::marching::run_marching(field, ctx);
    let lines = lines.values(Some(count));
    let lines = lines
        .into_iter()
        .tuples::<(_, _, _, _)>()
        .take_while(|&(a, b, c, d)| !(a.is_nan() || b.is_nan() || c.is_nan() || d.is_nan()))
        .map(|(a, b, c, d)| (point2(a, b), point2(c, d)))
        .collect::<Vec<_>>();

    lines
    //connect_lines(lines, inspector)
}

#[cfg(test)]
fn run_shape_paths(shape: Shape, width: u32, height: u32, provider: Provider) {
    use crate::impls::exec_shape;
    use crate::opencl::OpenClContext;
    use debug_helpers::print_path_segments;

    let ctx = OpenClContext::default();
    let mut buffer = exec_shape(
        &ctx,
        provider.duplicate(),
        shape,
        width,
        height,
        |_| unimplemented!(),
    );
    let mut extracted = extract_lines(&ctx, provider.duplicate(), &mut buffer);
    extracted.sort();

    let out = provider.text_writer("out.lines.txt");
    print_path_segments(out, &extracted);
}

#[expectation_test]
fn extract_circle(provider: Provider) {
    use extern_api::*;

    let shape = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));

    run_shape_paths(shape, 22, 22, provider);
}

#[expectation_test]
fn extract_subtraction(provider: Provider) {
    use extern_api::*;

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
*/
