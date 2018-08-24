extern crate expectation;
extern crate implicit;
extern crate serde;
extern crate snoot;
#[cfg(test)]
use expectation::extensions::TextDiffExtension;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::{BufRead, BufReader, Read};

use serde::de::DeserializeOwned;

pub fn deser<T: DeserializeOwned>(out: &str) -> T {
    use snoot::serde_serialization::*;
    let sexprs = snoot::simple_parse(out, &[], Some("<stdin>"));
    sexprs.diagnostics.assert_empty();
    assert!(sexprs.roots.len() == 1);

    let program = sexprs.roots.into_iter().next().unwrap();
    let deser = match deserialize::<T>(&program) {
        DeserializeResult::AllGood(v) => v,
        DeserializeResult::CouldntRecover(bag) | DeserializeResult::CouldRecover(_, bag) => {
            bag.assert_empty();
            panic!()
        }
    };
    deser
}

#[test]
fn expectation_test_all() {
    use implicit::inspector::*;
    use implicit::ocaml::{Bbox, Bounding};

    let tests_file =
        BufReader::new(File::open("./tests.txt").expect("tests.txt file should exist"));
    let lines = tests_file.lines();
    for test_name in lines.map(|line| line.unwrap()) {
        expectation::expect(&format!("expectation_test_{}", test_name), |provider| {
            let mut test_file = File::open(format!("./tests/{}.shape", test_name)).unwrap();
            let mut test_contents = String::new();
            test_file.read_to_string(&mut test_contents).unwrap();

            let deser: Option<(implicit::ocaml::Command, implicit::ocaml::Bounding)> =
                deser(&test_contents);
            let (command, bbox) = deser.unwrap();

            provider.debug(format!("command.txt"), &command).unwrap();
            provider.debug(format!("bbox.txt"), &bbox).unwrap();

            let (w, h) = match bbox {
                Bounding::Positive(Bbox { x, y, w, h }) => (w + x, h + y),
                other => panic!("box of {:?} not handled yet", other),
            };
            let (w, h) = (w.ceil() + 1.0, h.ceil() + 1.0);

            implicit::exec::exec(
                command,
                provider.duplicate(),
                w as usize,
                h as usize,
            );
        });
    }
}
