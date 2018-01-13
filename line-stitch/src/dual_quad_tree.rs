use ::*;
use euclid;
use fnv::FnvHashMap as HashMap;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct DqtId(u32);
pub struct DualQuadTree<S> {
    id: u32,
    pub id_to_segment: HashMap<DqtId, (PathSegment<S>, ItemId, ItemId)>,
    pub starts: QuadTree<DqtId, S, [(ItemId, euclid::TypedRect<f32, S>); 32]>,
    pub ends: QuadTree<DqtId, S, [(ItemId, euclid::TypedRect<f32, S>); 32]>,
    pub ambiguity_points: QuadTree<Point<S>, S, [(ItemId, euclid::TypedRect<f32, S>); 32]>,
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
                /* max-depth */ 64,
                size_hint,
            ),
            ends: QuadTree::new(aabb, true, 0, 64, 64, size_hint),
            ambiguity_points: QuadTree::new(aabb, true, 0, 64, 64, size_hint / 100),
        }
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
            let mut query = self.starts.query(query_aabb);
            let amnt = self.take_nearest(point, true, &mut query);
            for (&id, _, _) in query.into_iter().take(amnt) {
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
            let mut query = self.ends.query(query_aabb);
            let amnt = self.take_nearest(point, false, &mut query);
            for (&id, _, _) in query.into_iter().take(amnt) {
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

    fn take_nearest<'a, 'o>(
        &self, point: Point<S>, is_start: bool, points: &mut [(&dual_quad_tree::DqtId, euclid::TypedRect<f32, S>, aabb_quadtree::ItemId)]
    ) -> usize {
        use std::cmp::Ordering;
        if points.len() == 0 {
            return 0;
        }
        let dist_for_id = |id| {
            let elem = &self.id_to_segment.get(id).unwrap().0;
            let pa = if is_start { elem.first() } else { elem.last() };

            let dist = (point - pa).square_length();
            dist
        };

        points.sort_by(|&(ida, _, _), &(idb, _, _)| {
            let dist_a = dist_for_id(ida);
            let dist_b = dist_for_id(idb);

            return dist_a.partial_cmp(&dist_b).unwrap_or(Ordering::Equal);
        });

        let dsmall = dist_for_id(points[0].0);

        let mut number_with_dsmall = 0;
        for &(id, _, _) in points.iter() {
            if dist_for_id(id) == dsmall {
                number_with_dsmall += 1;
            } else {
                break;
            }
        }
        return number_with_dsmall;
    }
}


fn reverse_and_return<S>(mut v: PathSegment<S>) -> PathSegment<S> {
    v.path.reverse();
    v
}
