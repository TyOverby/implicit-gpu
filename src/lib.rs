#![deny(missing_docs)]
//! A crate for optimizing line drawing for plotters and

extern crate aabb_quadtree;
extern crate fnv;
extern crate itertools;
#[cfg(test)]
extern crate permutohedron;
extern crate smallvec;

mod dual_quad_tree;
mod optimize;
mod test;
mod prune;
pub(crate) mod util;

use aabb_quadtree::*;
use smallvec::SmallVec;
use dual_quad_tree::*;


pub use optimize::optimize;
pub use prune::prune;

/// A single path segment that may be merged with other path segments.
#[derive(Debug, PartialEq, Eq, Clone)]
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
    /// TODO: doc
    pub fn new<P: Into<SmallVec<[Point; 2]>>>(path: P, epsilon: f32) -> PathSegment {
        let mut path = path.into();

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
        let closed = query_rect.contains(&last_pt);
        if closed {
            path.pop();
        }

        PathSegment {
            path: path,
            closed: closed,
        }
    }

    fn first(&self) -> Point {
        *self.path.first().unwrap()
    }

    fn last(&self) -> Point {
        *self.path.last().unwrap()
    }

    /// TODO: document
    pub fn length_2(&self) -> f32 {
        self.path
            .as_slice()
            .windows(2)
            .map(|s| dist_2(s[0], s[1]))
            .sum()
    }

    /// TODO: document
    pub fn length(&self) -> f32 {
        self.path
            .as_slice()
            .windows(2)
            .map(|s| dist_2(s[0], s[1]).sqrt())
            .sum()
    }
}

fn dist_2(Point { x: x1, y: y1 }: Point, Point { x: x2, y: y2 }: Point) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    dx * dx + dy * dy
}

impl Spatial for Point {
    fn aabb(&self) -> geom::Rect {
        geom::Rect::null_at(&geom::Point {
            x: self.x,
            y: self.y,
        })
    }
}
