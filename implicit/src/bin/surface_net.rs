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
    let sub = Ast::Sub(arena.alloc(Ast::Constant(r)), arena.alloc(sqrt));
    Ast::Mul(arena.alloc_extend(vec![sub, Ast::Constant(1.0)]))
}

fn main() {
    let arena = Arena::new();
    let ctx = OpenClContext::default();
    let main = sphere(10.0, 10.0, 10.0, 5.0, &arena);
    let cutout = sphere(12.0, 12.0, 12.0, 3.0, &arena);
    let cutout = Ast::Mul(arena.alloc_extend(vec![cutout, Ast::Constant(-1.0)]));
    let program = Ast::Max(arena.alloc_extend(vec![main, cutout]));
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
    let (index_buffer, count, pos_buffer, normal_buffer) =
        implicit::surface_net::run_surface_net(&field_buffer, &ctx);
    let index_buffer = index_buffer.values(Some(count));
    let pos_buffer = pos_buffer.values();
    let normal_buffer = normal_buffer.values();

    for position in pos_buffer.chunks(3) {
        if !position[0].is_nan() {
            eprintln!("p {} {} {}", position[0], position[1], position[2]);
        }
    }

    let mut max_x = std::f32::NEG_INFINITY;
    let mut max_y = std::f32::NEG_INFINITY;
    let mut max_z = std::f32::NEG_INFINITY;

    let mut min_x = std::f32::INFINITY;
    let mut min_y = std::f32::INFINITY;
    let mut min_z = std::f32::INFINITY;

    println!("solid test");
    for slice in index_buffer.chunks(3) {
        let a = slice[0] as usize * 3;
        let b = slice[1] as usize * 3;
        let c = slice[2] as usize * 3;

        let pa_x = pos_buffer[a + 0];
        let pa_y = pos_buffer[a + 1];
        let pa_z = pos_buffer[a + 2];

        let pb_x = pos_buffer[b + 0];
        let pb_y = pos_buffer[b + 1];
        let pb_z = pos_buffer[b + 2];

        let pc_x = pos_buffer[c + 0];
        let pc_y = pos_buffer[c + 1];
        let pc_z = pos_buffer[c + 2];

        max_x = max_x.max(pa_x).max(pb_x).max(pc_x);
        max_y = max_y.max(pa_y).max(pb_y).max(pc_y);
        max_z = max_z.max(pa_z).max(pb_z).max(pc_z);

        min_x = min_x.min(pa_x).min(pb_x).min(pc_x);
        min_y = min_y.min(pa_y).min(pb_y).min(pc_y);
        min_z = min_z.min(pa_z).min(pb_z).min(pc_z);

        let na_x = normal_buffer[a + 0];
        let na_y = normal_buffer[a + 1];
        let na_z = normal_buffer[a + 2];

        let nb_x = normal_buffer[b + 0];
        let nb_y = normal_buffer[b + 1];
        let nb_z = normal_buffer[b + 2];

        let nc_x = normal_buffer[c + 0];
        let nc_y = normal_buffer[c + 1];
        let nc_z = normal_buffer[c + 2];

        let norm_x = na_x + nb_x + nc_x;
        let norm_y = na_y + nb_y + nc_y;
        let norm_z = na_z + nb_z + nc_z;

        let norm_len = (norm_x * norm_x + norm_y * norm_y + norm_z * norm_z).sqrt();
        let norm_x = norm_x / norm_len;
        let norm_y = norm_y / norm_len;
        let norm_z = norm_z / norm_len;

        println!("\tfacet normal {} {} {}", norm_x, norm_y, norm_z);
        println!("\t\touter loop");
        println!("\t\t\tvertex {} {} {}", pa_x, pa_y, pa_z);
        println!("\t\t\tvertex {} {} {}", pb_x, pb_y, pb_z);
        println!("\t\t\tvertex {} {} {}", pc_x, pc_y, pc_z);
        println!("\t\tendloop");
        println!("\tendfacet")
    }
    println!("endsolid test");

    eprintln!("maxes {} {} {}", max_x, max_y, max_z);
    eprintln!("mins {} {} {}", min_x, min_y, min_z);
}
