#[derive(Clone)]
pub struct OutputScene {
    pub figures: Vec<OutputFigure>,
}

#[derive(Clone)]
pub struct OutputFigure {
    pub shapes: Vec<OutputShape>,
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
        additive: Vec<Vec<(f32, f32)>>,
        subtractive: Vec<Vec<(f32, f32)>>,
    },
    Lines(Vec<Vec<(f32, f32)>>),
}
