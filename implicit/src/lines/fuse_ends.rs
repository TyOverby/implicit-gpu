use super::{LineType, EPSILON};
use super::geom::Point;

// Takes linetypes and attempts to fuse their ends together.
pub fn fuse_ends(lines: Vec<LineType>) -> (Vec<LineType>, bool) {
    fn remove_dup(points: &mut Vec<Point>) {
        let first = points.first().cloned();
        let last = points.last().cloned();
        if let (Some(first), Some(last)) = (first, last) {
            if first.distance_2(&last) < EPSILON {
                points.pop();
            }
        }
    }

    let mut out = vec![];
    let mut made_progress = false;
    for line in lines {
        match line {
            LineType::Joined(mut points) => {
                let prev_len = points.len();
                remove_dup(&mut points);
                let post_len = points.len();
                if post_len < prev_len {
                    made_progress = true;
                }
                if post_len != 0 {
                    out.push(LineType::Joined(points));
                }
            },
            LineType::Unjoined(mut points) => {
                let prev_len = points.len();
                remove_dup(&mut points);
                let post_len = points.len();
                if post_len != 0 {
                    if post_len < prev_len {
                        made_progress = true;
                        out.push(LineType::Joined(points));
                    } else {
                        out.push(LineType::Unjoined(points));
                    }
                }
            }
        }
    }
    (out, made_progress)
}
