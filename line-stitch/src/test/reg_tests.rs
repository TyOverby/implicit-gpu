use euclid::{point2, Point2D, UnknownUnit};
use *;

const EPSILON: f32 = 0.001;
// CIRCLES 99 actually has 97 circles
const CIRCLES_99_DATA: &'static [((f32, f32), (f32, f32))] = include!("./regressions/99_circles.txt");
const ONE_CIRCLE_DATA: &'static [((f32, f32), (f32, f32))] = include!("./regressions/one_circle.txt");

fn process(input: &[((f32, f32), (f32, f32))]) -> Vec<[Point2D<f32>; 2]> {
    input
        .iter()
        .map(|&((x1, y1), (x2, y2))| [point2::<_, UnknownUnit>(x1, y1), point2::<_, UnknownUnit>(x2, y2)])
        .collect::<Vec<_>>()
}

#[test]
fn circles_99_prune() {
    let data = process(CIRCLES_99_DATA);

    let input_count = data.len();
    let out: Vec<_> = prune(data.iter().map(Clone::clone), EPSILON, true).into_iter().collect();
    assert_eq!(out.len(), input_count);
}

#[test]
fn circles_99_connect() {
    let data = process(CIRCLES_99_DATA);
    let out = connect_obvious(data, EPSILON, true);
    for segment in &out {
        assert!(segment.closed);
    }

    // Note: this is actually correct, 99 circles actually has 97 circles
    assert_eq!(out.len(), 97);
}

#[test]
fn one_circle_prune() {
    let data = process(ONE_CIRCLE_DATA);
    let input_count = data.len();
    let out: Vec<_> = prune(data, EPSILON, true).into_iter().collect();
    assert_eq!(out.len(), input_count);
}

#[test]
fn one_circle_connect() {
    let data = process(ONE_CIRCLE_DATA);
    let out = connect_obvious(data, EPSILON, true);
    assert_eq!(out.len(), 1);
    assert!(out[0].closed);
}
