#![deny(missing_docs)]
//! A crate for optimizing line drawing for plotters and

extern crate aabb_quadtree;
extern crate euclid;
extern crate fnv;
extern crate itertools;
#[cfg(test)]
extern crate permutohedron;
extern crate smallvec;

mod dual_quad_tree;
mod graph_stitch;
mod optimize;
mod test;
mod prune;
mod zero_area_loop;
pub(crate) mod util;

use aabb_quadtree::*;
use smallvec::SmallVec;
use dual_quad_tree::*;

pub use optimize::optimize;
pub use prune::prune;
pub use graph_stitch::connect_unconnected as graph_stitch;
pub use zero_area_loop::remove_zero_area_loops;

type Point<S> = euclid::TypedPoint2D<f32, S>;

/// A single path segment that may be merged with other path segments.
#[derive(PartialEq, Clone)]
pub struct PathSegment<S> {
    /// The path of points
    pub path: SmallVec<[Point<S>; 2]>,
    /// True if the end of the path segment is the same as the
    /// beginning of the path segment.
    pub closed: bool,
}

impl<S> ::std::fmt::Debug for PathSegment<S> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("PathSegment")
            .field("path", &self.path)
            .field("closed", &self.closed)
            .finish()
    }
}

impl<S> PathSegment<S> {
    /// TODO: doc
    pub fn new<P: Into<SmallVec<[Point<S>; 2]>>>(path: P, epsilon: f32) -> PathSegment<S> {
        let mut path = path.into();

        assert!(path.len() > 1);
        let first = path.first().cloned().unwrap();
        let last = path.last().cloned().unwrap();
        let first_pt: Point<S> = Point::new(first.x, first.y);
        let last_pt: Point<S> = Point::new(last.x, last.y);

        let query_rect = util::centered_with_radius(first_pt, epsilon);
        let closed = query_rect.contains(&last_pt);
        if closed {
            path.pop();
        }

        PathSegment {
            path: path,
            closed: closed,
        }
    }

    fn first(&self) -> Point<S> {
        *self.path.first().unwrap()
    }

    fn last(&self) -> Point<S> {
        *self.path.last().unwrap()
    }

    /// TODO: document
    pub fn length_2(&self) -> f32 {
        self.path
            .as_slice()
            .windows(2)
            .map(|s| (s[1] - s[0]).square_length())
            .sum()
    }

    /// TODO: document
    pub fn length(&self) -> f32 {
        self.path
            .as_slice()
            .windows(2)
            .map(|s| (s[1] - s[0]).length())
            .sum()
    }
}
