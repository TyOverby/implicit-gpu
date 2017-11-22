use debug::*;
use lines::LineType;
use lines::util::geom;
use nodes::Node;
use opencl::FieldBuffer;
use output::{OutputScene, OutputShape};
use std::fs::create_dir_all;
use std::path::PathBuf;
use telemetry::*;

pub struct DumpTelemetry {
    path: PathBuf,
    field_writer: Option<Box<Fn(PathBuf, &FieldBuffer)>>,
    line_writer: Option<Box<Fn(PathBuf, &[((f32, f32), (f32, f32))])>>,
}

impl DumpTelemetry {
    pub fn new<P: Into<PathBuf>>(root: P) -> DumpTelemetry {
        DumpTelemetry {
            path: root.into(),
            field_writer: None,
            line_writer: None,
        }
    }

    pub fn with_field_writer<F: 'static + Fn(PathBuf, &FieldBuffer)>(self, f: F) -> DumpTelemetry {
        DumpTelemetry {
            field_writer: Some(Box::new(f)),
            ..self
        }
    }

    pub fn with_line_writer<F: 'static + Fn(PathBuf, &[((f32, f32), (f32, f32))])>(self, f: F) -> DumpTelemetry {
        DumpTelemetry {
            line_writer: Some(Box::new(f)),
            ..self
        }
    }

    fn scene_path(&self, _: TelemetryLocation, file: &str) -> PathBuf {
        create_dir_all(&self.path).unwrap();
        self.path.join(file)
    }

    fn figure_path(&self, tloc: TelemetryLocation, file: &str) -> PathBuf {
        let base = self.path.join(tloc.t_figure_path());
        create_dir_all(&base).unwrap();
        base.join(file)
    }

    fn shape_path(&self, tloc: TelemetryLocation, file: &str) -> PathBuf {
        let base = self.path.join(tloc.t_shape_path());
        create_dir_all(&base).unwrap();
        base.join(file)
    }

    fn intermediate_path(&self, tloc: TelemetryLocation, file: &str) -> PathBuf {
        let base = self.path.join(tloc.t_intermediate_path());
        create_dir_all(&base).unwrap();
        base.join(file)
    }
}


impl Telemetry for DumpTelemetry {
    fn scene_started(&mut self) { ::flame::start("scene"); }

    fn shape_finished(&mut self, tloc: TelemetryLocation, buffer: &FieldBuffer, lines: &[((f32, f32), (f32, f32))]) {
        let _guard = ::flame::start_guard("telemetry shape_finished");

        save_field_buffer(buffer, self.shape_path(tloc, "field.png"), ColorMode::Debug);

        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(self.shape_path(tloc, "field.values"), buffer);
        }

        if let Some(line_writer) = self.line_writer.as_ref() {
            (line_writer)(self.shape_path(tloc, "outlines.lines"), lines);
        }
    }

    fn shape_line_pre_prune(&mut self, tloc: TelemetryLocation, lines: &[geom::Line]) {
        use std::fs::File;
        let _guard = ::flame::start_guard("telemetry shape_line_pre_prune");
        let file = File::create(self.shape_path(tloc, "pre-pruned.svg")).unwrap();
        output_svg_lines(file, lines.iter().cloned());
    }

    fn shape_line_pruned(&mut self, tloc: TelemetryLocation, lines: &[geom::Line]) {
        use std::fs::File;

        let _guard = ::flame::start_guard("telemetry shape_line_pruned");
        let file = File::create(self.shape_path(tloc, "pruned.svg")).unwrap();
        output_svg_lines(file, lines.iter().cloned());
    }

    fn shape_line_joined(&mut self, tloc: TelemetryLocation, lines: &[LineType]) {
        use std::fs::File;

        let _guard = ::flame::start_guard("telemetry shape_line_joined");
        let file = File::create(self.shape_path(tloc, "joined.svg")).unwrap();
        output_svg_linetype(file, lines.iter());
    }

    fn shape_line_connected(&mut self, tloc: TelemetryLocation, lines: &[LineType]) {
        use std::fs::File;

        let _guard = ::flame::start_guard("telemetry shape_line_connected");
        let file = File::create(self.shape_path(tloc, "connected.svg")).unwrap();
        output_svg_linetype(file, lines.iter());
    }

    fn intermediate_eval_basic(&mut self, tloc: TelemetryLocation, buffer: &FieldBuffer, program: &str, node: &Node) {
        let _guard = ::flame::start_guard("telemetry intermediate_eval_basic");

        save_field_buffer(buffer, self.intermediate_path(tloc, "field.png"), ColorMode::Debug);
        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(self.intermediate_path(tloc, "field.values"), buffer);
        }

        ::latin::file::write(self.intermediate_path(tloc, "shader.c"), program).unwrap();
        ::latin::file::write(self.intermediate_path(tloc, "node.txt"), format!("{:#?}", node)).unwrap();
    }

    fn intermediate_eval_poly(&mut self, tloc: TelemetryLocation, buffer: &FieldBuffer) {
        let _guard = ::flame::start_guard("telemetry intermediate_eval_poly");

        save_field_buffer(buffer, self.intermediate_path(tloc, "field.png"), ColorMode::Debug);
        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(self.intermediate_path(tloc, "field.values"), buffer);
        }
    }

    fn figure_finished(&mut self, tloc: TelemetryLocation, figure: &[OutputShape]) {
        use export::svg;
        use output::{OutputFigure};
        let _guard = ::flame::start_guard("telemetry figure_finished");

        let svg_path = self.figure_path(tloc, "figure.svg");
        svg::write_to_file(
            svg_path,
            OutputFigure {
                shapes: figure.iter().cloned().collect(),
                left: 0.0,
                top: 0.0,
                width: 0.0,
                height: 0.0,
            },
        ).unwrap();
    }
    fn scene_finished(&mut self, tloc: TelemetryLocation, scene: &OutputScene) {
        use export::svg;
        use std::fs::File;

        ::flame::end("scene");
        assert!(scene.figures.len() == 1);
        svg::write_to_file(self.scene_path(tloc, "scene.svg"), scene.figures[0].clone()).unwrap();
        ::flame::dump_text_to_writer(File::create(self.scene_path(tloc, "scene.perf")).unwrap()).unwrap();
        ::flame::clear();
    }

    fn scene_bounding_box(&mut self, tloc: TelemetryLocation, x: f32, y: f32, w: f32, h: f32) {
        use std::fs::File;
        use std::io::Write;
        let mut perf_file = File::create(self.scene_path(tloc, "scene.aabb")).unwrap();
        write!(perf_file, "x y: {} {}\nw h: {} {}", x, y, w, h).unwrap();
    }
}
