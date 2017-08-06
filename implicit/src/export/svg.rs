
use output::*;
use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::Path;
use vectorphile::Canvas;
use vectorphile::backend::DrawOptions;
use vectorphile::svg::SvgBackend;

pub fn write_to<R: Write>(write: R, mut out: OutputScene) -> IoResult<()> {

    assert!(out.figures.len() == 1, "only support 1 figure per scene");
    let figure = out.figures.pop().unwrap();

    let mut canvas = Canvas::new(SvgBackend::new(write)?);

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
                    options,
                )?;
            }
            LineGroup::Lines(_lines) => {
                unimplemented!();
            }
        }
    }
    canvas.close()?;

    Ok(())
}

pub fn write_to_file<P: AsRef<Path>>(path: P, out: OutputScene) -> IoResult<()> {
    write_to(File::create(path)?, out)
}
