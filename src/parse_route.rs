use combine::{self, Parser};
use combine::*;
use regex;
use hyper::Method;

#[derive(Debug, Eq, PartialEq)]
pub struct ParseResult {
    method: Option<Method>,
    segments: Vec<Segment>,
}

#[derive(Debug, Eq, PartialEq)]
enum SegmentPiece {
    Literal(String),
    Pattern(String),
}

#[derive(Debug, Eq, PartialEq)]
struct Segment(Vec<SegmentPiece>);

type StrParser<'a, R> = combine::Parser<Output = R, Input = &'a str>;

impl ParseResult {
    pub fn compile(self) -> (Option<Method>, String) {
        use std::fmt::Write;
        use self::SegmentPiece::*;

        let mut out = String::new();
        write!(out, "/?").unwrap();
        for segment in self.segments {
            for piece in segment.0 {
                match piece {
                    Literal(l) => write!(out, "{}", regex::escape(&l)),
                    Pattern(p) => write!(out, "(?P<{}>[^/]+)", regex::escape(&p)),
                }.unwrap()
            }
            write!(out, "/").unwrap();
        }
        write!(out, "?").unwrap();

        return (self.method, out);
    }
}

fn verb<'a>() -> Box<StrParser<'a, Option<Method>>> {
    use combine::char::string_cmp;
    use hyper::Method::*;
    use std::ascii::AsciiExt;
    let verbs = &[
        (Options, "Options"),
        (Get, "Get"),
        (Post, "Post"),
        (Put, "Put"),
        (Delete, "Delete"),
        (Head, "Head"),
        (Trace, "Trace"),
        (Connect, "Connect"),
        (Patch, "Patch"),
    ];

    let construct_verb_parser = |method: Method, s: &'static str| {
        string_cmp(s, |a, b| AsciiExt::eq_ignore_ascii_case(&a, &b)).map(move |_| method.clone())
    };

    let verbs_combinator: Vec<_> = verbs
        .iter()
        .cloned()
        .map(|(a, b)| construct_verb_parser(a, b))
        .collect();
    let verbs_total = combine::choice(verbs_combinator);
    let verbs_and_space = verbs_total.and(many1::<String, _>(token(' '))).map(|x| x.0);
    optional(verbs_and_space).boxed()
}

fn segment_piece<'a>() -> Box<StrParser<'a, SegmentPiece>> {
    use combine::char::*;
    let literal =
        many1::<Vec<_>, _>(none_of("{}/".chars())).map(|r| r.into_iter().collect::<String>());
    let name = token('_')
        .or(letter())
        .and(many(token('_').or(alpha_num())))
        .map(|(f, mut r): (char, String)| {
            r.insert(0, f);
            r
        });
    let pattern = token('{')
        .and(name)
        .and(token('}'))
        .map(|((_, r), _)| SegmentPiece::Pattern(r));

    literal
        .map(|r| SegmentPiece::Literal(r))
        .or(pattern)
        .boxed()
}

fn segment<'a>() -> Box<StrParser<'a, Segment>> {
    many1(segment_piece())
        .map(Segment)
        .and(optional(token('/')))
        .map(|(v, _)| v)
        .boxed()
}

fn url<'a>() -> Box<StrParser<'a, Vec<Segment>>> {
    optional(token('/'))
        .and(many1(segment()))
        .map(|(_, v)| v)
        .boxed()
}

pub fn parse<'a>(input: &'a str) -> Result<ParseResult, ParseError<&'a str>> {
    use combine::Parser;

    verb()
        .and(optional(token('/')))
        .and(url())
        .map(|((v, _), u)| {
            ParseResult {
                method: v,
                segments: u,
            }
        })
        .parse(input)
        .map(|(result, _remaining)| result)
}

#[cfg(test)]
mod test {
    use combine::ParseError;
    use super::*;
    use hyper::Method::*;
    use super::SegmentPiece::*;

    fn run(input: &str) -> Result<super::ParseResult, ParseError<&str>> {
        super::parse(input)
    }

    fn run_ok(input: &str) -> super::ParseResult {
        run(input).unwrap()
    }

    fn compile_ok(input: &str) -> String {
        run(input).unwrap().compile().1
    }

    #[test]
    fn compile_a_few() {
        assert_eq!(compile_ok("abc"), r"/?abc/?");
        assert_eq!(compile_ok("abc/def"), r"/?abc/def/?");
        assert_eq!(compile_ok("{named}"), r"/?(?P<named>[^/]+)/?");
        assert_eq!(compile_ok("{a}-{b}"), r"/?(?P<a>[^/]+)\-(?P<b>[^/]+)/?");
        assert_eq!(compile_ok("a/{named}/b"), r"/?a/(?P<named>[^/]+)/b/?");
        assert_eq!(compile_ok("{a}/{b}"), r"/?(?P<a>[^/]+)/(?P<b>[^/]+)/?");
    }

    #[test]
    fn segment_piece() {
        use self::SegmentPiece::*;

        let res = super::segment_piece().parse("abc").unwrap().0;
        assert_eq!(res, Literal("abc".into()));

        let res = super::segment_piece().parse("{abc}").unwrap().0;
        assert_eq!(res, Pattern("abc".into()));

        let res = super::segment_piece().parse("{{abc}}");
        assert!(res.is_err());

        let res = super::segment_piece().parse("{}");
        assert!(res.is_err());

        let res = super::segment_piece().parse("}");
        assert!(res.is_err());

        let res = super::segment_piece().parse("{");
        assert!(res.is_err());

        let res = super::segment_piece().parse("{abc");
        assert!(res.is_err());

        let res = super::segment_piece().parse("/");
        assert!(res.is_err());
    }

    #[test]
    fn segment() {
        use self::SegmentPiece::*;
        let res = super::segment().parse("abc").unwrap().0;
        assert_eq!(res, Segment(vec![Literal("abc".into())]));

        let res = super::segment().parse("{abc}").unwrap().0;
        assert_eq!(res, Segment(vec![Pattern("abc".into())]));

        let res = super::segment().parse("abc{def}").unwrap().0;
        assert_eq!(
            res,
            Segment(vec![Literal("abc".into()), Pattern("def".into())])
        );

        let res = super::segment().parse("abc{def}ghi").unwrap().0;
        assert_eq!(
            res,
            Segment(vec![
                Literal("abc".into()),
                Pattern("def".into()),
                Literal("ghi".into()),
            ])
        );
    }

    #[test]
    fn url() {
        use self::SegmentPiece::*;

        let res = super::url().parse("abc").unwrap().0;
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0[0], Literal("abc".into()));

        let res = super::url().parse("/abc").unwrap().0;
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0[0], Literal("abc".into()));

        let res = super::url().parse("abc/").unwrap().0;
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0[0], Literal("abc".into()));

        let res = super::url().parse("/abc/").unwrap().0;
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0[0], Literal("abc".into()));

        let res = super::url().parse("/abc/def").unwrap().0;
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].0[0], Literal("abc".into()));
        assert_eq!(res[1].0[0], Literal("def".into()));
    }

    #[test]
    fn basic_with_method() {
        assert_eq!(
            run_ok("GET foo"),
            super::ParseResult {
                method: Some(Get),
                segments: vec![Segment(vec![Literal("foo".into())])],
            }
        );

        assert_eq!(
            run_ok("GET foo/bar"),
            super::ParseResult {
                method: Some(Get),
                segments: vec![
                    Segment(vec![Literal("foo".into())]),
                    Segment(vec![Literal("bar".into())]),
                ],
            }
        );

        assert_eq!(
            run_ok("GET /foo/bar"),
            super::ParseResult {
                method: Some(Get),
                segments: vec![
                    Segment(vec![Literal("foo".into())]),
                    Segment(vec![Literal("bar".into())]),
                ],
            }
        );

        assert_eq!(
            run_ok("GET /foo/bar/"),
            super::ParseResult {
                method: Some(Get),
                segments: vec![
                    Segment(vec![Literal("foo".into())]),
                    Segment(vec![Literal("bar".into())]),
                ],
            }
        );

        assert_eq!(
            run_ok("foo"),
            super::ParseResult {
                method: None,
                segments: vec![Segment(vec![Literal("foo".into())])],
            }
        );

        assert_eq!(
            run_ok("foo/bar"),
            super::ParseResult {
                method: None,
                segments: vec![
                    Segment(vec![Literal("foo".into())]),
                    Segment(vec![Literal("bar".into())]),
                ],
            }
        );

        assert_eq!(
            run_ok("/foo/bar"),
            super::ParseResult {
                method: None,
                segments: vec![
                    Segment(vec![Literal("foo".into())]),
                    Segment(vec![Literal("bar".into())]),
                ],
            }
        );

        assert_eq!(
            run_ok("/foo/bar/"),
            super::ParseResult {
                method: None,
                segments: vec![
                    Segment(vec![Literal("foo".into())]),
                    Segment(vec![Literal("bar".into())]),
                ],
            }
        );
    }
}
