use output::*;
use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::Path;

use vectorphile::backend::DrawOptions;
use vectorphile::svg::SvgBackend;
use vectorphile::Canvas;

pub fn write_to<W: Write>(write: W, figure: OutputFigure) -> IoResult<()> {
    let mut canvas = Canvas::new(SvgBackend::new(write)?);

    for shape in figure.shapes {
        match shape.lines {
            LineGroup::Polygon {
                filled,
                additive,
                subtractive,
            } => {
                let options = if filled {
                    DrawOptions::filled(shape.color)
                } else {
                    DrawOptions::stroked(shape.color, 2.0)
                };

                canvas.draw_holy_polygon(additive, subtractive, options)?;
            }
            LineGroup::Lines(_lines) => {
                unimplemented!();
            }
        }
    }
    canvas.close()?;

    Ok(())
}

pub fn write_to_file<P: AsRef<Path>>(path: P, out: OutputFigure) -> IoResult<()> {
    write_to(File::create(path)?, out)
}
