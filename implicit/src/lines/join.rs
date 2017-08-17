use super::*;

pub fn join_lines<I>(lines: I, telemetry: &mut Telemetry, tloc: TelemetryLocation) -> (Vec<LineType>, QuadTree<geom::Line>)
where
    I: Iterator<Item = geom::Line>,
{
    // Get rid of "lines" that have the same start and end
    // point.
    let lines = lines.filter(|&geom::Line(geom::Point { x: x1, y: y1 }, geom::Point { x: x2, y: y2 })| {
        x1 != x2 || y1 != y2
    });

    let lines = lines.collect::<Vec<_>>();
    if lines.len() == 0 {
        return (Vec::new(), QuadTree::default(geom::Rect::null()));
    }

    join_lines_internal(lines, telemetry, tloc)
}

fn join_lines_internal(lines: Vec<geom::Line>, telemetry: &mut Telemetry, tloc: TelemetryLocation) -> (Vec<LineType>, QuadTree<geom::Line>) {
    telemetry.shape_line_pre_prune(tloc, &lines);

    let mut resolution = 0.0f32;
    for &line in &lines {
        let bb = line.bounding_box();
        resolution = resolution.max(bb.width().max(bb.height()));
    }

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
    for &line in &lines {
        tree.insert(line);
    }

    let mut tree = remove_peninsulas(tree, resolution);

    let after: Vec<geom::Line> = tree.iter().map(|(_, &(line, _))| line).collect();
    telemetry.shape_line_pruned(tloc, &after);

    let tree_dup = tree.clone();

    let mut out = vec![];

    while !tree.is_empty() {
        let first_id = tree.first().unwrap();
        let (segment, _) = tree.remove(first_id).unwrap();

        match continue_with(segment.0, segment.1, tree, resolution)
        {
            Ok((mut pts, tr)) => {
                pts.insert(0, segment.1);
                pts.insert(0, segment.0);
                tree = tr;
                out.push(LineType::Joined(pts));
            }
            Err((mut pts, tr)) => {
                pts.insert(0, segment.0);
                pts.insert(0, segment.1);
                tree = tr;
                out.push(LineType::Unjoined(pts));
            }
        }

    }

    (out, tree_dup)
}

fn remove_peninsulas(mut tree: QuadTree<geom::Line>, resolution: f32) -> QuadTree<geom::Line> {

    // Optimization Opporitunity: When you remove a line, nearby lines are likely to be the
    // next to go.
    loop {
        let mut any_removed = false;

        let lines = tree.iter().map(|(&id, &(line, _))| (id, line)).collect::<Vec<_>>();

        for (id, line) in lines {

            let left_side = geom::Rect::centered_with_radius(&line.0, resolution / 4.0);
            let right_side = geom::Rect::centered_with_radius(&line.1, resolution / 4.0);

            let is_peninsula = {
                let shares_endpoint = |geom::Line(q1, q2)| {
                    let geom::Line(p1, p2) = line;
                    // optimization for when we're comparing against our own line;
                    if p1 == q1 { return true; }

                    let closest =
                        p1.distance(&q1).min(
                        p2.distance(&q1)).min(
                        p1.distance(&q2).min(
                        p2.distance(&q2)));

                    return closest < (resolution / 4.0);
                };
                let q_left = tree.query(left_side).into_iter().filter(|&(&l, _, _)| shares_endpoint(l));
                let q_right = tree.query(right_side).into_iter().filter(|&(&l, _, _)| shares_endpoint(l));

                q_left.count() < 2 || q_right.count() < 2
            };

            let is_dot = (line.0).distance(&line.1) < 0.001;

            let should_remove = is_peninsula | is_dot;

            if should_remove {
                tree.remove(id);
                any_removed = true;
            }
        }

        if !any_removed {
            break;
        }
    }

    tree
}

fn continue_with(goal: geom::Point, mut last: geom::Point, mut tree: QuadTree<geom::Line>, resolution: f32)
    -> Result<(Vec<geom::Point>, QuadTree<geom::Line>), (Vec<geom::Point>, QuadTree<geom::Line>)> {
    let mut points = Vec::new();

    loop {
        let near_last = get_lines_near(last, &tree, resolution);

        if near_last.len() == 0 {
            return Err((points, tree));
        } else if near_last.len() == 1 {
            let (line, id, _) = near_last[0];
            tree.remove(id);
            let this = furthest_end_from_line(last, line);
            points.push(this);
            last = this;
        } else {
            for &(_, _, d) in &near_last {
                debug_assert!(d == near_last[0].2);
            }

            let mut continued = near_last
                .into_iter()
                .map(|(line, id, _)| {
                    let this = furthest_end_from_line(last, line);
                    let mut ct = tree.clone();
                    ct.remove(id);
                    (this, continue_with(goal, this, ct, resolution))
                })
                .collect::<Vec<_>>();

            // Sort the continuation options such that
            // * OKs show up before errors
            // * OKs that have long lengths show up before low lengths
            // * Errs that have short lengths will show up before errs
            // with long lengths.
            continued.sort_by_key(|&(_, ref res)| match res {
                &Ok((ref pts, _)) => -(pts.len() as i64),
                &Err((ref pts, _)) => pts.len() as i64,
            });

            match continued.into_iter().next().unwrap() {
                (pt, Ok((pts, tree))) => {
                    points.push(pt);
                    points.extend(pts);
                    return Ok((points, tree));
                }
                (pt, Err((pts, tree))) => {
                    points.push(pt);
                    points.extend(pts);
                    return Err((points, tree));
                }
            }
        }

        if goal.distance_2(&last) < resolution {
            points.pop();
            return Ok((points, tree));
        }
    }
}

fn furthest_end_from_line(target: geom::Point, line: geom::Line) -> geom::Point {
    if line.0.distance_2(&target) < line.1.distance_2(&target) {
        line.1
    } else {
        line.0
    }
}

fn get_lines_near(target: geom::Point, tree: &QuadTree<geom::Line>, resolution: f32) -> Vec<(geom::Line, QuadId, f32)> {
    let query = geom::Rect::centered_with_radius(&target, resolution / 4.0);
    let mut near_target = tree.query(query)
        .into_iter()
        .map(|(line, _, id)| {
            let da = line.0.distance_2(&target);
            let db = line.1.distance_2(&target);
            (line.clone(), id, da.min(db))
        })
        .collect::<Vec<_>>();

    // If we only have 0 or 1 elements, no need to do any
    // sorting, filtering, or anything
    if near_target.len() == 0 || near_target.len() == 1 {
        return near_target;
    }

    // If all of the lines are the same distance away, then no
    // need to sort and filter.
    if near_target.windows(2).all(|window| window[0].2 == window[1].2) {
        return near_target;
    }

    // Find the smallest distance and only use those
    near_target.sort_by(|&(_, _, d1), &(_, _, d2)| {
        d1.partial_cmp(&d2).unwrap_or(::std::cmp::Ordering::Equal)
    });

    let smallest_dist = near_target.first().unwrap().2;

    // Pop off all the other lines that don't have that small
    // distance.
    loop {
        if near_target.last().unwrap().2 == smallest_dist {
            break;
        } else {
            near_target.pop();
        }
    }

    near_target
}
