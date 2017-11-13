extern crate happy;
extern crate hyper;
extern crate implicit;
extern crate mime;

use happy::{RequestInfo, Response};
use hyper::header::ContentType;
use implicit::scene::Scene;
use implicit::telemetry::NullTelemetry;
use std::panic::catch_unwind;

fn process(_: RequestInfo, scene: Scene) -> Response {
    let out = catch_unwind(|| {
        let out = implicit::run_scene(&scene, &mut NullTelemetry);
        let mut out_svg: Vec<u8> = Vec::new();
        implicit::export::svg::write_to(&mut out_svg, out).unwrap();
        out_svg
    });

    match out {
        Ok(svg_bytes) => {
            let svg = "image/svg+xml".parse::<mime::Mime>().unwrap();
            Response::new().with_body(svg_bytes).with_header(ContentType(svg))
        }
        Err(_) => Response::new()
            .with_body("Oh no, panic!")
            .with_status(hyper::StatusCode::InternalServerError),
    }
}

fn validate(_: RequestInfo, _scene: Scene) -> String { "looks good!".into() }

fn main() {
    happy::create()
        .custom_response("api/process", process)
        .api("api/validate", validate)
        .static_dir("../implicit-ts/dist")
        .run();
}
