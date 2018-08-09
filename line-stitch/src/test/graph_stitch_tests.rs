use super::util::*;
use euclid::{point2, UnknownUnit};
use graph_stitch;
use permutohedron::heap_recursive as permute;

type PathSegment = ::PathSegment<UnknownUnit>;

const EPSILON: f32 = 0.001;

#[derive(Clone)]
struct Problem {
    input: Vec<PathSegment>,
    expected: Vec<PathSegment>,
    epsilon: f32,
    only_starts: bool,
    allow_ambiguous: bool,
}

fn default_problem() -> Problem {
    Problem {
        input: vec![],
        expected: vec![],
        epsilon: EPSILON,
        only_starts: true,
        allow_ambiguous: false,
    }
}

fn run(mut p: Problem) {
    let new_p = p.clone();
    permute(&mut p.input, |input| {
        let output = graph_stitch(input.to_vec());
        if let Err(e) = assert_same(&output, &new_p.expected, !new_p.only_starts) {
            print!("{}", e);
            panic!();
        }
    });
}

#[test]
fn no_segments() {
    run(Problem {
        input: vec![],
        expected: vec![],
        ..default_problem()
    });
}

#[test]
fn one_segment_that_is_closed() {
    run(Problem {
        input: vec![PathSegment::new_and_potentially_close(
            vec![
                point2(0.0, 0.0),
                point2(0.0, 1.0),
                point2(1.0, 1.0),
                point2(0.0, 0.0),
            ],
            EPSILON,
        )],
        expected: vec![PathSegment::new_and_potentially_close(
            vec![
                point2(0.0, 0.0),
                point2(0.0, 1.0),
                point2(1.0, 1.0),
                point2(0.0, 0.0),
            ],
            EPSILON,
        )],
        ..default_problem()
    });
}

#[test]
fn two_segments_that_are_close_to_each_other() {
    run(Problem {
        input: vec![
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 0.0), point2(0.0, 1.0)],
                EPSILON,
            ),
            PathSegment::new_and_potentially_close(
                vec![point2(0.0, 1.0), point2(1.0, 1.0), point2(0.0, 0.0)],
                EPSILON,
            ),
        ],
        expected: vec![PathSegment::new_and_potentially_close(
            vec![
                point2(0.0, 0.0),
                point2(0.0, 1.0),
                point2(1.0, 1.0),
                point2(0.0, 0.0),
            ],
            EPSILON,
        )],
        ..default_problem()
    });
}
