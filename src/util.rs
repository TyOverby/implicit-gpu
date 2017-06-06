pub fn is_clockwise(pts: &[(f64, f64)]) -> bool {
    assert!(pts.len() > 0);
    let mut total = 0.0;
    for slice in pts.windows(2) {
        let a = slice[0];
        let b = slice[1];
        total += (b.0 - a.0) * (b.1 + a.1);
    }
    {
        let a = pts[0];
        let b = pts[pts.len() - 1];
        total += (b.0 - a.0) * (b.1 + a.1);
    }
    total > 0.0
}
