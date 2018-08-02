use rayon::prelude::*;
use *;

/// Remoes all line segments that can't possibly be part of
/// a cycle.
pub fn prune<P, I, S: Send + Sync + 'static>(segments: I, epsilon: f32, only_starts: bool) -> DualQuadTree<S>
where
    I: IntoIterator<Item = P>,
    P: Into<smallvec::SmallVec<[Point<S>; 2]>>,
{
    let mut dual_qt = util::populate(segments, epsilon);

    while prune_one_iter(&mut dual_qt, epsilon, only_starts) {}
    dual_qt
}

fn prune_one_iter<S: Send + Sync + 'static>(dual_qt: &mut DualQuadTree<S>, epsilon: f32, only_starts: bool) -> bool {
    let _guard = ::flame::start_guard("prune_one_iter");
    let mut made_progress = false;

    ::flame::start("finding items to remove");
    let to_remove: Vec<_> = dual_qt
        .id_to_segment
        //.par_iter()
        .iter()
        .filter_map(|(&id, &(ref path, _, _))| {
            let (start, end) = (path.first(), path.last());

            let a = dual_qt.has_forward_neighbor(id, start, epsilon);
            let b = || dual_qt.has_backward_neighbor(id, start, epsilon);

            let c = dual_qt.has_backward_neighbor(id, end, epsilon);
            let d = || dual_qt.has_forward_neighbor(id, end, epsilon);

            let should_be_removed = (a || (!only_starts && b())) && (c || (!only_starts && d()));
            if should_be_removed{
                None
            } else {
                Some(id)
            }
        })
        .collect();
    ::flame::end("finding items to remove");

    ::flame::start("removing");
    for id in to_remove {
        dual_qt.remove(id);
        made_progress = true;
    }
    ::flame::end("removing");

    made_progress
}
