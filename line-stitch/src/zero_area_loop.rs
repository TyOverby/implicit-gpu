use ::*;
use euclid::approxeq::ApproxEq;
use util::*;

type Rect<S> = euclid::TypedRect<f32, S>;

/// TODO: Document
///
pub fn remove_zero_area_loops<I, S: 'static>(segments: I, epsilon: f32) -> Vec<(Point<S>, Point<S>)>
where I: Into<Vec<(Point<S>, Point<S>)>> {
    let collected = segments.into();
    let size_hint = collected.len();
    let aabb = compute_bounding_box(collected.iter().flat_map(|&(p1, p2)| vec![p1, p2]));
    let aabb = aabb.inflate(1.0f32.max(aabb.size.width / 10.0), 1.0f32.max(aabb.size.height / 10.0));
    let mut quad_tree = QuadTree::default(aabb, size_hint);
    let eps = Point::new(epsilon, epsilon);

    'outer: for (p1a, p2a) in collected {
        if p1a.approx_eq_eps(&p2a, &eps) {
            continue;
        }

        let query = Rect::from_points(&[p1a, p2a]);
        let q_result: Vec<_> = quad_tree
            .query(query)
            .into_iter()
            .map(|(&line, _, id)| (line, id))
            .collect();

        for ((p1b, p2b), id) in q_result {
            // We are already in the tree TODO: should we remove these duplicates?
            /*
            if p1a.approx_eq_eps(&p1b, &eps) && p2a.approx_eq_eps(&p2b, &eps) {
                continue 'outer;
            }

            */
            // Our inverse is already in the tree
            if p1a.approx_eq_eps(&p2b, &eps) && p2a.approx_eq_eps(&p1b, &eps) {
                quad_tree.remove(id);
                continue 'outer;
            }
        }

        quad_tree.insert_with_box((p1a, p2a), query);
    }

    quad_tree.iter().map(|(_, &(l, _))| l).collect()
}
