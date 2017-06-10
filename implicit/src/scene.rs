use super::nodes::NodeRef;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum LineMode {
    Solid,
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum DrawMode {
    Filled,
    Line(LineMode),
}

#[derive(Debug, PartialEq, PartialOrd )]
pub struct Shape {
    pub color: (u8, u8, u8),
    pub draw_mode: DrawMode,
    pub node: NodeRef,
}


impl ::std::cmp::Eq for Shape { }
impl ::std::cmp::Ord for Shape {
    fn cmp(&self, other: &Shape) -> ::std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(::std::cmp::Ordering::Less)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Figure {
    pub shapes: Vec<Shape>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Scene {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,

    pub unit: String,
    pub simplify: bool,

    pub figures: Vec<Figure>,
}
