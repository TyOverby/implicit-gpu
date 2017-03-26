use super::errors::*;

use std::collections::HashMap;
use tendril::StrTendril;
use snoot::diagnostic::DiagnosticBuilder;
use snoot::parse::{SexprKind, Span};
use snoot::Sexpr;

pub struct PropList {
    map: HashMap<StrTendril, (SexprKind, Span, StrTendril)>,
}

impl PropList {
    pub fn get_number<'a, T: Into<StrTendril>>(&'a self,
                                               key: T,
                                               prop_span: &Span)
                                               -> Result<f32, DiagnosticBuilder> {
        self.get(key, SexprKind::Terminal, prop_span).and_then(|(text, span)| {
            text.parse().map_err(|_| expected_number(span, text))
        })
    }

    pub fn get<'a, T: Into<StrTendril>>(&'a self,
                                        key: T,
                                        typ: SexprKind,
                                        prop_span: &Span)
                                        -> Result<(StrTendril, &Span), DiagnosticBuilder> {
        let key = key.into();
        match self.map.get(&key) {
            Some(&(kind, ref span, ref string)) if kind == typ => Ok((string.clone(), span)),
            Some(&(kind, ref span, _)) => Err(bad_value_kind(span, typ, kind)),
            None => Err(required_property(prop_span, key.as_ref(), typ)),
        }
    }
}


pub fn parse_properties<'a>(proplist: &'a Sexpr,
                            errors: &mut Vec<DiagnosticBuilder>)
                            -> (bool, PropList) {
    if let &Sexpr::List { ref children, .. } = proplist {
        let mut iter = children.iter();
        let mut map = HashMap::new();
        let mut all_ok = true;
        while iter.len() != 0 {
            // unwrap ok because we aren't empty
            let head = iter.next().unwrap();
            if let &Sexpr::Terminal(_, ref span) = head {
                if let Some(&Sexpr::Terminal(_, ref span)) = iter.next() {
                    if !(span.text().as_ref() == ":") {
                        all_ok = false;
                        errors.push(missing_colon(span));
                        continue;
                    }
                } else {
                    all_ok = false;
                    errors.push(missing_colon(span));
                    continue;
                }

                let key = span.text().clone();
                if let Some(value) = iter.next() {
                    map.insert(key,
                               (value.kind(), value.span().clone(), value.text().clone()));
                } else {
                    all_ok = false;
                    errors.push(missing_value(span));
                }
            } else {
                all_ok = false;
                errors.push(invalid_property_name(head.span()));
            }
        }
        (all_ok, PropList { map: map })
    } else {
        errors.push(expected_property_list(proplist.span(), proplist.kind()));
        (false, PropList { map: HashMap::new() })
    }
}

#[cfg(test)]
mod prop_test {
    use super::{parse_properties, PropList};
    use snoot::{simple_parse, Result as ParseResult};
    use snoot::parse::Span;
    use snoot::diagnostic::DiagnosticBuilder;

    fn props_ok(input: &str) -> (PropList, Span) {
        let ParseResult { roots, diagnostics } = simple_parse(input, &[":"]);
        assert!(diagnostics.is_empty());
        assert!(roots.len() == 1);

        let bag = roots.into_iter().next().unwrap();
        let mut error_builders = vec![];
        let (all_ok, props) = parse_properties(&bag, &mut error_builders);
        if !all_ok {
            panic!("{:?}",
                   error_builders.into_iter().map(|a| a.build().to_string()).collect::<Vec<_>>());
        }
        (props, bag.span().clone())
    }

    #[test]
    fn test_parse_props() {
        let (props, span) = props_ok("{a: 5 b : 10}");

        assert_eq!(props.get_number("a", &span).map_err(DiagnosticBuilder::build).unwrap(),
                   5.0);
        assert_eq!(props.get_number("b", &span).map_err(DiagnosticBuilder::build).unwrap(),
                   10.0);
    }
}

