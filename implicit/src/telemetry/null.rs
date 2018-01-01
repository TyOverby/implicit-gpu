use super::*;
use geometry::{PathSegment, Point};

pub struct NullTelemetry;

impl Telemetry for NullTelemetry {
    fn shape_finished(&mut self, _t: TelemetryLocation, _buffer: &FieldBuffer, _lines: &[((f32, f32), (f32, f32))]) {}
    fn intermediate_eval_basic(&mut self, _t: TelemetryLocation, _buffer: &FieldBuffer, _program: &str, _node: &Node) {}
    fn intermediate_eval_poly(&mut self, _t: TelemetryLocation, _buffer: &FieldBuffer) {}

    fn lines_0_input(&mut self, _t: TelemetryLocation, _lines: &[(Point, Point)]) {}
    fn lines_1_zero_area_removed(&mut self, _t: TelemetryLocation, _lines: &[(Point, Point)]) {}
    fn lines_2_pruned(&mut self, _t: TelemetryLocation, _lines: &[PathSegment]) {}
    fn lines_3_obvious_connected(&mut self, _t: TelemetryLocation, _lines: &[PathSegment]) {}
    fn lines_4_graph_stitched(&mut self, _t: TelemetryLocation, _lines: &[PathSegment]) {}
    fn figure_finished(&mut self, _t: TelemetryLocation, _figure: &[OutputShape]) {}

    fn scene_started(&mut self) {}
    fn scene_bounding_box(&mut self, _t: TelemetryLocation, _: f32, _: f32, _: f32, _: f32) {}
    fn scene_finished(&mut self, _t: TelemetryLocation, _scene: &OutputScene) {}
}
