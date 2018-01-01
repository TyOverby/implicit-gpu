use euclid::point2;
use geometry::{compare_points, point_in_poly, PathSegment, Point};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Polygon {
    pub points: Vec<Point>,
}

impl ::std::cmp::PartialOrd for Polygon {
    fn partial_cmp(&self, other: &Polygon) -> Option<::std::cmp::Ordering> {
        let l = ::std::cmp::min(self.points.len(), other.points.len());
        let lhs = &self.points[..l];
        let rhs = &other.points[..l];

        for i in 0..l {
            match compare_points(lhs[i], rhs[i]) {
                Some(::std::cmp::Ordering::Equal) => (),
                non_eq => return non_eq,
            }
        }

        self.points.len().partial_cmp(&other.points.len())
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
pub struct PolyGroup {
    pub additive: Vec<Polygon>,
    pub subtractive: Vec<Polygon>,
}

impl ::std::iter::FromIterator<(f32, f32)> for Polygon {
    fn from_iter<T>(iterator: T) -> Polygon
    where T: IntoIterator<Item = (f32, f32)> {
        let mut iterator = iterator.into_iter();
        let mut out: Vec<Point> = vec![];

        let first = if let Some(first) = iterator.next() {
            point2(first.0, first.1)
        } else {
            return Polygon { points: vec![] };
        };

        out.push(first);

        for (x, y) in iterator {
            let pt = point2(x, y);
            out.push(pt);
            out.push(pt);
        }

        out.push(first);

        Polygon { points: out }
    }
}

impl PolyGroup {
    pub fn single_additive(points: Vec<Point>) -> PolyGroup {
        PolyGroup {
            additive: vec![Polygon { points }],
            subtractive: vec![],
        }
    }
}

pub fn separate_polygons(bag: Vec<PathSegment>) -> (Vec<PathSegment>, Vec<PathSegment>) {
    let _guard = ::flame::start_guard("separate_polygons");

    fn contains(a: &[Point], b: &[Point]) -> bool { point_in_poly(a, b[0]) }

    let mut additive_or_subtractive = vec![];
    for (i, a) in bag.iter().enumerate() {
        let mut inside_count = 0;
        for (j, b) in bag.iter().enumerate() {
            if i == j {
                continue;
            }
            if contains(&b.path, &a.path) {
                inside_count += 1;
            }
        }

        additive_or_subtractive.push(inside_count % 2 == 0);
    }

    let (additive, subtractive): (Vec<_>, Vec<_>) = bag.into_iter()
        .zip(additive_or_subtractive.into_iter())
        .partition(|&(_, i)| i);
    let additive = additive.into_iter().map(|(b, _)| b).collect();
    let subtractive = subtractive.into_iter().map(|(b, _)| b).collect();

    (additive, subtractive)
}
