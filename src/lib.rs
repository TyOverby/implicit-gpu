#![deny(missing_docs)]
//! A crate for optimizing line drawing for plotters and

extern crate aabb_quadtree;
extern crate fnv;
extern crate smallvec;

mod dual_quad_tree;
mod optimize;
#[cfg(test)]
mod test;

use aabb_quadtree::*;
use smallvec::SmallVec;
use dual_quad_tree::*;

pub use optimize::optimize;

/// A single path segment that may be merged with other path segments.
#[derive(Debug, PartialEq, Eq)]
pub struct PathSegment(SmallVec<[Point; 1]>);

/// A single point in 2d space
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl std::cmp::Eq for Point {}


impl PathSegment {
    fn first(&self) -> Point {
        *self.0.first().unwrap()
    }

    fn last(&self) -> Point {
        *self.0.last().unwrap()
    }
}

impl Spatial for Point {
    fn aabb(&self) -> geom::Rect {
        geom::Rect::null_at(&geom::Point {
            x: self.x,
            y: self.y,
        })
    }
}
