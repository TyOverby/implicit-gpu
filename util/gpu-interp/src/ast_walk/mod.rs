use super::Ast;

pub fn interpret(ast: &Ast, x: f32, y: f32, z: f32) -> f32 {
    match ast {
        Ast::Buffer(_) => unimplemented!(),
        Ast::DistToPoly(_) => unimplemented!(),
        Ast::Constant(c) => *c,
        Ast::Transform { target, matrix } => {
            let ::euclid::Point3D { x, y, z, .. } = matrix
                .transform_point3d(&::euclid::point3(x, y, z))
                .unwrap();
            interpret(target, x, y, z)
        }
        Ast::X => x,
        Ast::Y => y,
        Ast::Z => z,
        Ast::Add(list) => list.iter().map(|a| interpret(a, x, y, z)).sum(),
        Ast::Mul(list) => list.iter().map(|a| interpret(a, x, y, z)).product(),
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
        Ast::Neg(a) => -interpret(a, x, y, z),
        Ast::Sqrt(a) => interpret(a, x, y, z).sqrt(),
        Ast::Square(a) => {
            let v = interpret(a, x, y, z);
            v * v
        }
    }
}
