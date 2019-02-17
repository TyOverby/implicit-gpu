use super::util::*;
use connect_obvious;
use euclid::{point2, UnknownUnit};
use permutohedron::heap_recursive as permute;

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
        let output = connect_obvious(input.to_vec(), new_p.epsilon, new_p.only_starts);
        assert_same(&output, &new_p.expected, !new_p.only_starts).unwrap();
    });
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
        input: vec![vec![point2(0.0, 0.0)]],
        expected: vec![],
        ..default_problem()
    });
}

#[test]
fn segment_with_one_line() {
    run(Problem {
        input: vec![vec![point2(0.0, 0.0), point2(1.0, 1.0)]],
        expected: vec![PathSegment::new_and_potentially_close(
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            EPSILON,
        )],
        ..default_problem()
    });
}

#[test]
fn segment_with_one_line_but_the_line_is_really_short() {
    run(Problem {
        input: vec![vec![point2(0.0, 0.0), point2(0.0, 0.1)]],
        expected: vec![PathSegment::new(vec![point2(0.0, 0.0), point2(0.0, 0.1)])],
        ..default_problem()
    });
}

#[test]
fn segment_with_two_disjoint_lines() {
    run(Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(3.0, 3.0), point2(4.0, 4.0)],
        ],
        expected: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(3.0, 3.0), point2(4.0, 4.0)],
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
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(1.0, 1.0), point2(2.0, 2.0)],
        ],
        expected: vec![PathSegment::new_and_potentially_close(
            vec![point2(0.0, 0.0), point2(1.0, 1.0), point2(2.0, 2.0)],
            EPSILON,
        )],
        ..default_problem()
    });
}

#[test]
fn segment_with_two_connected_lines_going_the_wrong_way() {
    run(Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(2.0, 2.0), point2(1.0, 1.0)],
        ],
        expected: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(2.0, 2.0), point2(1.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
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
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(2.0, 2.0), point2(1.0, 1.0)],
        ],
        expected: vec![PathSegment::new_and_potentially_close(
            vec![point2(2.0, 2.0), point2(1.0, 1.0), point2(0.0, 0.0)],
            EPSILON,
        )],
        ..default_problem()
    });
}

#[test]
fn path_segment_is_not_closed() {
    let ps = PathSegment::new_and_potentially_close(
        vec![point2(0.0, 0.0), point2(1.0, 1.0), point2(2.0, 2.0)],
        EPSILON,
    );

    assert_eq!(ps.closed, false);
}

#[test]
fn path_segment_closes() {
    let ps = PathSegment::new_and_potentially_close(
        vec![
            point2(0.0, 0.0),
            point2(1.0, 1.0),
            point2(1.0, 0.0),
            point2(0.0, 0.0),
        ],
        EPSILON,
    );

    assert_eq!(ps.closed, true);
}

#[test]
fn can_build_cycles() {
    run(Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(1.0, 1.0), point2(1.0, 0.0)],
            vec![point2(1.0, 0.0), point2(0.0, 0.0)],
        ],
        expected: vec![PathSegment::new_and_potentially_close(
            vec![
                point2(0.0, 0.0),
                point2(1.0, 1.0),
                point2(1.0, 0.0),
                point2(0.0, 0.0),
            ],
            EPSILON,
        )],
        ..default_problem()
    });
}

#[test]
fn doesnt_continue_on_ambiguities() {
    let problem = Problem {
        input: vec![
            vec![point2(0.0, 0.0), point2(1.0, 1.0)],
            vec![point2(1.0, 1.0), point2(2.0, 2.0)],
            vec![point2(1.0, 1.0), point2(2.0, 3.0)],
        ],
        expected: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(1.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(1.0, 1.0), point2(2.0, 2.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(1.0, 1.0), point2(2.0, 3.0)],
                EPSILON,
            ),
        ],
        ..default_problem()
    };

    run(problem);
}

// TODO: make a double-diamond <><> shape and assert that
// there aren't any closed paths Wait, are diamond shapes
// inherantly bad?  I think it's totally fine.
