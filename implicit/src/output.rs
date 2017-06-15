pub struct OutputScene {
    pub figures: Vec<OutputFigure>,
}

pub struct OutputFigure {
    pub shapes: Vec<OutputShape>,
}

pub struct OutputShape {
    pub color: (u8, u8, u8),
    pub lines: LineGroup,
}

pub enum LineGroup {
    Polygon {
        filled: bool,
        additive: Vec<Vec<(f32, f32)>>,
        subtractive: Vec<Vec<(f32, f32)>>,
    },
    Lines(Vec<Vec<(f32, f32)>>),
}
