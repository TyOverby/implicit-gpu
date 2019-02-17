use super::util::*;
use euclid::{point2, UnknownUnit};
use permutohedron::heap_recursive as permute;
use prune;

type Point = ::Point<UnknownUnit>;
type PathSegment = ::PathSegment<UnknownUnit>;

const EPSILON: f32 = 0.001;

#[derive(Clone)]
struct Problem {
    input: Vec<Vec<Point>>,
    expected: Vec<PathSegment>,
    epsilon: f32,
    only_starts: bool,
}

fn default_problem() -> Problem {
    Problem {
        input: vec![],
        expected: vec![],
        epsilon: EPSILON,
        only_starts: true,
    }
}

fn run(mut p: Problem) {
    let new_p = p.clone();
    permute(&mut p.input, |input| {
        let output: Vec<_> = prune(input.to_vec(), new_p.epsilon, new_p.only_starts)
            .into_iter()
            .collect();
        if let Err(e) = assert_same(&output, &new_p.expected, !new_p.only_starts) {
            panic!("{}", e);
        }
    });
}

#[test]
pub fn prune_removes_single_line() {
    let p = Problem {
        input: vec![vec![point2(0.0, 0.0), point2(1.0, 1.0)]],
        expected: vec![],
        ..default_problem()
    };

    run(Problem {
        only_starts: true,
        ..p.clone()
    });

    run(Problem {
        only_starts: false,
        ..p
    });
}

#[test]
pub fn prune_removes_two_duplicate_lines() {
    run(Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
        ],
        expected: vec![],
        ..default_problem()
    });
}

#[test]
pub fn prune_doesnt_remove_two_duplicate_lines_if_only_starts_is_off() {
    run(Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
        ],
        expected: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
                EPSILON,
            ),
        ],
        only_starts: false,
        ..default_problem()
    });
}

#[test]
pub fn prune_removes_a_middle_line() {
    let p = Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(1.0, 1.0), point2(2.0, 2.0)],
            vec![point2(2.0, 2.0), point2(3.0, 3.0)],
        ],
        expected: vec![],
        ..default_problem()
    };

    run(Problem {
        only_starts: true,
        ..p.clone()
    });
    run(Problem {
        only_starts: false,
        ..p
    });
}

#[test]
pub fn prune_doesnt_remove_a_triangle() {
    let p = Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(1.0, 1.0), point2(1.0, 0.0)],
            vec![point2(1.0, 0.0), point2(0.0, 0.0)],
        ],
        expected: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(1.0, 1.0), point2(1.0, 0.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(1.0, 0.0), point2(0.0, 0.0)],
                EPSILON,
            ),
        ],
        ..default_problem()
    };

    run(Problem {
        only_starts: true,
        ..p.clone()
    });
    run(Problem {
        only_starts: false,
        ..p
    });
}

#[test]
fn prune_doesnt_remove_a_cycle_between_two_lines() {
    let p = Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(1.0, 1.0), point2(0.0, 0.0)],
        ],
        expected: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(1.0, 1.0), point2(0.0, 0.0)],
                EPSILON,
            ),
        ],
        ..default_problem()
    };

    run(Problem {
        only_starts: true,
        ..p.clone()
    });
    run(Problem {
        only_starts: false,
        ..p
    });
}

#[test]
fn removes_a_dangling_line_off_the_front_of_a_cycle() {
    let p = Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(1.0, 1.0), point2(0.0, 0.0)],
            vec![point2(-1.0, -1.0), point2(0.0, 0.0)],
        ],
        expected: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(1.0, 1.0), point2(0.0, 0.0)],
                EPSILON,
            ),
        ],
        ..default_problem()
    };

    run(Problem {
        only_starts: true,
        ..p.clone()
    });
    run(Problem {
        only_starts: false,
        ..p
    });

    let p = Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(1.0, 1.0), point2(0.0, 0.0)],
            vec![point2(0.0, 0.0), point2(-1.0, -1.0)],
        ],
        expected: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(1.0, 1.0), point2(0.0, 0.0)],
                EPSILON,
            ),
        ],
        ..default_problem()
    };

    run(Problem {
        only_starts: true,
        ..p.clone()
    });
    run(Problem {
        only_starts: false,
        ..p
    });
}

#[test]
fn removes_a_dangling_line_off_the_back_of_a_cycle() {
    let p = Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(1.0, 1.0), point2(0.0, 0.0)],
            vec![point2(2.0, 2.0), point2(1.0, 1.0)],
        ],
        expected: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(1.0, 1.0), point2(0.0, 0.0)],
                EPSILON,
            ),
        ],
        ..default_problem()
    };

    run(Problem {
        only_starts: true,
        ..p.clone()
    });
    run(Problem {
        only_starts: false,
        ..p
    });

    let p = Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(1.0, 1.0), point2(0.0, 0.0)],
            vec![point2(1.0, 1.0), point2(2.0, 2.0)],
        ],
        expected: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(1.0, 1.0), point2(0.0, 0.0)],
                EPSILON,
            ),
        ],
        ..default_problem()
    };

    run(Problem {
        only_starts: true,
        ..p.clone()
    });
    run(Problem {
        only_starts: false,
        ..p
    });
}
