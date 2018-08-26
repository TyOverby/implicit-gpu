use opencl::{FieldBuffer, OpenClContext};
use polygon::run_poly_raw_with_sign;

#[cfg(test)]
use expectation::{extensions::ImageDiffExtension, Provider};
#[cfg(test)]
use ocaml::Shape;

pub fn exec_freeze(ctx: &OpenClContext, field: &FieldBuffer) -> FieldBuffer {
    let (width, height) = field.size();
    let (lines_buffer, count) = ::marching::run_marching(&field, ctx);
    run_poly_raw_with_sign(lines_buffer, field, width, height, count as usize, ctx)
}

#[cfg(test)]
fn freeze_shape_helper(shape: Shape, width: usize, height: usize, provider: Provider) {
    use debug::*;
    use exec::exec_shape;

    let ctx = OpenClContext::default();

    let before_buffer = exec_shape(&ctx, shape, width, height, |_| unimplemented!());
    let after_buffer = exec_freeze(&ctx, &before_buffer);

    let w_color = provider.png_writer("before.color.png");
    save_field_buffer(&before_buffer, w_color, ColorMode::Debug);
    let w_bw = provider.png_writer("before.bw.png");
    save_field_buffer(&before_buffer, w_bw, ColorMode::BlackAndWhite);

    let w_color = provider.png_writer("after.color.png");
    save_field_buffer(&after_buffer, w_color, ColorMode::Debug);
    let w_bw = provider.png_writer("after.bw.png");
    save_field_buffer(&after_buffer, w_bw, ColorMode::BlackAndWhite);
}

expectation_test!{
    fn expectation_test_freeze_circle(provider: Provider) {
        use ocaml::*;

        let shape = Shape::Terminal(Terminal::Circle(Circle {
            x: 11.0,
            y: 11.0,
            r: 10.0,
        }));

        freeze_shape_helper(shape, 22, 22, provider);
    }
}

expectation_test!{
    fn expectation_test_freeze_subtraction(provider: Provider) {
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

        freeze_shape_helper(c, 22, 22, provider);
    }
}

expectation_test!{
    fn expectation_test_freeze_union(provider: Provider) {
        use ocaml::*;

        let a = Shape::Terminal(Terminal::Circle(Circle {
            x: 11.0,
            y: 11.0,
            r: 10.0,
        }));

        let b = Shape::Terminal(Terminal::Circle(Circle {
            x: 21.0,
            y: 11.0,
            r: 10.0,
        }));

        let c = Shape::Union(vec![a, b]);

        freeze_shape_helper(c, 44, 22, provider);
    }
}
