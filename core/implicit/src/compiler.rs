use crate::opencl::FieldBuffer;
use extern_api::{Id, Shape, Terminal};
use gpu_interp::Ast;
use typed_arena::Arena;

pub fn compile<'a, F>(shape: &Shape, arena: &'a Arena<Ast<'a>>, find_buffer: &F) -> Ast<'a>
where
    F: Fn(Id) -> FieldBuffer,
{
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
        Shape::Terminal(Terminal::Field(id)) => {
            let buffer = find_buffer(*id);
            Ast::Buffer(buffer)
        }
        Shape::Terminal(Terminal::Rect(rect)) => {
            let ::extern_api::Rect { x, y, w, h } = *rect;
            let top = (x, y, x + w, y);
            let right = (x + w, y, x + w, y + h);
            let bot = (x + w, y + h, x, y + h);
            let left = (x, y + h, x, y);
            Ast::DistToPoly(vec![top, right, bot, left])
        }
        Shape::Not(target) => {
            let child = compile(target, arena, find_buffer);
            Ast::Neg(arena.alloc(child))
        }
        Shape::Union(shapes) => {
            let children = shapes
                .into_iter()
                .map(|s| compile(s, arena, find_buffer))
                .collect::<Vec<_>>();
            Ast::Min(arena.alloc_extend(children))
        }
        Shape::Intersection(shapes) => {
            let children = shapes
                .into_iter()
                .map(|s| compile(s, arena, find_buffer))
                .collect::<Vec<_>>();
            Ast::Max(arena.alloc_extend(children))
        }
        Shape::Modulate(target, how_much) => {
            let child = compile(target, arena, find_buffer);
            Ast::Add(arena.alloc_extend(vec![child, Ast::Constant(-*how_much)].into_iter()))
        }
        Shape::Transform(target, matrix) => {
            let child = compile(target, arena, find_buffer);
            Ast::Transform {
                target: arena.alloc(child),
                matrix: matrix.inverse().unwrap().to_3d(),
            }
        }
    }
}
