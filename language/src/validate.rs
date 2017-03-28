/*extern crate implicit_language;
extern crate tendril;

use std::io::{self, Read};
use implicit_language::{ParseResult, parse};
use tendril::StrTendril;

fn main() {

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let buffer: StrTendril = buffer.into();

    let ParseResult { diagnostics, .. } = parse(buffer, "");

    print!("{}", diagnostics.to_json());
}

*/

