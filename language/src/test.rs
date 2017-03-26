use implicit::nodes::StaticNode;
use super::*;

fn parse_ok(text: &str) -> StaticNode {
    let ParseResult { root, diagnostics } = parse(text, "");
    diagnostics.assert_empty();
    return root.unwrap();
}

#[test]
fn correct_circle() {
    let actual = create_node!(a, {
        a(Node::Circle {
              x: 50.0,
              y: 20.0,
              r: 10.0,
          })
    });

    assert_eq!(actual, parse_ok("(circle {r: 10 x: 50 y: 20})"));
}

#[test]
fn correct_or() {
    let actual = create_node!(a, {
        a(Node::Or(vec![a(Node::Circle {
                              x: 10.0,
                              y: 10.0,
                              r: 10.0,
                          }),
                        a(Node::Circle {
                              x: 20.0,
                              y: 20.0,
                              r: 20.0,
                          })]))
    });

    assert_eq!(actual,
               parse_ok("(or (circle {x:10 y:10 r:10}) (circle {x:20 y:20 r:20}))"));
}

#[test]
fn correct_break() {
    let actual = create_node!(a, {
        a(Node::Break(a(Node::Circle {
                            x: 10.0,
                            y: 10.0,
                            r: 10.0,
                        })))
    });

    assert_eq!(actual, parse_ok("(break (circle {x:10 y:10 r:10}))"));
}

#[test]
fn correct_freeze() {
    let actual = create_node!(a, {
        a(Node::Freeze(a(Node::Circle {
                             x: 10.0,
                             y: 10.0,
                             r: 10.0,
                         })))
    });

    assert_eq!(actual, parse_ok("(freeze (circle {x:10 y:10 r:10}))"));
}

