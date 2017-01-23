#[macro_use]
extern crate implicit;
extern crate typed_arena;
extern crate ocl;
extern crate flame;

use implicit::nodes::*;
use implicit::compiler::*;
use implicit::opencl::OpenClContext;
use implicit::evaluator::Evaluator;
use implicit::image::{save_field_buffer, ColorMode};

fn basic_poly() -> PolyGroup {
    PolyGroup {
        additive: vec![
            vec![(0.0, 0.0), (300.0, 0.0), (400.0, 300.0), (200.0, 100.0)].into_iter().collect()
        ],
        subtractive: vec![],
    }
}

fn main() {
    let ctx = OpenClContext::default();

    // Build a node tree
    let stat = create_node!(a, {
        a(Node::Or(vec![
            a(Node::Modulate(-20.0,
                a(Node::And(vec![
                    a(Node::Circle{ x: 50.0, y: 50.0, r: 50.0 }),
                    a(Node::Break(a(Node::Not(a(Node::Circle{ x: 100.0, y: 100.0, r: 50.0 }))))),
                ]))
            )),
            a(Node::Polygon(basic_poly()))
        ]))
    });


    // Group them into a nest
    let mut nest = Nest::new();
    let target = nest.group(stat.node());

    // Create a new Execution Context
    let evaluator = Evaluator::new(nest, 1000, 1000, None);
    let result = evaluator.evaluate(target, &ctx);

    save_field_buffer(&result, "out-field.png", ColorMode::BlackAndWhite);

    // Print the timings
    ::flame::dump_stdout();
}
