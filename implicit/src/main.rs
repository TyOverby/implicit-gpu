extern crate implicit;
extern crate serde;
extern crate snoot;

use implicit::ocaml::*;
use snoot::serde_serialization::{deserialize, DeserializeResult};
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut out = String::new();
    let stdin = stdin();
    stdin.lock().read_to_string(&mut out).unwrap();

    let sexprs = snoot::simple_parse(out, &[], Some("<stdin>"));
    sexprs.diagnostics.assert_empty();
    assert!(sexprs.roots.len() == 1);

    let program = sexprs.roots.into_iter().next().unwrap();
    let deser = match deserialize::<Command>(&program) {
        DeserializeResult::AllGood(v) => v,
        DeserializeResult::CouldntRecover(bag) | DeserializeResult::CouldRecover(_, bag) => {
            bag.assert_empty();
            panic!()
        }
    };

    println!("{:?}", deser);
}
