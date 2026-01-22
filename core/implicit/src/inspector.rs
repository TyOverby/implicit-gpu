use crate::geometry::PathSegment;
use crate::geometry::Point;
use crate::opencl::FieldBuffer;

use expectation::{Provider, extensions::*};

pub type BoxedInspector = Box<dyn Inspector>;

pub trait Inspector {
    fn duplicate(&self) -> BoxedInspector;
    fn specialize(&self, name: &str) -> BoxedInspector;
    fn write_ast(&self, name: &str, ast: &::gpu_interp::Ast);
    fn write_compiled(&self, name: &str, ast: &::gpu_interp::gpu::bytecode::CompilationResult);
    fn write_field(&self, name: &str, buffer: &mut FieldBuffer);
    fn write_segments(&self, name: &str, segments: &[PathSegment]);
    fn write_lines(&self, name: &str, lines: &[(Point, Point)]);
    fn do_slow(&self, f: &dyn Fn());
}

impl Inspector for () {
    fn duplicate(&self) -> BoxedInspector {
        Box::new(())
    }
    fn specialize(&self, _name: &str) -> BoxedInspector {
        Box::new(())
    }
    fn write_compiled(&self, _name: &str, _ast: &::gpu_interp::gpu::bytecode::CompilationResult) {}
    fn write_ast(&self, _name: &str, _ast: &::gpu_interp::Ast) {}
    fn write_field(&self, _name: &str, _buffer: &mut FieldBuffer) {}
    fn write_segments(&self, _name: &str, _segments: &[PathSegment]) {}
    fn write_lines(&self, _name: &str, _lines: &[(Point, Point)]) {}

    fn do_slow(&self, _f: &dyn Fn()) {}
}

impl Inspector for Provider {
    fn duplicate(&self) -> BoxedInspector {
        Box::new(self.clone())
    }
    fn specialize(&self, name: &str) -> BoxedInspector {
        Box::new(self.subdir(name))
    }
    fn write_ast(&self, name: &str, ast: &::gpu_interp::Ast) {
        use std::io::Write;
        let mut w_text = self.text_writer(format!("{}.ast.txt", name));
        write!(w_text, "{:#?}", ast).unwrap();
    }
    fn write_compiled(&self, name: &str, ast: &::gpu_interp::gpu::bytecode::CompilationResult) {
        use std::io::Write;
        let mut w_text = self.text_writer(format!("{}.compiled.txt", name));
        write!(w_text, "{:#?}", ast).unwrap();
    }
    fn write_field(&self, name: &str, buffer: &mut FieldBuffer) {
        use crate::debug::*;
        let w_color = self.png_writer(format!("{}.color.png", name));
        save_field_buffer(buffer, w_color, ColorMode::Debug);
        let w_bw = self.png_writer(format!("{}.bw.png", name));
        save_field_buffer(buffer, w_bw, ColorMode::BlackAndWhite);

        let mut buf = self.custom_test("buffer.buf", |_, _| Ok(true), |_, _, _, _| Ok(()));
        buffer_dump::write(&mut buf, buffer).unwrap();
    }

    fn write_segments(&self, name: &str, segments: &[PathSegment]) {
        use crate::debug::*;

        let tuple_writer = self.text_writer(format!("{}.txt", name));
        print_path_segments(tuple_writer, segments);

        let svg_writer = self.text_writer(format!("{}.svg", name));
        svg_path_segments(svg_writer, segments).unwrap();
    }

    fn write_lines(&self, name: &str, lines: &[(Point, Point)]) {
        use std::io::Write;
        let mut writer = self.text_writer(format!("{}.txt", name));
        for line in lines {
            writeln!(writer, "{:?}", line).unwrap();
        }
    }

    fn do_slow(&self, f: &dyn Fn()) {
        f()
    }
}
