use geometry::{PathSegment, Point};
use line_stitch::*;
use std::cmp::Ordering;

const EPSILON: f32 = 0.001;

/// Performs the following line operations:
/// 0. Sort incoming lines
/// 1. Remove Zero Area Loops
/// 2. Prune Disconnected Edges
/// 3. Connect Obvious
/// 4. Graph Stitch
pub fn connect_lines<I>(lines: I, _simplify: bool) -> Vec<PathSegment>
where
    I: Into<Vec<(Point, Point)>>,
{
    let _guard = ::flame::start_guard("connect_lines");
    let mut lines = lines.into();

    // 0: Sort incoming lines to reduce nondeterminism
    ::flame::span_of("sort lines", || {
        lines.sort_by(
            |&(Point { x: a1, y: b1, .. }, Point { x: c1, y: d1, .. }),
             &(Point { x: a2, y: b2, .. }, Point { x: c2, y: d2, .. })| {
                (a1, b1, c1, d1)
                    .partial_cmp(&(a2, b2, c2, d2))
                    .unwrap_or(Ordering::Equal)
            },
        )
    });
    //telemetry.lines_0_input(tloc, &lines);

    // 1: Remove Zero Area Loops
    let lines = ::flame::span_of("zero area loops", || remove_zero_area_loops(lines, EPSILON));
    //telemetry.lines_1_zero_area_removed(tloc, &lines);

    // 2: Prune Disconected Edges
    let dual_qt = ::flame::span_of("prune", || {
        prune(lines.into_iter().map(|(p1, p2)| [p1, p2]), EPSILON, true)
    });
    //telemetry.lines_2_pruned(tloc, &|| dual_qt.slow_iter());

    // 3: Connect Obvious
    // TODO: fix this hack with the first parameter to
    // connect_obvious
    let lines = ::flame::span_of("connect obvious", || {
        connect_obvious_from_dual_qt(dual_qt, EPSILON, true)
    });
    //telemetry.lines_3_obvious_connected(tloc, &lines);

    // 4: Graph Stitch
    let lines = ::flame::span_of("graph stitch", || graph_stitch(lines));
    //telemetry.lines_4_graph_stitched(tloc, &lines);

    return lines;
}
