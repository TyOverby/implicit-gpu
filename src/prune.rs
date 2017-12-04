use ::*;

/// Remoes all line segments that can't possibly be part of a cycle.
pub fn prune<P, I>(
    segments: I,
    epsilon: f32,
    only_starts: bool,
) -> Vec<PathSegment>
where
    I: IntoIterator<Item = P>,
    P: Into<smallvec::SmallVec<[Point; 2]>>,
{

}
