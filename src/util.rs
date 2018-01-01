use euclid::TypedPoint2D;

pub fn is_clockwise<K>(pts: &[TypedPoint2D<f32, K>]) -> bool {
    assert!(pts.len() > 0);
    let mut total = 0.0f32;
    for slice in pts.windows(2) {
        let a = slice[0];
        let b = slice[1];
        total += (b.x - a.x) * (b.y + a.y);
    }
    {
        let a = pts[0];
        let b = pts[pts.len() - 1];
        total += (b.x - a.x) * (b.y + a.y);
    }
    total > 0.0
}
