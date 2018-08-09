use euclid::approxeq::ApproxEq;
use util::*;
use *;

type Rect<S> = euclid::TypedRect<f32, S>;

/// TODO: Document
///
pub fn remove_zero_area_loops<I, S: 'static>(segments: I, epsilon: f32) -> Vec<(Point<S>, Point<S>)>
where
    I: Into<Vec<(Point<S>, Point<S>)>>,
{
    let collected = segments.into();
    let size_hint = collected.len();
    let aabb = compute_bounding_box(collected.iter().flat_map(|&(p1, p2)| vec![p1, p2]));
    let aabb = aabb.inflate(
        1.0f32.max(aabb.size.width / 10.0),
        1.0f32.max(aabb.size.height / 10.0),
    );
    let mut quad_tree: QuadTree<_, _, [_; 32]> = QuadTree::new(aabb, true, 0, 64, 64, size_hint);
    let eps = Point::new(epsilon, epsilon);

    for (p1a, p2a) in collected {
        if p1a.approx_eq_eps(&p2a, &eps) {
            continue;
        }

        let query = Rect::from_points(&[p1a, p2a]);
        let result = quad_tree.custom_query(query, &mut |id, _| {
            let fetched = quad_tree.get(id);
            if fetched == Some(&(p2a, p1a)) {
                Err(id)
            } else {
                Ok(())
            }
        });

        if let Err(id) = result {
            quad_tree.remove(id);
        } else {
            quad_tree.insert_with_box((p1a, p2a), query);
        }
    }

    quad_tree.iter().map(|(_, &(l, _))| l).collect()
}
