use super::*;
use *;

fn run_test(ast: AstPtr, x: f32, y: f32, z: f32) -> f32 {
    let mut jit = JIT::new();
    let comp = jit.compile("foo", ast);
    let foo = comp.unwrap_or_else(|msg| {
        eprintln!("error: {}", msg);
        std::process::exit(1);
    });
    let foo = unsafe { ::std::mem::transmute::<_, fn(f32, f32, f32) -> f32>(foo) };
    foo(x, y, z)
}

#[test]
fn single_constant() {
    assert_eq!(run_test(&Ast::Constant(5.0), 0.0, 0.0, 0.0), 5.0);
}

#[test]
fn single_x() {
    assert_eq!(run_test(&Ast::X, 1.0, 2.0, 3.0), 1.0);
}

#[test]
fn single_y() {
    assert_eq!(run_test(&Ast::Y, 1.0, 2.0, 3.0), 2.0);
}

#[test]
fn single_z() {
    assert_eq!(run_test(&Ast::Z, 1.0, 2.0, 3.0), 3.0);
}

#[test]
fn x_plus_y() {
    assert_eq!(run_test(&Ast::Add(&[Ast::X, Ast::Y]), 1.0, 2.0, 0.0), 3.0);
}

#[test]
fn x_plus() {
    assert_eq!(run_test(&Ast::Add(&[Ast::X]), 1.0, 0.0, 0.0), 1.0);
}

#[test]
fn x_plus_y_plus_z() {
    assert_eq!(
        run_test(&Ast::Add(&[Ast::X, Ast::Y, Ast::Z]), 1.0, 2.0, 3.0),
        6.0
    );
}

#[test]
fn x_mul_y() {
    assert_eq!(run_test(&Ast::Mul(&[Ast::X, Ast::Y]), 2.0, 3.0, 0.0), 6.0);
}

#[test]
fn x_mul() {
    assert_eq!(run_test(&Ast::Mul(&[Ast::X]), 1.0, 0.0, 0.0), 1.0);
}

#[test]
fn x_mul_y_mul_z() {
    assert_eq!(
        run_test(&Ast::Mul(&[Ast::X, Ast::Y, Ast::Z]), 2.0, 3.0, 4.0),
        24.0
    );
}

#[test]
fn x_sub_y() {
    assert_eq!(run_test(&Ast::Sub(&Ast::X, &Ast::Y), 2.0, 3.0, 0.0), -1.0);
}

#[test]
fn x_min() {
    assert_eq!(run_test(&Ast::Min(&[Ast::X]), 2.0, 3.0, 0.0), 2.0);
    assert_eq!(run_test(&Ast::Min(&[Ast::X]), 3.0, 1.0, 0.0), 3.0);
}

#[test]
fn x_max_y() {
    assert_eq!(run_test(&Ast::Max(&[Ast::X, Ast::Y]), 2.0, 3.0, 0.0), 3.0);
    assert_eq!(run_test(&Ast::Max(&[Ast::X, Ast::Y]), 3.0, 1.0, 0.0), 3.0);
}

#[test]
fn x_max() {
    assert_eq!(run_test(&Ast::Max(&[Ast::X]), 2.0, 3.0, 0.0), 2.0);
    assert_eq!(run_test(&Ast::Max(&[Ast::X]), 3.0, 1.0, 0.0), 3.0);
}

#[test]
fn x_neg() {
    assert_eq!(run_test(&Ast::Neg(&Ast::X), 2.0, 3.0, 0.0), -2.0);
    assert_eq!(run_test(&Ast::Neg(&Ast::X), 3.0, 1.0, 0.0), -3.0);
}

#[test]
fn x_sqrt() {
    assert_eq!(run_test(&Ast::Sqrt(&Ast::X), 4.0, 3.0, 0.0), 2.0);
    assert_eq!(run_test(&Ast::Sqrt(&Ast::X), 9.0, 1.0, 0.0), 3.0);
}

#[test]
fn x_square() {
    assert_eq!(run_test(&Ast::Square(&Ast::X), 2.0, 3.0, 0.0), 4.0);
    assert_eq!(run_test(&Ast::Square(&Ast::X), 3.0, 1.0, 0.0), 9.0);
}

#[test]
fn x_abs() {
    assert_eq!(run_test(&Ast::Abs(&Ast::X), 2.0, 3.0, 0.0), 2.0);
    assert_eq!(run_test(&Ast::Abs(&Ast::X), -2.0, 1.0, 0.0), 2.0);
}

#[test]
fn easy_scale() {
    let ast = &Ast::Transform {
        target: &Ast::Add(&[Ast::X, Ast::Y]),
        matrix: ::euclid::Transform3D::create_scale(2.0, 2.0, 2.0),
    };
    assert_eq!(run_test(ast, 1.0, 1.0, 0.0), 4.0);
    assert_eq!(run_test(ast, 1.0, 0.0, 0.0), 2.0);
    assert_eq!(run_test(ast, 0.0, 1.0, 0.0), 2.0);
}

#[test]
fn x_scale() {
    let ast = &Ast::Transform {
        target: &Ast::X,
        matrix: ::euclid::Transform3D::create_scale(2.0, 2.0, 2.0),
    };
    assert_eq!(run_test(ast, 1.0, 1.0, 0.0), 2.0);
    assert_eq!(run_test(ast, 1.0, 0.0, 1.0), 2.0);
    assert_eq!(run_test(ast, 0.0, 1.0, 0.0), 0.0);
}

#[test]
fn y_scale() {
    let ast = &Ast::Transform {
        target: &Ast::Y,
        matrix: ::euclid::Transform3D::create_scale(2.0, 2.0, 2.0),
    };
    assert_eq!(run_test(ast, 1.0, 1.0, 1.0), 2.0);
    assert_eq!(run_test(ast, 1.0, 0.0, 1.0), 0.0);
    assert_eq!(run_test(ast, 0.0, 1.0, 1.0), 2.0);
}

#[test]
fn z_scale() {
    let ast = &Ast::Transform {
        target: &Ast::Z,
        matrix: ::euclid::Transform3D::create_scale(2.0, 2.0, 2.0),
    };
    assert_eq!(run_test(ast, 1.0, 1.0, 1.0), 2.0);
    assert_eq!(run_test(ast, 1.0, 0.0, 1.0), 2.0);
    assert_eq!(run_test(ast, 0.0, 1.0, 1.0), 2.0);
}
