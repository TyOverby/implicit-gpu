#![deny(missing_docs)]
//! A crate for optimizing line drawing for plotters and

extern crate aabb_quadtree;
extern crate fnv;
extern crate smallvec;
extern crate itertools;

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
pub struct PathSegment {
    /// The path of points
    pub path: SmallVec<[Point; 2]>,
    /// True if the end of the path segment is the same as the
    /// beginning of the path segment.
    pub closed: bool,
}

/// A single point in 2d space
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl std::cmp::Eq for Point {}


impl PathSegment {
    fn new(path: SmallVec<[Point; 2]>, epsilon: f32) -> PathSegment {
        assert!(path.len() > 1);
        let first = path.first().cloned().unwrap();
        let last = path.last().cloned().unwrap();
        let first_pt = geom::Point {
                x: first.x,
                y: first.y,
        };
        let last_pt = geom::Point {
                x: last.x,
                y: last.y,
        };
        let query_rect = geom::Rect::centered_with_radius(&first_pt, epsilon);

        PathSegment {
            path: path,
            closed: query_rect.contains(&last_pt),
        }
    }

    fn first(&self) -> Point {
        *self.path.first().unwrap()
    }

    fn last(&self) -> Point {
        *self.path.last().unwrap()
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
