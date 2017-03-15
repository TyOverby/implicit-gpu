use tendril::StrTendril;
use snoot::parse::{Span, SexprKind};
use snoot::error::ErrorBuilder;

pub fn invalid_syntax(span: &Span<StrTendril>) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new("invalid syntax", span)
}

pub fn invalid_shape_name(span: &Span<StrTendril>) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new("invalid shape name", span)
}

pub fn not_a_shape(span: &Span<StrTendril>, kind: SexprKind) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new(format!("expected shape, found {:?}", kind), span)
}

pub fn unrecognized_shape(span: &Span<StrTendril>, name: &str) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new(format!("unrecognized shape name: {}", name), span)
}

pub fn expected_number(span: &Span<StrTendril>, actual: StrTendril) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new(format!("expected number, found {}", actual), span)
}

pub fn required_property(span: &Span<StrTendril>, name: &str, typ: SexprKind) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new(format!("required property \"{}\" of type {:?}", name, typ), span)
}

pub fn bad_value_kind(span: &Span<StrTendril>, expected: SexprKind, actual: SexprKind) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new(format!("bad value kind, expected: {:?} but got {:?}", expected, actual), span)
}

pub fn invalid_property_name(span: &Span<StrTendril>) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new("invalid property name", span)
}

pub fn missing_colon(span: &Span<StrTendril>) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new("missing colon in property list", span)
}

pub fn missing_value(span: &Span<StrTendril>) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new("missing vlaue in property list", span)
}

pub fn expected_property_list_exists(span: &Span<StrTendril>) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new(format!("expected property list"), span)
}

pub fn expected_children(span: &Span<StrTendril>) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new(format!("expected children"), span)
}

pub fn expected_one_child(span: &Span<StrTendril>, actual: usize) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new(format!("expected exactly one child, found {}", actual), span)
}

pub fn expected_property_list(span: &Span<StrTendril>, typ: SexprKind) -> ErrorBuilder<StrTendril> {
    ErrorBuilder::new(format!("expected property list, found {:?}", typ), span)
}
