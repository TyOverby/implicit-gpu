use fnv::FnvHashMap as HashMap;
use ::*;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct DqtId(u32);
pub struct DualQuadTree {
    id: u32,
    id_to_segment: HashMap<DqtId, (PathSegment, ItemId, ItemId)>,
    starts: QuadTree<DqtId>,
    ends: QuadTree<DqtId>,
}

impl DualQuadTree {
    pub fn new(aabb: geom::Rect) -> DualQuadTree {
        DualQuadTree {
            id: 0,
            id_to_segment: HashMap::default(),
            starts: QuadTree::default(aabb),
            ends: QuadTree::default(aabb),
        }
    }

    pub fn insert(&mut self, segment: PathSegment) {
        let id = self.id;
        self.id += 1;
        let id = DqtId(id);

        let start = segment.first();
        let end = segment.last();

        let start_id = self.starts.insert_with_box(id, start.aabb());
        let end_id = self.ends.insert_with_box(id, end.aabb());
        self.id_to_segment.insert(id, (segment, start_id, end_id));
    }

    pub fn pop(&mut self) -> Option<PathSegment> {
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

    pub fn remove(&mut self, dqt_id: DqtId) -> Option<PathSegment> {
        let (segment, start_id, end_id) = self.id_to_segment.remove(&dqt_id).unwrap();
        self.starts.remove(start_id);
        self.ends.remove(end_id);
        return Some(segment);
    }

    pub fn is_empty(&self) -> bool {
        self.id_to_segment.is_empty()
    }

    pub fn query_forward(
        &mut self,
        point: Point,
        epsilon: f32,
        only_starts: bool,
        allow_ambiguous: bool,
    ) -> Option<PathSegment> {
        let (start, end) = self.query_impl(point, epsilon, allow_ambiguous);
        match (start, end, allow_ambiguous, only_starts) {
            (Some(a), None, _, _) => self.remove(a),
            (None, Some(_), _, true) => None,
            (None, Some(b), _, false) => self.remove(b).map(reverse_and_return),
            (None, None, _, _) => None,
            (Some(a), Some(_), true, _) => self.remove(a),
            (Some(_), Some(_), false, _) => None,
        }
    }

    pub fn query_backward(
        &mut self,
        point: Point,
        epsilon: f32,
        only_starts: bool,
        allow_ambiguous: bool,
    ) -> Option<PathSegment> {
        let (start, end) = self.query_impl(point, epsilon, allow_ambiguous);
        println!("(start: {:?},  end: {:?})", start, end);
        match (end, start, allow_ambiguous, only_starts) {
            (Some(a), None, _, _) => self.remove(a),
            (None, Some(_), _, true) => None,
            (None, Some(b), _, false) => self.remove(b).map(reverse_and_return),
            (None, None, _, _) => None,
            (Some(a), Some(_), true, _) => self.remove(a),
            (Some(_), Some(_), false, _) => None,
        }
    }

    fn query_impl(
        &self,
        point: Point,
        epsilon: f32,
        allow_ambiguous: bool,
    ) -> (Option<DqtId>, Option<DqtId>) {
        let query_aabb = point.aabb().expand(epsilon, epsilon, epsilon, epsilon);

        let query_starts = move || {
            let mut out = None;
            let query = self.starts
                .query(query_aabb)
                .into_iter()
                .map(|(&id, _, _)| id);
            for id in query {
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

        let query_ends = move || {
            let mut out = None;
            let query = self.ends
                .query(query_aabb)
                .into_iter()
                .map(|(&id, _, _)| id);
            for id in query {
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

        let res = (query_starts(), query_ends());

        match  res {
            (Ok(a), Ok(b)) => (a, b),
            _ => (None, None),
        }
    }
}

fn reverse_and_return(mut v: PathSegment) -> PathSegment {
    v.0.reverse();
    v
}
