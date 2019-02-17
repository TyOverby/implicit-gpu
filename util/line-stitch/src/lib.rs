#![deny(missing_docs)]
//! A crate for optimizing line drawing for plotters and laser cutters.

extern crate aabb_quadtree;
extern crate euclid;
extern crate flame;
extern crate fnv;
extern crate itertools;
#[cfg(test)]
extern crate permutohedron;
extern crate rayon;
extern crate smallvec;

mod connect_obvious;
mod dual_quad_tree;
mod graph_stitch;
mod prune;
mod test;
pub(crate) mod util;
mod zero_area_loop;

use aabb_quadtree::*;
pub use connect_obvious::{connect_obvious, connect_obvious_from_dual_qt};
use dual_quad_tree::*;
pub use graph_stitch::connect_unconnected as graph_stitch;
pub use prune::prune;
use smallvec::SmallVec;
use std::cell::Cell;
use std::cmp::Ordering;
use std::iter::{FromIterator, IntoIterator};
pub use zero_area_loop::remove_zero_area_loops;

type Point<S> = euclid::TypedPoint2D<f32, S>;

/// A single path segment that may be merged with other
/// path segments.
#[derive(Clone)]
pub struct PathSegment<S> {
    /// The path of points
    pub path: SmallVec<[Point<S>; 2]>,
    /// True if the end of the path segment is the same as the
    /// beginning of the path segment.
    pub closed: bool,
    length_2: Cell<f32>,
    length: Cell<f32>,
}

impl<S> PartialEq for PathSegment<S> {
    fn eq(&self, other: &Self) -> bool {
        if self.closed != other.closed {
            return false;
        }
        if self.path.len() != other.path.len() {
            return false;
        }

        for (a, b) in self.path.iter().zip(other.path.iter()) {
            if a != b {
                return false;
            }
        }
        return true;
    }
}
impl<S> Eq for PathSegment<S> {}

impl<S> PartialOrd for PathSegment<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use std::cmp::Ordering;

        if self.path.len() < other.path.len() {
            return Some(Ordering::Less);
        } else if self.path.len() > other.path.len() {
            return Some(Ordering::Greater);
        } else if self.length() < other.length() {
            return Some(Ordering::Less);
        } else if self.length() > other.length() {
            return Some(Ordering::Greater);
        } else {
            for (a, b) in self.path.iter().zip(other.path.iter()) {
                match (a.x.partial_cmp(&b.x), a.y.partial_cmp(&b.y)) {
                    (None, _) | (_, None) => return None,
                    (Some(a), Some(b)) => match a.then(b) {
                        Ordering::Equal => {}
                        other => return Some(other),
                    },
                }
            }
        }
        return Some(Ordering::Equal);
    }
}
impl<S> Ord for PathSegment<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

unsafe impl<S> Sync for PathSegment<S> {}

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
    pub fn new_and_potentially_close<P: Into<SmallVec<[Point<S>; 2]>>>(
        path: P,
        epsilon: f32,
    ) -> PathSegment<S> {
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
            length_2: Cell::new(0.0),
            length: Cell::new(0.0),
        }
    }

    /// TODO: doc
    pub fn new<P: Into<SmallVec<[Point<S>; 2]>>>(path: P) -> PathSegment<S> {
        let path = path.into();
        assert!(path.len() > 1);
        PathSegment {
            path: path,
            closed: false,
            length_2: Cell::new(0.0),
            length: Cell::new(0.0),
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
        if self.length_2.get() != 0.0 {
            return self.length_2.get();
        }

        let length_2 = self
            .path
            .as_slice()
            .windows(2)
            .map(|s| (s[1] - s[0]).square_length())
            .sum();

        self.length_2.set(length_2);

        return length_2;
    }

    /// TODO: document
    pub fn length(&self) -> f32 {
        if self.length.get() != 0.0 {
            return self.length.get();
        }

        let length = self
            .path
            .as_slice()
            .windows(2)
            .map(|s| (s[1] - s[0]).length())
            .sum();
        self.length.set(length);

        return length;
    }
}

impl<S> IntoIterator for PathSegment<S> {
    type Item = Point<S>;
    type IntoIter = smallvec::IntoIter<[Point<S>; 2]>;
    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

impl<S> FromIterator<Point<S>> for PathSegment<S> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Point<S>>,
    {
        PathSegment::new(iter.into_iter().collect::<Vec<_>>())
    }
}
