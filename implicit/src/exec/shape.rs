use inspector::BoxedInspector;
use ocaml::{Id, Shape};
use opencl::{FieldBuffer, OpenClContext};

#[cfg(test)]
use expectation::{extensions::*, Provider};
use expectation_plugin::expectation_test;

pub fn exec_shape<F>(
    ctx: &OpenClContext,
    inspector: BoxedInspector,
    shape: Shape,
    width: usize,
    height: usize,
    buffer_find: F,
) -> FieldBuffer
where
    F: Fn(Id) -> FieldBuffer,
{
    let arena = ::typed_arena::Arena::new();
    let output = ::compiler::compile(&shape, &arena, &buffer_find);
    inspector.write_ast("ast", &output);

    let compiled = ::gpu_interp::gpu::compile(&output);
    inspector.write_compiled("compiled", &compiled);
    let mut buf = ::gpu_interp::gpu::execute(
        compiled,
        width as u32,
        height as u32,
        1,
        ::gpu_interp::gpu::Triad {
            context: ctx.context().clone(),
            queue: ctx.queue().clone(),
        },
    );

    FieldBuffer {
        dims: (width, height, 1),
        internal: buf.to_opencl(ctx.queue()).clone(),
    }
}

#[cfg(test)]
fn run_shape_helper(
    ctx: &OpenClContext,
    shape: Shape,
    width: usize,
    height: usize,
    provider: Provider,
    fields: &[FieldBuffer],
) -> FieldBuffer {
    use debug::*;
    use inspector::Inspector;

    let buffer = exec_shape(ctx, provider.duplicate(), shape, width, height, |i| {
        fields[i as usize].clone()
    });

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&buffer, w_color, ColorMode::Debug);
    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&buffer, w_bw, ColorMode::BlackAndWhite);

    buffer
}

#[expectation_test]
fn exec_circle(provider: Provider) {
    use ocaml::*;

    let ctx = OpenClContext::default();
    let shape = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));

    run_shape_helper(&ctx, shape, 22, 22, provider, &[]);
}

#[expectation_test]
fn exec_circle_with_matrix(provider: Provider) {
    use euclid::*;
    use ocaml::*;

    let ctx = OpenClContext::default();
    let shape = Shape::Transform(
        Box::new(Shape::Terminal(Terminal::Circle(Circle {
            x: 11.0,
            y: 11.0,
            r: 10.0,
        }))),
        Transform2D::identity().post_scale(2.0, 1.0),
    );

    run_shape_helper(&ctx, shape, 44, 22, provider, &[]);
}

#[expectation_test]
fn exec_rounded_rect_with_scale_on_top(provider: Provider) {
    use euclid::*;
    use ocaml::Rect;
    use ocaml::*;

    let ctx = OpenClContext::default();
    let inner_rect = Shape::Terminal(Terminal::Rect(Rect {
        x: 6.0,
        y: 6.0,
        w: 10.0,
        h: 10.0,
    }));
    let rounded_rect = Shape::Modulate(Box::new(inner_rect), 5.0);
    let scaled = Shape::Transform(Box::new(rounded_rect), Transform2D::create_scale(3.0, 1.0));

    run_shape_helper(&ctx, scaled, 66, 24, provider, &[]);
}

#[expectation_test]
fn exec_rect_translated(provider: Provider) {
    use euclid::*;
    use ocaml::Rect;
    use ocaml::*;

    let ctx = OpenClContext::default();
    let inner_rect = Shape::Terminal(Terminal::Rect(Rect {
        x: 6.0,
        y: 6.0,
        w: 10.0,
        h: 10.0,
    }));

    let translated = Shape::Transform(
        Box::new(inner_rect),
        Transform2D::create_translation(5.0, 5.0),
    );

    run_shape_helper(&ctx, translated, 33, 24, provider, &[]);
}
#[expectation_test]
fn exec_circle_translated(provider: Provider) {
    use euclid::*;
    use ocaml::*;

    let ctx = OpenClContext::default();
    let inner_rect = Shape::Terminal(Terminal::Circle(Circle {
        x: 0.0,
        y: 0.0,
        r: 5.0,
    }));

    let translated = Shape::Transform(
        Box::new(inner_rect),
        Transform2D::create_translation(6.0, 6.0),
    );

    run_shape_helper(&ctx, translated, 33, 24, provider, &[]);
}

#[expectation_test]
fn exec_rect(provider: Provider) {
    use ocaml::Rect;
    use ocaml::*;

    let ctx = OpenClContext::default();
    let shape = Shape::Terminal(Terminal::Rect(Rect {
        x: 1.0,
        y: 1.0,
        w: 20.0,
        h: 20.0,
    }));

    run_shape_helper(&ctx, shape, 22, 22, provider, &[]);
}

#[expectation_test]
fn exec_field(provider: Provider) {
    use ocaml::*;

    let ctx = OpenClContext::default();
    let circle = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));

    let circle_field = run_shape_helper(&ctx, circle, 22, 22, provider.subdir("inner"), &[]);

    let shape = Shape::Terminal(Terminal::Field(0));

    run_shape_helper(&ctx, shape, 22, 22, provider, &[circle_field]);
}

#[expectation_test]
fn exec_field_intersection(provider: Provider) {
    use ocaml::*;

    let ctx = OpenClContext::default();
    let circle_1 = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));
    let circle_2 = Shape::Terminal(Terminal::Circle(Circle {
        x: 15.0,
        y: 15.0,
        r: 10.0,
    }));

    let circle_field_1 = run_shape_helper(&ctx, circle_1, 22, 22, provider.subdir("c1"), &[]);
    let circle_field_2 = run_shape_helper(&ctx, circle_2, 22, 22, provider.subdir("c2"), &[]);

    let shape = Shape::Intersection(vec![
        Shape::Terminal(Terminal::Field(0)),
        Shape::Terminal(Terminal::Field(1)),
    ]);

    run_shape_helper(
        &ctx,
        shape,
        22,
        22,
        provider,
        &[circle_field_1, circle_field_2],
    );
}
