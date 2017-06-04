#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Polygon {
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct PolyGroup {
    pub additive: Vec<Polygon>,
    pub subtractive: Vec<Polygon>,
}

impl ::std::iter::FromIterator<(f32, f32)> for Polygon {
    fn from_iter<T>(iterator: T) -> Polygon where T: IntoIterator<Item = (f32, f32)> {
        let mut iterator = iterator.into_iter();
        let mut xs = vec![];
        let mut ys = vec![];

        let (fx, fy) = if let Some(first) = iterator.next() {
            first
        } else {
            return Polygon { xs: xs, ys: ys };
        };

        xs.push(fx);
        ys.push(fy);

        for (x, y) in iterator {
            xs.push(x);
            xs.push(x);

            ys.push(y);
            ys.push(y);
        }

        xs.push(fx);
        ys.push(fy);

        Polygon { xs: xs, ys: ys }
    }
}

impl PolyGroup {
    pub fn single_additive(xs: Vec<f32>, ys: Vec<f32>) -> PolyGroup {
        PolyGroup {
            additive: vec![Polygon { xs: xs, ys: ys }],
            subtractive: vec![],
        }
    }
}
