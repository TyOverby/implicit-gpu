#![allow(unused_variables, dead_code)]

pub mod backend;
mod util;
pub mod svg;

use backend::{DrawOptions, Command};

pub struct Canvas<T> {
    backend: T
}

impl <T: backend::DrawBackend> Canvas<T> {
    pub fn new(t: T) -> Canvas<T> {
        Canvas {
            backend: t
        }
    }

    fn write_closed_polygon<I>(&mut self, mut i: I) -> Result<(), T::Error>
    where I: Iterator<Item=(f64, f64)>{
        if let Some((x, y)) = i.next() {
            self.backend.apply(Command::MoveTo{x, y})?;
            for (x, y) in i {
                self.backend.apply(Command::LineTo{x, y})?;
            }
        }
        Ok(())
    }

    pub fn draw_closed_polygon(&mut self, points: &[(f64, f64)], options: DrawOptions) -> Result<(), T::Error> {
        if points.len() == 0 { return Ok(()); }

        self.backend.apply(Command::StartShape(options))?;
        if util::is_clockwise(points) {
            self.write_closed_polygon(points.into_iter().cloned())?;
        } else {
            self.write_closed_polygon(points.into_iter().cloned().rev())?;
        }
        self.backend.apply(Command::CloseShape)
    }

    pub fn draw_holy_polygon<'a, I1, I2>(&mut self, additive: I1, subtractive: I2, options: DrawOptions) -> Result<(), T::Error>
    where I1: IntoIterator<Item=&'a[(f64, f64)]>, I2: IntoIterator<Item=&'a[(f64, f64)]> {
        self.backend.apply(Command::StartShape(options))?;
        for outline in additive {
            let iter = outline.iter().cloned().chain(Some(outline[0]));
            if util::is_clockwise(outline) {
                self.write_closed_polygon(iter)?;
            } else {
                self.write_closed_polygon(iter.rev())?;
            }
        }

        for outline in subtractive {
            let iter = outline.iter().cloned().chain(Some(outline[0]));
            if util::is_clockwise(outline) {
                self.write_closed_polygon(iter.rev())?;
            } else {
                self.write_closed_polygon(iter)?;
            }
        }
        self.backend.apply(Command::EndShape)
    }

    pub fn close(self) -> Result<(), T::Error> {
        self.backend.close()
    }
}
