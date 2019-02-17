mod drag;
mod extract;
mod freeze;
mod noise;
mod poly;
mod shape;

pub use self::drag::*;
pub use self::extract::*;
pub use self::freeze::*;
pub use self::noise::*;
pub use self::poly::*;
pub use self::shape::*;

use expectation_plugin::expectation_test;
use geometry::PathSegment;
use inspector::*;
use ocaml::*;
use opencl::{FieldBuffer, OpenClContext};
use std::collections::HashMap;

#[cfg(test)]
use expectation::{extensions::TextDiffExtension, Provider};

pub fn exec(
    command: Command,
    inspector: BoxedInspector,
    width: u32,
    height: u32,
) -> HashMap<Id, Vec<PathSegment>> {
    let ctx = OpenClContext::default();
    let mut mapping = HashMap::new();
    let mut output = HashMap::new();
    exec_inner(
        &ctx,
        command,
        &mut mapping,
        &mut output,
        inspector,
        width,
        height,
    );
    output
}

fn exec_inner(
    ctx: &OpenClContext,
    command: Command,
    mapping: &mut HashMap<Id, FieldBuffer>,
    output: &mut HashMap<Id, Vec<PathSegment>>,
    inspector: BoxedInspector,
    width: u32,
    height: u32,
) {
    match command {
        Command::Simplex(id, simplex) => {
            let mut field = get_noise(ctx, width, height, simplex.cutoff, simplex.matrix);
            inspector.write_field(&format!("simplex_{}", id), &mut field);
            mapping.insert(id, field);
        }
        Command::Define(id, Value::BasicShape(shape)) => {
            let mut field = exec_shape(ctx, inspector.duplicate(), shape, width, height, |id| {
                mapping[&id].clone()
            });
            inspector.write_field(&format!("shape_{}", id), &mut field);
            mapping.insert(id, field);
        }
        Command::Define(id, Value::Polygon(poly)) => {
            let mut field = exec_poly(ctx, poly, width, height);
            inspector.write_field(&format!("poly_{}", id), &mut field);
            mapping.insert(id, field);
        }
        Command::Freeze { target, id } => {
            let field = {
                let buffer = mapping.get_mut(&target).unwrap();
                let mut field = exec_freeze(ctx, buffer);
                inspector.write_field(&format!("freeze_{}", id), &mut field);
                field
            };
            mapping.insert(id, field);
        }
        Command::Drag { target, id, dx, dy } => {
            let field = {
                let buffer = mapping.get_mut(&target).unwrap();
                let mut field = exec_drag(ctx, buffer, dx, dy);
                inspector.write_field(&format!("drag_{}", id), &mut field);
                field
            };
            mapping.insert(id, field);
        }
        Command::Concurrently(commands) | Command::Serially(commands) => {
            for (i, command) in commands.into_iter().enumerate() {
                exec_inner(
                    ctx,
                    command,
                    mapping,
                    output,
                    inspector.specialize(&format!("instr_{}", i)),
                    width,
                    height,
                );
            }
        }
        Command::Export(id) => {
            let buffer = mapping.get_mut(&id).unwrap();
            let lines = extract_lines(ctx, inspector, buffer);
            output.insert(id, lines);
        }
    }
}

#[expectation_test]
fn exec_program_single(provider: Provider) {
    use debug::print_path_segments;
    use ocaml::*;

    let shape = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));

    let program = Command::Serially(vec![
        Command::Define(0, Value::BasicShape(shape)),
        Command::Export(0),
    ]);

    let out = exec(program, provider.duplicate(), 22, 22);
    for (id, lines) in out {
        let writer = provider.text_writer(format!("export_{}.lines.txt", id));
        print_path_segments(writer, &lines);
    }
}

#[expectation_test]
fn exec_program_with_multiple(provider: Provider) {
    use debug::print_path_segments;
    use euclid::*;
    use ocaml::*;

    let shape = Shape::Terminal(Terminal::Circle(Circle {
        x: 11.0,
        y: 11.0,
        r: 10.0,
    }));

    let polygon = Polygon {
        points: vec![
            point2(1.0, 1.0),
            point2(15.0, 1.0),
            point2(15.0, 1.0),
            point2(15.0, 15.0),
            point2(15.0, 15.0),
            point2(1.0, 1.0),
        ],
        matrix: Matrix::identity(),
    };

    let combiner = Shape::Intersection(vec![
        Shape::Terminal(Terminal::Field(0)),
        Shape::Terminal(Terminal::Field(1)),
    ]);

    let program = Command::Serially(vec![
        Command::Define(0, Value::BasicShape(shape)),
        Command::Define(1, Value::Polygon(polygon)),
        Command::Define(2, Value::BasicShape(combiner)),
        Command::Export(0),
        Command::Export(1),
        Command::Export(2),
    ]);

    let out = exec(program, provider.duplicate(), 22, 22);
    for (id, lines) in out {
        let writer = provider.text_writer(format!("export_{}.lines.txt", id));
        print_path_segments(writer, &lines);
    }
}

#[expectation_test]
fn exec_program_single_noise(provider: Provider) {
    use debug::print_path_segments;
    use ocaml::*;

    let program = Command::Serially(vec![
        Command::Simplex(
            0,
            Simplex {
                cutoff: 0.5,
                matrix: Matrix::identity(),
            },
        ),
        Command::Export(0),
    ]);

    let out = exec(program, provider.duplicate(), 100, 100);
    for (id, lines) in out {
        let writer = provider.text_writer(format!("export_{}.lines.txt", id));
        print_path_segments(writer, &lines);
    }
}
