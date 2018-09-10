mod extract;
mod freeze;
mod noise;
mod poly;
mod shape;

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
    width: usize,
    height: usize,
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
    width: usize,
    height: usize,
) {
    match command {
        Command::Define(id, Value::BasicShape(shape)) => {
            let field = exec_shape(ctx, shape, width, height, |id| mapping[&id].clone());
            inspector.write_field(&format!("shape_{}", id), &field);
            mapping.insert(id, field);
        }
        Command::Define(id, Value::Polygon(poly)) => {
            let field = exec_poly(ctx, poly, width, height);
            inspector.write_field(&format!("poly_{}", id), &field);
            mapping.insert(id, field);
        }
        Command::Freeze { target, id } => {
            let field = exec_freeze(ctx, &mapping[&target]);
            inspector.write_field(&format!("freeze_{}", id), &field);
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
            let lines = extract_lines(ctx, inspector, &mapping[&id]);
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
