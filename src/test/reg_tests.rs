use ::*;

const EPSILON: f32 = 0.001;

#[test]
fn circles_99() {
    let data = include!("./regressions/99_circles.txt");
    let data = data.iter().map(|&((x1, y1), (x2, y2))| {
        vec![Point { x: x1, y: y1 }, Point { x: x2, y: y2 }]
    });

    let out = optimize(data, EPSILON, true, false);
    assert_eq!(out.len(), 99);
    for segment in out {
        assert!(segment.closed);
    }
}
