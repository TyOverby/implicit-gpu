extern crate buffer_dump;
extern crate euclid;
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
    sub
}

fn torus<'a>(_x: f32, _y: f32, _z: f32, r: f32, a: f32, arena: &'a Arena<Ast<'a>>) -> Ast<'a> {
    let x2 = Ast::Square(arena.alloc(Ast::X));
    let y2 = Ast::Square(arena.alloc(Ast::Y));
    let z2 = Ast::Square(arena.alloc(Ast::Z));
    let r2 = Ast::Square(arena.alloc(Ast::Constant(r)));
    let a2 = Ast::Square(arena.alloc(Ast::Constant(a)));

    let lhs = Ast::Add(arena.alloc_extend(vec![
        x2.clone(),
        y2.clone(),
        z2,
        r2.clone(),
        Ast::Neg(arena.alloc(a2)),
    ]));
    let lhs = Ast::Square(arena.alloc(lhs));

    let rhs = Ast::Mul(arena.alloc_extend(vec![
        Ast::Constant(4.0),
        r2,
        Ast::Add(arena.alloc_extend(vec![x2, y2])),
    ]));

    Ast::Sub(arena.alloc(lhs), arena.alloc(rhs))
    //Ast::Neg(arena.alloc(Ast::Sub(arena.alloc(lhs), arena.alloc(rhs))))
}

fn main() {
    let arena = Arena::new();
    let ctx = OpenClContext::default();
    let factor: u32 = 3;
    /*
    let main = sphere(10.0, 10.0, 10.0, 5.0, &arena);
    let cutout = sphere(12.0, 12.0, 12.0, 3.0, &arena);
    let cutout = Ast::Mul(arena.alloc_extend(vec![cutout, Ast::Constant(-1.0)]));
    let program = Ast::Max(arena.alloc_extend(vec![main, cutout]));
    let program = Ast::Transform {
        target: arena.alloc(program),
        matrix: euclid::Transform3D::create_scale(
            1.0 / (factor as f32),
            1.0 / (factor as f32),
            1.0 / (factor as f32),
        ),
    };*/
    let torus = arena.alloc(torus(20.0, 20.0, 20.0, 25.0, 10.0, &arena));
    let a = Ast::Transform {
        target: torus,
        matrix: euclid::Transform3D::create_rotation(1.0, 0.0, 0.0, euclid::Angle::radians(1.5708)),
    };
    let b = Ast::Transform {
        target: torus,
        matrix: euclid::Transform3D::create_rotation(0.0, 1.0, 0.0, euclid::Angle::radians(1.5708)),
    };
    let c = Ast::Transform {
        target: torus,
        matrix: euclid::Transform3D::create_rotation(0.0, 0.0, 1.0, euclid::Angle::radians(1.5708)),
    };
    let _sphere = sphere(0.0, 0.0, 0.0, 5.0, &arena);
    let program = Ast::Transform {
        target: arena.alloc(Ast::Max(arena.alloc_extend(vec![a, /*sphere,*/ b, c]))),
        matrix: euclid::Transform3D::create_translation(-40.0, -40.0, -40.0),
    };
    eprintln!("compiled: {:#?}", program);
    let compiled = ::gpu_interp::gpu::compile(&program);
    let mut field_buffer = ::gpu_interp::gpu::execute(
        compiled,
        40 * factor,
        40 * factor,
        40 * factor,
        ::gpu_interp::gpu::Triad {
            context: ctx.context().clone(),
            queue: ctx.queue().clone(),
        },
    );
    let mut out_dbg = std::io::BufWriter::new(std::fs::File::create("out.buf").unwrap());
    buffer_dump::write(&mut out_dbg, &mut field_buffer).unwrap();

    let (index_buffer, count, mut pos_buffer, mut normal_buffer) =
        implicit::surface_net::run_surface_net(&mut field_buffer, &ctx);
    let index_buffer = index_buffer.values(Some(count));
    let pos_buffer = pos_buffer.to_memory();
    let normal_buffer = normal_buffer.to_memory();

    /*
    for position in pos_buffer.chunks(3) {
        if !position[0].is_nan() {
            eprintln!("p {} {} {}", position[0], position[1], position[2]);
        }
    }
    */

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
