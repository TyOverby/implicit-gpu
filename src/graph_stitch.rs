use super::*;
use aabb_quadtree::{ItemId, QuadTree};
use euclid::{TypedRect, point2, vec2};
use euclid::approxeq::ApproxEq;
use util::{centered_with_radius, compute_bounding_box};
use std::collections::{HashMap, HashSet};

// TODO: *LOTS* of optimization opporitunities here

type Rect<S> = TypedRect<f32, S>;

#[derive(Clone)]
struct Graph<S> {
    tree: QuadTree<PathSegment<S>, S>,
}

type VisitedSet = HashMap<ItemId, f32>;
type Path = Vec<(ItemId, f32)>;

fn is_close<S>(p1: Point<S>, p2: Point<S>) -> bool {
    p1.approx_eq_eps(&p2, &point2(0.001, 0.001))
}

impl<S> Graph<S> {
    fn new(v: Vec<PathSegment<S>>) -> Graph<S> {
        let v: Vec<_> = v.into_iter()
            .map(|v| (compute_bounding_box(vec![v.first(), v.last()]), v))
            .collect();

        let mut rect = Rect::new(point2(0.0, 0.0), vec2(0.0, 0.0).to_size());
        for &(ref r, _) in &v {
            rect = rect.union(r);
        }
        let rect = rect.inflate(
            2.0f32.max(rect.size.width / 10.0),
            2.0f32.max(rect.size.height / 10.0),
        );

        let mut tree = QuadTree::new(rect, false, 4, 16, 4);

        for (bb, v) in v {
            tree.insert_with_box(v, bb);
        }

        Graph { tree: tree }
    }

    fn connected_to(&self, id: ItemId) -> Vec<ItemId> {
        let segment = self.tree.get(id).unwrap();

        let last_point = segment.last();
        let query = centered_with_radius(last_point, 1.0 / 4.0);
        return self.tree
            .query(query)
            .into_iter()
            .filter_map(|(ref v, _, id2)| {
                if id == id2 {
                    return None;
                } else if is_close(segment.last(), v.first()) {
                    Some(id2)
                } else {
                    None
                }
            })
            .collect();
    }

    fn length_of(&self, id: ItemId) -> f32 {
        let segment = self.tree.get(id).unwrap();
        segment.length()
    }

    fn remove(&mut self, id: ItemId) -> PathSegment<S> {
        self.tree.remove(id).unwrap().0
    }
    fn try_remove(&mut self, id: ItemId) {
        self.tree.remove(id);
    }
}

fn recur<S>(
    at: ItemId,
    current_length: f32,
    graph: &Graph<S>,
    visited: &mut VisitedSet,
    path: &mut Path,
    best_possible: &mut f32,
    possible: &mut Vec<Path>,
    dead_ends: &mut Vec<Path>,
) {
    let length = graph.length_of(at);

    if let Some(prior_length) = visited.get(&at) {
        if *prior_length >= current_length {
            return;
        }

        if let Some(pos) = path.iter().position(|&(id, _)| id == at) {
            let mut pos_path = path.clone();
            pos_path.drain(0..pos);
            possible.push(pos_path);
            *best_possible = (current_length + length).max(*best_possible);
            return;
        }
    }

    let mut neighbors: Vec<_> = graph
        .connected_to(at)
        .into_iter()
        .map(|a| (a, graph.length_of(a)))
        .collect();

    if neighbors.is_empty() {
        let mut dead_path = path.clone();
        dead_path.push((at, length));
        dead_ends.push(dead_path);
        return;
    }

    path.push((at, length));
    visited.insert(at, current_length);

    // TODO: is this actually faster?  Back up with data
    neighbors.sort_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap());

    for (neighbor, _) in neighbors.into_iter().rev() {
        if neighbor != at {
            recur(
                neighbor,
                current_length + length,
                graph,
                visited,
                path,
                best_possible,
                possible,
                dead_ends,
            );
        }
    }

    path.pop();
    // visited.remove(&at);
}

fn one_iter<S>(mut graph: Graph<S>) -> (Graph<S>, Vec<PathSegment<S>>) {
    use std::cmp::{Ordering, PartialOrd};
    let mut best_possible = 0.0;
    let mut possible = vec![];
    let mut dead_ends = vec![];
    let first_id = graph.tree.iter().next().unwrap().0.clone();

    recur(
        first_id,
        0.0,
        &graph,
        &mut HashMap::new(),
        &mut vec![],
        &mut best_possible,
        &mut possible,
        &mut dead_ends,
    );

    let mut possible: Vec<_> = possible
        .into_iter()
        .map(|path| {
            let length: f32 = path.iter().map(|&(_, l)| l).sum();
            let items = path.into_iter().map(|(p, _)| p).collect::<Vec<_>>();
            (items, length)
        })
        .collect();

    // Start with the longest loops
    possible.sort_by(|&(_, al), &(_, bl)| {
        al.partial_cmp(&bl).unwrap_or(Ordering::Equal)
    });
    possible.reverse();

    let mut out = vec![];
    let mut visited_loops = HashSet::new();
    let mut trash_points = HashSet::new();
    trash_points.extend(
        dead_ends
            .into_iter()
            .flat_map(|v| v.into_iter().map(|(p, _)| p)),
    );

    for (l00p, _) in possible {
        let intersects = l00p.iter().any(|point| visited_loops.contains(point));
        if intersects {
            trash_points.extend(l00p);
        } else {
            visited_loops.extend(l00p.iter().cloned());
            // TODO: this flattens things but the edge conditions might
            // be weird.
            out.push(
                l00p.into_iter()
                    .flat_map(|pt| graph.remove(pt))
                    .collect::<PathSegment<_>>(),
            );
        }
    }

    for pt in visited_loops.into_iter().chain(trash_points.into_iter()) {
        graph.try_remove(pt);
    }

    (graph, out)
}

fn try_solve<S>(mut graph: Graph<S>) -> Vec<PathSegment<S>> {
    let mut out = vec![];
    while !graph.tree.is_empty() {
        let (ng, pts) = one_iter(graph);
        graph = ng;
        out.extend(pts);
    }

    out
}

///
/// TODO: document
pub fn connect_unconnected<S>(joined: Vec<PathSegment<S>>) -> Vec<PathSegment<S>> {
    let (mut good, bad) = joined.into_iter().partition::<Vec<_>, _>(|a| a.closed);

    let graph = Graph::new(bad);
    let solved = try_solve(graph);

    good.extend(solved.into_iter());

    good
}
