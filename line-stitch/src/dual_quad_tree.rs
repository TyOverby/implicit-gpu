use ::*;
use euclid;
use fnv::FnvHashMap as HashMap;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct DqtId(u32);
pub struct DualQuadTree<S> {
    id: u32,
    pub id_to_segment: HashMap<DqtId, (PathSegment<S>, ItemId, ItemId)>,
    pub starts: QuadTree<DqtId, S>,
    pub ends: QuadTree<DqtId, S>,
    pub ambiguity_points: QuadTree<Point<S>, S>,
}

impl<S: 'static> DualQuadTree<S> {
    pub fn new(aabb: euclid::TypedRect<f32, S>, size_hint: usize) -> DualQuadTree<S> {
        let hasher: fnv::FnvBuildHasher = Default::default();
        DualQuadTree {
            id: 0,
            id_to_segment: HashMap::with_capacity_and_hasher(size_hint, hasher),
            starts: QuadTree::new(
                aabb,
                true,
                /* min children */ 0,
                /* max_children */ 64,
                /* max-depth */ 16,
                size_hint,
            ),
            ends: QuadTree::new(aabb, true, 0, 64, 16, size_hint),
            ambiguity_points: QuadTree::new(aabb, true, 0, 64, 16, size_hint / 100),
        }
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = (DqtId, &'a PathSegment<S>)> + 'a> {
        let iterator = self.id_to_segment.iter().map(|(&k, &(ref p, _, _))| (k, p));
        Box::new(iterator) as Box<Iterator<Item = (DqtId, &PathSegment<S>)> + 'a>
    }

    pub fn into_iter(self) -> Box<Iterator<Item = PathSegment<S>>> {
        let iterator = self.id_to_segment.into_iter().map(|(_, (p, _, _))| p);
        Box::new(iterator) as Box<Iterator<Item = PathSegment<S>>>
    }

    pub fn insert(&mut self, segment: PathSegment<S>) {
        let id = self.id;
        self.id += 1;
        let id = DqtId(id);

        let start = segment.first();
        let end = segment.last();

        let start_id = self.starts.insert_with_box(id, start.aabb()).unwrap();
        let end_id = self.ends.insert_with_box(id, end.aabb()).unwrap();
        self.id_to_segment.insert(id, (segment, start_id, end_id));
    }

    pub fn pop(&mut self) -> Option<PathSegment<S>> {
        let dqt_id = {
            let first = self.id_to_segment.iter().next();
            if let Some((&dqt_id, _)) = first {
                dqt_id
            } else {
                return None;
            }
        };

        self.remove(dqt_id)
    }

    pub fn len(&self) -> usize { self.id_to_segment.len() }

    pub fn remove(&mut self, dqt_id: DqtId) -> Option<PathSegment<S>> {
        let (segment, start_id, end_id) = self.id_to_segment.remove(&dqt_id).unwrap();
        self.starts.remove(start_id);
        self.ends.remove(end_id);
        return Some(segment);
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool { self.id_to_segment.is_empty() }

    pub fn has_forward_neighbor(&self, id: DqtId, point: Point<S>, epsilon: f32) -> bool {
        let query_aabb = point.aabb().inflate(epsilon * 2.0, epsilon * 2.0);
        self.ends
            .query(query_aabb)
            .into_iter()
            .filter(|&(&qid, _, _)| qid != id)
            .count() != 0
    }

    pub fn has_backward_neighbor(&self, id: DqtId, point: Point<S>, epsilon: f32) -> bool {
        let query_aabb = point.aabb().inflate(epsilon * 2.0, epsilon * 2.0);
        self.starts
            .query(query_aabb)
            .into_iter()
            .filter(|&(&qid, _, _)| qid != id)
            .count() != 0
    }

    pub fn query_forward(&mut self, point: Point<S>, epsilon: f32, only_starts: bool, allow_ambiguous: bool) -> Option<PathSegment<S>> {
        self.query_direction(false, point, epsilon, only_starts, allow_ambiguous)
    }

    pub fn query_backward(&mut self, point: Point<S>, epsilon: f32, only_starts: bool, allow_ambiguous: bool) -> Option<PathSegment<S>> {
        self.query_direction(true, point, epsilon, only_starts, allow_ambiguous)
    }

    fn query_direction(
        &mut self, should_swap: bool, point: Point<S>, epsilon: f32, only_starts: bool, allow_ambiguous: bool
    ) -> Option<PathSegment<S>> {
        let (mut start, mut end) = self.query_impl(point, epsilon, allow_ambiguous);
        if should_swap {
            std::mem::swap(&mut start, &mut end);
        }
        let (start, end) = (start, end);

        if only_starts {
            match (start, end) {
                // A start and an end at this point means that there is likely a better
                // path between those two segments.
                (Ok(Some(_)), Ok(Some(_))) => {
                    self.ambiguity_points.insert(point);
                    None
                }
                // ignore errors here for now
                (Ok(Some(a)), _) => self.remove(a),
                (Ok(None), _) => None,
                (Err(_), _) => {
                    self.ambiguity_points.insert(point);
                    None
                }
            }
        } else {
            match (start, end, allow_ambiguous) {
                (Ok(None), Ok(None), _) => None,
                (Ok(Some(a)), Ok(Some(_)), true) => self.remove(a),
                (Ok(Some(_)), Ok(Some(_)), false) => {
                    self.ambiguity_points.insert(point);
                    None
                }
                (Ok(Some(a)), Ok(None), _) => self.remove(a),
                (Ok(None), Ok(Some(b)), _) => self.remove(b).map(reverse_and_return),
                (Err(_), _, _) | (_, Err(_), _) => {
                    self.ambiguity_points.insert(point);
                    None
                }
            }
        }
    }

    fn query_impl(&mut self, point: Point<S>, epsilon: f32, allow_ambiguous: bool) -> (Result<Option<DqtId>, ()>, Result<Option<DqtId>, ()>) {
        let query_aabb = point.aabb().inflate(epsilon * 2.0, epsilon * 2.0);
        if self.ambiguity_points.query(query_aabb).len() > 0 {
            return (Ok(None), Ok(None));
        }

        let query_starts = || {
            let mut out = None;
            let query = self.starts
                .query(query_aabb)
                .into_iter()
                .map(|(&id, _, _)| (id, self.id_to_segment.get(&id).unwrap().0.first()));
            let trimmed = take_nearest(point, query);
            for id in trimmed {
                if allow_ambiguous {
                    return Ok(Some(id));
                } else {
                    if out.is_some() {
                        return Err(());
                    }
                    out = Some(id)
                }
            }
            return Ok(out);
        };

        let query_ends = || {
            let mut out = None;
            let query = self.ends
                .query(query_aabb)
                .into_iter()
                .map(|(&id, _, _)| (id, self.id_to_segment.get(&id).unwrap().0.last()));
            let trimmed = take_nearest(point, query);
            for id in trimmed {
                if allow_ambiguous {
                    return Ok(Some(id));
                } else {
                    if out.is_some() {
                        return Err(());
                    }
                    out = Some(id)
                }
            }
            return Ok(out);
        };

        (query_starts(), query_ends())
    }
}

fn take_nearest<S, I>(point: Point<S>, points: I) -> Vec<DqtId>
where I: Iterator<Item = (DqtId, Point<S>)> {
    use std::cmp::Ordering;
    let points = points.collect::<Vec<_>>();
    if points.is_empty() {
        return vec![];
    }

    let mut with_distance = points
        .into_iter()
        .map(|(id, pt)| {
            let dist = (point - pt).square_length();
            (id, pt, dist)
        })
        .collect::<Vec<_>>();

    with_distance.sort_by(|&(_, _, dist1), &(_, _, dist2)| {
        return dist1.partial_cmp(&dist2).unwrap_or(Ordering::Equal);
    });

    let (_, _, dsmall) = with_distance[0];

    return with_distance
        .into_iter()
        .take_while(|&(_, _, d)| d == dsmall)
        .map(|(id, _, _)| id)
        .collect();
}

fn reverse_and_return<S>(mut v: PathSegment<S>) -> PathSegment<S> {
    v.path.reverse();
    v
}
