#[macro_use]
extern crate serde_derive;
extern crate happy;

#[derive(Deserialize)]
struct LeftPad {
    string: String,
    padding: usize,
}

fn left_pad(_: happy::RequestInfo, problem: LeftPad) -> String {
    let LeftPad { string, padding } = problem;
    format!("{:width$}", string, width = padding)
}

pub fn main() {
    happy::create()
        .static_dir("static")
        .api("api/left-pad", left_pad)
        .run();
}
