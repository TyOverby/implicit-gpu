use ::*;
use euclid::{Point2D, UnknownUnit, point2};

const EPSILON: f32 = 0.001;
const CIRCLES_99_DATA: &'static [((f32, f32), (f32, f32))] = include!("./regressions/99_circles.txt");
const ONE_CIRCLE_DATA: &'static [((f32, f32), (f32, f32))] = include!("./regressions/one_circle.txt");

fn process(input: &[((f32, f32), (f32, f32))]) -> Vec<[Point2D<f32>; 2]> {
    input
        .iter()
        .map(|&((x1, y1), (x2, y2))| {
            [
                point2::<_, UnknownUnit>(x1, y1),
                point2::<_, UnknownUnit>(x2, y2),
            ]
        })
        .collect::<Vec<_>>()
}

#[test]
fn circles_99_prune() {
    let data = process(CIRCLES_99_DATA);

    let input_count = data.len();
    let out = prune(data.iter().map(Clone::clone), EPSILON, true);
    assert_eq!(out.len(), input_count);
}

#[test]
fn circles_99_connect() {
    let data = process(CIRCLES_99_DATA);

    let out = connect_obvious(data, EPSILON, true, false);
    let mut ok = true;
    for segment in &out {
        println!("{:?}", segment.path);
        ok = false;
    }
    assert!(ok);
    assert_eq!(out.len(), 99);
}

#[test]
fn one_circle() {
    let data = process(ONE_CIRCLE_DATA);
    let input_count = data.len();
    let out = prune(data, EPSILON, true);
    assert_eq!(out.len(), input_count);
}
