use super::backend::*;
use std::io::{Error, Write};

pub struct SvgBackend<W> {
    pub out: W,
    pub float_precision: usize,
}

impl<W: Write> SvgBackend<W> {
    pub fn new(mut out: W) -> Result<SvgBackend<W>, Error> {
        write!(out, "<svg xmlns=\"http://www.w3.org/2000/svg\">\n")?;
        Ok(SvgBackend {
            out: out,
            float_precision: 5,
        })
    }
}

impl<W: Write> DrawBackend for SvgBackend<W> {
    type Error = Error;

    fn apply(&mut self, command: Command) -> Result<(), Error> {
        use super::backend::Command::*;

        match command {
            StartShape(options) => {
                let style = match (
                    options.fill_color,
                    options.stroke_color,
                    options.stroke_size,
                ) {
                    (Some(fc), Some(sc), sz) => format!(
                        "fill: rgb{:?}; stroke: rgb{:?}; stroke-width: {:?};",
                        fc, sc, sz
                    ),
                    (None, Some(sc), sz) => {
                        format!("fill: none; stroke: rgb{:?}; stroke-width: {:?};", sc, sz)
                    }
                    (Some(fc), None, _) => {
                        format!("fill: rgb{:?}; stroke: none; stroke-width: none;", fc)
                    }
                    (None, None, fc) => {
                        format!("fill: rgb(0, 0, 0); stroke: none; stroke-width: {};", fc)
                    }
                };
                write!(
                    &mut self.out,
                    r#"    <path fill-rule="evenodd" style="{}" d=""#,
                    style
                )
            }
            MoveTo { x, y } => write!(
                &mut self.out,
                "\nM{:.p$},{:.p$} ",
                x,
                y,
                p = self.float_precision
            ),
            LineTo { x, y } => write!(
                &mut self.out,
                "\nL{:.p$},{:.p$} ",
                x,
                y,
                p = self.float_precision
            ),
            CubicCurveTo {
                cx1,
                cy1,
                cx2,
                cy2,
                x,
                y,
            } => unimplemented!(),
            QuadraticCurveTo { cx, cy, x, y } => unimplemented!(),
            ArcTo {
                rx,
                ry,
                rotation,
                large_arc,
                sweep,
                x,
                y,
            } => unimplemented!(),
            CloseShape => writeln!(&mut self.out, r#"z"/>"#),
            EndShape => writeln!(&mut self.out, r#""/>"#),
        }
    }
    fn close(mut self) -> Result<(), Error> {
        let script = r#"
<script>
let svgs = document.children[0];
let paths = Array.from(svgs.children).filter(function(c) {
    var orig = c.style.stroke;
    c.onmouseover = function() {
        c.style.stroke = "rgb(0, 0, 255)";
    };
    c.onmouseout = function () {
        c.style.stroke = orig;
    };
    c.onclick = function () {
        svgs.removeChild()
    }
});

for (let path of paths) {
    console.log(path);
}
</script>"#;
        write!(&mut self.out, "{}", script)?;
        write!(&mut self.out, "</svg>")
    }
}

/*
#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;

    fn run_in_canvas<F>(f: F) -> String
    where
        F: FnOnce(&mut Canvas<SvgBackend<&mut Vec<u8>>>) -> Result<(), std::io::Error>,
    {
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
        assert_eq!(
            run_in_canvas(|canvas| Ok(())).trim(),
            r#"
<svg xmlns="http://www.w3.org/2000/svg">
</svg>
        "#.trim()
        )
    }

    #[test]
    fn empty_polygon() {
        assert_eq!(
            run_in_canvas(|canvas| {
                canvas.draw_closed_polygon::<_, ()>(vec![], DrawOptions::default())?;
                Ok(())
            }).trim(),
            r#"
<svg xmlns="http://www.w3.org/2000/svg">
</svg>
        "#.trim()
        )
    }

    #[test]
    fn triangle_polygon() {
        assert_eq!(
            run_in_canvas(|canvas| {
                canvas.draw_closed_polygon::<_, ()>(
                    vec![point2(0.0, 0.0), point2(0.0, 50.0), point2(50.0, 0.0)],
                    DrawOptions::default(),
                )?;
                Ok(())
            }).trim(),
            r#"
<svg xmlns="http://www.w3.org/2000/svg">
    <path fill-rule="evenodd" d="M0,0 L0,50 L50,0 z"/>
</svg>
        "#.trim()
        )
    }

    #[test]
    fn triangle_polygon_reversed() {
        assert_eq!(
            run_in_canvas(|canvas| {
                canvas.draw_closed_polygon::<_, ()>(
                    vec![point2(50.0, 0.0), point2(0.0, 50.0), point2(0.0, 0.0)],
                    DrawOptions::default(),
                )?;
                Ok(())
            }).trim(),
            r#"
<svg xmlns="http://www.w3.org/2000/svg">
    <path fill-rule="evenodd" d="M0,0 L0,50 L50,0 z"/>
</svg>
        "#.trim()
        )
    }

    #[test]
    fn triangle_polygon_reversed_with_holes() {
        assert_eq!(
            run_in_canvas(|canvas| canvas.draw_holy_polygon::<_, _, ()>(
                vec![vec![point2(50.0, 0.0), point2(0.0, 50.0), point2(0.0, 0.0)]],
                vec![vec![
                    point2(30.0, 10.0),
                    point2(10.0, 30.0),
                    point2(10.0, 10.0),
                ]],
                DrawOptions::default(),
            )).trim(),
            r#"
<svg xmlns="http://www.w3.org/2000/svg">
    <path fill-rule="evenodd" d="M50,0 L0,0 L0,50 L50,0 M30,10 L10,30 L10,10 L30,10 "/>
</svg>
        "#.trim()
        )
    }
}
*/
