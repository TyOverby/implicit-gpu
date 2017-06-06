#![allow(unused_variables, dead_code)]

pub mod backend;
mod util;
pub mod svg;

use backend::{DrawOptions, DrawBackend, Command};

pub struct Canvas<T> {
    backend: T
}

impl <T: backend::DrawBackend> Canvas<T> {
    fn new(t: T) -> Canvas<T> {
        Canvas {
            backend: t
        }
    }

    fn draw_closed_polygon(&mut self, points: &[(f64, f64)], options: DrawOptions) -> Result<(), T::Error> {
        if points.len() == 0 { return Ok(()); }

        fn write_closed_polygon<B, I>(mut i: I, backend: &mut B, options: DrawOptions) -> Result<(), B::Error>
        where I: Iterator<Item=(f64, f64)>, B: DrawBackend {
            if let Some((x, y)) = i.next() {
                backend.apply(Command::StartShape(options))?;
                backend.apply(Command::MoveTo{x, y})?;
                for (x, y) in i {
                    backend.apply(Command::LineTo{x, y})?;
                }
                backend.apply(Command::CloseShape)?;
            }
            Ok(())
        }

        if util::is_clockwise(points) {
            write_closed_polygon(points.into_iter().cloned(), &mut self.backend, options)
        } else {
            write_closed_polygon(points.into_iter().cloned().rev(), &mut self.backend, options)
        }
    }

    fn close(self) -> Result<(), T::Error> {
        self.backend.close()
    }
}
