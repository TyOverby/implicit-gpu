use ::*;

pub fn populate<I, P>(segments: I, epsilon: f32) -> DualQuadTree
where
    I: IntoIterator<Item = P>,
    P: Into<smallvec::SmallVec<[Point; 2]>>,
{
    let mut all_segments = vec![];
    let mut scene_aabb = geom::Rect::null();

    for segment in segments.into_iter().map(Into::into).filter(|a| a.len() > 1) {
        let segment = PathSegment::new(segment, epsilon);
        if segment.length_2() < epsilon {
            continue;
        }

        let first = segment.first();
        let last = segment.last();

        scene_aabb.expand_to_include(&geom::Point {
            x: first.x,
            y: first.y,
        });
        scene_aabb.expand_to_include(&geom::Point {
            x: last.x,
            y: last.y,
        });
        all_segments.push(segment);
    }
    let w = scene_aabb.width() / 20.0;
    let h = scene_aabb.height() / 20.0;
    scene_aabb = scene_aabb.expand(w, h, w, h);

    let mut dual_qt = DualQuadTree::new(scene_aabb);
    for segment in all_segments {
        dual_qt.insert(segment);
    }

    dual_qt
}
