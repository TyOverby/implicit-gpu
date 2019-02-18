use crate::opencl::{FieldBuffer, OpenClContext};
use crate::polygon::run_poly_raw_with_sign;
use expectation_plugin::expectation_test;

#[cfg(test)]
use expectation::{extensions::ImageDiffExtension, Provider};
use extern_api::Matrix;
#[cfg(test)]
use extern_api::Shape;

pub fn exec_freeze(ctx: &OpenClContext, field: &mut FieldBuffer) -> FieldBuffer {
    let (field_width, field_height) = (field.width, field.height);
    let (lines_buffer, count) = crate::impls::run_marching(field, ctx);
    run_poly_raw_with_sign(
        lines_buffer,
        field,
        field_width,
        field_height,
        count as usize,
        Matrix::identity(),
        ctx,
    )
}

#[cfg(test)]
fn freeze_shape_helper(shape: Shape, width: u32, height: u32, provider: Provider) {
    use crate::impls::exec_shape;
    use debug_helpers::*;

    let ctx = OpenClContext::default();

    let mut before_buffer = exec_shape(
        &ctx,
        provider.duplicate(),
        shape,
        width,
        height,
        |_| unimplemented!(),
    );
    let mut after_buffer = exec_freeze(&ctx, &mut before_buffer);

    let w_color = provider.png_writer("before.color.png");
    save_field_buffer(&mut before_buffer, w_color, ColorMode::Debug);
    let w_bw = provider.png_writer("before.bw.png");
    save_field_buffer(&mut before_buffer, w_bw, ColorMode::BlackAndWhite);

    let w_color = provider.png_writer("after.color.png");
    save_field_buffer(&mut after_buffer, w_color, ColorMode::Debug);
    let w_bw = provider.png_writer("after.bw.png");
    save_field_buffer(&mut after_buffer, w_bw, ColorMode::BlackAndWhite);
}

#[expectation_test]
fn freeze_circle(provider: Provider) {
    use extern_api::*;

    let shape = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));

    freeze_shape_helper(shape, 22, 22, provider);
}

#[expectation_test]
fn freeze_subtraction(provider: Provider) {
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

    freeze_shape_helper(c, 22, 22, provider);
}

#[expectation_test]
fn freeze_union(provider: Provider) {
    use extern_api::*;

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
