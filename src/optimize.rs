use super::*;
use std::cell::RefCell;
use itertools::{repeat_call, Itertools};

/// todo: doc
pub fn optimize<P, I>(
    segments: I,
    epsilon: f32,
    only_starts: bool,
    allow_ambiguous: bool,
) -> Vec<PathSegment>
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
    let dual_qt = RefCell::new(dual_qt);

    return repeat_call(|| dual_qt.borrow_mut().pop())
        .while_some()
        .filter_map(|head| {
            let mut borrowed = dual_qt.borrow_mut();
            chain_single(head, &mut *borrowed, epsilon, only_starts, allow_ambiguous)
        })
        .map(|a| recombine_segments(a, epsilon))
        .collect();


    fn recombine_segments(segments: Vec<PathSegment>, epsilon: f32) -> PathSegment {
        let mut segment = SmallVec::with_capacity(segments.iter().map(|p| p.path.len()).sum());
        segment.extend_from_slice(&segments[0].path);
        for other_segment in &segments[1..] {
            segment.extend_from_slice(&other_segment.path[1..]);
        }
        PathSegment::new(segment, epsilon)
    }
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
        let next =
            dual_qt.query_backward(first_going_backwards, epsilon, only_starts, allow_ambiguous);
        if let Some(next) = next {
            first_going_backwards = next.first();
            combined.insert(0, next);
        } else {
            break;
        }
    }


    let total_count: usize = combined.iter().map(|a| a.path.len()).sum();

    if total_count > 1 {
        Some(combined)
    } else {
        None
    }
}
