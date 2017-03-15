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
use snoot::error::{Error, ErrorBuilder};
use snoot::parse::{Sexpr, Span};
use snoot::simple_parse;

use self::errors::*;
use self::properties::*;

pub struct ParseResult {
    pub root: Option<StaticNode>,
    pub errors: Vec<Error<StrTendril>>,
}

pub fn parse<'b, I: Into<StrTendril>>(input: I, filename: &'b str) -> ParseResult {
    let input: StrTendril = input.into();

    let snoot::parse::ParseResult { roots, diagnostics } = simple_parse(input, &[":"]);
    let mut errors: Vec<_> =
        diagnostics.into_iter().map(|d| d.into_error(Some(filename.into()))).collect();
    let mut error_builders = vec![];

    let root = match roots.len() {
        0 => {
            error_builders.push(ErrorBuilder::new("completely empty programs are not allowed", &Span::empty()));
            return ParseResult {
                root: None,
                errors: errors,
            };
        }
        1 => roots.into_iter().next().unwrap(),
        n => {
            let mut iter = roots.into_iter();
            let first = iter.next().unwrap();
            let last = iter.last().unwrap();

            let span = Span::from_spans(first.span(), last.span());

            error_builders.push(ErrorBuilder::new(format!("a program must only have one root, found {}", n), &span));
            first
        }
    };

    let node = create_node!(a, {
        parse_shape(&root, &a, &mut error_builders)
    });

    for eb in error_builders {
        errors.push(eb.with_file_name(filename).build());
    }

    ParseResult {
        root: Some(node),
        errors: errors,
    }
}

fn parse_shape<'o, F>(expr: &Sexpr<StrTendril>,
                      a: &F,
                      errors: &mut Vec<ErrorBuilder<StrTendril>>)
                      -> Option<&'o Node<'o>>
    where F: Fn(Node<'o>) -> &'o Node<'o>
{


    match expr {
        &Sexpr::List { ref children, ref span, .. } => {
            if children.len() == 0 {
                errors.push(invalid_syntax(span));
                None
            } else {
                match &children[0] {
                    &Sexpr::List { ref span, .. } |
                    &Sexpr::UnaryOperator { ref span, .. } |
                    &Sexpr::String(_, ref span) => {
                        errors.push(invalid_shape_name(span));
                        None
                    }
                    &Sexpr::Terminal(ref token, ref namespan) => {
                        match token.string.as_ref() {
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
                                errors.push(unrecognized_shape(namespan, other));
                                None
                            }
                        }
                    }
                }
            }
        }

        other => {
            errors.push(not_a_shape(other.span(), other.kind()));
            None
        }
    }
}

fn make_combinator<'o, F, A>(children: &[Sexpr<StrTendril>], f: F, span: &Span<StrTendril>, a: &A, errors: &mut Vec<ErrorBuilder<StrTendril>>) -> Option<&'o Node<'o>>
where A: Fn(Node<'o>) -> &'o Node<'o>, F: Fn(Vec<&'o Node<'o>>) -> Node<'o>
{
    if children.len() == 0 {
        errors.push(expected_children(span));
        None
    } else {
        let children = children.iter().filter_map(|c| parse_shape(c, a, errors));
        Some(a(f(children.collect())))
    }
}

fn make_singular<'o, F, A>(children: &[Sexpr<StrTendril>], f: F, span: &Span<StrTendril>, a: &A, errors: &mut Vec<ErrorBuilder<StrTendril>>) -> Option<&'o Node<'o>>
where A: Fn(Node<'o>) -> &'o Node<'o>, F: Fn(&'o Node<'o>) -> Node<'o>
{
    if children.len() == 0 {
        errors.push(expected_children(span));
        return None;
    } else if children.len() > 1 {
        errors.push(expected_one_child(span, children.len()));
    }
    let first = &children[0];
    match parse_shape(first, a, errors) {
        Some(c) => Some(a(f(c))),
        None => None
    }
}

fn parse_circle(children: &[Sexpr<StrTendril>], span: &Span<StrTendril>, errors: &mut Vec<ErrorBuilder<StrTendril>>) -> Option<Node<'static>> {
    macro_rules! attempt {
        ($v: expr, $default: expr) => {
            match $v {
                Ok(v) => v,
                Err(e) => {
                    errors.push(e);
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
        errors.push(expected_property_list_exists(span));
        None
    }
}
