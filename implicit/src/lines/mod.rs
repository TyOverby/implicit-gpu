use aabb_quadtree::{ItemId as QuadId, QuadTree};
use euclid::{self, UnknownUnit};
use geometry::{compare_points, point_in_poly, Line, Point, Rect};
use telemetry::{Telemetry, TelemetryLocation};

pub mod dash;
mod join;
mod simplify;
mod graph_stitch;

// const EPSILON: f32 = 0.001;
const OPT_EPSILON: f32 = 0.05;

pub struct DashSegment(pub Vec<Point>);

#[derive(PartialEq)]
pub enum LineType {
    Joined(Vec<Point>),
    Unjoined(Vec<Point>),
}

pub(crate) fn centered_with_radius(pt: Point, radius: f32) -> Rect {
    let half = euclid::vec2(radius, radius);
    euclid::TypedRect::new(pt - half, (half * 2.0).to_size())
}

pub fn separate_polygons(bag: Vec<Vec<Point>>) -> (Vec<Vec<Point>>, Vec<Vec<Point>>) {
    let _guard = ::flame::start_guard("separate_polygons");

    fn contains(a: &[Point], b: &[Point]) -> bool { point_in_poly(a, b[0]) }

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

    let (additive, subtractive): (Vec<_>, Vec<_>) = bag.into_iter()
        .zip(additive_or_subtractive.into_iter())
        .partition(|&(_, i)| i);
    let additive = additive.into_iter().map(|(b, _)| b).collect();
    let subtractive = subtractive.into_iter().map(|(b, _)| b).collect();

    (additive, subtractive)
}

pub fn connect_lines(
    mut lines: Vec<Line>, simplify: bool, telemetry: &mut Telemetry, tloc: TelemetryLocation
) -> (Vec<Vec<Point>>, QuadTree<Line, UnknownUnit>) {
    let _guard = ::flame::start_guard("connect_lines");
    use std::cmp::{Ordering, PartialOrd};

    fn rotate<T>(slice: &mut [T], at: usize) {
        {
            let (a, b) = slice.split_at_mut(at);
            a.reverse();
            b.reverse();
        }

        slice.reverse();
    }

    lines.sort_by(|l1, l2| l1.partial_cmp(&l2).unwrap_or(Ordering::Equal));

    let (mut joined, qt) = join::join_lines(lines, telemetry, tloc);

    // TODO: is this necessary?
    // joined.sort_by(|a, b|
    // a.partial_cmp(b).unwrap_or(Ordering::Equal));

    for line in &joined {
        if let &LineType::Unjoined(ref _pts) = line {
            // println!("{:?} .. {} .. {:?}\n", pts.first().unwrap(),
            // pts.len(), pts.last().unwrap());
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
        }).map(|mut v| {
            let (smallest_idx, _) = v.iter().enumerate().min_by(|&(_, a), &(_, b)|
                compare_points(*a, *b).unwrap_or(Ordering::Equal)).unwrap();
            rotate(&mut v, smallest_idx);
            v
        });

    (joined.collect(), qt)
}
