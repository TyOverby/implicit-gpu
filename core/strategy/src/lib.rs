extern crate extern_api;

use extern_api::*;
use std::borrow::Cow;

pub trait FieldBuffer {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn depth(&self) -> u32;

    fn values(&mut self) -> Cow<[f32]>;
}

pub trait LineBuffer {
    fn values(&mut self) -> Cow<[f32]>;
}

pub trait Strategy {
    type FieldBuf: FieldBuffer;
    type LineBuf: LineBuffer;

    fn march_2d(&self, buf: Self::FieldBuf) -> Self::LineBuf;

    fn drag_2d(&self, buf: Self::FieldBuf, dx: f32, dy: f32) -> Self::FieldBuf;
    fn freeze_2d(&self, buf: Self::FieldBuf) -> Self::FieldBuf;
    fn noise_2d(
        &self,
        width: u32,
        height: u32,
        cutoff: f32,
        matrix: extern_api::Matrix,
    ) -> Self::FieldBuf;
    fn poly_2d(&self, polygon: Polygon, width: u32, height: u32) -> Self::FieldBuf;

    fn shape<F>(&self, shape: Shape, width: u32, height: u32, buffer_find: F) -> Self::FieldBuf
    where
        F: Fn(Id) -> Self::FieldBuf;
}
