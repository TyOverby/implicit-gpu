use ::marching::util::geom;
use ::marching::util::quadtree::QuadTree;
//use self::{fuse_ends, dash, join, simplify, connect};

mod fuse_ends;
pub mod dash;
mod join;
mod simplify;
mod connect;

const EPSILON: f32 = 0.001;
const OPT_EPSILON: f32 = 0.05;

pub struct DashSegment(pub Vec<geom::Point>);

pub enum LineType {
    Joined(Vec<geom::Point>),
    Unjoined(Vec<geom::Point>)
}

type Point = (f32, f32);
type Line = (Point, Point);

pub fn connect_lines<I: IntoIterator<Item=Line>>(lines: I, simplify: bool) -> (Vec<Vec<Point>>, QuadTree<geom::Line>) {
    let (mut joined, qt) =
        join::join_lines(
            lines.into_iter().map(|((x1, y1), (x2, y2))| geom::Line(geom::Point{x: x1, y: y1}, geom::Point{x: x2, y: y2})));

    loop {
        let mut any_progress = false;
        let (joined_t, p) = fuse_ends::fuse_ends(joined);
        joined = joined_t;
        any_progress |= p;

        let (connected_t, p) = connect::connect_linetypes(joined);
        joined = connected_t;
        any_progress |= p;

        if !any_progress {
            break;
        }
    }

    let joined =
        // Simplification
        joined.into_iter().map(|lt| match lt {
            LineType::Joined(r) =>
                LineType::Joined(if simplify { simplify::simplify_line(r) } else { r }),
            LineType::Unjoined(r) =>
                LineType::Unjoined(if simplify { simplify::simplify_line(r) } else { r })
        // Remove lines that are too short
        }).filter(|lt| match lt {
            &LineType::Joined(ref r) | &LineType::Unjoined(ref r) => r.len() > 1
        // Take vectors out of LineType
        }).map(|lt| match lt {
            LineType::Joined(r) | LineType::Unjoined(r) => r
        // Convert geom::Point back to (f32, f32)
        }).map(|v|
            v.into_iter().map(|geom::Point{x, y}| (x, y)).collect()
        );

    (joined.collect(), qt)
}
