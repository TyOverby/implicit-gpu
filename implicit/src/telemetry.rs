use super::lines::util::geom;
use super::nodes::Node;
use super::opencl::FieldBuffer;
use super::output::{OutputScene, OutputShape};
use lines::LineType;
use std::fs::create_dir_all;
use std::path::PathBuf;

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
    fn shape_line_pre_prune(&mut self, _t: TelemetryLocation, _lines: &[geom::Line]) {}
    fn shape_line_pruned(&mut self, _t: TelemetryLocation, _lines: &[geom::Line]) {}
    fn shape_line_joined(&mut self, _t: TelemetryLocation, _lines: &[LineType]) {}
    fn figure_finished(&mut self, _t: TelemetryLocation, _figure: &[OutputShape]) {}

    fn scene_started(&mut self) {}
    fn scene_bounding_box(&mut self, _t: TelemetryLocation, _: f32, _: f32, _: f32, _: f32) {}
    fn scene_finished(&mut self, _t: TelemetryLocation, _scene: &OutputScene) {}
}

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

        ::debug::image::save_field_buffer(buffer, self.shape_path(tloc, "field.png"), ::debug::image::ColorMode::Debug);

        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(self.shape_path(tloc, "field.values"), buffer);
        }

        if let Some(line_writer) = self.line_writer.as_ref() {
            (line_writer)(self.shape_path(tloc, "outlines.lines"), lines);
        }

    }

    fn shape_line_pre_prune(&mut self, tloc: TelemetryLocation, lines: &[geom::Line]) {
        use std::fs::File;
        let _guard = ::flame::start_guard("telemetry shape_line_joined");
        let file = File::create(self.shape_path(tloc, "pre-pruned.svg")).unwrap();
        output_svg_lines(file, lines.iter().cloned());
    }

    fn shape_line_pruned(&mut self, tloc: TelemetryLocation, lines: &[geom::Line]) {
        use std::fs::File;

        let _guard = ::flame::start_guard("telemetry shape_line_joined");
        let file = File::create(self.shape_path(tloc, "pruned.svg")).unwrap();
        output_svg_lines(file, lines.iter().cloned());
    }

    fn shape_line_joined(&mut self, tloc: TelemetryLocation, lines: &[LineType]) {
        use std::fs::File;

        let _guard = ::flame::start_guard("telemetry shape_line_joined");
        let file = File::create(self.shape_path(tloc, "joined.svg")).unwrap();
        output_svg_linetype(file, lines.iter());
    }

    fn intermediate_eval_basic(&mut self, tloc: TelemetryLocation, buffer: &FieldBuffer, program: &str, node: &Node) {
        let _guard = ::flame::start_guard("telemetry intermediate_eval_basic");

        ::debug::image::save_field_buffer(
            buffer,
            self.intermediate_path(tloc, "field.png"),
            ::debug::image::ColorMode::Debug,
        );
        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(self.intermediate_path(tloc, "field.values"), buffer);
        }

        ::latin::file::write(self.intermediate_path(tloc, "shader.c"), program).unwrap();
        ::latin::file::write(self.intermediate_path(tloc, "node.txt"), format!("{:#?}", node)).unwrap();
    }

    fn intermediate_eval_poly(&mut self, tloc: TelemetryLocation, buffer: &FieldBuffer) {
        let _guard = ::flame::start_guard("telemetry intermediate_eval_poly");

        ::debug::image::save_field_buffer(
            buffer,
            self.intermediate_path(tloc, "field.png"),
            ::debug::image::ColorMode::Debug,
        );
        if let Some(field_writer) = self.field_writer.as_ref() {
            (field_writer)(self.intermediate_path(tloc, "field.values"), buffer);
        }
    }

    fn figure_finished(&mut self, tloc: TelemetryLocation, figure: &[OutputShape]) {
        use export::svg;
        use output::{OutputFigure, OutputScene};
        let _guard = ::flame::start_guard("telemetry figure_finished");

        let svg_path = self.figure_path(tloc, "figure.svg");
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
    fn scene_finished(&mut self, tloc: TelemetryLocation, scene: &OutputScene) {
        use export::svg;
        use std::fs::File;

        ::flame::end("scene");
        svg::write_to_file(self.scene_path(tloc, "scene.svg"), scene.clone()).unwrap();

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

fn output_svg_linetype<'a, I>(file: ::std::fs::File, lines: I)
where
    I: Iterator<Item = &'a LineType>,
{
    use vectorphile::Canvas;
    use vectorphile::backend::{Command, DrawBackend, DrawOptions};
    use vectorphile::svg::SvgBackend;
    let mut canvas = Canvas::new(SvgBackend::new(file).unwrap());

    for line in lines {
        let (pts, restitch) = match line {
            &LineType::Joined(ref pts) => {
                canvas
                    .apply(Command::StartShape(DrawOptions::stroked((0, 0, 0), 0.2)))
                    .unwrap();
                (&pts[..], true)
            }
            &LineType::Unjoined(ref pts) => {
                canvas
                    .apply(Command::StartShape(DrawOptions::stroked((255, 0, 0), 0.2)))
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

fn output_svg_lines<'a, I>(file: ::std::fs::File, lines: I)
where
    I: Iterator<Item = geom::Line>,
{
    use vectorphile::Canvas;
    use vectorphile::backend::{Command, DrawBackend, DrawOptions};
    use vectorphile::svg::SvgBackend;
    let mut canvas = Canvas::new(SvgBackend::new(file).unwrap());

    canvas
        .apply(Command::StartShape(DrawOptions::stroked((0, 0, 0), 0.2)))
        .unwrap();

    for geom::Line(p1, p2) in lines {
        canvas
            .apply(Command::MoveTo { x: p1.x as f64, y: p1.y as f64 })
            .unwrap();
        canvas
            .apply(Command::LineTo { x: p2.x as f64, y: p2.y as f64 })
            .unwrap();
    }

    canvas.apply(Command::EndShape).unwrap();

    canvas.close().unwrap();
}
