use super::*;

pub fn join_lines<I>(lines: I) -> (Vec<LineType>, QuadTree<geom::Line>)
where
    I: Iterator<Item = geom::Line>,
{
    let lines = lines.map(|geom::Line(geom::Point { x: x1, y: y1 }, geom::Point { x: x2, y: y2 })| {
        geom::Line(geom::Point { x: x1, y: y1 }, geom::Point { x: x2, y: y2 })
    });

    let lines = lines.collect::<Vec<_>>();
    if lines.len() == 0 {
        return (Vec::new(), QuadTree::default(geom::Rect::null()));
    }

    join_lines_internal(lines)
}

fn join_lines_internal(lines: Vec<geom::Line>) -> (Vec<LineType>, QuadTree<geom::Line>) {
    let resolution = lines[0].bounding_box().width();

    let mut aabb: Option<geom::Rect> = None;
    for line in &lines {
        if let Some(aabb) = aabb.as_mut() {
            *aabb = aabb.union_with(&line.bounding_box());
        }
        if aabb.is_none() {
            aabb = Some(line.bounding_box());
        }
    }

    let aabb = match aabb {
        Some(aabb) => aabb,
        None => return (vec![], QuadTree::new(geom::Rect::null(), false, 4, 16, 4)),
    };

    let mut tree = QuadTree::new(aabb, false, 4, 16, 4);
    for line in lines {
        tree.insert(line);
    }
    let tree_dup = tree.clone();

    let mut out = vec![];

    while !tree.is_empty() {
        let first_id = tree.first().unwrap();
        let (segment, _) = tree.remove(first_id).unwrap();
        let mut points = vec![segment.0, segment.1];
        let mut last = segment.1;

        loop {
            let closest = {
                let query = geom::Rect::centered_with_radius(&last, resolution / 2.0);
                let near_last = tree.query(query);
                let mut near_last = near_last.into_iter().map(|(line, _, id)| {
                    let da = line.0.distance_2(&last);
                    let db = line.0.distance_2(&last);
                    (line, id, da.min(db))
                }).collect::<Vec<_>>();

                near_last.sort_by(|&(_, _, d1), &(_, _, d2)| {
                    d1.partial_cmp(&d2).unwrap_or(::std::cmp::Ordering::Equal)
                });

                if near_last.len() >= 2 {
                    // println!("SPLIT {:?}", near_last.len());
                }

                let closest_line_opt = near_last.into_iter().next();
                closest_line_opt.map(|(a, b, c)| (a.clone(), b.clone(), c.clone()))
            };

            if let Some((line, id, _)) = closest {
                tree.remove(id);
                if line.0.distance_2(&last) < line.1.distance_2(&last) {
                    last = line.1;
                    points.push(line.1);
                } else {
                    last = line.0;
                    points.push(line.0);
                }
            } else {
                break;
            }
        }

        out.push(LineType::Unjoined(points));
    }

    (out, tree_dup)
}
