use super::*;

const EPSILON: f32 = 0.001;

fn assert_same(actual: Vec<PathSegment>, expected: Vec<PathSegment>) {
    if actual.len() != expected.len() {
        panic!(
            "assert_same wrong lengths {} vs {}\n actual: {:?}\n expected: {:?}",
            actual.len(),
            expected.len(),
            actual,
            expected
        );
    }

    for ex in &expected {
        let mut found = false;
        for ac in &actual {
            if ex == ac {
                found = true;
                break;
            }
        }

        if !found {
            panic!(
                "{:?} not found in actual\n actual: {:?}\n expected: {:?}",
                ex,
                actual,
                expected
            );
        }
    }
}

#[test]
fn no_lines() {
    let input: Vec<Vec<Point>> = vec![];
    let output = optimize(input, EPSILON, true, false);
    let expected = vec![];
    assert_same(output, expected);
}

#[test]
fn segment_with_only_one_point() {
    let input = vec![vec![Point { x: 0.0, y: 0.0 }]];
    let output = optimize(input, EPSILON, true, false);
    let expected = vec![];
    assert_same(output, expected);
}

#[test]
fn segment_with_one_line() {
    let input = vec![vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }]];
    let output = optimize(input, EPSILON, true, false);
    let expected = vec![
        PathSegment::new(
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }].into(),
            EPSILON,
        ),
    ];
    assert_same(output, expected);
}

#[test]
fn segment_with_two_disjoint_lines() {
    let input = vec![
        vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
        vec![Point { x: 3.0, y: 3.0 }, Point { x: 4.0, y: 4.0 }],
    ];
    let output = optimize(input, EPSILON, true, false);
    let expected = vec![
        PathSegment::new(
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }].into(),
            EPSILON,
        ),
        PathSegment::new(
            vec![Point { x: 3.0, y: 3.0 }, Point { x: 4.0, y: 4.0 }].into(),
            EPSILON,
        ),
    ];
    assert_same(output, expected);
}

#[test]
fn segment_with_two_connected_lines() {
    let input = vec![
        vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
        vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }],
    ];
    let output = optimize(input, EPSILON, true, false);
    let expected = vec![
        PathSegment::new(
            vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 1.0, y: 1.0 },
                Point { x: 2.0, y: 2.0 },
            ].into(),
            EPSILON,
        ),
    ];
    assert_same(output, expected);
}

#[test]
fn segment_with_two_connected_lines_going_the_wrong_way() {
    let input = vec![
        vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
        vec![Point { x: 2.0, y: 2.0 }, Point { x: 1.0, y: 1.0 }],
    ];
    let output = optimize(input, EPSILON, true, false);
    let expected = vec![
        PathSegment::new(
            vec![Point { x: 2.0, y: 2.0 }, Point { x: 1.0, y: 1.0 }].into(),
            EPSILON,
        ),
        PathSegment::new(
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }].into(),
            EPSILON,
        ),
    ];
    assert_same(output, expected);
}

#[test]
fn segment_with_two_connected_lines_going_the_wrong_way_but_only_starts_is_off() {
    let input = vec![
        vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
        vec![Point { x: 2.0, y: 2.0 }, Point { x: 1.0, y: 1.0 }],
    ];
    let output = optimize(input, EPSILON, false, false);
    let expected = vec![
        PathSegment::new(
            vec![
                Point { x: 2.0, y: 2.0 },
                Point { x: 1.0, y: 1.0 },
                Point { x: 0.0, y: 0.0 },
            ].into(),
            EPSILON,
        ),
    ];
    assert_same(output, expected);
}

#[test]
fn path_segment_is_not_closed() {
    let ps = PathSegment::new(
        vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 1.0 },
            Point { x: 2.0, y: 2.0 },
        ].into(),
        EPSILON,
    );

    assert_eq!(ps.closed, false);
}

#[test]
fn path_segment_closes() {
    let ps = PathSegment::new(
        vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 1.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 0.0, y: 0.0 },
        ].into(),
        EPSILON,
    );

    assert_eq!(ps.closed, true);
}
