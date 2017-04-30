use tendril::StrTendril;
use snoot::parse::{Span, SexprKind};
use snoot::diagnostic::{Diagnostic};

pub fn invalid_syntax(span: &Span) -> Diagnostic {
    diagnostic!(span, "invalid syntax")
}

pub fn invalid_shape_name(span: &Span, name: &str) -> Diagnostic {
    diagnostic!(span, "invalid shape name {}", name)
}

pub fn polygons_need_a_point(span: &Span) -> Diagnostic {
    diagnostic!(span, "Polygons require at least one point")
}

pub fn not_a_shape(span: &Span, kind: SexprKind) -> Diagnostic {
    diagnostic!(span, "expected shape, found {:?}", kind)
}

pub fn unrecognized_shape(span: &Span, name: &str) -> Diagnostic {
    diagnostic!(span, "unrecognized shape name: {}", name)
}

pub fn expected_number(span: &Span, actual: StrTendril) -> Diagnostic {
    diagnostic!(span, "expected number, found {}", actual)
}

pub fn required_property(span: &Span, name: &str, typ: SexprKind) -> Diagnostic {
    diagnostic!(span, "required property \"{}\" of type {:?}", name, typ)
}

pub fn bad_value_kind(span: &Span, expected: SexprKind, actual: SexprKind) -> Diagnostic {
    diagnostic!(span, "bad value kind, expected: {:?} but got {:?}", expected, actual)
}

pub fn invalid_property_name(span: &Span) -> Diagnostic {
    diagnostic!(span, "invalid property name")
}

pub fn missing_colon(span: &Span) -> Diagnostic {
    diagnostic!(span, "missing colon in property list")
}

pub fn missing_value(span: &Span) -> Diagnostic {
    diagnostic!(span, "missing value in property list")
}

pub fn expected_property_list_exists(span: &Span) -> Diagnostic {
    diagnostic!(span, "expected property list")
}

pub fn expected_children(span: &Span) -> Diagnostic {
    diagnostic!(span, "expected children")
}

pub fn expected_one_child(span: &Span, actual: usize) -> Diagnostic {
    diagnostic!(span, "expected exactly one child, found {}", actual)
}

pub fn expected_property_list(span: &Span, typ: SexprKind) -> Diagnostic {
    diagnostic!(span, "expected property list, found {:?}", typ)
}
