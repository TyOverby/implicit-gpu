pub fn is_clockwise<F: Copy + Into<f64>>(pts: &[(F, F)]) -> bool {
    assert!(pts.len() > 0);
    let mut total = 0.0;
    for slice in pts.windows(2) {
        let a = slice[0];
        let b = slice[1];
        total += ((b.0).into() - (a.0).into()) * ((b.1).into() + (a.1).into());
    }
    {
        let a = pts[0];
        let b = pts[pts.len() - 1];
        total += ((b.0).into() - (a.0).into()) * ((b.1).into() + (a.1).into());
    }
    total > 0.0
}
