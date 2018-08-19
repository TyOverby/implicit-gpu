#[cfg(test)]
use debug::*;
use euclid::Transform2D;
#[cfg(test)]
use expectation::{extensions::*, Provider};
use ocaml::Polygon;
use opencl::{FieldBuffer, OpenClContext};
use polygon::run_poly;

pub fn exec_poly(ctx: &OpenClContext, poly: Polygon, width: usize, height: usize) -> FieldBuffer {
    if !poly.mat.approx_eq(&Transform2D::identity()) {
        panic!("Only identity matrixes in circles are supported at the moment");
    }
    run_poly(poly.points, None, width, height, None, ctx).unwrap()
}

expectation_test!{
    fn expectation_test_exec_triangle(mut provider: Provider) {
        use ocaml::*;
        use euclid::*;

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
            mat: Transform2D::identity(),
        };
        let buffer = exec_poly(&ctx, polygon, 20, 20);

        let w_color = provider.png_writer("out.color.png");
        let w_bw = provider.png_writer("out.bw.png");
        save_field_buffer(&buffer, w_color, ColorMode::Debug);
        save_field_buffer(&buffer, w_bw, ColorMode::BlackAndWhite);
    }
}

expectation_test!{
    fn expectation_test_exec_line_BAD(mut provider: Provider) {
        use ocaml::*;
        use euclid::*;

        let ctx = OpenClContext::default();
        let polygon = Polygon {
            points: vec![
                point2(1.0, 1.0),
                point2(15.0, 15.0),

                point2(15.0, 15.0),
                point2(1.0, 1.0),
            ],
            mat: Transform2D::identity(),
        };
        let buffer = exec_poly(&ctx, polygon, 20, 20);

        let w_color = provider.png_writer("out.color.png");
        let w_bw = provider.png_writer("out.bw.png");
        save_field_buffer(&buffer, w_color, ColorMode::Debug);
        save_field_buffer(&buffer, w_bw, ColorMode::BlackAndWhite);
    }
}
