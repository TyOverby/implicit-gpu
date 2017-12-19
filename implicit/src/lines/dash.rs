
use super::*;
use geometry::Point;
use itertools::put_back;

fn extend(start: Point, end: Point, dist: f32) -> Point {
    let mut dx = end.x - start.x;
    let mut dy = end.y - start.y;
    let magnitude = (end - start).length();
    dx /= magnitude;
    dy /= magnitude;
    dx *= dist;
    dy *= dist;
    Point::new(start.x + dx, start.y + dy)
}

pub fn dashify<P, D>(points: P, dashes: D) -> Vec<DashSegment>
where
    P: Iterator<Item = (f32, f32)>,
    D: Iterator<Item = f32> + Clone,
{
    let mut dashes = dashes.cycle();
    let mut points = put_back(points.map(|p| Point::new(p.0, p.1)));
    let mut out = vec![];

    let mut on = true;
    let mut previous = points.next();
    let mut dst = dashes.next().expect("dashes is empty");

    let mut seg = vec![];
    if let Some(p) = previous {
        seg.push(p);
    }

    while let (Some(prev), Some(next)) = (previous, points.next()) {
        let mag = (next - prev).length();
        if mag > dst {
            let next_break = extend(prev, next, dst);

            if on {
                seg.push(next_break);
                out.push(DashSegment(seg));
                seg = vec![];
                on = false;
            } else {
                on = true;
                seg.push(next_break);
            }

            previous = Some(next_break);
            dst = dashes.next().unwrap();
            points.put_back(next);
        } else {
            if on {
                seg.push(next);
            }
            dst = dst - mag;
            previous = Some(next);
        }
    }

    if !seg.is_empty() {
        out.push(DashSegment(seg));
    }

    out
}
