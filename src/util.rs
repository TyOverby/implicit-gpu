use ::*;
use euclid::*;

pub fn populate<I, P, S: 'static>(segments: I, epsilon: f32) -> DualQuadTree<S>
where
    I: IntoIterator<Item = P>,
    P: Into<smallvec::SmallVec<[Point<S>; 2]>>,
{
    let mut all_segments = vec![];
    let mut scene_aabb: TypedRect<f32, S> = TypedRect::new(point2(0.0, 0.0), vec2(0.0, 0.0).to_size());

    for segment in segments.into_iter().map(Into::into).filter(|a| a.len() > 1) {
        let segment = PathSegment::new(segment, epsilon);
        if segment.length_2() < epsilon {
            continue;
        }

        let first = segment.first();
        let last = segment.last();

        scene_aabb = scene_aabb.union(&centered_with_radius(point2(first.x, first.y), epsilon * 10.0));
        scene_aabb = scene_aabb.union(&centered_with_radius(point2(last.x, last.y), epsilon * 10.0));
        all_segments.push(segment);
    }

    let mut dual_qt = DualQuadTree::new(scene_aabb);
    for segment in all_segments {
        dual_qt.insert(segment);
    }

    dual_qt
}
