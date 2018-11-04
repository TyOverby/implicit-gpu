use super::{Ast, Buffer};

pub mod ops {
    include!(concat!(env!("OUT_DIR"), "/opcodes.rs"));
}

#[derive(Debug, PartialEq)]
pub struct CompilationResult {
    pub code: Vec<u8>,
    pub constants: Vec<f32>,
    pub max_stack: u32,
    pub transform_depth: u32,
    pub buffers: Vec<Buffer>,
}

pub fn compile(ast: &Ast) -> CompilationResult {
    fn transform_depth(ast: &Ast) -> u32 {
        use std::cmp::max;
        match ast {
            Ast::X | Ast::Y | Ast::Z => 1,
            Ast::Constant(_) | Ast::Buffer(_) => 0,
            Ast::Transform { target, .. } => 1 + transform_depth(target),
            Ast::Sub(l, r) => max(transform_depth(l), transform_depth(r)),
            Ast::Mul(lst) | Ast::Add(lst) | Ast::Min(lst) | Ast::Max(lst) => {
                lst.iter().map(transform_depth).fold(0, max)
            }
            Ast::Abs(t) | Ast::Sqrt(t) | Ast::Neg(t) => transform_depth(t),
        }
    }
    fn depth(ast: &Ast) -> u32 {
        use std::cmp::max;
        match ast {
            Ast::X | Ast::Y | Ast::Z | Ast::Constant(_) | Ast::Buffer(_) => 1,
            Ast::Transform { target, .. } => depth(target),
            Ast::Sub(l, r) => max(depth(l), depth(r)) + 1,
            Ast::Mul(lst) | Ast::Add(lst) | Ast::Min(lst) | Ast::Max(lst) => lst.iter().map(depth).fold(0, max) + 1,
            Ast::Abs(t) | Ast::Sqrt(t) | Ast::Neg(t) => depth(t),
        }
    }

    fn compile_inner(
        ast: &Ast,
        code: &mut Vec<u8>,
        constants: &mut Vec<f32>,
        buffers: &mut Vec<Buffer>,
    ) {
        fn compile_inner_list(
            asts: &[Ast],
            code: &mut Vec<u8>,
            constants: &mut Vec<f32>,
            buffers: &mut Vec<Buffer>,
            op: u8,
            name: &str,
        ) {
            if asts.len() == 0 {
                panic!("{} with 0 children", name);
            }
            compile_inner(&asts[0], code, constants, buffers);
            for child in &asts[1..] {
                compile_inner(child, code, constants, buffers);
                code.push(op);
            }
        }
        match ast {
            Ast::Buffer(b) => {
                let idx = buffers.len();
                buffers.push(b.clone());
                assert!(buffers.len() < ops::BUFFER_COUNT);
                assert!(buffers.len() <= 255);
                code.push(idx as u8);
            }
            Ast::X => code.push(ops::X),
            Ast::Y => code.push(ops::Y),
            Ast::Z => code.push(ops::Z),
            Ast::Constant(c) => {
                if constants.len() >= 255 {
                    panic!("large constants not supported yet")
                }
                let idx = constants.len() as u8;
                constants.push(*c);
                code.push(ops::CONSTANT_SMALL);
                code.push(idx);
            }
            Ast::Transform { target, matrix } => {
                code.push(ops::PUSH_TRANSFORM);
                {
                    let mut push_const = |constant| {
                        if constants.len() >= 255 {
                            panic!("large constants not supported yet")
                        }
                        let idx = constants.len() as u8;
                        constants.push(constant);
                        code.push(idx);
                    };
                    push_const(matrix.m11);
                    push_const(matrix.m21);
                    push_const(matrix.m31);
                    push_const(matrix.m41);
                    push_const(matrix.m12);
                    push_const(matrix.m22);
                    push_const(matrix.m32);
                    push_const(matrix.m42);
                    push_const(matrix.m13);
                    push_const(matrix.m23);
                    push_const(matrix.m33);
                    push_const(matrix.m43);
                    push_const(matrix.m14);
                    push_const(matrix.m24);
                    push_const(matrix.m34);
                    push_const(matrix.m44);
                }
                compile_inner(target, code, constants, buffers);
                code.push(ops::POP_TRANSFORM);
            }
            Ast::Sub(l, r) => {
                compile_inner(l, code, constants, buffers);
                compile_inner(r, code, constants, buffers);
                code.push(ops::SUB);
            }
            Ast::Add(lst) => compile_inner_list(lst, code, constants, buffers, ops::ADD, "add"),
            Ast::Mul(lst) => compile_inner_list(lst, code, constants, buffers, ops::MUL, "mul"),
            Ast::Max(lst) => compile_inner_list(lst, code, constants, buffers, ops::MAX, "max"),
            Ast::Min(lst) => compile_inner_list(lst, code, constants, buffers, ops::MIN, "min"),
            Ast::Abs(t) => {
                compile_inner(t, code, constants, buffers);
                code.push(ops::ABS);
            }
            Ast::Sqrt(t) => {
                compile_inner(t, code, constants, buffers);
                code.push(ops::SQRT);
            }
            Ast::Neg(t) => {
                compile_inner(t, code, constants, buffers);
                code.push(ops::NEG);
            }
        }
    }

    let mut code = vec![];
    let mut constants = vec![];
    let max_stack = depth(ast);
    let transform_depth = transform_depth(ast);
    let mut buffers = vec![];
    compile_inner(ast, &mut code, &mut constants, &mut buffers);
    CompilationResult {
        code,
        constants,
        max_stack,
        buffers,
        transform_depth,
    }
}

#[test]
fn compile_basic_constant() {
    assert_eq!(
        compile(&Ast::Constant(10.0)),
        CompilationResult {
            code: vec![ops::CONSTANT_SMALL, 0],
            constants: vec![10.0],
            max_stack: 1,
            transform_depth: 0,
            buffers: vec![],
        }
    )
}

#[test]
fn compile_x() {
    assert_eq!(
        compile(&Ast::X),
        CompilationResult {
            code: vec![ops::X],
            constants: vec![],
            max_stack: 1,
            transform_depth: 1,
            buffers: vec![],
        }
    )
}

#[test]
fn compile_basic_buffer() {
    assert_eq!(
        compile(&Ast::Buffer(Buffer::debug())),
        CompilationResult {
            code: vec![0],
            constants: vec![],
            max_stack: 1,
            transform_depth: 0,
            buffers: vec![Buffer::debug()],
        }
    )
}

#[test]
fn compile_basic_max() {
    assert_eq!(
        compile(&Ast::Max(&[Ast::Constant(10.0), Ast::Constant(5.0)])),
        CompilationResult {
            code: vec![ops::CONSTANT_SMALL, 0, ops::CONSTANT_SMALL, 1, ops::MAX],
            constants: vec![10.0, 5.0],
            max_stack: 2,
            transform_depth: 0,
            buffers: vec![],
        }
    )
}

#[test]
fn compile_more_max() {
    assert_eq!(
        compile(&Ast::Max(&[
            Ast::Constant(10.0),
            Ast::Constant(5.0),
            Ast::Constant(2.0)
        ])),
        CompilationResult {
            code: vec![
                ops::CONSTANT_SMALL,
                0,
                ops::CONSTANT_SMALL,
                1,
                ops::MAX,
                ops::CONSTANT_SMALL,
                2,
                ops::MAX,
            ],
            constants: vec![10.0, 5.0, 2.0],
            max_stack: 2,
            transform_depth: 0,
            buffers: vec![],
        }
    )
}

#[test]
fn compile_a_transform() {
    assert_eq!(
        compile(&Ast::Transform {
            target: &Ast::Max(&[Ast::X, Ast::Y,]),
            matrix: ::euclid::Transform3D::create_translation(2.0, 2.0, 2.0)
        }),
        CompilationResult {
            code: vec![
                ops::PUSH_TRANSFORM,
                0,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15,
                ops::X,
                ops::Y,
                ops::MAX,
                ops::POP_TRANSFORM
            ],
            constants: vec![
                1.0, 0.0, 0.0, 2.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 1.0
            ],
            max_stack: 2,
            transform_depth: 2,
            buffers: vec![],
        }
    )
}
