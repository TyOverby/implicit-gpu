#[derive(Debug, PartialEq, Clone, PartialOrd, Deserialize, Serialize)]
pub struct Polygon {
    pub points: Vec<(f32, f32)>,
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Deserialize, Serialize)]
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
        let mut out = vec![];

        let first = if let Some(first) = iterator.next() {
            first
        } else {
            return Polygon { points: vec![] };
        };

        out.push(first);

        for pt in iterator {
            out.push(pt);
            out.push(pt);
        }

        out.push(first);

        Polygon { points: out }
    }
}

impl PolyGroup {
    pub fn single_additive(points: Vec<(f32, f32)>) -> PolyGroup {
        PolyGroup {
            additive: vec![Polygon { points }],
            subtractive: vec![],
        }
    }
}
