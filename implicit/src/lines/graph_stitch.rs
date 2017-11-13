use super::*;
use super::util::quadtree::*;
use std::collections::HashSet;

// TODO: *LOTS* of optimization opporitunities here

#[derive(Clone)]
struct Graph {
    tree: QuadTree<Vec<geom::Point>>,
}

type VisitedSet = HashSet<ItemId>;
type Path = Vec<(ItemId, f32)>;

fn is_close(p1: geom::Point, p2: geom::Point) -> bool { p1.close_to(&p2, 0.001) }

impl Graph {
    fn new(v: Vec<Vec<geom::Point>>) -> Graph {
        let v: Vec<_> = v.into_iter()
            .map(|v| {
                let mut rect = geom::Rect::null();
                for v in &v {
                    rect.expand_to_include(v);
                }
                (rect, v)
            })
            .collect();

        let mut rect = geom::Rect::null();
        for &(ref r, _) in &v {
            rect = rect.union_with(r);
        }

        let mut tree = QuadTree::new(rect.expand(2.0, 2.0, 2.0, 2.0), true, 4, 16, 4);

        for (bb, v) in v {
            tree.insert_with_box(v, bb);
        }

        Graph { tree: tree }
    }

    fn connected_to(&self, id: ItemId) -> Vec<ItemId> {
        let segment = self.tree.get(id).unwrap();

        let last_point = segment.last().cloned().unwrap();
        let query = geom::Rect::centered_with_radius(&last_point, 1.0 / 4.0);
        return self.tree
            .query(query)
            .into_iter()
            .filter_map(|(ref v, _, id2)| if id == id2 {
                return None;
            } else if is_close(segment.last().cloned().unwrap(), v.first().cloned().unwrap()) {
                Some(id2)
            } else {
                None
            })
            .collect();
    }

    fn length_of(&self, id: ItemId) -> f32 {
        let segment = self.tree.get(id).unwrap();
        let mut dist = 0.0;
        for window in segment.windows(2) {
            dist += window[0].distance(&window[1]);
        }

        dist
    }

    fn remove(&mut self, id: ItemId) -> Vec<geom::Point> { self.tree.remove(id).unwrap().0 }
    fn try_remove(&mut self, id: ItemId) { self.tree.remove(id); }
}

fn recur(
    at: ItemId, current_length: f32, graph: &Graph, visited: &mut VisitedSet, path: &mut Path, best_possible: &mut f32, possible: &mut Vec<Path>,
    dead_ends: &mut Vec<Path>,
) {
    let length = graph.length_of(at);

    if visited.contains(&at) {
        if current_length + length < *best_possible {
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

    let neighbors = graph.connected_to(at);
    if neighbors.is_empty() {
        let mut dead_path = path.clone();
        dead_path.push((at, length));
        dead_ends.push(dead_path);
        return;
    }

    path.push((at, length));
    visited.insert(at);

    for neighbor in neighbors {
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
    //visited.remove(&at);
}

fn one_iter(mut graph: Graph) -> (Graph, Vec<Vec<geom::Point>>) {
    use std::cmp::{Ordering, PartialOrd};
    let mut best_possible = 0.0;
    let mut possible = vec![];
    let mut dead_ends = vec![];
    let first_id = graph.tree.iter().next().unwrap().0.clone();

    recur(
        first_id,
        0.0,
        &graph,
        &mut HashSet::new(),
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
    possible.sort_by(|&(_, al), &(_, bl)| al.partial_cmp(&bl).unwrap_or(Ordering::Equal));
    possible.reverse();

    let mut out = vec![];
    let mut visited_loops = HashSet::new();
    let mut trash_points = HashSet::new();
    trash_points.extend(dead_ends.into_iter().flat_map(|v| v.into_iter().map(|(p, _)| p)));

    for (l00p, _) in possible {
        let intersects = l00p.iter().any(|point| visited_loops.contains(point));
        if intersects {
            trash_points.extend(l00p);
        } else {
            visited_loops.extend(l00p.iter().cloned());
            // TODO: this flattens things but the edge conditions might
            // be weird.
            out.push(l00p.into_iter().flat_map(|pt| graph.remove(pt)).collect::<Vec<_>>());
        }
    }

    for pt in visited_loops.into_iter().chain(trash_points.into_iter()) {
        graph.try_remove(pt);
    }

    (graph, out)
}

fn try_solve(mut graph: Graph) -> Vec<Vec<geom::Point>> {
    let _guard = ::flame::start_guard("solve graph stitch");

    let mut out = vec![];
    while !graph.tree.is_empty() {
        let (ng, pts) = one_iter(graph);
        graph = ng;
        out.extend(pts);
    }

    out
}

pub fn connect_unconnected(joined: Vec<LineType>) -> Vec<LineType> {
    let _guard = ::flame::start_guard("connect_unconnected");

    let (mut good, bad) = joined.into_iter().partition::<Vec<_>, _>(|a| match a {
        &LineType::Joined(_) => true,
        &LineType::Unjoined(_) => false,
    });

    let bad_internal = bad.into_iter()
        .filter_map(|uj| match uj {
            LineType::Unjoined(v) => if v.len() > 0 {
                Some(v)
            } else {
                None
            },
            _ => None,
        })
        .collect::<Vec<_>>();

    let graph = Graph::new(bad_internal);
    let solved = try_solve(graph);

    good.extend(solved.into_iter().map(LineType::Joined));

    good
}
