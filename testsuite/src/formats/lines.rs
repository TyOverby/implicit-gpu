use snoot::{Result as ParseResult, simple_parse};
use snoot::serde_serialization::{DeserializeResult, deserialize};
use std::fmt::Write;

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename="line")]
pub struct Line(pub f32, pub f32, pub f32, pub f32);

pub fn compare(expected: &str, expected_filename: &str, actual: &[Line]) -> Result<(), String> {
    let ex = text_to_vec(expected, expected_filename);

    if ex.len() != actual.len() {
        return Err(format!("Number of lines differ, {} vs {}", ex.len(), actual.len()));
    }

    for (i, (exl, acl)) in ex.into_iter().zip(actual.into_iter().map(|&l| l)).enumerate() {
        if (exl.0 - acl.0).abs() > 0.0001 {
            return Err(format!("Contents of line {} differ, {:?} vs {:?}", i, exl, acl));
        }
        if (exl.1 - acl.1).abs() > 0.0001 {
            return Err(format!("Contents of line {} differ, {:?} vs {:?}", i, exl, acl));
        }
        if (exl.2 - acl.2).abs() > 0.0001 {
            return Err(format!("Contents of line {} differ, {:?} vs {:?}", i, exl, acl));
        }
        if (exl.3 - acl.3).abs() > 0.0001 {
            return Err(format!("Contents of line {} differ, {:?} vs {:?}", i, exl, acl));
        }
    }

    Ok(())
}

pub fn lines_to_text<I: Iterator<Item = Line>>(lines: I) -> String {
    let mut buff = String::new();
    for Line(x1, y1, x2, y2) in lines {
        writeln!(&mut buff, "(line {:.6} {:.6} {:.6} {:.6})", x1, y1, x2, y2).unwrap();
    }
    buff
}

pub fn text_to_vec(text: &str, filename: &str) -> Vec<Line> {

    let ParseResult { roots, diagnostics } = simple_parse(text, &[], Some(filename));
    diagnostics.assert_empty();

    roots
        .iter()
        .map(|sexpr| deserialize::<Line>(sexpr))
        .collect::<DeserializeResult<Vec<Line>>>()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>()
}

#[test]
fn test_text_to_vec() {
    let s = "(line 1 2 3 4) (line 5 6 7 8)";
    assert_eq!(text_to_vec(s, "foo"), vec![Line(1.0, 2.0, 3.0, 4.0), Line(5.0, 6.0, 7.0, 8.0)])
}
