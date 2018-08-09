pub use self::dump::*;
pub use self::null::*;
pub(crate) use self::svg_helpers::*;
use debug::*;
use geometry::{PathSegment, Point};
use nodes::Node;
use opencl::FieldBuffer;
use output::{OutputScene, OutputShape};
use std::path::PathBuf;

mod dump;
mod null;
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

    pub fn new_intermediate(&mut self) {
        self.intermediate += 1;
    }

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
    fn shape_finished(
        &mut self,
        _t: TelemetryLocation,
        _buffer: &FieldBuffer,
        _lines: &[((f32, f32), (f32, f32))],
    ) {
    }

    fn lines_0_input(&mut self, _t: TelemetryLocation, _lines: &[(Point, Point)]) {}
    fn lines_1_zero_area_removed(&mut self, _t: TelemetryLocation, _lines: &[(Point, Point)]) {}
    fn lines_2_pruned<'b>(&mut self, _t: TelemetryLocation, _lines: &'b Fn() -> Vec<PathSegment>) {}
    fn lines_3_obvious_connected(&mut self, _t: TelemetryLocation, _lines: &[PathSegment]) {}
    fn lines_4_graph_stitched(&mut self, _t: TelemetryLocation, _lines: &[PathSegment]) {}

    fn intermediate_eval_basic(
        &mut self,
        _t: TelemetryLocation,
        _buffer: &FieldBuffer,
        _program: &str,
        _node: &Node,
    ) {
    }
    fn intermediate_eval_poly(&mut self, _t: TelemetryLocation, _buffer: &FieldBuffer) {}
    fn figure_finished(&mut self, _t: TelemetryLocation, _figure: &[OutputShape]) {}

    fn scene_started(&mut self) {}
    fn scene_bounding_box(&mut self, _t: TelemetryLocation, _: f32, _: f32, _: f32, _: f32) {}
    fn scene_finished(&mut self, _t: TelemetryLocation, _scene: &OutputScene) {}
}
