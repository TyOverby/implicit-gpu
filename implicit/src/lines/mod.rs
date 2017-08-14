use self::util::geom;
use self::util::quadtree::{QuadTree, ItemId as QuadId};
use ::telemetry::{Telemetry, TelemetryLocation};

mod fuse_ends;
pub mod util;
pub mod dash;
mod join;
mod simplify;
mod connect;

const EPSILON: f32 = 0.001;
const OPT_EPSILON: f32 = 0.05;

pub struct DashSegment(pub Vec<geom::Point>);

#[derive(PartialOrd, PartialEq)]
pub enum LineType {
    Joined(Vec<geom::Point>),
    Unjoined(Vec<geom::Point>),
}

type Point = (f32, f32);
type Line = (Point, Point);

pub fn separate_polygons(bag: Vec<Vec<Point>>) -> (Vec<Vec<Point>>, Vec<Vec<Point>>) {
    fn _compute_aabb(points: &[Point]) -> geom::Rect {
        let mut start = geom::Rect::null();
        for &(x, y) in points {
            start.expand_to_include(&geom::Point { x, y });
        }
        start
    }

    fn contains(a: &[Point], b: &[Point]) -> bool { geom::point_in_poly(a, b[0]) }

    // let bag_with_aabb: Vec<_> = bag.into_iter().map(|shape|
    // (compute_aabb(&shape), shape)).collect();

    let mut additive_or_subtractive = vec![];
    for (i, a) in bag.iter().enumerate() {
        let mut inside_count = 0;
        for (j, b) in bag.iter().enumerate() {
            if i == j {
                continue;
            }
            if contains(b, a) {
                inside_count += 1;
            }
        }

        additive_or_subtractive.push(inside_count % 2 == 0);
    }

    let (additive, subtractive): (Vec<_>, Vec<_>) = bag.into_iter().zip(additive_or_subtractive.into_iter()).partition(
        |&(_, i)| i,
    );
    let additive = additive.into_iter().map(|(b, _)| b).collect();
    let subtractive = subtractive.into_iter().map(|(b, _)| b).collect();

    (additive, subtractive)
}

pub fn connect_lines(mut lines: Vec<Line>, simplify: bool, telemetry: &mut Telemetry, tloc: TelemetryLocation)
-> (Vec<Vec<Point>>, QuadTree<geom::Line>) {
    use std::cmp::{PartialOrd, Ordering};

    fn rotate<T>(slice: &mut [T], at: usize) {
        {
            let (a, b) = slice.split_at_mut(at);
            a.reverse();
            b.reverse();
        }

        slice.reverse();
    }

    lines.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let (mut joined, qt) = join::join_lines(lines.into_iter().map(|((x1, y1), (x2, y2))| {
        geom::Line(geom::Point { x: x1, y: y1 }, geom::Point { x: x2, y: y2 })
    }));

    joined.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

/*
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
    */

    telemetry.shape_line_joined(tloc, &joined);
    for line in &joined {
        if let &LineType::Unjoined(ref _pts) = line {
            //println!("{:?} .. {} .. {:?}\n", pts.first().unwrap(), pts.len(), pts.last().unwrap());
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
            v.into_iter().map(|geom::Point{x, y}| (x, y)).collect::<Vec<_>>()
        // Rotate the vectors to start at the smallest point.
        // This can be removed later if it's a perf issue
        ).map(|mut v| {
            let (smallest_idx, _) = v.iter().enumerate().min_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap_or(Ordering::Equal)).unwrap();
            rotate(&mut v, smallest_idx);
            v
        });

    (joined.collect(), qt)
}
