extern crate implicit;
extern crate tendril;
extern crate typed_arena;
#[macro_use]
extern crate snoot;

mod errors;
mod properties;
#[cfg(test)]
mod test;


use std::sync::Arc;
use self::errors::*;
use self::properties::*;
use implicit::nodes::{Node, PolyGroup};
use snoot::Sexpr;
use snoot::diagnostic::DiagnosticBag;
use snoot::parse::Span;
use snoot::simple_parse;
use tendril::StrTendril;

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
    pub root: Option<Node>,
    pub diagnostics: DiagnosticBag,
}

impl ParseResult {
    pub fn unwrap(self) -> Node {
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

    let node = parse_shape(&root, &mut diagnostics);

    ParseResult {
        root: node,
        diagnostics: diagnostics,
    }
}

fn parse_shape(expr: &Sexpr, errors: &mut DiagnosticBag) -> Option<Node> {
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
                            "circle" => parse_circle(&children[1..], span, errors),
                            "rect" => parse_rect(&children[1..], span, errors),
                            "polygon" => parse_polygon(&children[1..], span, errors),
                            "or" => make_combinator(&children[1..], Node::Or, span, errors),
                            "subtract" => parse_subtraction(&children[1..], span, errors),
                            "and" => make_combinator(&children[1..], Node::And, span, errors),
                            "not" => make_singular(&children[1..], Node::Not, span, errors),
                            "break" => make_singular(&children[1..], Node::Break, span, errors),
                            "freeze" => make_singular(&children[1..], Node::Freeze, span, errors),
                            "grow" => parse_modulate(&children[1..], span, true, errors),
                            "shrink" => parse_modulate(&children[1..], span, false, errors),
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

fn parse_subtraction(children: &[Sexpr], span: &Span, errors: &mut DiagnosticBag) -> Option<Node>
{
    match children.len() {
        0 => {
            errors.add(expected_two_children(span, 0));
            None
        }
        1 => parse_shape(&children[0], errors),
        2 => {
            let additive = parse_shape(&children[0], errors);
            let subtractive = parse_shape(&children[1], errors);
            match (additive, subtractive) {
                (Some(add), Some(sub)) => Some(Node::And(vec![Arc::new(add), Arc::new(Node::Not(Arc::new(sub)))])),
                _ => None,
            }
        }
        n => {
            errors.add(expected_two_children(span, n));
            parse_shape(&children[0], errors)
        }
    }
}

fn make_combinator<F>(children: &[Sexpr], f: F, span: &Span, errors: &mut DiagnosticBag) -> Option<Node>
    where F: Fn(Vec<Arc<Node>>) -> Node
{
    if children.len() == 0 {
        errors.add(expected_children(span));
        None
    } else {
        let children = children.iter().filter_map(|c| parse_shape(c, errors));
        Some(f(children.map(Arc::new).collect()))
    }
}

fn make_singular<'o, F>(children: &[Sexpr], f: F, span: &Span, errors: &mut DiagnosticBag) -> Option<Node>
    where F: Fn(Arc<Node>) -> Node
{
    if children.len() == 0 {
        errors.add(expected_children(span));
        return None;
    } else if children.len() > 1 {
        errors.add(expected_one_child(span, children.len()));
    }
    let first = &children[0];
    match parse_shape(first, errors) {
        Some(c) => Some(f(Arc::new(c))),
        None => None,
    }
}

fn parse_polygon(children: &[Sexpr], span: &Span, errors: &mut DiagnosticBag) -> Option<Node> {
    let mut parsed = Vec::with_capacity(children.len());

    for child in children {
        let (ok, proplist) = parse_properties(child, errors);
        if !ok {
            return None;
        }
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
            out_xs.push(x);
            out_xs.push(x);
            out_ys.push(y);
            out_ys.push(y);
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

fn parse_circle(children: &[Sexpr], span: &Span, errors: &mut DiagnosticBag) -> Option<Node> {

    if let Some(proplist) = children.get(0) {
        let (ok, proplist) = parse_properties(proplist, errors);
        if !ok {
            return None;
        }
        let radius = proplist.get_number("radius", span);
        let radius = radius.or_else(|_| proplist.get_number("r", span));
        let radius = attempt!(radius, 10.0, errors);
        let x = attempt!(proplist.get_number("x", span), 10.0, errors);
        let y = attempt!(proplist.get_number("y", span), 10.0, errors);
        Some(Node::Circle { r: radius, x: x, y: y })
    } else {
        errors.add(expected_property_list_exists(span));
        None
    }
}

fn parse_rect(children: &[Sexpr], span: &Span, errors: &mut DiagnosticBag) -> Option<Node> {
    if let Some(proplist) = children.get(0) {
        let (ok, proplist) = parse_properties(proplist, errors);
        if !ok {
            return None;
        }
        let x = attempt!(proplist.get_number("x", span), 10.0, errors);
        let y = attempt!(proplist.get_number("y", span), 10.0, errors);
        let w = attempt!(proplist.get_number("w", span), 10.0, errors);
        let h = attempt!(proplist.get_number("h", span), 10.0, errors);

        // THIS IS A GIANT HACK.  REMOVE ASAP

        let input = format!(
            "{{x: {a} y: {b}}} {{x: {c} y: {b}}} {{x: {c} y: {d}}} {{x: {a} y: {d}}}",
            a = x,
            b = y,
            c = x + w,
            d = x + h,
        );

        let snoot::Result { roots, diagnostics: diag } = simple_parse(input, &[":"], None);
        errors.append(diag);

        parse_polygon(&roots[..], &Span::empty(), errors)
    } else {
        errors.add(expected_property_list_exists(span));
        None
    }
}

fn parse_modulate(children: &[Sexpr], span: &Span, grow: bool, errors: &mut DiagnosticBag) -> Option<Node>
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


    match parse_shape(&children[1], errors) {
        Some(shape) => Some(Node::Modulate(how_much as f32, Arc::new(shape))),
        None => None,
    }
}
