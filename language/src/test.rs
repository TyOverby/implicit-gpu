
use super::*;
use implicit::nodes::StaticNode;

fn parse_ok(text: &str) -> StaticNode {
    let ParseResult { root, diagnostics } = parse(text, "");
    diagnostics.assert_empty();
    return root.unwrap();
}

#[test]
fn correct_circle() {
    let actual = create_node!(
        a, {
            a(Node::Circle { x: 50.0, y: 20.0, r: 10.0 })
        }
    );

    assert_eq!(actual, parse_ok("(circle {r: 10 x: 50 y: 20})"));
}

/*
#[test]
fn correct_rect() {
    let actual = create_node!(a, {
        a(Node::Rect {
              x: 50.0,
              y: 20.0,
              w: 10.0,
              h: 30.0,
          })
    });

    assert_eq!(actual, parse_ok("(rect {x:50 y:20 w:10 h:30})"));
}*/

#[test]
fn correct_or() {
    let actual = create_node!(
        a, {
            a(Node::Or(vec![a(Node::Circle { x: 10.0, y: 10.0, r: 10.0 }), a(Node::Circle { x: 20.0, y: 20.0, r: 20.0 })],),)
        }
    );

    assert_eq!(actual, parse_ok("(or (circle {x:10 y:10 r:10}) (circle {x:20 y:20 r:20}))"));
}

#[test]
fn correct_break() {
    let actual = create_node!(
        a, {
            a(Node::Break(a(Node::Circle { x: 10.0, y: 10.0, r: 10.0 })))
        }
    );

    assert_eq!(actual, parse_ok("(break (circle {x:10 y:10 r:10}))"));
}

#[test]
fn correct_freeze() {
    let actual = create_node!(
        a, {
            a(Node::Freeze(a(Node::Circle { x: 10.0, y: 10.0, r: 10.0 })))
        }
    );

    assert_eq!(actual, parse_ok("(freeze (circle {x:10 y:10 r:10}))"));
}

#[test]
fn basic_polygon() {
    let actual = create_node!(
        a, {
            a(Node::Polygon(PolyGroup::single_additive(vec![5.0, 5.0], vec![10.0, 10.0])))
        }
    );

    assert_eq!(actual, parse_ok("(polygon {x: 5 y: 10})"));
}

#[test]
fn more_complexicated_polygon() {
    let actual = create_node!(
        a, {
            a(Node::Polygon(PolyGroup::single_additive(vec![5.0, 15.0, 15.0, 5.0], vec![10.0, 20.0, 20.0, 10.0]),),)
        }
    );

    assert_eq!(actual, parse_ok("(polygon {x: 5 y: 10} {x: 15.0 y: 20.0})"));
}

#[test]
fn even_more_complexicated_polygon() {
    let actual = create_node!(
        a, {
            a(Node::Polygon(PolyGroup::single_additive(vec![5.0, 15.0, 15.0, 30.0, 30.0, 5.0], vec![10.0, 20.0, 20.0, 50.0, 50.0, 10.0]),),)
        }
    );

    assert_eq!(actual, parse_ok("(polygon {x: 5 y: 10} {x: 15 y: 20} {x: 30 y: 50})"));
}

#[test]
fn test_grow_shrink() {
    let actual = create_node!(
        a, {
            a(
                Node::Modulate(
                    -13.0,
                    a(Node::Polygon(PolyGroup::single_additive(vec![5.0, 15.0, 15.0, 30.0, 30.0, 5.0], vec![10.0, 20.0, 20.0, 50.0, 50.0, 10.0]),),),
                ),
            )
        }
    );
    assert_eq!(actual, parse_ok("(grow 13 (polygon {x: 5 y: 10} {x: 15 y: 20} {x: 30 y: 50}))"));

    let actual = create_node!(
        a, {
            a(
                Node::Modulate(
                    13.0,
                    a(Node::Polygon(PolyGroup::single_additive(vec![5.0, 15.0, 15.0, 30.0, 30.0, 5.0], vec![10.0, 20.0, 20.0, 50.0, 50.0, 10.0]),),),
                ),
            )
        }
    );
    assert_eq!(
        actual,
        parse_ok("(shrink 13 (polygon {x: 5 y: 10} {x: 15 y: 20} {x: 30 y: 50}))"),
    );
}

#[test]
fn subtraction() {
    let actual = create_node!(
        a, {
            a(Node::Circle { x: 0.0, y: 0.0, r: 30.0 })
        }
    );
    assert_eq!(actual, parse_ok("(subtract (circle {x:0 y:0 r:30}))"));

    let actual = create_node!(
        a, {
            a(Node::And(vec![a(Node::Circle { x: 0.0, y: 0.0, r: 30.0 }), a(Node::Not(a(Node::Circle { x: 0.0, y: 0.0, r: 10.0 })))],),)
        }
    );
    assert_eq!(actual, parse_ok("(subtract (circle {x:0 y:0 r:30}) (circle {x:0 y:0 r:10}))"));
}

#[test]
fn not() {
    let actual = create_node!(
        a, {
            a(Node::Not(a(Node::Circle { x: 0.0, y: 0.0, r: 30.0 })))
        }
    );
    assert_eq!(actual, parse_ok("(not (circle {x:0 y:0 r:30}))"));
}
