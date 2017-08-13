use super::*;

pub fn join_lines<I>(lines: I) -> (Vec<LineType>, QuadTree<geom::Line>)
where
    I: Iterator<Item = geom::Line>,
{
    // Get rid of "lines" that have the same start and end point.
    let lines = lines.filter(|&geom::Line(geom::Point{x: x1, y: y1}, geom::Point{x: x2, y: y2})| {
        x1 != x2 || y1 != y2
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

        match continue_with(segment.0, segment.1, tree.clone(), resolution) {
            Ok((pts, tr)) => {
                points.extend(pts);
                tree = tr;
            }
            Err(tr) => {
                tree = tr;
            }
        }

        out.push(LineType::Unjoined(points));
    }

    (out, tree_dup)
}

fn continue_with(goal: geom::Point, mut last: geom::Point, mut tree: QuadTree<geom::Line>, resolution: f32)
    -> Result<(Vec<geom::Point>, QuadTree<geom::Line>), QuadTree<geom::Line>> {
    let mut points = Vec::new();
    loop {
        let closest = {
            let query = geom::Rect::centered_with_radius(&last, resolution / 2.0);
            let mut near_last = {
                tree.query(query)
                    .into_iter()
                    .map(|(line, _, id)| {
                        let da = line.0.distance_2(&last);
                        let db = line.0.distance_2(&last);
                        (line.clone(), id, da.min(db))
                    })
                    .collect::<Vec<_>>()
            };

            near_last.sort_by(|&(_, _, d1), &(_, _, d2)| {
                d1.partial_cmp(&d2).unwrap_or(::std::cmp::Ordering::Equal)
            });

            for &(line, _, _) in &near_last {
                if distance_to_closest_end_2(goal, line) < resolution {
                    return Ok((points, tree));
                }
            }

            // Remove any lines that were close but not the closest
            if near_last.len() >= 1 {
                let closest_dist = near_last[0].2;
                near_last = near_last
                    .into_iter()
                    .take_while(|&(_, _, d)| d == closest_dist)
                    .collect();
            }

            match near_last.len() {
                0 => return Err(tree),
                1 => near_last[0],
                n => {
                    match near_last
                        .into_iter()
                        .map(|(line, id, _)| {
                            let mut tc = tree.clone();
                            tc.remove(id);
                            continue_with(goal, furthest_end_from_line(last, line), tc, resolution)
                        })
                        .filter_map(Result::ok)
                        .min_by_key(|&(ref pts, _)| pts.len())
                    {
                        None => return Err(tree),
                        Some((pts, tr)) => {
                            println!("up");
                            let last_pt = pts.last().cloned().unwrap();
                            points.extend(pts);
                            last = last_pt;
                            tree = tr;
                            continue;
                        }
                    }
                }
            }
        };

        let (line, id, _) = closest;

        tree.remove(id);
        let t = furthest_end_from_line(last, line);
        last = t;
        points.push(last);
        if last == goal {
            break;
        }
    }

    Ok((points, tree))
}

fn furthest_end_from_line(target: geom::Point, line: geom::Line) -> geom::Point {
    if line.0.distance_2(&target) < line.1.distance_2(&target) {
        line.1
    } else {
        line.0
    }
}

fn distance_to_closest_end_2(target: geom::Point, line: geom::Line) -> f32 {
    let a = line.0.distance_2(&target);
    let b = line.1.distance_2(&target);
    if a < b {
        a
    } else {
        b
    }
}
