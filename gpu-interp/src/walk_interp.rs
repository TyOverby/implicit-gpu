use super::Ast;

pub fn interpret(ast: &Ast, x: f32, y: f32, z: f32) -> f32 {
    match ast {
        Ast::Constant(c) => *c,
        Ast::Buffer(_) => unimplemented!(),
        Ast::Transform { .. } => unimplemented!(),
        Ast::X => x,
        Ast::Y => y,
        Ast::Z => z,
        Ast::Add(list) => list.iter().map(|a| interpret(a, x, y, z)).sum(),
        Ast::Sub(l, r) => interpret(l, x, y, z) - interpret(r, x, y, z),
        Ast::Max(list) => {
            if list.len() == 0 {
                panic!()
            }
            list.iter()
                .map(|a| interpret(a, x, y, z))
                .fold(::std::f32::MIN, |a, b| a.max(b))
        }
        Ast::Min(list) => {
            if list.len() == 0 {
                panic!()
            }
            list.iter()
                .map(|a| interpret(a, x, y, z))
                .fold(::std::f32::MAX, |a, b| a.max(b))
        }
        Ast::Abs(a) => interpret(a, x, y, z).abs(),
        Ast::Sqrt(a) => interpret(a, x, y, z).sqrt(),
    }
}
