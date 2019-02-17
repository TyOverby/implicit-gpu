extern crate expectation;
extern crate expectation_shared;
extern crate flame;
extern crate implicit;
extern crate serde;
extern crate snoot;
extern crate extern_api;

use expectation::Provider;
use expectation_shared::filesystem::RealFileSystem;
use extern_api::*;
use implicit::inspector::Inspector;
use snoot::serde_serialization::{deserialize, DeserializeResult};
use std::io::Read;
use std::io::{stdin, stdout};

fn main() {
    let mut out = String::new();
    let stdin = stdin();
    stdin.lock().read_to_string(&mut out).unwrap();

    let sexprs = snoot::simple_parse(out, &[], Some("<stdin>"));
    sexprs.diagnostics.assert_empty();
    assert!(sexprs.roots.len() == 1);

    let program = sexprs.roots.into_iter().next().unwrap();
    let (command, (w, h)) = match deserialize::<Option<(Command, (f32, f32))>>(&program) {
        DeserializeResult::AllGood(None) => panic!("deserialized into none"),
        DeserializeResult::AllGood(Some(v)) => v,
        DeserializeResult::CouldntRecover(bag) | DeserializeResult::CouldRecover(_, bag) => {
            bag.assert_empty();
            panic!()
        }
    };

    let output = implicit::exec::exec(
        command,
        Provider::new(
            Box::new(RealFileSystem {
                root: "./main_out/actual".into(),
            }),
            Box::new(RealFileSystem {
                root: "./main_out/expected".into(),
            }),
        )
        .duplicate(),
        //Box::new(()),
        w.ceil() as u32,
        h.ceil() as u32,
    );
    let output: Vec<_> = output.into_iter().flat_map(|(_, v)| v).collect();
    implicit::debug::svg_path_segments(stdout(), &output).unwrap();

    flame::dump_html(std::fs::File::create("perf.html").unwrap()).unwrap();
    flame::dump_text_to_writer(::std::io::stderr()).unwrap();
}
