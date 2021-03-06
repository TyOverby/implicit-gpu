use geometry::{PathSegment, Point};
use inspector::*;
use line_stitch::*;
use std::cmp::Ordering;

const EPSILON: f32 = 0.001;

/// Performs the following line operations:
/// 0. Sort incoming lines
/// 1. Remove Zero Area Loops
/// 2. Prune Disconnected Edges
/// 3. Connect Obvious
/// 4. Graph Stitch
pub fn connect_lines<I>(lines: I, inspector: BoxedInspector) -> Vec<PathSegment>
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
    inspector.write_lines("0-input", &lines);
    inspector.do_slow(&|| {
        let segments: Vec<PathSegment> = lines
            .iter()
            .map(|line| PathSegment::new(vec![line.0, line.1]))
            .collect();
        inspector.write_segments("0-input", &segments);
    });

    // 1: Remove Zero Area Loops
    let lines = ::flame::span_of("zero area loops", || remove_zero_area_loops(lines, EPSILON));
    inspector.write_lines("1-zero_area_removed", &lines);
    inspector.do_slow(&|| {
        let segments: Vec<PathSegment> = lines
            .iter()
            .map(|line| PathSegment::new(vec![line.0, line.1]))
            .collect();
        inspector.write_segments("1-zero_area_removed", &segments);
    });

    // 2: Prune Disconected Edges
    let dual_qt = ::flame::span_of("prune", || {
        prune(lines.into_iter().map(|(p1, p2)| [p1, p2]), EPSILON, true)
    });
    inspector.do_slow(&|| {
        let collected = dual_qt.slow_iter();
        inspector.write_segments("2-pruned", &collected);
    });

    // 3: Connect Obvious
    // TODO: fix this hack with the first parameter to
    // connect_obvious
    let lines = ::flame::span_of("connect obvious", || {
        connect_obvious_from_dual_qt(dual_qt, EPSILON, true)
    });
    inspector.write_segments("3-connected_obvious", &lines);

    // 4: Graph Stitch
    let lines = ::flame::span_of("graph stitch", || graph_stitch(lines));
    inspector.write_segments("4-stitched", &lines);

    return lines;
}
