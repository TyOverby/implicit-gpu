use gpu_interp::Ast;
use ocaml::{Shape, Terminal};
use typed_arena::Arena;

pub fn compile<'a>(shape: &Shape, arena: &'a Arena<Ast<'a>>) -> Ast<'a> {
    match shape {
        Shape::Terminal(Terminal::Circle(c)) => {
            let dx = Ast::Sub(arena.alloc(Ast::X), arena.alloc(Ast::Constant(c.x)));
            let dy = Ast::Sub(arena.alloc(Ast::Y), arena.alloc(Ast::Constant(c.y)));
            // TODO: don't duplicate this entire branch.
            let dx2 = Ast::Square(arena.alloc(dx));
            let dy2 = Ast::Square(arena.alloc(dy));
            let dx2_plus_dy2 = Ast::Add(arena.alloc_extend(vec![dx2, dy2]));
            let sqrt = Ast::Sqrt(arena.alloc(dx2_plus_dy2));
            Ast::Sub(arena.alloc(Ast::Constant(c.r)), arena.alloc(sqrt))
        }
        Shape::Terminal(_) => unimplemented!(),
        Shape::Not(target) => {
            let child = compile(target, arena);
            Ast::Neg(arena.alloc(child))
        }
        Shape::Union(shapes) => {
            let children = arena.alloc_extend(shapes.into_iter().map(|s| compile(s, arena)));
            Ast::Max(children)
        }
        Shape::Intersection(shapes) => {
            let children = arena.alloc_extend(shapes.into_iter().map(|s| compile(s, arena)));
            Ast::Max(children)
        }
        Shape::Modulate(target, how_much) => {
            let child = compile(target, arena);
            Ast::Add(arena.alloc_extend(vec![child, Ast::Constant(*how_much)].into_iter()))
        }
        Shape::Transform(target, matrix) => {
            let child = compile(target, arena);
            Ast::Transform {
                target: arena.alloc(child),
                matrix: matrix.to_3d().inverse().unwrap(),
            }
        }
    }
}
