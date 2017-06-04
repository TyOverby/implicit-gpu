use itertools::put_back;
use ::marching::util::geom;
use super::*;

fn extend(start: geom::Point, end: geom::Point, dist: f32) -> geom::Point {
    let mut dx = end.x - start.x;
    let mut dy = end.y - start.y;
    let magnitude = (end - start).magnitude();
    dx /= magnitude;
    dy /= magnitude;
    dx *= dist;
    dy *= dist;
    geom::Point {
        x: start.x + dx,
        y: start.y + dy,
    }
}

pub fn dashify<P, D>(points: P, dashes: D) -> Vec<DashSegment>
where P: Iterator<Item=(f32, f32)>, D: Iterator<Item=f32> + Clone {
    let mut dashes = dashes.cycle();
    let mut points = put_back(points.map(|p| geom::Point{x:p.0, y:p.1}));
    let mut out = vec![];

    let mut on = true;
    let mut previous = points.next();
    let mut dst = dashes.next().expect("dashes is empty");

    let mut seg = vec![];
    if let Some(p) = previous {
        seg.push(p);
    }

    while let (Some(prev), Some(next)) = (previous, points.next()) {
        let mag = (next - prev).magnitude();
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
