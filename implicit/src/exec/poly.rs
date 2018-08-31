#[cfg(test)]
use debug::*;
#[cfg(test)]
use expectation::{extensions::*, Provider};
use expectation_plugin::expectation_test;
use ocaml::Polygon;
use opencl::{FieldBuffer, OpenClContext};
use polygon::run_poly;

pub fn exec_poly(ctx: &OpenClContext, poly: Polygon, width: usize, height: usize) -> FieldBuffer {
    run_poly(poly.points, None, width, height, poly.matrix, ctx).unwrap()
}

#[expectation_test]
fn exec_triangle(mut provider: Provider) {
    use euclid::*;
    use ocaml::*;

    let ctx = OpenClContext::default();
    let polygon = Polygon {
        points: vec![
            point2(1.0, 1.0),
            point2(15.0, 1.0),
            point2(15.0, 1.0),
            point2(15.0, 15.0),
            point2(15.0, 15.0),
            point2(1.0, 1.0),
        ],
        matrix: Matrix::identity(),
    };
    let buffer = exec_poly(&ctx, polygon, 20, 20);

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&buffer, w_bw, ColorMode::BlackAndWhite);
}

#[expectation_test]
fn exec_line_BAD(mut provider: Provider) {
    use euclid::*;
    use ocaml::*;

    let ctx = OpenClContext::default();
    let polygon = Polygon {
        points: vec![
            point2(1.0, 1.0),
            point2(15.0, 15.0),
            point2(15.0, 15.0),
            point2(1.0, 1.0),
        ],
        matrix: Matrix::identity(),
    };
    let buffer = exec_poly(&ctx, polygon, 20, 20);

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&buffer, w_bw, ColorMode::BlackAndWhite);
}

#[expectation_test]
fn exec_triangle_scaled(mut provider: Provider) {
    use euclid::*;
    use ocaml::*;

    let ctx = OpenClContext::default();
    let polygon = Polygon {
        points: vec![
            point2(1.0, 1.0),
            point2(15.0, 1.0),
            point2(15.0, 1.0),
            point2(15.0, 15.0),
            point2(15.0, 15.0),
            point2(1.0, 1.0),
        ],
        matrix: Matrix::create_scale(2.0, 1.0),
    };
    let buffer = exec_poly(&ctx, polygon, 40, 20);

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&buffer, w_bw, ColorMode::BlackAndWhite);
}
