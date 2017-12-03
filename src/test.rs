use super::*;

const EPSILON: f32 = 0.001;

#[derive(Clone)]
struct Problem {
    input: Vec<Vec<Point>>,
    expected: Vec<PathSegment>,
    epsilon: f32,
    only_starts: bool,
    allow_ambiguous: bool,
}

fn default_problem() -> Problem {
    Problem {
        input: vec![],
        expected: vec![],
        epsilon: 0.001,
        only_starts: true,
        allow_ambiguous: false,
    }
}

fn run(mut p: Problem) {
    for _ in 0..p.input.len() {
        let new_p = p.clone();
        rotate_1(&mut p.input);

        let output = optimize(
            new_p.input,
            new_p.epsilon,
            new_p.only_starts,
            new_p.allow_ambiguous,
        );
        assert_same(output, new_p.expected, !p.only_starts).unwrap();
    }

    fn assert_same(
        actual: Vec<PathSegment>,
        expected: Vec<PathSegment>,
        permit_reversed: bool,
    ) -> Result<(), String> {
        if actual.len() != expected.len() {
            return Err(format!(
                "assert_same wrong lengths {} vs {}\n actual: {:?}\n expected: {:?}",
                actual.len(),
                expected.len(),
                actual,
                expected
            ));
        }

        for ex in &expected {
            let mut found = false;
            for ac in &actual {
                if is_equal(&ex, &ac, permit_reversed) {
                    found = true;
                    break;
                }
            }

            if !found {
                return Err(format!(
                    "{:?} not found in actual\n actual: {:?}\n expected: {:?}",
                    ex,
                    actual,
                    expected
                ));
            }
        }

        return Ok(());

        fn is_equal(expected: &PathSegment, actual: &PathSegment, permit_reversed: bool) -> bool {
            if expected.path.len() != actual.path.len() || expected.closed != actual.closed {
                return false;
            }
            let mut path = actual.path.clone();
            let basic_shifted = is_shifted_of(&expected.path, &mut path);
            let reverse_shifted = permit_reversed && {
                path.reverse();
                is_shifted_of(&expected.path, &mut path)
            };

            return basic_shifted || reverse_shifted;
        }

        fn is_shifted_of<T: Eq + std::fmt::Debug>(goal: &[T], actual: &mut [T]) -> bool {
            if goal.len() != actual.len() {
                return false;
            }
            for _ in 0..goal.len() {
                if goal == actual {
                    return true;
                }
                rotate_1(actual);
            }

            return false;
        }
    }

    fn rotate_1<T>(arr: &mut [T]) {
        {
            let (left, right) = arr.split_at_mut(1);
            left.reverse();
            right.reverse();
        }
        arr.reverse();
    }
}


#[test]
fn no_lines() {
    run(Problem {
        input: vec![],
        expected: vec![],
        ..default_problem()
    });
}

#[test]
fn segment_with_only_one_point() {
    run(Problem {
        input: vec![vec![Point { x: 0.0, y: 0.0 }]],
        expected: vec![],
        ..default_problem()
    });
}

#[test]
fn segment_with_one_line() {
    run(Problem {
        input: vec![vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }]],
        expected: vec![
            PathSegment::new(
                vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
                EPSILON,
            ),
        ],
        ..default_problem()
    });
}

#[test]
fn segment_with_two_disjoint_lines() {
    run(Problem {
        input: vec![
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
            vec![Point { x: 3.0, y: 3.0 }, Point { x: 4.0, y: 4.0 }],
        ],
        expected: vec![
            PathSegment::new(
                vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
                EPSILON,
            ),
            PathSegment::new(
                vec![Point { x: 3.0, y: 3.0 }, Point { x: 4.0, y: 4.0 }],
                EPSILON,
            ),
        ],
        ..default_problem()
    });
}

#[test]
fn segment_with_two_connected_lines() {
    run(Problem {
        input: vec![
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
            vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }],
        ],
        expected: vec![
            PathSegment::new(
                vec![
                    Point { x: 0.0, y: 0.0 },
                    Point { x: 1.0, y: 1.0 },
                    Point { x: 2.0, y: 2.0 },
                ],
                EPSILON,
            ),
        ],
        ..default_problem()
    });
}

#[test]
fn segment_with_two_connected_lines_going_the_wrong_way() {
    run(Problem {
        input: vec![
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
            vec![Point { x: 2.0, y: 2.0 }, Point { x: 1.0, y: 1.0 }],
        ],
        expected: vec![
            PathSegment::new(
                vec![Point { x: 2.0, y: 2.0 }, Point { x: 1.0, y: 1.0 }],
                EPSILON,
            ),
            PathSegment::new(
                vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
                EPSILON,
            ),
        ],
        ..default_problem()
    });
}

#[test]
fn segment_with_two_connected_lines_going_the_wrong_way_but_only_starts_is_off() {
    run(Problem {
        only_starts: false,
        input: vec![
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
            vec![Point { x: 2.0, y: 2.0 }, Point { x: 1.0, y: 1.0 }],
        ],
        expected: vec![
            PathSegment::new(
                vec![
                    Point { x: 2.0, y: 2.0 },
                    Point { x: 1.0, y: 1.0 },
                    Point { x: 0.0, y: 0.0 },
                ],
                EPSILON,
            ),
        ],
        ..default_problem()
    });
}

#[test]
fn path_segment_is_not_closed() {
    let ps = PathSegment::new(
        vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 1.0 },
            Point { x: 2.0, y: 2.0 },
        ],
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
        ],
        EPSILON,
    );

    assert_eq!(ps.closed, true);
}

#[test]
fn can_build_cycles() {
    run(Problem {
        input: vec![
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
            vec![Point { x: 1.0, y: 1.0 }, Point { x: 1.0, y: 0.0 }],
            vec![Point { x: 1.0, y: 0.0 }, Point { x: 0.0, y: 0.0 }],
        ],
        expected: vec![
            PathSegment::new(
                vec![
                    Point { x: 0.0, y: 0.0 },
                    Point { x: 1.0, y: 1.0 },
                    Point { x: 1.0, y: 0.0 },
                    Point { x: 0.0, y: 0.0 },
                ],
                EPSILON,
            ),
        ],
        ..default_problem()
    });
}

#[test]
fn doesnt_continue_on_ambiguities() {
    let problem = Problem {
        input: vec![
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
            vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }],
            vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 3.0 }],
        ],
        expected: vec![
            PathSegment::new(
                vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
                EPSILON,
            ),
            PathSegment::new(
                vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }],
                EPSILON,
            ),
            PathSegment::new(
                vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 3.0 }],
                EPSILON,
            ),
        ],
        ..default_problem()
    };

    run(problem);
}
