use tendril::StrTendril;
use snoot::parse::{Span, SexprKind};
use snoot::diagnostic::DiagnosticBuilder;

pub fn invalid_syntax(span: &Span) -> DiagnosticBuilder {
    DiagnosticBuilder::new("invalid syntax", span)
}

pub fn invalid_shape_name(span: &Span) -> DiagnosticBuilder {
    DiagnosticBuilder::new("invalid shape name", span)
}

pub fn not_a_shape(span: &Span, kind: SexprKind) -> DiagnosticBuilder {
    DiagnosticBuilder::new(format!("expected shape, found {:?}", kind), span)
}

pub fn unrecognized_shape(span: &Span, name: &str) -> DiagnosticBuilder {
    DiagnosticBuilder::new(format!("unrecognized shape name: {}", name), span)
}

pub fn expected_number(span: &Span, actual: StrTendril) -> DiagnosticBuilder {
    DiagnosticBuilder::new(format!("expected number, found {}", actual), span)
}

pub fn required_property(span: &Span, name: &str, typ: SexprKind) -> DiagnosticBuilder {
    DiagnosticBuilder::new(format!("required property \"{}\" of type {:?}", name, typ), span)
}

pub fn bad_value_kind(span: &Span, expected: SexprKind, actual: SexprKind) -> DiagnosticBuilder {
    DiagnosticBuilder::new(format!("bad value kind, expected: {:?} but got {:?}", expected, actual), span)
}

pub fn invalid_property_name(span: &Span) -> DiagnosticBuilder {
    DiagnosticBuilder::new("invalid property name", span)
}

pub fn missing_colon(span: &Span) -> DiagnosticBuilder {
    DiagnosticBuilder::new("missing colon in property list", span)
}

pub fn missing_value(span: &Span) -> DiagnosticBuilder {
    DiagnosticBuilder::new("missing vlaue in property list", span)
}

pub fn expected_property_list_exists(span: &Span) -> DiagnosticBuilder {
    DiagnosticBuilder::new(format!("expected property list"), span)
}

pub fn expected_children(span: &Span) -> DiagnosticBuilder {
    DiagnosticBuilder::new(format!("expected children"), span)
}

pub fn expected_one_child(span: &Span, actual: usize) -> DiagnosticBuilder {
    DiagnosticBuilder::new(format!("expected exactly one child, found {}", actual), span)
}

pub fn expected_property_list(span: &Span, typ: SexprKind) -> DiagnosticBuilder {
    DiagnosticBuilder::new(format!("expected property list, found {:?}", typ), span)
}
