use ::*;

/// Remoes all line segments that can't possibly be part of a cycle.
pub fn prune<P, I>(segments: I, epsilon: f32, only_starts: bool) -> Vec<PathSegment>
where
    I: IntoIterator<Item = P>,
    P: Into<smallvec::SmallVec<[Point; 2]>>,
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

fn prune_one_iter(dual_qt: &mut DualQuadTree, epsilon: f32, only_starts: bool) -> bool {
    let mut made_progress = false;
    let mut to_remove = vec![];

    for (id, path) in dual_qt.iter() {
        let (start, end) = (path.first(), path.last());

        println!("{:?}", dual_qt.starts.iter());

        let a = dual_qt.has_forward_neighbor(id, start, epsilon);
        let b = || dual_qt.has_backward_neighbor(id, start, epsilon);

        let c = dual_qt.has_backward_neighbor(id, end, epsilon);
        let d = || dual_qt.has_forward_neighbor(id, end, epsilon);

        let should_be_kept = (a || (!only_starts && b())) && (c || (!only_starts && d()));
        if !should_be_kept {
            to_remove.push(id);
        }
    }

    for id in to_remove {
        dual_qt.remove(id);
        made_progress = true;
    }

    made_progress
}
