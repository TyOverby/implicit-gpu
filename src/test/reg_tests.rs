use ::*;
use euclid::{UnknownUnit, point2};

const EPSILON: f32 = 0.001;

#[test]
fn circles_99() {
    let data = include!("./regressions/99_circles.txt");
    let data = data.iter().map(|&((x1, y1), (x2, y2))| {
        vec![
            point2::<_, UnknownUnit>(x1, y1),
            point2::<_, UnknownUnit>(x2, y2),
        ]
    });

    let out = connect_obvious(data, EPSILON, true, false);
    assert_eq!(out.len(), 99);
    for segment in out {
        assert!(segment.closed);
    }
}

#[test]
fn one_circle() {
    let data = include!("./regressions/one_circle.txt");
    let data = data.iter()
                   .map(|&(p1, p2)| [point2::<_, UnknownUnit>(p1.0, p1.1), point2::<_, UnknownUnit>(p2.0, p2.1)])
                   .collect::<Vec<_>>();
    let input_count = data.len();
    let out = prune(data, EPSILON, true);

    assert_eq!(out.len(), input_count);
}
