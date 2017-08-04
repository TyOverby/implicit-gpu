#[macro_use]
extern crate serde_derive;
extern crate twf;

#[derive(Deserialize)]
struct LeftPad {
    string: String,
    padding: usize,
}

fn left_pad(_: twf::api::RequestInfo, problem: LeftPad) -> String {
    let LeftPad { string, padding } = problem;
    format!("{:width$}", string, width = padding)
}

pub fn main() {
    happy::start()
        .static_dir("static")
        .api("api/left-pad", left_pad)
        .run();
}
