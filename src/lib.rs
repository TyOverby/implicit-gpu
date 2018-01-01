#![allow(unused_variables, dead_code)]

extern crate euclid;

pub mod backend;
mod util;
pub mod svg;

use euclid::TypedPoint2D;
use backend::{Command, DrawOptions};
use std::ops::{Deref, DerefMut};

pub struct Canvas<T> {
    backend: T,
}

impl<T> Deref for Canvas<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.backend
    }
}

impl<T> DerefMut for Canvas<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.backend
    }
}

impl<T: backend::DrawBackend> Canvas<T> {
    pub fn new(t: T) -> Canvas<T> {
        Canvas { backend: t }
    }

    fn write_closed_polygon<I, K>(&mut self, i: I) -> Result<(), T::Error>
    where
        I: IntoIterator<Item = TypedPoint2D<f32, K>>,
    {
        let mut i = i.into_iter();
        if let Some(point) = i.next() {
            self.backend.apply(Command::MoveTo {
                x: point.x as f64,
                y: point.y as f64,
            })?;
            for point in i {
                self.backend.apply(Command::LineTo {
                    x: point.x as f64,
                    y: point.y as f64,
                })?;
            }
        }
        Ok(())
    }

    pub fn draw_closed_polygon<I, K>(
        &mut self,
        points: I,
        options: DrawOptions,
    ) -> Result<(), T::Error>
    where
        I: IntoIterator<Item = TypedPoint2D<f32, K>>,
    {
        let mut points = points.into_iter().collect::<Vec<_>>();
        if !util::is_clockwise(&points[..]) {
            points.reverse();
        }

        self.backend.apply(Command::StartShape(options))?;
        self.write_closed_polygon(points)?;
        self.backend.apply(Command::CloseShape)
    }

    pub fn draw_holy_polygon<'a, I1, I2, K: 'static>(
        &mut self,
        additive: I1,
        subtractive: I2,
        options: DrawOptions,
    ) -> Result<(), T::Error>
    where
        I1: IntoIterator<Item = Vec<TypedPoint2D<f32, K>>>,
        I2: IntoIterator<Item = Vec<TypedPoint2D<f32, K>>>,
    {
        self.backend.apply(Command::StartShape(options))?;
        for outline in additive {
            let iter = outline.iter().cloned().chain(Some(outline[0]));
            if util::is_clockwise(&outline) {
                self.write_closed_polygon(iter)?;
            } else {
                self.write_closed_polygon(iter.rev())?;
            }
        }

        for outline in subtractive {
            let iter = outline.iter().cloned().chain(Some(outline[0]));
            if util::is_clockwise(&outline) {
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
