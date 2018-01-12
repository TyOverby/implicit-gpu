use ::*;
use rayon::prelude::*;

/// Remoes all line segments that can't possibly be part of
/// a cycle.
pub fn prune<P, I, S: Send + Sync + 'static>(segments: I, epsilon: f32, only_starts: bool) -> Vec<PathSegment<S>>
where
    I: IntoIterator<Item = P>,
    P: Into<smallvec::SmallVec<[Point<S>; 2]>>,
{
    let mut dual_qt = util::populate(segments, epsilon);

    loop {
        let made_progress = prune_one_iter(&mut dual_qt, epsilon, only_starts);
        if !made_progress {
            break;
        }
    }

    dual_qt.into_iter().collect()
}

fn prune_one_iter<S: Send + Sync + 'static>(dual_qt: &mut DualQuadTree<S>, epsilon: f32, only_starts: bool) -> bool {
    let mut made_progress = false;
    let to_remove: Vec<_> = dual_qt
        .id_to_segment
        .par_iter()
        .filter_map(|(&id, &(ref path, _, _))| {
            let (start, end) = (path.first(), path.last());

            let a = dual_qt.has_forward_neighbor(id, start, epsilon);
            let b = || dual_qt.has_backward_neighbor(id, start, epsilon);

            let c = dual_qt.has_backward_neighbor(id, end, epsilon);
            let d = || dual_qt.has_forward_neighbor(id, end, epsilon);

            let should_be_kept = (a || (!only_starts && b())) && (c || (!only_starts && d()));
            if !(should_be_kept) {
                Some(id)
            } else {
                None
            }
        })
        .collect();

    for id in to_remove {
        dual_qt.remove(id);
        made_progress = true;
    }

    made_progress
}
