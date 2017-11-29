extern crate happy;
extern crate hyper;
extern crate implicit;
extern crate chrono;
extern crate latin;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use happy::RequestInfo;
use implicit::scene::Scene;
use implicit::telemetry::DumpTelemetry;
use std::panic::catch_unwind;

#[derive(Deserialize)]
struct SceneRequest {
    scene: Scene,
    source: String,
}

#[derive(Serialize)]
struct FigureResult {
    svg: String,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct SceneError;

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("faile")
    }
}

impl std::error::Error for SceneError {
    fn description(&self) -> &str {"oh no panic"}
    fn cause(&self) -> Option<&std::error::Error> { None }
}

fn process(_: RequestInfo, scene: SceneRequest) -> Result<Vec<FigureResult>, SceneError> {
    catch_unwind(|| {
        let current_timestamp = chrono::Local::now();
        let dump_dir = format!("dumps/{:?}", current_timestamp);
        ::std::fs::create_dir_all(&dump_dir).unwrap();
        latin::file::write(format!("{}/source.ts", dump_dir), scene.source).unwrap();
        let mut telemetry = DumpTelemetry::new(dump_dir);

        let out = implicit::run_scene(&scene.scene, &mut telemetry);

        out.figures.into_iter().map(|figure| {
            let mut out_svg: Vec<u8> = Vec::new();
            let (l, t, w, h) = (figure.left, figure.top, figure.width, figure.height);
            implicit::export::svg::write_to(&mut out_svg, figure).unwrap();
            FigureResult {
                svg: String::from_utf8(out_svg).unwrap(),
                left: l,
                top: t,
                width: w,
                height: h,
            }
        }).collect::<Vec<_>>()
    }).map_err(|_| SceneError)
}

fn validate(_: RequestInfo, _scene: Scene) -> String { "looks good!".into() }

fn main() {
    happy::create()
        .result_api("api/process", process)
        .api("api/validate", validate)
        .static_dir("../implicit-ts/dist")
        .run();
}
