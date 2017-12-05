use ::*;
use super::util::*;
use permutohedron::heap_recursive as permute;

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
        let output = prune(input.to_vec(), new_p.epsilon, new_p.only_starts);
        assert_same(&output, &new_p.expected, !new_p.only_starts).unwrap();
    });
}

#[test]
pub fn prune_removes_single_line() {
    run(Problem {
        input: vec![vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }]],
        expected: vec![],
        ..default_problem()
    });
}

#[test]
pub fn prune_removes_two_duplicate_lines() {
    run(Problem {
        input: vec![
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
        ],
        expected: vec![],
        ..default_problem()
    });
}

#[test]
pub fn prune_doesnt_remove_two_duplicate_lines_if_only_starts_is_off() {
    run(Problem {
        input: vec![
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
            vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }],
        ],
        expected: vec![
            PathSegment::new(vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }], EPSILON),
            PathSegment::new(vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }], EPSILON),
        ],
        only_starts: false,
        ..default_problem()
    });
}
