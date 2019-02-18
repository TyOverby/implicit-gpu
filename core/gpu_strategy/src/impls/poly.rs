use crate::opencl::{FieldBuffer, OpenClContext};
use crate::polygon::run_poly;
#[cfg(test)]
use debug_helpers::*;
#[cfg(test)]
use expectation::{extensions::*, Provider};
use expectation_plugin::expectation_test;
use extern_api::Polygon;

pub fn exec_poly(ctx: &OpenClContext, poly: Polygon, width: u32, height: u32) -> FieldBuffer {
    run_poly(poly.points, None, width, height, poly.matrix, ctx).unwrap()
}

#[expectation_test]
fn exec_triangle(provider: Provider) {
    use euclid::*;
    use extern_api::*;

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
    let mut buffer = exec_poly(&ctx, polygon, 20, 20);

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&mut buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&mut buffer, w_bw, ColorMode::BlackAndWhite);
}

#[expectation_test]
fn exec_line_bad(provider: Provider) {
    use euclid::*;
    use extern_api::*;

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
    let mut buffer = exec_poly(&ctx, polygon, 20, 20);

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&mut buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&mut buffer, w_bw, ColorMode::BlackAndWhite);
}

#[expectation_test]
fn exec_triangle_scaled(provider: Provider) {
    use euclid::*;
    use extern_api::*;

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
    let mut buffer = exec_poly(&ctx, polygon, 40, 20);

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&mut buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&mut buffer, w_bw, ColorMode::BlackAndWhite);
}
