use super::nodes::Node;
use super::opencl::FieldBuffer;
use super::output::{OutputScene, OutputShape};
use lines::LineType;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

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

    fn scene_path(&self) -> PathBuf { PathBuf::new() }
    fn figure_path(&self) -> PathBuf {
        let mut r = PathBuf::new();
        r.push(format!("figure_{}", self.figure));
        r
    }

    fn shape_path(&self) -> PathBuf {
        let mut r = PathBuf::new();
        r.push(format!("figure_{}", self.figure));
        r.push(format!("shape_{}", self.shape));
        r
    }

    fn intermediate_path(&self) -> PathBuf {
        let mut r = PathBuf::new();
        r.push(format!("figure_{}", self.figure));
        r.push(format!("shape_{}", self.shape));
        r.push(format!("intermediate_{}", self.intermediate));
        r
    }
}

pub trait Telemetry {
    fn shape_finished(&mut self, t: TelemetryLocation, buffer: &FieldBuffer, lines: &[((f32, f32), (f32, f32))]);
    fn shape_line_joined(&mut self, t: TelemetryLocation, lines: &[LineType]);
    fn intermediate_eval_basic(&mut self, t: TelemetryLocation, buffer: &FieldBuffer, program: &str, node: &Node);
    fn intermediate_eval_poly(&mut self, t: TelemetryLocation, buffer: &FieldBuffer);
    fn figure_finished(&mut self, t: TelemetryLocation, figure: &[OutputShape]);

    fn scene_started(&mut self);
    fn scene_bounding_box(&mut self, t: TelemetryLocation, f32, f32, f32, f32);
    fn scene_finished(&mut self, t: TelemetryLocation, scene: &OutputScene);
}

pub struct NullTelemetry;

impl Telemetry for NullTelemetry {
    fn shape_finished(&mut self, _t: TelemetryLocation, _buffer: &FieldBuffer, _lines: &[((f32, f32), (f32, f32))]) {}
    fn intermediate_eval_basic(&mut self, _t: TelemetryLocation, _buffer: &FieldBuffer, _program: &str, _node: &Node) {}
    fn intermediate_eval_poly(&mut self, _t: TelemetryLocation, _buffer: &FieldBuffer) {}
    fn shape_line_joined(&mut self, _t: TelemetryLocation, _lines: &[LineType]) {}
    fn figure_finished(&mut self, _t: TelemetryLocation, _figure: &[OutputShape]) {}

    fn scene_started(&mut self) {}
    fn scene_bounding_box(&mut self, _t: TelemetryLocation, _: f32, _: f32, _: f32, _: f32) {}
    fn scene_finished(&mut self, _t: TelemetryLocation, _scene: &OutputScene) {}
}

pub struct DumpTelemetry {
    path: PathBuf,
    field_writer: Option<Box<Fn(&Path, &FieldBuffer)>>,
    line_writer: Option<Box<Fn(&Path, &[((f32, f32), (f32, f32))])>>,
}

impl DumpTelemetry {
    pub fn new<P: Into<PathBuf>>(root: P) -> DumpTelemetry {
        DumpTelemetry {
            path: root.into(),
            field_writer: None,
            line_writer: None,
        }
    }

    pub fn with_field_writer<F: 'static + Fn(&Path, &FieldBuffer)>(self, f: F) -> DumpTelemetry {
        DumpTelemetry {
            field_writer: Some(Box::new(f)),
            ..self
        }
    }

    pub fn with_line_writer<F: 'static + Fn(&Path, &[((f32, f32), (f32, f32))])>(self, f: F) -> DumpTelemetry {
        DumpTelemetry {
            line_writer: Some(Box::new(f)),
            ..self
        }
    }
}

impl Telemetry for DumpTelemetry {
    fn scene_started(&mut self) { ::flame::start("scene"); }

    fn shape_finished(&mut self, t: TelemetryLocation, buffer: &FieldBuffer, lines: &[((f32, f32), (f32, f32))]) {
        let _guard = ::flame::start_guard("telemetry shape_finished");

        create_dir_all(&self.path).unwrap();
        let path_base = self.path.join(t.shape_path());
        let image_location = path_base.join("field.png");

        ::debug::image::save_field_buffer(buffer, image_location, ::debug::image::ColorMode::Debug);

        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(&path_base.join("field.values"), buffer);
        }

        if let Some(line_writer) = self.line_writer.as_ref() {
            (line_writer)(&path_base.join("outline.lines"), lines);
        }

    }

    fn shape_line_joined(&mut self, t: TelemetryLocation, lines: &[LineType]) {
        use vectorphile::Canvas;
        use vectorphile::backend::{Command, DrawBackend, DrawOptions};
        use vectorphile::svg::SvgBackend;
        use std::fs::File;

        let _guard = ::flame::start_guard("telemetry shape_line_joined");

        let path_base = self.path.join(t.intermediate_path());
        create_dir_all(&path_base).unwrap();
        let file = File::create(path_base.join("joined.svg")).unwrap();
        let mut canvas = Canvas::new(SvgBackend::new(file).unwrap());
        for line in lines {
            let (pts, restitch) = match line {
                &LineType::Joined(ref pts) => {
                    canvas
                        .apply(Command::StartShape(DrawOptions::stroked((0, 0, 0), 0.1)))
                        .unwrap();
                    (&pts[..], true)
                }
                &LineType::Unjoined(ref pts) => {
                    canvas
                        .apply(Command::StartShape(DrawOptions::stroked((255, 0, 0), 0.1)))
                        .unwrap();
                    (&pts[..], false)
                }
            };
            if pts.len() > 0 {
                canvas
                    .apply(Command::MoveTo {
                        x: pts[0].x as f64,
                        y: pts[0].y as f64,
                    })
                    .unwrap();
                canvas
                    .apply_all(
                        pts.iter()
                            .skip(1)
                            .map(|pt| Command::LineTo { x: pt.x as f64, y: pt.y as f64 }),
                    )
                    .unwrap();

                if restitch {
                    canvas
                        .apply(Command::LineTo {
                            x: pts[0].x as f64,
                            y: pts[0].y as f64,
                        })
                        .unwrap();
                }
            }
            canvas.apply(Command::EndShape).unwrap();
        }
        canvas.close().unwrap();
    }

    fn intermediate_eval_basic(&mut self, t: TelemetryLocation, buffer: &FieldBuffer, program: &str, node: &Node) {
        let _guard = ::flame::start_guard("telemetry intermediate_eval_basic");

        let path_base = self.path.join(t.intermediate_path());
        create_dir_all(&path_base).unwrap();

        ::debug::image::save_field_buffer(buffer, path_base.join("field.png"), ::debug::image::ColorMode::Debug);
        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(&path_base.join("field.values"), buffer);
        }

        ::latin::file::write(path_base.join("shader.c"), program).unwrap();
        ::latin::file::write(path_base.join("node.txt"), format!("{:#?}", node)).unwrap();
    }

    fn intermediate_eval_poly(&mut self, t: TelemetryLocation, buffer: &FieldBuffer) {
        let _guard = ::flame::start_guard("telemetry intermediate_eval_poly");
        let path_base = self.path.join(t.intermediate_path());
        create_dir_all(&path_base).unwrap();

        ::debug::image::save_field_buffer(buffer, path_base.join("field.png"), ::debug::image::ColorMode::Debug);
        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(&path_base.join("field.values"), buffer);
        }
    }

    fn figure_finished(&mut self, t: TelemetryLocation, figure: &[OutputShape]) {
        use export::svg;
        use output::{OutputFigure, OutputScene};
        let _guard = ::flame::start_guard("telemetry figure_finished");

        let path_base = self.path.join(t.figure_path());
        create_dir_all(&path_base).unwrap();

        let svg_path = path_base.join("figure.svg");
        svg::write_to_file(
            svg_path,
            OutputScene {
                figures: vec![
                    OutputFigure {
                        shapes: figure.iter().cloned().collect(),
                    },
                ],
            },
        ).unwrap();
    }
    fn scene_finished(&mut self, t: TelemetryLocation, scene: &OutputScene) {
        use export::svg;
        use std::fs::File;

        ::flame::end("scene");
        let path_base = self.path.join(t.scene_path());
        let svg_path = path_base.join("scene.svg");
        svg::write_to_file(svg_path, scene.clone()).unwrap();

        let perf_file = File::create(path_base.join("scene.perf")).unwrap();
        ::flame::dump_text_to_writer(perf_file).unwrap();
        ::flame::clear();
    }

    fn scene_bounding_box(&mut self, t: TelemetryLocation, x: f32, y: f32, w: f32, h: f32) {
        use std::fs::File;
        use std::io::Write;
        let bb_path = self.path.join(t.scene_path());
        let mut perf_file = File::create(bb_path.join("scene.aabb")).unwrap();
        write!(perf_file, "x y: {} {}\nw h: {} {}", x, y, w, h).unwrap();
    }
}
