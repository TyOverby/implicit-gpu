use geometry::{Point, compare_points};
use euclid::point2;

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
            match compare_points(lhs[i], rhs[i]){
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
    where
        T: IntoIterator<Item = (f32, f32)>,
    {
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
