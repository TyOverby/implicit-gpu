use compiler::{compile, CompileResult};
use ocaml::{Id, Shape};
use opencl::{FieldBuffer, OpenClContext};

#[cfg(test)]
use expectation::{extensions::*, Provider};

pub fn exec_shape<F>(
    ctx: &OpenClContext,
    shape: Shape,
    width: usize,
    height: usize,
    buffer_find: F,
) -> FieldBuffer
where
    F: Fn(Id) -> FieldBuffer,
{
    let mut writer: Vec<u8> = vec![];
    let CompileResult { dependencies, .. } = compile(&shape, &mut writer).unwrap();
    let kernel = ctx.compile("apply", String::from_utf8(writer).unwrap());

    let out = ctx.field_buffer(width, height, None);

    let mut kc = kernel
        .queue(ctx.queue().clone())
        .gws([width, height])
        .arg_buf(out.buffer())
        .arg_scl(width as u64);

    for dep in dependencies {
        kc = kc.arg_buf(buffer_find(dep).buffer());
    }

    unsafe { kc.enq().unwrap() };

    out
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

    let buffer = exec_shape(ctx, shape, width, height, |i| fields[i as usize].clone());

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&buffer, w_color, ColorMode::Debug);
    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&buffer, w_bw, ColorMode::BlackAndWhite);

    buffer
}

expectation_test!{
    fn expectation_test_exec_circle(provider: Provider) {
        use euclid::*;
        use ocaml::*;

        let ctx = OpenClContext::default();
        let shape = Shape::Terminal(Terminal::Circle(Circle {
            x: 11.0,
            y: 11.0,
            r: 10.0,
            mat: Transform2D::identity(),
        }));

        run_shape_helper(&ctx, shape, 22, 22, provider, &[]);
    }
}

expectation_test!{
    fn expectation_test_exec_circle_with_matrix(provider: Provider) {
        use euclid::*;
        use ocaml::*;

        let ctx = OpenClContext::default();
        let shape = Shape::Terminal(Terminal::Circle(Circle {
            x: 11.0,
            y: 11.0,
            r: 10.0,
            mat: Transform2D::identity().post_scale(2.0, 1.0),
        }));

        run_shape_helper(&ctx, shape, 44, 22, provider, &[]);
    }
}

expectation_test!{
    fn expectation_test_exec_rect(provider: Provider) {
        use euclid::*;
        use ocaml::*;
        use ocaml::Rect;

        let ctx = OpenClContext::default();
        let shape = Shape::Terminal(Terminal::Rect(Rect {
            x: 1.0,
            y: 1.0,
            w: 20.0,
            h: 20.0,
            mat: Transform2D::identity(),
        }));

        run_shape_helper(&ctx, shape, 22, 22, provider, &[]);
    }
}

expectation_test!{
    fn expectation_test_exec_field(provider: Provider) {
        use euclid::*;
        use ocaml::*;

        let ctx = OpenClContext::default();
        let circle = Shape::Terminal(Terminal::Circle(Circle {
            x: 11.0,
            y: 11.0,
            r: 10.0,
            mat: Transform2D::identity(),
        }));

        let circle_field = run_shape_helper(&ctx, circle, 22, 22, provider.subdir("inner"), &[]);

        let shape = Shape::Terminal(Terminal::Field(0));

        run_shape_helper(&ctx, shape, 22, 22, provider, &[circle_field]);
    }
}

expectation_test!{
    fn expectation_test_exec_field_intersection(provider: Provider) {
        use euclid::*;
        use ocaml::*;

        let ctx = OpenClContext::default();
        let circle_1 = Shape::Terminal(Terminal::Circle(Circle {
            x: 11.0,
            y: 11.0,
            r: 10.0,
            mat: Transform2D::identity(),
        }));
        let circle_2 = Shape::Terminal(Terminal::Circle(Circle {
            x: 15.0,
            y: 15.0,
            r: 10.0,
            mat: Transform2D::identity(),
        }));

        let circle_field_1 = run_shape_helper(&ctx, circle_1, 22, 22, provider.subdir("c1"), &[]);
        let circle_field_2 = run_shape_helper(&ctx, circle_2, 22, 22, provider.subdir("c2"), &[]);

        let shape =
            Shape::Intersection(vec![
                Shape::Terminal(Terminal::Field(0)),
                Shape::Terminal(Terminal::Field(1)),
            ]);

        run_shape_helper(&ctx, shape, 22, 22, provider, &[circle_field_1, circle_field_2]);
    }
}
