use ::*;
use euclid::*;

pub fn populate<I, P, S: 'static>(segments: I, epsilon: f32) -> DualQuadTree<S>
where
    I: IntoIterator<Item = P>,
    P: Into<smallvec::SmallVec<[Point<S>; 2]>>,
{
    let mut all_segments = vec![];
    let mut scene_aabb: TypedRect<f32, S> =
        TypedRect::new(point2(0.0, 0.0), vec2(0.0, 0.0).to_size());

    for segment in segments.into_iter().map(Into::into).filter(|a| a.len() > 1) {
        let segment = PathSegment::new(segment, epsilon);
        if segment.length_2() < epsilon {
            continue;
        }

        let first = segment.first();
        let last = segment.last();

        scene_aabb = scene_aabb.union(&centered_with_radius(
            point2(first.x, first.y),
            epsilon * 10.0,
        ));
        scene_aabb = scene_aabb.union(&centered_with_radius(
            point2(last.x, last.y),
            epsilon * 10.0,
        ));
        all_segments.push(segment);
    }

    let mut dual_qt = DualQuadTree::new(scene_aabb);
    for segment in all_segments {
        dual_qt.insert(segment);
    }

    dual_qt
}

pub(crate) fn centered_with_radius<S>(pt: Point<S>, radius: f32) -> euclid::TypedRect<f32, S> {
    let half = euclid::vec2(radius, radius);
    euclid::TypedRect::new(pt - half, (half * 2.0).to_size())
}

// TODO: rename
pub fn compute_bounding_box<S, I: IntoIterator<Item = Point<S>>>(i: I) -> TypedRect<f32, S> {
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

    TypedRect::new(
        point2(min_x, min_y),
        vec2(max_x - min_x, max_y - min_y).to_size(),
    )
}
