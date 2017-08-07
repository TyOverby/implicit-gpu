extern crate happy;
extern crate hyper;
extern crate mime;
extern crate implicit;

use implicit::scene::Scene;
use happy::{RequestInfo, Response};
use hyper::header::ContentType;

fn process(_: RequestInfo, scene: Scene) -> Response {
    let mut telemetry = implicit::telemetry::NullTelemetry;
    let out = implicit::run_scene(&scene, &mut telemetry);
    let mut out_svg: Vec<u8> = Vec::new();
    implicit::export::svg::write_to(&mut out_svg, out).unwrap();
    let svg = "image/svg+xml".parse::<mime::Mime>().unwrap();
    Response::new()
        .with_body(out_svg)
        .with_header(ContentType(svg))
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