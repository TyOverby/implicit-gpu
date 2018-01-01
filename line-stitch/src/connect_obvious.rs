use ::*;
use util::*;
use std::cell::RefCell;
use itertools::{repeat_call, Itertools};

/// todo: doc
pub fn connect_obvious<P, I, S: 'static>(
    segments: I,
    epsilon: f32,
    only_starts: bool,
    allow_ambiguous: bool,
) -> Vec<PathSegment<S>>
where
    I: IntoIterator<Item = P>,
    P: Into<smallvec::SmallVec<[Point<S>; 2]>>,
{
    let dual_qt = RefCell::new(populate(segments, epsilon));

    return repeat_call(|| dual_qt.borrow_mut().pop())
        .while_some()
        .filter_map(|head| {
            let mut borrowed = dual_qt.borrow_mut();
            chain_single(head, &mut *borrowed, epsilon, only_starts, allow_ambiguous)
        })
        .map(|a| recombine_segments(a, epsilon))
        .collect();


    fn recombine_segments<S>(segments: Vec<PathSegment<S>>, epsilon: f32) -> PathSegment<S> {
        let mut segment = SmallVec::with_capacity(segments.iter().map(|p| p.path.len()).sum());
        segment.extend_from_slice(&segments[0].path);

        for other_segment in &segments[1..] {
            segment.extend_from_slice(&other_segment.path[1..]);
        }

        PathSegment::new(segment, epsilon)
    }
}

fn chain_single<S: 'static>(
    start: PathSegment<S>,
    dual_qt: &mut DualQuadTree<S>,
    epsilon: f32,
    only_starts: bool,
    allow_ambiguous: bool,
) -> Option<Vec<PathSegment<S>>> {
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
