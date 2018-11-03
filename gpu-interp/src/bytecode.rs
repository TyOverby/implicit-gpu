use super::Ast;

mod ops {
    include!(concat!(env!("OUT_DIR"), "/opcodes.rs"));
}

#[derive(Debug, PartialEq)]
pub struct CompilationResult {
    pub code: Vec<u8>,
    pub constants: Vec<f32>,
    pub max_stack: u32,
}

pub fn compile(ast: &Ast) -> CompilationResult {
    fn count_constants(ast: &Ast) -> u32 {
        match ast {
            Ast::X | Ast::Y | Ast::Z => 0,
            Ast::Constant(_) => 1,
            Ast::Sub(l, r) => count_constants(l) + count_constants(r),
            Ast::Add(lst) | Ast::Min(lst) | Ast::Max(lst) => lst.iter().map(count_constants).sum(),
            Ast::Abs(t) | Ast::Sqrt(t) => count_constants(t),
        }
    }
    fn depth(ast: &Ast) -> u32 {
        use std::cmp::max;
        match ast {
            Ast::X | Ast::Y | Ast::Z | Ast::Constant(_) => 1,
            Ast::Sub(l, r) => max(depth(l), depth(r)) + 1,
            Ast::Add(lst) | Ast::Min(lst) | Ast::Max(lst) => lst.iter().map(depth).fold(0, max) + 1,
            Ast::Abs(t) | Ast::Sqrt(t) => depth(t),
        }
    }

    fn compile_inner(ast: &Ast, code: &mut Vec<u8>, constants: &mut Vec<f32>) {
        fn compile_inner_list(
            asts: &[Ast],
            code: &mut Vec<u8>,
            constants: &mut Vec<f32>,
            op: u8,
            name: &str,
        ) {
            if asts.len() == 0 {
                panic!("{} with 0 children", name);
            }
            compile_inner(&asts[0], code, constants);
            for child in &asts[1..] {
                compile_inner(child, code, constants);
                code.push(op);
            }
        }
        match ast {
            Ast::X => code.push(ops::X),
            Ast::Y => code.push(ops::Y),
            Ast::Z => code.push(ops::Z),
            Ast::Constant(c) => {
                let idx = constants.len() as u8;
                constants.push(*c);
                code.push(ops::CONSTANT_SMALL);
                code.push(idx);
            }
            Ast::Sub(l, r) => {
                compile_inner(l, code, constants);
                compile_inner(r, code, constants);
                code.push(ops::SUB);
            }
            Ast::Add(lst) => compile_inner_list(lst, code, constants, ops::ADD, "add"),
            Ast::Max(lst) => compile_inner_list(lst, code, constants, ops::MAX, "max"),
            Ast::Min(lst) => compile_inner_list(lst, code, constants, ops::MIN, "min"),
            Ast::Abs(t) => {
                compile_inner(t, code, constants);
                code.push(ops::ABS);
            }
            Ast::Sqrt(t) => {
                compile_inner(t, code, constants);
                code.push(ops::SQRT);
            }
        }
    }

    let mut code = vec![];
    let constant_count = count_constants(ast);
    if constant_count > 255 {
        panic!("more than 255 constants!");
    }
    let mut constants = vec![];
    let max_stack = depth(ast);
    compile_inner(ast, &mut code, &mut constants);
    CompilationResult {
        code,
        constants,
        max_stack,
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
        }
    )
}
