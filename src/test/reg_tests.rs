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
