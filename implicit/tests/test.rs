#[macro_use]
extern crate implicit_gpu;
extern crate typed_arena;

mod util;

use implicit_gpu::nodes::{Node, PolyGroup};

#[test]
fn single_circle() {
    let node = create_node!(a, {
        a(Node::Circle{ x: 50.0, y: 50.0, r: 50.0 })
    });

    util::run_test("circles", node.node(), 100, 100);
}

#[test]
fn circles_or() {
    let node = create_node!(a, {
        a(Node::Or(vec![
            a(Node::Circle{ x: 50.0, y: 50.0, r: 50.0 }),
            a(Node::Circle{ x: 100.0, y: 100.0, r: 50.0 })
        ]))
    });

    util::run_test("circles_or", node.node(), 150, 150)
}

#[test]
fn circles_and() {
    let node = create_node!(a, {
        a(Node::And(vec![
            a(Node::Circle{ x: 50.0, y: 50.0, r: 50.0 }),
            a(Node::Circle{ x: 100.0, y: 100.0, r: 50.0 })
        ]))
    });

    util::run_test("circles_and", node.node(), 150, 150);
}

fn poly() -> PolyGroup {
    PolyGroup {
        additive: vec![
            vec![(0.0, 0.0), (300.0, 0.0), (400.0, 300.0), (200.0, 100.0)]
                .into_iter().map(|(x, y)| {(x / 2.0 + 50.0, y / 2.0 + 50.0)}).collect()
        ],
        subtractive: vec![],
    }
}

#[test]
fn simple_polygon() {
    let node = create_node!(a, {
        a(Node::Polygon(poly()))
    });

    util::run_test("simple_polygon", node.node(), 300, 250);
}

#[test]
fn poly_ops() {
    let node = create_node!(a, {
        a(Node::And(vec![
            a(Node::Circle{ x: 175.0, y: 80.0, r: 75.0 }),
            a(Node::Polygon(poly()))
        ]))
    });

    util::run_test("poly_ops", node.node(), 300, 250);
}

#[test]
fn frozen_circle() {
    let node = create_node!(a, {
        a(Node::Freeze(a(Node::Circle{x: 55.0, y: 55.0, r: 50.0})))
    });

    util::run_test("frozen_circle", node.node(), 110, 110);
}
