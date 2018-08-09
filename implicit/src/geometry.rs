use aabb_quadtree;
use euclid::{self, point2, UnknownUnit};
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

pub fn point_in_poly(polygon: &[Point], p: Point) -> bool {
    let mut i = 0;
    let mut j = polygon.len() - 1;
    let mut c = false;

    while i < polygon.len() {
        if ((polygon[i].y > p.y) != (polygon[j].y > p.y))
            && (p.x < (polygon[j].x - polygon[i].x) * (p.y - polygon[i].y)
                / (polygon[j].y - polygon[i].x + polygon[i].x))
        {
            c = !c;
        }

        j = i;
        i += 1;
    }

    return c;
}

// TODO: this is *way* too expensive
pub fn bb_for_line(l: Line, margin: f32) -> Rect {
    compute_bounding_box(vec![l.0, l.1]).inflate(margin, margin)
}

// TODO: rename
pub fn compute_bounding_box<I: IntoIterator<Item = Point>>(i: I) -> Rect {
    use euclid::{point2, vec2};
    use std::f32;

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for pt in i {
        min_x = min_x.min(pt.x);
        min_y = min_y.min(pt.y);

        max_x = max_x.max(pt.x);
        max_y = max_y.max(pt.y);
    }

    Rect::new(
        point2(min_x, min_y),
        vec2(max_x - min_x, max_y - min_y).to_size(),
    )
}

pub(crate) fn centered_with_radius(pt: Point, radius: f32) -> Rect {
    let half = euclid::vec2(radius, radius);
    euclid::TypedRect::new(pt - half, (half * 2.0).to_size())
}

pub fn distance_from_line_to_point(line: Line, point: Point) -> f32 {
    #[inline(always)]
    fn sqr(x: f32) -> f32 {
        x * x
    }
    #[inline(always)]
    fn dist2(v: Point, w: Point) -> f32 {
        sqr(v.x - w.x) + sqr(v.y - w.y)
    }
    #[inline(always)]
    fn dist_to_segment_squared(p: Point, v: Point, w: Point) -> f32 {
        let l2 = dist2(v, w);
        //  TODO: epsilon
        if l2 == 0.0 {
            return dist2(p, v);
        }
        let t = ((p.x - v.x) * (w.x - v.x) + (p.y - v.y) * (w.y - v.y)) / l2;
        if t < 0.0 {
            dist2(p, v)
        } else if t > 1.0 {
            dist2(p, w)
        } else {
            dist2(p, point2(v.x + t * (w.x - v.x), v.y + t * (w.y - v.y)))
        }
    }

    dist_to_segment_squared(point, line.0, line.1).sqrt()
}
