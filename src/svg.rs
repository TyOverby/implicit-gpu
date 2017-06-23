use super::backend::*;
use std::io::{Error, Write};

pub struct SvgBackend<W> {
    out: W,
}

impl <W: Write> SvgBackend<W> {
    pub fn new(mut out: W) -> Result<SvgBackend<W>, Error> {
        write!(out, "<svg xmlns=\"http://www.w3.org/2000/svg\">\n")?;
        Ok(SvgBackend {
            out: out
        })
    }
}

impl <W: Write> DrawBackend for SvgBackend<W> {
    type Error = Error;

    fn apply(&mut self, command: Command) -> Result<(), Error>{
        use super::backend::Command::*;

        match command {
            StartShape(options) => write!(&mut self.out, r#"    <path fill-rule="evenodd" d=""#),
            MoveTo { x , y } => write!(&mut self.out, "M{},{} ", x, y),
            LineTo { x, y } => write!(&mut self.out, "L{},{} ", x, y),
            CubicCurveTo { cx1, cy1, cx2, cy2, x, y, } => unimplemented!(),
            QuadraticCurveTo { cx, cy, x, y, } => unimplemented!(),
            ArcTo { rx, ry, rotation, large_arc, sweep, x, y, } => unimplemented!(),
            CloseShape => writeln!(&mut self.out, r#"z"/>"#),
            EndShape => writeln!(&mut self.out, r#""/>"#),
        }
    }
    fn close(mut self) -> Result<(), Error> {
        write!(&mut self.out, "</svg>")
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;

    fn run_in_canvas<F>(f: F) -> String
    where F: FnOnce(&mut Canvas<SvgBackend<&mut Vec<u8>>>) -> Result<(), std::io::Error> {
        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut canvas = Canvas::new(SvgBackend::new(&mut buffer).unwrap());
            f(&mut canvas).unwrap();
            canvas.close().unwrap();
        }
        return String::from_utf8(buffer).unwrap();
    }

    #[test]
    fn empty() {
        assert_eq!(run_in_canvas(|canvas| {
            Ok(())
        }).trim(), r#"
<svg xmlns="http://www.w3.org/2000/svg">
</svg>
        "#.trim())
    }

    #[test]
    fn empty_polygon() {
        assert_eq!(run_in_canvas(|canvas| {
            canvas.draw_closed_polygon(&[], DrawOptions::default())?;
            Ok(())
        }).trim(), r#"
<svg xmlns="http://www.w3.org/2000/svg">
</svg>
        "#.trim())
    }

    #[test]
    fn triangle_polygon() {
        assert_eq!(run_in_canvas(|canvas| {
            canvas.draw_closed_polygon(&[(0.0, 0.0), (0.0, 50.0), (50.0, 0.0)], DrawOptions::default())?;
            Ok(())
        }).trim(), r#"
<svg xmlns="http://www.w3.org/2000/svg">
    <path fill-rule="evenodd" d="M0,0 L0,50 L50,0 z"/>
</svg>
        "#.trim())
    }

    #[test]
    fn triangle_polygon_reversed() {
        assert_eq!(run_in_canvas(|canvas| {
            canvas.draw_closed_polygon(&[(50.0, 0.0), (0.0, 50.0), (0.0, 0.0)], DrawOptions::default())?;
            Ok(())
        }).trim(), r#"
<svg xmlns="http://www.w3.org/2000/svg">
    <path fill-rule="evenodd" d="M0,0 L0,50 L50,0 z"/>
</svg>
        "#.trim())
    }

    #[test]
    fn triangle_polygon_reversed_with_holes() {
        assert_eq!(run_in_canvas(|canvas| {
            canvas.draw_holy_polygon(
                vec![&[(50.0, 0.0), (0.0, 50.0), (0.0, 0.0)] as &[_]],
                vec![&[(30.0, 10.0), (10.0, 30.0), (10.0, 10.0)] as &[_]],
                DrawOptions::default()
            )
        }).trim(), r#"
<svg xmlns="http://www.w3.org/2000/svg">
    <path fill-rule="evenodd" d="M50,0 L0,0 L0,50 L50,0 M30,10 L10,30 L10,10 L30,10 "/>
</svg>
        "#.trim())
    }
}
