use geometry::PathSegment;
use geometry::Point;
use opencl::FieldBuffer;

#[cfg(test)]
use expectation::{extensions::*, Provider};

pub type BoxedInspector = Box<Inspector>;

pub trait Inspector {
    fn duplicate(&self) -> BoxedInspector;
    fn specialize(&self, name: &str) -> BoxedInspector;
    fn write_field(&self, name: &str, buffer: &FieldBuffer);
    fn write_segments(&self, name: &str, segments: &[PathSegment]);
    fn write_lines(&self, name: &str, lines: &[(Point, Point)]);
    fn do_slow(&self, f: &Fn());
}

impl Inspector for () {
    fn duplicate(&self) -> BoxedInspector {
        Box::new(())
    }
    fn specialize(&self, _name: &str) -> BoxedInspector {
        Box::new(())
    }
    fn write_field(&self, _name: &str, _buffer: &FieldBuffer) {}
    fn write_segments(&self, _name: &str, _segments: &[PathSegment]) {}
    fn write_lines(&self, _name: &str, _lines: &[(Point, Point)]) {}

    fn do_slow(&self, _f: &Fn()) {}
}

#[cfg(test)]
impl Inspector for Provider {
    fn duplicate(&self) -> BoxedInspector {
        Box::new(self.clone())
    }
    fn specialize(&self, name: &str) -> BoxedInspector {
        Box::new(self.subdir(name))
    }
    fn write_field(&self, name: &str, buffer: &FieldBuffer) {
        use debug::*;
        let w_color = self.png_writer(format!("{}.color.png", name));
        save_field_buffer(&buffer, w_color, ColorMode::Debug);
        let w_bw = self.png_writer(format!("{}.bw.png", name));
        save_field_buffer(&buffer, w_bw, ColorMode::BlackAndWhite);
    }

    fn write_segments(&self, name: &str, segments: &[PathSegment]) {
        use exec::print_path_segments;
        let writer = self.text_writer(format!("{}.txt", name));
        print_path_segments(writer, segments);
    }

    fn write_lines(&self, name: &str, lines: &[(Point, Point)]) {
        use std::io::Write;
        let mut writer = self.text_writer(format!("{}.txt", name));
        for line in lines {
            writeln!(writer, "{:?}", line);
        }
    }

    fn do_slow(&self, f: &Fn()) {
        f()
    }
}
