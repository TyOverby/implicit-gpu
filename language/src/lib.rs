#[macro_use]
extern crate implicit;
extern crate tendril;
extern crate typed_arena;
#[macro_use]
extern crate snoot;

mod errors;
mod properties;
#[cfg(test)]
mod test;

use tendril::StrTendril;
use implicit::nodes::{Node, StaticNode, PolyGroup};
use snoot::diagnostic::{DiagnosticBag};
use snoot::parse::Span;
use snoot::Sexpr;
use snoot::simple_parse;

use self::errors::*;
use self::properties::*;

macro_rules! attempt {
    ($v: expr, $default: expr, $errors: expr) => {
        match $v {
            Ok(v) => v,
            Err(e) => {
                $errors.add(e);
                $default
            }
        }
    };
}

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

    let snoot::Result { roots, mut diagnostics } = simple_parse(input, &[":"], Some(filename));

    let root = match roots.len() {
        0 => {
            diagnostics.add(diagnostic!(&Span::empty(), "completely empty programs are not allowed"));
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

            diagnostics.add(diagnostic!(&span, "a program must only have one root, found {}", n));
            first
        }
    };

    let node = create_node!(a, {
        parse_shape(&root, &a, &mut diagnostics)
    });

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
                        errors.add(invalid_shape_name(span, span.text().as_ref()));
                        None
                    }
                    &Sexpr::Terminal(_, ref namespan) => {
                        match namespan.text().as_ref() {
                            "circle" => parse_circle(&children[1..], span, errors).map(a),
                            "rect" => parse_rect(&children[1..], span, errors).map(a),
                            "polygon" => parse_polygon(&children[1..], span, errors).map(a),
                            "or" => make_combinator(&children[1..], Node::Or, span, a, errors),
                            "and" => make_combinator(&children[1..], Node::And, span, a, errors),
                            "not" => make_singular(&children[1..], Node::Not, span, a, errors),
                            "break" => make_singular(&children[1..], Node::Break, span, a, errors),
                            "freeze" => make_singular(&children[1..], Node::Freeze, span, a, errors),
                            "grow" => parse_modulate(&children[1..], span, true, a, errors).map(a),
                            "shrink" => parse_modulate(&children[1..], span, false, a, errors).map(a),
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

fn parse_polygon(children: &[Sexpr], span: &Span, errors: &mut DiagnosticBag) -> Option<Node<'static>> {
    let mut parsed = Vec::with_capacity(children.len());

    for child in children {
        let (ok, proplist) = parse_properties(child, errors);
        if !ok { return None }
        let x = attempt!(proplist.get_number("x", child.span()), 0.0, errors);
        let y = attempt!(proplist.get_number("y", child.span()), 0.0, errors);
        parsed.push((x, y));
    }

    let mut out_xs = Vec::with_capacity(parsed.len() * 2 + 1);
    let mut out_ys = Vec::with_capacity(parsed.len() * 2 + 1);

    let mut first = true;
    for &(x, y) in &parsed {
        if first {
            out_xs.push(x);
            out_ys.push(y);
            first = false;
        } else {
            out_xs.push(x); out_xs.push(x);
            out_ys.push(y); out_ys.push(y);
        }
    }

    if let Some(&(sx, sy)) = parsed.first() {
        out_xs.push(sx);
        out_ys.push(sy);
    } else {
        errors.add(polygons_need_a_point(span));
        return None;
    }

    Some(Node::Polygon(PolyGroup::single_additive(out_xs, out_ys)))
}

fn parse_circle(children: &[Sexpr], span: &Span, errors: &mut DiagnosticBag) -> Option<Node<'static>> {

    if let Some(proplist) = children.get(0) {
        let (ok, proplist) = parse_properties(proplist, errors);
        if !ok { return None }
        let radius = proplist.get_number("radius", span);
        let radius = radius.or_else(|_| proplist.get_number("r", span));
        let radius = attempt!(radius, 10.0, errors);
        let x = attempt!(proplist.get_number("x", span), 10.0, errors);
        let y = attempt!(proplist.get_number("y", span), 10.0, errors);
        Some(Node::Circle{r: radius, x: x, y: y})
    } else {
        errors.add(expected_property_list_exists(span));
        None
    }
}

fn parse_rect(children: &[Sexpr], span: &Span, errors: &mut DiagnosticBag) -> Option<Node<'static>> {
    if let Some(proplist) = children.get(0) {
        let (ok, proplist) = parse_properties(proplist, errors);
        if !ok { return None }
        let x = attempt!(proplist.get_number("x", span), 10.0, errors);
        let y = attempt!(proplist.get_number("y", span), 10.0, errors);
        let w = attempt!(proplist.get_number("w", span), 10.0, errors);
        let h = attempt!(proplist.get_number("h", span), 10.0, errors);

        // THIS IS A GIAN HACK.  REMOVE ASAP

        let input = format!(
            "{{x: {a} y: {b}}} {{x: {c} y: {b}}} {{x: {c} y: {d}}} {{x: {a} y: {d}}}",
            a = x, b = y, c = x + w, d = x + h);

        let snoot::Result { roots, diagnostics: diag } = simple_parse(input, &[":"], None);
        errors.append(diag);

        parse_polygon(&roots[..], &Span::empty(), errors)
    } else {
        errors.add(expected_property_list_exists(span));
        None
    }
}

fn parse_modulate<'o, F>(children: &[Sexpr], span: &Span, grow: bool, a: &F, errors: &mut DiagnosticBag) -> Option<Node<'o>>
where F: Fn(Node<'o>) -> &'o Node<'o>
{
    if children.len() != 2 {
        errors.add(expected_children(span));
        return None;
    }

    let how_much = if let Some(n) = children[0].expect_float(errors) {
        n * (if grow { -1.0 } else { 1.0 })
    } else {
        return None;
    };


    match parse_shape(&children[1], a, errors) {
        Some(shape) => Some(Node::Modulate(how_much as f32, shape)),
        None => None
    }
}
