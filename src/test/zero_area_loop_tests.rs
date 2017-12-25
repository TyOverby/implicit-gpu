use ::remove_zero_area_loops;
use super::util::*;
use permutohedron::heap_recursive as permute;
use euclid::{UnknownUnit};

type Point = ::Point<UnknownUnit>;
type PathSegment = ::PathSegment<UnknownUnit>;
const EPSILON: f32 = 0.001;

#[derive(Clone)]
struct Problem {
    input: Vec<(Point, Point)>,
    expected: Vec<(Point, Point)>,
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
    let expected: Vec<_> = new_p.expected.clone().into_iter().map(|(a, b)| PathSegment::new(vec![a, b], new_p.epsilon)).collect();

    permute(&mut p.input, |input| {
        let output = remove_zero_area_loops(input.to_vec(), new_p.epsilon);
        let output: Vec<_> = output.into_iter().map(|(a, b)| PathSegment::new(vec![a, b], new_p.epsilon)).collect();

        if let Err(e) = assert_same(&output, &expected, !new_p.only_starts) {
            print!("${}", e);
            panic!();
        }
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
fn one_line() {
    run(Problem {
        input: vec![(Point::new(0.0, 0.0), Point::new(1.0, 0.0))],
        expected: vec![(Point::new(0.0, 0.0), Point::new(1.0, 0.0))],
        ..default_problem()
    });
}

#[test]
fn nearby_lines() {
    run(Problem {
        input: vec![
            (Point::new(0.0, 0.0), Point::new(1.0, 0.0)),
            (Point::new(1.0, 0.0), Point::new(1.0, 1.0)),
        ],
        expected: vec![
            (Point::new(0.0, 0.0), Point::new(1.0, 0.0)),
            (Point::new(1.0, 0.0), Point::new(1.0, 1.0)),
            ],
        ..default_problem()
    });

    run(Problem {
        input: vec![
            (Point::new(0.0, 0.0), Point::new(1.0, 1.0)),
            (Point::new(1.0, 1.0), Point::new(2.0, 2.0)),
        ],
        expected: vec![
            (Point::new(0.0, 0.0), Point::new(1.0, 1.0)),
            (Point::new(1.0, 1.0), Point::new(2.0, 2.0)),
            ],
        ..default_problem()
    });
}

#[test]
fn inverted_lines_are_removed() {
    run(Problem {
        input: vec![
            (Point::new(0.0, 0.0), Point::new(1.0, 0.0)),
            (Point::new(1.0, 0.0), Point::new(0.0, 0.0)),
        ],
        expected: vec![ ],
        ..default_problem()
    });
}
