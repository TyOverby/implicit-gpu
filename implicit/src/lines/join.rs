use super::*;
use super::graph_stitch::connect_unconnected;
use euclid::{UnknownUnit, point2, vec2};
use geometry::{Line, Rect, Point, bb_for_line};

pub fn join_lines<I>(lines: I, telemetry: &mut Telemetry, tloc: TelemetryLocation) -> (Vec<LineType>, QuadTree<Line, UnknownUnit>)
where
    I: IntoIterator<Item = Line>,
{
    let _guard = ::flame::start_guard("join_lines");

    // Get rid of "lines" that have the same start and end point.
    let lines = lines.into_iter().filter(|&Line(Point { x: x1, y: y1, .. }, Point { x: x2, y: y2, .. })| {
        x1 != x2 || y1 != y2
    });

    let lines = lines.collect::<Vec<_>>();
    if lines.len() == 0 {
        return (Vec::new(), QuadTree::default(Rect::new(point2(0.0, 0.0), vec2(0.0, 0.0).to_size())));
    }

    let (out, t) = join_lines_internal(lines, telemetry, tloc);

    let out = connect_unconnected(out);
    telemetry.shape_line_connected(tloc, &out);
    (out, t)
}

fn join_lines_internal(lines: Vec<Line>, telemetry: &mut Telemetry, tloc: TelemetryLocation) -> (Vec<LineType>, QuadTree<Line, UnknownUnit>) {
    let _guard = ::flame::start_guard("join_lines_internal");
    telemetry.shape_line_pre_prune(tloc, &lines);

    let mut resolution = 0.0f32;
    for &line in &lines {
        let bb = bb_for_line(line, 0.0);
        resolution = resolution.max(bb.size.width.max(bb.size.height));
    }

    let mut aabb: Option<Rect> = None;
    for &line in &lines {
        if let Some(aabb) = aabb.as_mut() {
            *aabb = aabb.union(&bb_for_line(line, resolution));
        }
        if aabb.is_none() {
            aabb = Some(bb_for_line(line, resolution));
        }
    }


    let aabb = match aabb {
        Some(aabb) => {
            // Give the bounding box some extra room
            let padding = resolution * 2.0;
            aabb.inflate(padding, padding)
        }
        None => return (vec![], QuadTree::new(Rect::new(point2(0.0, 0.0), vec2(0.0, 0.0).to_size()), false, 4, 16, 4)),
    };

    let mut tree = QuadTree::new(aabb, false, 4, 16, 4);
    for &line in &lines {
        tree.insert(line);
    }

    let mut tree = remove_peninsulas(tree, resolution);

    let after: Vec<Line> = tree.iter().map(|(_, &(line, _))| line).collect();
    telemetry.shape_line_pruned(tloc, &after);

    let tree_dup = tree.clone();

    let mut out = vec![];

    let mut inflection_point_tree = QuadTree::new(aabb, false, 4, 16, 4);

    while !tree.is_empty() {
        let first_id = tree.first().unwrap();
        let (segment, _) = tree.remove(first_id).unwrap();

        match continue_with(segment.0, segment.1, tree, &inflection_point_tree, resolution) {
            Ok((mut pts, tr)) => {
                pts.insert(0, segment.1);
                pts.insert(0, segment.0);
                tree = tr;
                out.push(LineType::Joined(pts));
            }
            Err((pts, tr)) => {
                let (mut back_pts, back_tree) = match continue_with(segment.1, segment.0, tr, &inflection_point_tree, resolution) {
                    Ok(back) => back,
                    Err(back) => back,
                };

                back_pts.reverse();
                back_pts.push(segment.0);
                back_pts.push(segment.1);
                back_pts.extend(pts);

                inflection_point_tree.insert(back_pts.first().unwrap().clone());
                inflection_point_tree.insert(back_pts.last().unwrap().clone());

                tree = back_tree;
                out.push(LineType::Unjoined(back_pts));
            }
        }

    }

    telemetry.shape_line_joined(tloc, &out);

    (out, tree_dup)
}

fn remove_peninsulas(mut tree: QuadTree<Line, UnknownUnit>, resolution: f32) -> QuadTree<Line, UnknownUnit> {
    let _guard = ::flame::start_guard("remove_peninsulas");

    // Optimization Opporitunity: When you remove a line,
    // nearby lines are likely to be the
    // next to go.
    loop {
        let mut any_removed = false;

        let lines = tree.iter().map(|(&id, &(line, _))| (id, line)).collect::<Vec<_>>();

        for (id, Line(p1, p2)) in lines {

            let left_side = centered_with_radius(p1, resolution / 2.0);
            let right_side = centered_with_radius(p2, resolution / 2.0);

            let is_peninsula = {
                let shares_endpoint = |Line(q1, q2)| {
                    // optimization for when we're comparing against our own
                    // line;
                    if p1 == q1 && p2 == q2 {
                        return false;
                    }

                    let closest = (p2 - q1).length().min((p1 - q2).length());
                    return closest < (resolution / 4.0);
                };

                let q_left = tree.query(left_side)
                    .into_iter()
                    .filter(|&(&l, _, _)| shares_endpoint(l));
                let q_right = tree.query(right_side)
                    .into_iter()
                    .filter(|&(&l, _, _)| shares_endpoint(l));

                q_left.count() < 1 || q_right.count() < 1
            };

            let is_dot = (p1 - p2).square_length() < 0.0001;

            let should_remove = is_peninsula || is_dot;

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

fn continue_with(goal: Point, mut last: Point, mut tree: QuadTree<Line, UnknownUnit>, inflection_points: &QuadTree<Point, UnknownUnit>, resolution: f32)
    -> Result<(Vec<Point>, QuadTree<Line, UnknownUnit>), (Vec<Point>, QuadTree<Line, UnknownUnit>)> {
    let mut points = Vec::new();

    loop {
        if near_inflection(last, resolution, inflection_points) {
            return Err((points, tree));
        }

        let near_last = get_lines_near(last, &tree, resolution);
        let close_to_goal = (goal - last).square_length() < resolution / 4.0;

        match (near_last.len(), close_to_goal) {
            // Nothing left, near goal, no inflection point close by
            (0, true) => {
                points.pop();
                return Ok((points, tree));
            }
            // We have other options and we're near the goal
            (1, true) => return Err((points, tree)),
            // We have one option, not near a goal or inflection point. Take it!
            (1, false) => {
                let (line, id) = near_last[0];
                tree.remove(id);
                last = furthest_end_from_line(last, line);
                points.push(last);
            }
            (_, _) => return Err((points, tree)),
        }
    }
}

fn near_inflection(pt: Point, res: f32, inflection_points: &QuadTree<Point, UnknownUnit>) -> bool {
    let inflection_query = centered_with_radius(pt, res / 4.0);
    let near_inflection_point = inflection_points.query(inflection_query).len() > 0;
    near_inflection_point
}

fn furthest_end_from_line(target: Point, line: Line) -> Point {
    if (line.0 - target).square_length() < (line.1 - target).square_length() {
        line.1
    } else {
        line.0
    }
}

fn get_lines_near(target: Point, tree: &QuadTree<Line, UnknownUnit>, resolution: f32) -> Vec<(Line, QuadId)> {
    let query = centered_with_radius(target, resolution / 4.0);
    let near_target = tree.query(query)
        .into_iter()
        .map(|(line, _, id)| {
            (line.clone(), id)
        })
        .filter(|&(ref line, _)| {
            (target - line.0).square_length() < resolution / 4.0 || (target - line.1).square_length() < resolution / 4.0
        })
        .collect::<Vec<_>>();

    return near_target;
}
