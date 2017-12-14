extern crate chrono;
extern crate flame;
extern crate happy;
extern crate hyper;
extern crate implicit;
extern crate latin;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use happy::RequestInfo;
use implicit::scene::Scene;
use implicit::telemetry::DumpTelemetry;
use std::panic::catch_unwind;
use std::sync::Mutex;

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
#[derive(Serialize)]
struct Output {
    figures: Vec<FigureResult>,
    perf: Vec<flame::Thread>,
}

#[derive(Debug)]
struct SceneError;

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { f.write_str("faile") }
}

impl std::error::Error for SceneError {
    fn description(&self) -> &str { "oh no panic" }
    fn cause(&self) -> Option<&std::error::Error> { None }
}

lazy_static!(static ref CONTEXT: Mutex<Option<implicit::opencl::OpenClContext>> = { Mutex::new(None) };);

fn process(_: RequestInfo, scene: SceneRequest) -> Result<Output, SceneError> {
    catch_unwind(|| {
        flame::start("request");
        let ctx = { CONTEXT.lock().unwrap().take() };
        let current_timestamp = chrono::Local::now();
        let dump_dir = format!("dumps/{:?}", current_timestamp);
        ::std::fs::create_dir_all(&dump_dir).unwrap();
        latin::file::write(format!("{}/source.ts", dump_dir), scene.source).unwrap();
        let mut telemetry = DumpTelemetry::new(dump_dir);

        let (out, ctx) = implicit::run_scene(scene.scene, &mut telemetry, ctx);
        *CONTEXT.lock().unwrap() = Some(ctx);

        let figures = out.figures
            .into_iter()
            .map(|figure| {
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
            })
            .collect::<Vec<_>>();

        flame::end("request");
        let perf = flame::threads();
        flame::clear();
        Output { figures: figures, perf: perf }
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
