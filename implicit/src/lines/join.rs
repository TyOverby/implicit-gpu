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

        match continue_with(segment.0, segment.1, tree.clone(), resolution).map_err(|e|
              (e, continue_with(segment.1, segment.0, tree.clone(), resolution))) {
                  Ok((mut pts, tr)) => {
                      pts.insert(0, segment.1);
                      pts.insert(0, segment.0);
                      tree = tr;
                      out.push(LineType::Joined(pts));
                  },
                  Err((_, Ok((mut pts, tr)))) => {
                      pts.insert(0, segment.0);
                      pts.insert(0, segment.1);
                      tree = tr;
                      out.push(LineType::Joined(pts));
                  }
                  Err(((pts1, tr1), Err((pts2, tr2)))) => {
                      if pts1.len() < pts2.len() {
                          let (mut pts, tr)  = (pts1, tr1);
                          pts.insert(0, segment.1);
                          pts.insert(0, segment.0);
                          tree = tr;
                          out.push(LineType::Unjoined(pts));
                      } else {
                          let (mut pts, tr)  = (pts2, tr2);
                          pts.insert(0, segment.0);
                          pts.insert(0, segment.1);
                          tree = tr;
                          out.push(LineType::Unjoined(pts));
                      };
                  }
              }

    }

    (out, tree_dup)
}

fn continue_with(goal: geom::Point, mut last: geom::Point, mut tree: QuadTree<geom::Line>, resolution: f32)
    -> Result<(Vec<geom::Point>, QuadTree<geom::Line>),
              (Vec<geom::Point>, QuadTree<geom::Line>)> {
    let mut points = Vec::new();

    loop {
        let near_last = get_lines_near(last, &tree, resolution);

        if near_last.len() == 0 {
            return Err((points, tree));
        }
        else if near_last.len() == 1 {
            let (line, id, _) = near_last[0];
            tree.remove(id);
            let this = furthest_end_from_line(last, line);
            points.push(this);
            last = this;
        } else {
            let mut continued = near_last.into_iter().map(|(line, id, _)| {
                let this = furthest_end_from_line(last, line);
                let mut ct = tree.clone();
                ct.remove(id);
                (this, continue_with(goal, this, ct, resolution))
            }).collect::<Vec<_>>();

            // Sort the continuation options such that
            // * OKs show up before errors
            // * OKs that have long lengths show up before low lengths
            // * Errs that have short lengths will show up before errs with long lengths.
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
            return Ok((points, tree))
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
    let query = geom::Rect::centered_with_radius(&target, resolution / 2.0);
    let mut near_target =
        tree.query(query)
            .into_iter()
            .map(|(line, _, id)| {
                let da = line.0.distance_2(&target);
                let db = line.1.distance_2(&target);
                (line.clone(), id, da.min(db))
            })
            .collect::<Vec<_>>();

    // If we only have 0 or 1 elements, no need to do any sorting, filtering, or anything
    if near_target.len() == 0 || near_target.len() == 1 {
        return near_target;
    }

    // If all of the lines are the same distance away, then no need to sort and filter.
    if near_target.windows(2).all(|window| { window[0].2 == window[1].2} ) {
        return near_target;
    }

    // Find the smallest distance and only use those
    near_target.sort_by(|&(_, _, d1), &(_, _, d2)| {
        d1.partial_cmp(&d2).unwrap_or(::std::cmp::Ordering::Equal)
    });

    let smallest_dist = near_target.last().unwrap().2;

    // Pop off all the other lines that don't have that small distance.
    loop {
        if near_target.last().unwrap().2 == smallest_dist {
            break;
        } else {
            near_target.pop();
        }
    }

    near_target
}
