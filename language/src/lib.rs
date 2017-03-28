#[macro_use]
extern crate implicit;
extern crate tendril;
extern crate typed_arena;
extern crate snoot;

mod errors;
mod properties;
#[cfg(test)]
mod test;

use tendril::StrTendril;
use implicit::nodes::Node;
use implicit::nodes::StaticNode;
use snoot::diagnostic::{DiagnosticBuilder, DiagnosticBag};
use snoot::parse::Span;
use snoot::Sexpr;
use snoot::simple_parse;

use self::errors::*;
use self::properties::*;

pub struct ParseResult {
    pub root: Option<StaticNode>,
    pub diagnostics: DiagnosticBag,
}

impl ParseResult {
    pub fn unwrap(self) -> StaticNode {
        self.diagnostics.assert_empty();
        self.root.unwrap()
    }
}

pub fn parse<'b, I: Into<StrTendril>>(input: I, filename: &'b str) -> ParseResult {
    let input: StrTendril = input.into();

    let snoot::Result { roots, mut diagnostics } = simple_parse(input, &[":"]);

    let root = match roots.len() {
        0 => {
            diagnostics.add(DiagnosticBuilder::new("completely empty programs are not allowed", &Span::empty()).build());
            diagnostics.set_filename(filename);
            return ParseResult {
                root: None,
                diagnostics: diagnostics,
            };
        }
        1 => roots.into_iter().next().unwrap(),
        n => {
            let mut iter = roots.into_iter();
            let first = iter.next().unwrap();
            let last = iter.last().unwrap();

            let span = Span::from_spans(first.span(), last.span());

            diagnostics.add(DiagnosticBuilder::new(format!("a program must only have one root, found {}", n), &span).build());
            first
        }
    };

    let node = create_node!(a, {
        parse_shape(&root, &a, &mut diagnostics)
    });

    diagnostics.set_filename(filename);
    ParseResult {
        root: Some(node),
        diagnostics: diagnostics,
    }
}

fn parse_shape<'o, F>(expr: &Sexpr,
                      a: &F,
                      errors: &mut DiagnosticBag)
                      -> Option<&'o Node<'o>>
    where F: Fn(Node<'o>) -> &'o Node<'o>
{


    match expr {
        &Sexpr::List { ref children, ref span, .. } => {
            if children.len() == 0 {
                errors.add(invalid_syntax(span));
                None
            } else {
                match &children[0] {
                    &Sexpr::List { ref span, .. } |
                    &Sexpr::UnaryOperator { ref span, .. } |
                    &Sexpr::String(_, ref span) => {
                        errors.add(invalid_shape_name(span));
                        None
                    }
                    &Sexpr::Terminal(_, ref namespan) => {
                        match namespan.text().as_ref() {
                            "circle" => {
                                parse_circle(&children[1..], span, errors).map(a)
                            },
                            "or" => {
                                make_combinator(&children[1..], Node::Or, span, a, errors)
                            }
                            "and" => {
                                make_combinator(&children[1..], Node::And, span, a, errors)
                            }
                            "not" => {
                                make_singular(&children[1..], Node::Not, span, a, errors)
                            }
                            "break" => {
                                make_singular(&children[1..], Node::Break, span, a, errors)
                            }
                            "freeze" => {
                                make_singular(&children[1..], Node::Freeze, span, a, errors)
                            }

                            other => {
                                errors.add(unrecognized_shape(namespan, other));
                                None
                            }
                        }
                    }
                }
            }
        }

        other => {
            errors.add(not_a_shape(other.span(), other.kind()));
            None
        }
    }
}

fn make_combinator<'o, F, A>(children: &[Sexpr], f: F, span: &Span, a: &A, errors: &mut DiagnosticBag) -> Option<&'o Node<'o>>
where A: Fn(Node<'o>) -> &'o Node<'o>, F: Fn(Vec<&'o Node<'o>>) -> Node<'o>
{
    if children.len() == 0 {
        errors.add(expected_children(span));
        None
    } else {
        let children = children.iter().filter_map(|c| parse_shape(c, a, errors));
        Some(a(f(children.collect())))
    }
}

fn make_singular<'o, F, A>(children: &[Sexpr], f: F, span: &Span, a: &A, errors: &mut DiagnosticBag) -> Option<&'o Node<'o>>
where A: Fn(Node<'o>) -> &'o Node<'o>, F: Fn(&'o Node<'o>) -> Node<'o>
{
    if children.len() == 0 {
        errors.add(expected_children(span));
        return None;
    } else if children.len() > 1 {
        errors.add(expected_one_child(span, children.len()));
    }
    let first = &children[0];
    match parse_shape(first, a, errors) {
        Some(c) => Some(a(f(c))),
        None => None
    }
}

/*
fn parse_polygon(children: &[Sexpr], span: &Span, errors: &mut Vec<DiagnosticBag>) -> Option<Node<'static>> {
    unimplemented!()
}
*/

fn parse_circle(children: &[Sexpr], span: &Span, errors: &mut DiagnosticBag) -> Option<Node<'static>> {
    macro_rules! attempt {
        ($v: expr, $default: expr) => {
            match $v {
                Ok(v) => v,
                Err(e) => {
                    errors.add(e);
                    $default
                }
            }
        };
    }

    if let Some(proplist) = children.get(0) {
        let (ok, proplist) = parse_properties(proplist, errors);
        if !ok { return None }
        let radius = proplist.get_number("radius", span);
        let radius = radius.or_else(|_| proplist.get_number("r", span));
        let radius = attempt!(radius, 10.0);
        let x = attempt!(proplist.get_number("x", span), 10.0);
        let y = attempt!(proplist.get_number("y", span), 10.0);
        Some(Node::Circle{r: radius, x: x, y: y})
    } else {
        errors.add(expected_property_list_exists(span));
        None
    }
}
