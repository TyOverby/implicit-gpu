use super::opencl::FieldBuffer;
use super::output::{OutputShape, OutputScene};
use super::nodes::Node;
use std::path::{PathBuf, Path};
use std::fs::create_dir_all;

pub trait Telemetry {
    fn shape_finished(&mut self, buffer: &FieldBuffer, lines: &[((f32, f32), (f32, f32))]);
    fn intermediate_eval_basic(&mut self, buffer: &FieldBuffer, program: &str, node: &Node);
    fn intermediate_eval_poly(&mut self, buffer: &FieldBuffer);
    fn figure_finished(&mut self, figure: &[OutputShape]);

    fn scene_started(&mut self);
    fn scene_finished(&mut self, scene: &OutputScene);
}

pub struct NullTelemetry;

impl Telemetry for NullTelemetry {
    fn shape_finished(&mut self, _buffer: &FieldBuffer, _lines: &[((f32, f32), (f32, f32))]) {}
    fn intermediate_eval_basic(&mut self, _buffer: &FieldBuffer, _program: &str, _node: &Node) {}
    fn intermediate_eval_poly(&mut self, _buffer: &FieldBuffer) {}
    fn figure_finished(&mut self, _figure: &[OutputShape]) {}
    fn scene_started(&mut self) {}
    fn scene_finished(&mut self, _scene: &OutputScene) {}
}

pub struct DumpTelemetry {
    path: PathBuf,
    field_writer: Option<Box<Fn(&Path, &FieldBuffer)>>,
    line_writer: Option<Box<Fn(&Path, &[((f32, f32), (f32, f32))])>>,
    shape_count: usize,
    figure_count: usize,
    intermediate_count: usize,
}

impl DumpTelemetry {
    pub fn new<P: Into<PathBuf>>(root: P) -> DumpTelemetry {
        DumpTelemetry {
            path: root.into(),
            field_writer: None,
            line_writer: None,
            shape_count: 0,
            figure_count: 0,
            intermediate_count: 0,
        }
    }

    pub fn with_field_writer<F: 'static + Fn(&Path, &FieldBuffer)>(self, f: F) -> DumpTelemetry {
        DumpTelemetry {
            field_writer: Some(Box::new(f)),
            .. self
        }
    }

    pub fn with_line_writer<F: 'static + Fn(&Path, &[((f32, f32), (f32, f32))])>(self, f: F) -> DumpTelemetry {
        DumpTelemetry {
            line_writer: Some(Box::new(f)),
            .. self
        }
    }

    pub fn path_for_figure(&mut self) -> PathBuf {
        let r = self.path.join(format!("figure-{}", self.figure_count));
        self.intermediate_count = 0;
        self.shape_count = 0;
        self.figure_count += 1;
        r
    }

    pub fn path_for_shape(&mut self) -> PathBuf {
        let r = self.path.join(format!("shape-{}-{}", self.figure_count, self.shape_count));
        self.intermediate_count = 0;
        self.shape_count += 1;
        r
    }

    pub fn path_for_intermediate_polygon(&mut self) -> PathBuf {
        let r = self.path.join(format!("intermediate-{}-{}-{}-poly", self.figure_count, self.shape_count, self.intermediate_count));
        self.intermediate_count += 1;
        r
    }

    pub fn path_for_intermediate_basic(&mut self) -> PathBuf {
        let r = self.path.join(format!("intermediate-{}-{}-{}-basic", self.figure_count, self.shape_count, self.intermediate_count));
        self.intermediate_count += 1;
        r
    }
}

impl Telemetry for DumpTelemetry {
    fn scene_started(&mut self) {
        ::flame::start("scene");
    }

    fn shape_finished(&mut self, buffer: &FieldBuffer, lines: &[((f32, f32), (f32, f32))]) {
        let _guard = ::flame::start_guard("telemetry shape_finished");

        create_dir_all(&self.path).unwrap();
        let path_base = self.path_for_shape();
        let image_location = path_base.with_extension("png");

        ::debug::image::save_field_buffer(buffer, image_location, ::debug::image::ColorMode::Debug);

        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(&path_base, buffer);
        }

        if let Some(line_writer) = self.line_writer.as_ref() {
            (line_writer)(&path_base, lines);
        }

    }

    fn intermediate_eval_basic(&mut self, buffer: &FieldBuffer, program: &str, node: &Node) {
        let _guard = ::flame::start_guard("telemetry intermediate_eval_basic");
        create_dir_all(&self.path).unwrap();
        let path_base = self.path_for_intermediate_basic();
        ::debug::image::save_field_buffer(buffer, path_base.with_extension("png"), ::debug::image::ColorMode::Debug);
        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(&path_base, buffer);
        }
        ::latin::file::write(path_base.with_extension("c"), program).unwrap();
        ::latin::file::write(path_base.with_extension("node"), format!("{:#?}", node)).unwrap();
    }

    fn intermediate_eval_poly(&mut self, buffer: &FieldBuffer) {
        let _guard = ::flame::start_guard("telemetry intermediate_eval_poly");
        create_dir_all(&self.path).unwrap();
        let path_base = self.path_for_intermediate_polygon();
        ::debug::image::save_field_buffer(buffer, path_base.with_extension("png"), ::debug::image::ColorMode::Debug);
        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(&path_base, buffer);
        }
    }

    fn figure_finished(&mut self, figure: &[OutputShape]) {
        use export::svg;
        use output::{OutputScene, OutputFigure};
        let _guard = ::flame::start_guard("telemetry figure_finished");

        create_dir_all(&self.path).unwrap();
        let path_base = self.path_for_figure();
        let svg_path = path_base.with_extension("svg");
        svg::write_out(svg_path, OutputScene {
            figures: vec![
                OutputFigure {
                    shapes: figure.iter().cloned().collect()
                }
            ]
        }).unwrap();
    }
    fn scene_finished(&mut self, scene: &OutputScene) {
        use export::svg;
        use std::fs::File;

        ::flame::end("scene");
        let svg_path = self.path.join("scene").with_extension("svg");
        svg::write_out(svg_path, scene.clone()).unwrap();

        let perf_file = File::create(self.path.join("scene").with_extension("perf")).unwrap();
        ::flame::dump_text_to_writer(perf_file).unwrap();
        ::flame::clear();
    }
}
