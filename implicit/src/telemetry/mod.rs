pub use self::dump::*;
pub use self::null::*;
pub(crate) use self::svg_helpers::*;
use debug::*;
use lines::LineType;
use lines::util::geom;
use nodes::Node;
use opencl::FieldBuffer;
use output::{OutputScene, OutputShape};
use std::path::PathBuf;

mod null;
mod dump;
mod svg_helpers;

#[derive(Clone, Copy)]
pub struct TelemetryLocation {
    figure: usize,
    shape: usize,
    intermediate: usize,
}

impl TelemetryLocation {
    pub fn new() -> TelemetryLocation {
        TelemetryLocation {
            figure: 0,
            shape: 0,
            intermediate: 0,
        }
    }

    pub fn new_figure(&mut self) {
        self.figure += 1;
        self.shape = 0;
        self.intermediate = 0;
    }

    pub fn new_shape(&mut self) {
        self.shape += 1;
        self.intermediate = 0;
    }

    pub fn new_intermediate(&mut self) { self.intermediate += 1; }

    fn t_figure_path(&self) -> PathBuf {
        let mut r = PathBuf::new();
        r.push(format!("figure_{}", self.figure));
        r
    }

    fn t_shape_path(&self) -> PathBuf {
        let mut r = PathBuf::new();
        r.push(format!("figure_{}", self.figure));
        r.push(format!("shape_{}", self.shape));
        r
    }

    fn t_intermediate_path(&self) -> PathBuf {
        let mut r = PathBuf::new();
        r.push(format!("figure_{}", self.figure));
        r.push(format!("shape_{}", self.shape));
        r.push(format!("intermediate_{}", self.intermediate));
        r
    }
}

pub trait Telemetry {
    fn shape_finished(&mut self, t: TelemetryLocation, buffer: &FieldBuffer, lines: &[((f32, f32), (f32, f32))]);
    fn shape_line_pre_prune(&mut self, t: TelemetryLocation, lines: &[geom::Line]);
    fn shape_line_pruned(&mut self, t: TelemetryLocation, lines: &[geom::Line]);
    fn shape_line_joined(&mut self, t: TelemetryLocation, lines: &[LineType]);
    fn shape_line_connected(&mut self, t: TelemetryLocation, lines: &[LineType]);
    fn intermediate_eval_basic(&mut self, t: TelemetryLocation, buffer: &FieldBuffer, program: &str, node: &Node);
    fn intermediate_eval_poly(&mut self, t: TelemetryLocation, buffer: &FieldBuffer);
    fn figure_finished(&mut self, t: TelemetryLocation, figure: &[OutputShape]);

    fn scene_started(&mut self);
    fn scene_bounding_box(&mut self, t: TelemetryLocation, f32, f32, f32, f32);
    fn scene_finished(&mut self, t: TelemetryLocation, scene: &OutputScene);
}
