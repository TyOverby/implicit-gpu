extern crate happy;
extern crate hyper;
extern crate mime;
extern crate implicit;

use implicit::scene::Scene;
use happy::{RequestInfo, Response};
use hyper::header::ContentType;
use std::panic::catch_unwind;
use implicit::telemetry::NullTelemetry;

fn process(_: RequestInfo, scene: Scene) -> Response {
    let out = catch_unwind(|| implicit::run_scene(&scene, &mut NullTelemetry));

    if let Ok(out) = out {
        let mut out_svg: Vec<u8> = Vec::new();
        implicit::export::svg::write_to(&mut out_svg, out).unwrap();
        let svg = "image/svg+xml".parse::<mime::Mime>().unwrap();
        Response::new()
            .with_body(out_svg)
            .with_header(ContentType(svg))
    } else {
        Response::new()
            .with_body("Oh no, panic!")
            .with_status(hyper::StatusCode::InternalServerError)
    }
}

fn validate(_: RequestInfo, _scene: Scene) -> String {
    "looks good!".into()
}

fn main() {
    happy::create()
        .custom_response("api/process", process)
        .api("api/validate", validate)
        .static_dir("../implicit-ts")
        .run();
}
