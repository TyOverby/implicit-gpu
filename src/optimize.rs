use super::*;

/// todo: doc
pub fn optimize<P, I>(
    segments: I,
    epsilon: f32,
    only_starts: bool,
    allow_ambiguous: bool,
) -> Vec<PathSegment>
where
    I: IntoIterator<Item = P>,
    P: Into<smallvec::SmallVec<[Point; 1]>>,
{
    let mut all_segments = vec![];
    let mut scene_aabb = geom::Rect::null();

    for segment in segments.into_iter().map(Into::into).filter(|a| a.len() > 1) {
        let segment = PathSegment(segment);

        let first = segment.first();
        let last = segment.first();

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

    let mut dual_qt = DualQuadTree::new(scene_aabb);
    for segment in all_segments {
        dual_qt.insert(segment);
    }

    let mut out = vec![];
    while !dual_qt.is_empty() {
        // ok to unwrap because dual_qt is not empty.
        let head = dual_qt.pop().unwrap();
        if let Some(seg_chain) =
            chain_single(head, &mut dual_qt, epsilon, only_starts, allow_ambiguous)
        {
            out.push(seg_chain);
        }
    }

    out.into_iter()
        .map(|chained_segments| {
            let mut segment =
                SmallVec::with_capacity(chained_segments.iter().map(|p| p.0.len()).sum());
            segment.extend_from_slice(&chained_segments[0].0);
            for other_segment in &chained_segments[1..] {
                segment.extend_from_slice(&other_segment.0[1..]);
            }
            PathSegment(segment)
        })
        .collect()
}

fn chain_single(
    start: PathSegment,
    dual_qt: &mut DualQuadTree,
    epsilon: f32,
    only_starts: bool,
    allow_ambiguous: bool,
) -> Option<Vec<PathSegment>> {
    let mut last_going_forward = start.last();
    let mut first_going_backwards = start.first();
    let mut combined: Vec<_> = vec![start];

    loop {
        let next = dual_qt.query_forward(last_going_forward, epsilon, only_starts, allow_ambiguous);
        if let Some(next) = next {
            last_going_forward = next.last();
            combined.push(next);
        } else {
            break;
        }
    }

    loop {
        println!("{:?}", first_going_backwards);
        let next = dual_qt.query_backward(first_going_backwards, epsilon, only_starts, allow_ambiguous);
        if let Some(next) = next {
            first_going_backwards = next.first();
            combined.insert(0, next);
        } else {
            break;
        }
    }


    let total_count: usize = combined.iter().map(|a| a.0.len()).sum();

    if total_count > 1 {
        Some(combined)
    } else {
        None
    }
}
