use aabb_quadtree;
use euclid::{self, UnknownUnit};
use line_stitch;

pub type Point = euclid::Point2D<f32>;
pub type Rect = euclid::Rect<f32>;
pub type PathSegment = line_stitch::PathSegment<euclid::UnknownUnit>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line(pub Point, pub Point);

pub fn compare_points(
    Point { x: x1, y: y1, .. }: Point,
    Point { x: x2, y: y2, .. }: Point,
) -> Option<::std::cmp::Ordering> {
    let xc = x1.partial_cmp(&x2);
    let yc = y1.partial_cmp(&y2);
    match (xc, yc) {
        (None, _) | (_, None) => None,
        (Some(xc), Some(yc)) => Some(xc.then(yc)),
    }
}

impl ::std::cmp::PartialOrd for Line {
    fn partial_cmp(&self, other: &Line) -> Option<::std::cmp::Ordering> {
        let p1c = compare_points(self.0, other.0);
        let p2c = compare_points(self.1, other.1);

        match (p1c, p2c) {
            (None, _) | (_, None) => None,
            (Some(xc), Some(yc)) => Some(xc.then(yc)),
        }
    }
}

impl aabb_quadtree::Spatial<UnknownUnit> for Line {
    fn aabb(&self) -> Rect {
        bb_for_line(*self, 0.001)
    }
}

pub fn bb_for_line(l: Line, margin: f32) -> Rect {
    use euclid::{point2, vec2};
    let Line(Point { x: x1, y: y1, .. }, Point { x: x2, y: y2, .. }) = l;
    let min_x = x1.min(x2);
    let min_y = y1.min(y2);

    let max_x = x1.max(x2);
    let max_y = y1.max(y2);

    Rect::new(
        point2(min_x, min_y),
        vec2(max_x - min_x, max_y - min_y).to_size(),
    )
    .inflate(margin, margin)
}
