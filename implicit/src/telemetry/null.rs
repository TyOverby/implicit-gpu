use super::{Telemetry, TelemetryLocation};
use output::OutputScene;

pub struct NullTelemetry;
impl Telemetry for NullTelemetry {}

pub struct OnlyFlameTelemetry;
impl Telemetry for OnlyFlameTelemetry {
    fn scene_started(&mut self) { ::flame::start("scene"); }
    fn scene_finished(&mut self, _: TelemetryLocation, _: &OutputScene) { ::flame::end("scene"); }
}
