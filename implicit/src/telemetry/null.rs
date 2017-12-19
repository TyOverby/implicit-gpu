use super::*;
use geometry::Line;

pub struct NullTelemetry;

impl Telemetry for NullTelemetry {
    fn shape_finished(&mut self, _t: TelemetryLocation, _buffer: &FieldBuffer, _lines: &[((f32, f32), (f32, f32))]) {}
    fn intermediate_eval_basic(&mut self, _t: TelemetryLocation, _buffer: &FieldBuffer, _program: &str, _node: &Node) {}
    fn intermediate_eval_poly(&mut self, _t: TelemetryLocation, _buffer: &FieldBuffer) {}
    fn shape_line_pre_prune(&mut self, _t: TelemetryLocation, _lines: &[Line]) {}
    fn shape_line_pruned(&mut self, _t: TelemetryLocation, _lines: &[Line]) {}
    fn shape_line_joined(&mut self, _t: TelemetryLocation, _lines: &[LineType]) {}
    fn shape_line_connected(&mut self, _t: TelemetryLocation, _lines: &[LineType]) {}
    fn figure_finished(&mut self, _t: TelemetryLocation, _figure: &[OutputShape]) {}

    fn scene_started(&mut self) {}
    fn scene_bounding_box(&mut self, _t: TelemetryLocation, _: f32, _: f32, _: f32, _: f32) {}
    fn scene_finished(&mut self, _t: TelemetryLocation, _scene: &OutputScene) {}
}
