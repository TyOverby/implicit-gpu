use geometry::Point;

#[derive(Clone)]
pub struct OutputScene {
    pub figures: Vec<OutputFigure>,
}

#[derive(Clone)]
pub struct OutputFigure {
    pub shapes: Vec<OutputShape>,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone)]
pub struct OutputShape {
    pub color: (u8, u8, u8),
    pub lines: LineGroup,
}

#[derive(Clone)]
pub enum LineGroup {
    Polygon {
        filled: bool,
        additive: Vec<Vec<Point>>,
        subtractive: Vec<Vec<Point>>,
    },
    Lines(Vec<Vec<Point>>),
}
