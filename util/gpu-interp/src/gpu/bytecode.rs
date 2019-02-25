use *;

pub mod ops {
    #[allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/opcodes.rs"));
}

#[derive(PartialOrd, PartialEq)]
struct OrderedF32(f32);

pub struct ConstantCache {
    map: ::std::collections::BTreeMap<OrderedF32, u32>,
}
impl Eq for OrderedF32 {}

impl Ord for OrderedF32 {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.partial_cmp(other)
            .unwrap_or(::std::cmp::Ordering::Equal)
    }
}

impl ConstantCache {
    fn new() -> ConstantCache {
        ConstantCache {
            map: ::std::collections::BTreeMap::new(),
        }
    }
    fn record(&mut self, value: f32) -> u32 {
        let size = self.map.len();
        *self.map.entry(OrderedF32(value)).or_insert(size as u32)
    }
    fn to_vec(self) -> Vec<f32> {
        let mut v = self.map.into_iter().collect::<Vec<_>>();
        v.sort_by_key(|&(_, v)| v);
        v.into_iter().map(|(OrderedF32(k), _)| k).collect()
    }
    fn len(&self) -> u32 {
        self.map.len() as u32
    }
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
            Ast::X | Ast::Y | Ast::Z | Ast::DistToPoly(_) => 1,
            Ast::Constant(_) | Ast::Buffer(_) => 0,
            Ast::Transform { target, .. } => 1 + transform_depth(target),
            Ast::Sub(l, r) => max(transform_depth(l), transform_depth(r)),
            Ast::Mul(lst) | Ast::Add(lst) | Ast::Min(lst) | Ast::Max(lst) => {
                lst.iter().map(transform_depth).fold(0, max)
            }
            Ast::Square(t) | Ast::Abs(t) | Ast::Sqrt(t) | Ast::Neg(t) => transform_depth(t),
        }
    }
    fn depth(ast: &Ast) -> u32 {
        use std::cmp::max;
        match ast {
            Ast::X | Ast::Y | Ast::Z | Ast::Constant(_) | Ast::Buffer(_) => 1,
            Ast::DistToPoly(v) => v.len() as u32,
            Ast::Transform { target, .. } => depth(target),
            Ast::Sub(l, r) => max(depth(l), depth(r)) + 1,
            Ast::Mul(lst) | Ast::Add(lst) | Ast::Min(lst) | Ast::Max(lst) => {
                lst.iter().map(depth).fold(0, max) + 1
            }
            Ast::Square(t) | Ast::Abs(t) | Ast::Sqrt(t) | Ast::Neg(t) => depth(t),
        }
    }

    fn compile_inner(
        ast: &Ast,
        code: &mut Vec<u8>,
        constants: &mut ConstantCache,
        buffers: &mut Vec<Buffer>,
    ) {
        fn push_const(constant: f32, code: &mut Vec<u8>, constants: &mut ConstantCache) {
            if constants.len() >= 255 {
                panic!("large constants not supported yet")
            }
            code.push(constants.record(constant) as u8);
        }
        fn compile_inner_list(
            asts: &[Ast],
            code: &mut Vec<u8>,
            constants: &mut ConstantCache,
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
                code.push(ops::CONSTANT_SMALL);
                push_const(*c, code, constants);
            }
            Ast::DistToPoly(v) => {
                for (x1, y1, x2, y2) in v {
                    code.push(ops::DIST_TO_LINE);
                    push_const(*x1, code, constants);
                    push_const(*y1, code, constants);
                    push_const(*x2, code, constants);
                    push_const(*y2, code, constants);
                }
                code.push(ops::COLLECT_POLY);
                assert!(v.len() <= 255);
                code.push(v.len() as u8);
            }
            Ast::Transform { target, matrix } => {
                code.push(ops::PUSH_TRANSFORM);
                push_const(matrix.m11, code, constants);
                push_const(matrix.m12, code, constants);
                push_const(matrix.m13, code, constants);
                push_const(matrix.m14, code, constants);
                push_const(matrix.m21, code, constants);
                push_const(matrix.m22, code, constants);
                push_const(matrix.m23, code, constants);
                push_const(matrix.m24, code, constants);
                push_const(matrix.m31, code, constants);
                push_const(matrix.m32, code, constants);
                push_const(matrix.m33, code, constants);
                push_const(matrix.m34, code, constants);
                push_const(matrix.m41, code, constants);
                push_const(matrix.m42, code, constants);
                push_const(matrix.m43, code, constants);
                push_const(matrix.m44, code, constants);
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
            Ast::Square(t) => {
                compile_inner(t, code, constants, buffers);
                code.push(ops::SQUARE);
            }
        }
    }

    let mut code = vec![];
    let mut constants = ConstantCache::new();
    let max_stack = depth(ast);
    let transform_depth = transform_depth(ast);
    let mut buffers = vec![];
    compile_inner(ast, &mut code, &mut constants, &mut buffers);
    CompilationResult {
        code,
        constants: constants.to_vec(),
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
                1,
                1,
                1,
                0,
                1,
                1,
                1,
                1,
                0,
                1,
                2,
                2,
                2,
                0,
                ops::X,
                ops::Y,
                ops::MAX,
                ops::POP_TRANSFORM
            ],
            constants: vec![1.0, 0.0, 2.0],
            max_stack: 2,
            transform_depth: 2,
            buffers: vec![],
        }
    )
}
