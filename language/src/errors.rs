use tendril::StrTendril;
use snoot::parse::{Span, SexprKind};
use snoot::diagnostic::{Diagnostic, DiagnosticBuilder};

pub fn invalid_syntax(span: &Span) -> Diagnostic {
    DiagnosticBuilder::new("invalid syntax", span).build()
}

pub fn invalid_shape_name(span: &Span) -> Diagnostic {
    DiagnosticBuilder::new("invalid shape name", span).build()
}

pub fn not_a_shape(span: &Span, kind: SexprKind) -> Diagnostic {
    DiagnosticBuilder::new(format!("expected shape, found {:?}", kind), span).build()
}

pub fn unrecognized_shape(span: &Span, name: &str) -> Diagnostic {
    DiagnosticBuilder::new(format!("unrecognized shape name: {}", name), span).build()
}

pub fn expected_number(span: &Span, actual: StrTendril) -> Diagnostic {
    DiagnosticBuilder::new(format!("expected number, found {}", actual), span).build()
}

pub fn required_property(span: &Span, name: &str, typ: SexprKind) -> Diagnostic {
    DiagnosticBuilder::new(format!("required property \"{}\" of type {:?}", name, typ), span).build()
}

pub fn bad_value_kind(span: &Span, expected: SexprKind, actual: SexprKind) -> Diagnostic {
    DiagnosticBuilder::new(format!("bad value kind, expected: {:?} but got {:?}", expected, actual), span).build()
}

pub fn invalid_property_name(span: &Span) -> Diagnostic {
    DiagnosticBuilder::new("invalid property name", span).build()
}

pub fn missing_colon(span: &Span) -> Diagnostic {
    DiagnosticBuilder::new("missing colon in property list", span).build()
}

pub fn missing_value(span: &Span) -> Diagnostic {
    DiagnosticBuilder::new("missing vlaue in property list", span).build()
}

pub fn expected_property_list_exists(span: &Span) -> Diagnostic {
    DiagnosticBuilder::new(format!("expected property list"), span).build()
}

pub fn expected_children(span: &Span) -> Diagnostic {
    DiagnosticBuilder::new(format!("expected children"), span).build()
}

pub fn expected_one_child(span: &Span, actual: usize) -> Diagnostic {
    DiagnosticBuilder::new(format!("expected exactly one child, found {}", actual), span).build()
}

pub fn expected_property_list(span: &Span, typ: SexprKind) -> Diagnostic {
    DiagnosticBuilder::new(format!("expected property list, found {:?}", typ), span).build()
}
