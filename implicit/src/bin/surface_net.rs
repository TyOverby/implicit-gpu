extern crate gpu_interp;
extern crate implicit;
extern crate typed_arena;

use gpu_interp::*;
use implicit::opencl::*;
use typed_arena::Arena;

fn sphere<'a>(x: f32, y: f32, z: f32, r: f32, arena: &'a Arena<Ast<'a>>) -> Ast<'a> {
    let dx = Ast::Sub(arena.alloc(Ast::X), arena.alloc(Ast::Constant(x)));
    let dy = Ast::Sub(arena.alloc(Ast::Y), arena.alloc(Ast::Constant(y)));
    let dz = Ast::Sub(arena.alloc(Ast::Z), arena.alloc(Ast::Constant(z)));

    let dx2 = Ast::Square(arena.alloc(dx));
    let dy2 = Ast::Square(arena.alloc(dy));
    let dz2 = Ast::Square(arena.alloc(dz));

    let dx2_plus_dy2_plus_dz2 = Ast::Add(arena.alloc_extend(vec![dx2, dy2, dz2]));
    let sqrt = Ast::Sqrt(arena.alloc(dx2_plus_dy2_plus_dz2));
    Ast::Sub(arena.alloc(Ast::Constant(r)), arena.alloc(sqrt))
}

fn main() {
    let arena = Arena::new();
    let ctx = OpenClContext::default();
    let program = sphere(10.0, 10.0, 10.0, 5.0, &arena);
    let compiled = ::gpu_interp::compile(&program);
    let mut buf = ::gpu_interp::execute(
        compiled,
        20,
        20,
        20,
        ::gpu_interp::Triad {
            context: ctx.context().clone(),
            queue: ctx.queue().clone(),
        },
    );
    let field_buffer = FieldBuffer {
        dims: (20, 20, 20),
        internal: buf.to_opencl(ctx.queue()).clone(),
    };
    let (triangle_buffer, count) = implicit::surface_net::run_surface_net(&field_buffer, &ctx);
    let triangle_buffer = triangle_buffer.values(Some(count));
    for slice in triangle_buffer.chunks(9) {
        for point in slice.chunks(3) {
            println!("{}, {}, {}", point[0], point[1], point[2]);
        }
        println!();
    }
}
