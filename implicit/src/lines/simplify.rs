
use super::OPT_EPSILON;
use geometry::{distance_from_line_to_point, Line, Point};

/// Takes a line of points and joins lines that are very
/// similar to being
/// the same line.
pub fn simplify_line(pts: Vec<Point>) -> Vec<Point> {
    if pts.len() <= 2 {
        return pts;
    }
    let mut pts = pts.into_iter();
    let mut out = vec![];

    let mut first = pts.next().unwrap();
    let mut prev = pts.next().unwrap();
    out.push(first);

    while let Some(p) = pts.next() {
        let line = Line(first, p);
        let dist_to_prev = distance_from_line_to_point(line, prev);
        if dist_to_prev < OPT_EPSILON {
            prev = p;
        } else {
            out.push(prev);
            first = prev;
            prev = p;
        }
    }

    out.push(prev);
    out
}
