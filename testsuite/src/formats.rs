pub mod field {
    use implicit::opencl::FieldBuffer;
    use std::fmt::Write;
    use snoot::{simple_parse, Result as ParseResult, Sexpr};
    use snoot::diagnostic::{DiagnosticBag};

    pub fn compare(expected: &str, expected_filename: &str, actual: ((usize, usize), Vec<f32>)) -> Result<(), String> {
        let expected = text_to_vec(expected, expected_filename);
        if expected.0 != actual.0 {
            return Err(format!("size of field differs: {:?} vs {:?}", expected.0, actual.0));
        }

        for (i, (exv, acv)) in expected.1.into_iter().zip(actual.1.into_iter()).enumerate() {
            if (exv - acv).abs() > 0.0001 {
                return Err(format!("value at index {} differs: {} vs {}", i, exv, acv));
            }
        }

        Ok(())
    }

    pub fn field_to_text(field: &FieldBuffer) -> String {
        let (width, height) = field.size();
        let values = field.values();
        let mut values = &values[..];
        let mut buff = String::new();

        writeln!(&mut buff, "(size {} {})", width, height).unwrap();
        for _ in 0..height {
            let row = &values[..width];
            values = &values[width..];

            write!(&mut buff, "(row").unwrap();
            for v in row {
                write!(&mut buff, " {:.6}", v).unwrap();
            }
            writeln!(&mut buff, ")").unwrap();
        }

        buff
    }

    pub fn text_to_vec(text: &str, filename: &str) -> ((usize, usize), Vec<f32>) {
        fn parse_size(sexpr: &Sexpr, bag: &mut DiagnosticBag) -> (usize, usize) {
            let children = sexpr.expect_list_with_symbol("size", bag).unwrap_or_default();
            if children.len() != 2 {
                bag.add(diagnostic!(sexpr.span(), "size must contain two numbers"));
                return (0, 0);
            } else {
                let width = children[0].expect_int(bag).unwrap_or_default();
                let height = children[1].expect_int(bag).unwrap_or_default();
                (width as usize, height as usize)
            }
        }

        fn parse_row(sexpr: &Sexpr, bag: &mut DiagnosticBag) -> Vec<f32> {
            let children = sexpr.expect_list_with_symbol("row", bag).unwrap_or_default();
            children.iter().map(|c| c.expect_float(bag).unwrap_or_default() as f32).collect()
        }

        let ParseResult{roots, mut diagnostics} = simple_parse(text, &[], Some(filename));

        assert!(roots.len() != 0);
        let (width, height) = parse_size(&roots[0], &mut diagnostics);
        let mut rows = vec![];

        for row in &roots[1..] {
            let row = parse_row(row, &mut diagnostics);
            assert_eq!(row.len(), width);
            rows.push(row);
        }

        assert_eq!(rows.len(), height);
        diagnostics.assert_no_errors();

        ((width, height), rows.into_iter().flat_map(|a|a).collect())
    }
}

pub mod lines {
    use std::fmt::Write;
    use snoot::{simple_parse, Result as ParseResult};
    use ::snoot::serde_serialization::{deserialize, DeserializeResult};

    #[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
    #[serde(rename="line")]
    pub struct Line(pub f32, pub f32, pub f32, pub f32);

    pub fn compare(expected: &str, expected_filename: &str, actual: &[Line]) -> Result<(), String> {
        let ex = text_to_vec(expected, expected_filename);

        if ex.len() != actual.len() {
            return Err(format!("Number of lines differ, {} vs {}", ex.len(), actual.len()));
        }

        for (i, (exl, acl)) in ex.into_iter().zip(actual.into_iter().map(|&l|l)).enumerate() {
            if (exl.0 - acl.0).abs() > 0.0001 {
                return Err(format!("Contents of line {} differ, {:?} vs {:?}", i, exl, acl))
            }
            if (exl.1 - acl.1).abs() > 0.0001 {
                return Err(format!("Contents of line {} differ, {:?} vs {:?}", i, exl, acl))
            }
            if (exl.2 - acl.2).abs() > 0.0001 {
                return Err(format!("Contents of line {} differ, {:?} vs {:?}", i, exl, acl))
            }
            if (exl.3 - acl.3).abs() > 0.0001 {
                return Err(format!("Contents of line {} differ, {:?} vs {:?}", i, exl, acl))
            }
        }

        Ok(())
    }

    pub fn lines_to_text<I: Iterator<Item=Line>>(lines: I) -> String {
        let mut buff = String::new();
        for Line(x1, y1, x2, y2) in lines {
            writeln!(&mut buff, "(line {:.6} {:.6} {:.6} {:.6})", x1, y1, x2, y2).unwrap();
        }
        buff
    }

    pub fn text_to_vec(text: &str, filename: &str) -> Vec<Line> {

        let ParseResult{roots, diagnostics} = simple_parse(text, &[], Some(filename));
        diagnostics.assert_empty();

        roots.iter()
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
}
