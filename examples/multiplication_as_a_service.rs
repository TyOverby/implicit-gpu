extern crate twf;
extern crate void;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Problem {
    a: f64, 
    b: f64, 
}

fn multiply(_: twf::api::RequestInfo, problem: Problem) -> Result<f64, void::Void> {
    Ok(problem.a * problem.b)
}

pub fn main() {
    twf::api::Server::new()
        .static_dir("static")
        .api("api/multiply", multiply)
        .run();
}
