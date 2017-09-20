use super::*;
use super::util::quadtree::*;

// TODO: *LOTS* of optimization opporitunities here

#[derive(Clone)]
struct Graph {
    tree: QuadTree<Vec<geom::Point>>,
}

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
}

fn try_solve_inner(goal: Option<geom::Point>, id: ItemId, mut graph: Graph) -> Result<(Graph, Vec<Vec<geom::Point>>, f32), Graph> {
    let target_length = graph.length_of(id);
    let neighbors = graph.connected_to(id);
    let target_path = graph.remove(id);

    let mut solutions = vec![];
    for neighbor in neighbors {
        let goal = goal.or(Some(target_path.first().cloned().unwrap()));
        if let Ok((g, mut p, d)) = try_solve_inner(goal, neighbor, graph.clone()) {
            p.push(target_path.clone());
            solutions.push((g, p, d + target_length));
        }
    }

    match goal {
        Some(g) => {
            if is_close(g, target_path.last().cloned().unwrap()){
                solutions.push((graph.clone(), vec![target_path], target_length));
            }
        }
        None => {
            let first = target_path.first().cloned().unwrap();
            let last = target_path.last().cloned().unwrap();
            if is_close(first, last) {
                solutions.push((graph.clone(), vec![target_path], target_length));
            }
        }
    }

    solutions.sort_by(|a, b| (a.2).partial_cmp(&b.2).unwrap());
    solutions.into_iter().last().ok_or(graph)
}

fn try_solve(mut graph: Graph) -> Vec<Vec<geom::Point>> {
    let _guard = ::flame::start_guard("solve graph stitch");

    let mut out = vec![];
    while !graph.tree.is_empty() {
        let first_id = graph.tree.iter().next().unwrap().0.clone();
        match try_solve_inner(None, first_id, graph) {
            Ok((g, mut path, _)) => {
                path.reverse();
                let path = path.into_iter().flat_map(|mut a| { a.pop(); a.into_iter() }).collect();
                graph = g;
                out.push(path);
            }
            Err(g) => {
                graph = g;
            }
        }
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
