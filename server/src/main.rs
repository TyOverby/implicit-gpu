extern crate iron;
extern crate serde_json;
extern crate staticfile;
extern crate mount;
extern crate implicit;

use staticfile::Static;
use mount::Mount;
use iron::prelude::*;
use iron::status;
use implicit::scene::Scene;
use implicit::telemetry;

fn hello_world(req: &mut Request) -> IronResult<Response> {
    use iron::mime::Mime;

    let scene: Scene = ::serde_json::from_reader(&mut req.body).unwrap();
    let result = implicit::run_scene(&scene, &mut telemetry::NullTelemetry);

    let svg_mimetype: Mime = "image/svg+xml".parse().unwrap();
    Ok(Response::with((status::Ok, svg_mimetype, "Hello World!")))
}

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new("../implicit-ts"));
    mount.mount("/api", hello_world);

    let _server = Iron::new(mount).http("localhost:3000").unwrap();
    println!("On 3000");
}
