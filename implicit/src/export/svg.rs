use std::path::Path;
use std::fs::File;
use std::io::Result as IoResult;
use output::*;
use vectordraw::Canvas;
use vectordraw::backend::DrawOptions;
use vectordraw::svg::SvgBackend;


pub fn write_out<P: AsRef<Path>>(path: P, mut out: OutputScene) -> IoResult<()> {
    assert!(out.figures.len() == 1, "only support 1 figure per scene");
    let figure = out.figures.pop().unwrap();

    let mut canvas = Canvas::new(SvgBackend::new(File::create(path)?)?);

    for shape in figure.shapes {
        match shape.lines {
            LineGroup::Polygon { filled, additive, subtractive } => {
                let options = if filled {
                    DrawOptions::filled(shape.color)
                } else {
                    DrawOptions::stroked(shape.color, 2.0)
                };

                canvas.draw_holy_polygon(
                    additive.iter().map(Vec::as_slice),
                    subtractive.iter().map(Vec::as_slice),
                    options)?;
            }
            LineGroup::Lines(_lines) => {
                unimplemented!();
            }
        }
    }
    canvas.close()?;

    Ok(())
}
