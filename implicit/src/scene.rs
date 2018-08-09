use super::nodes::NodeRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineMode {
    Solid,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DrawMode {
    Filled,
    Line(LineMode),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Shape {
    pub color: (u8, u8, u8),
    pub draw_mode: DrawMode,
    pub implicit: NodeRef,
}

impl ::std::cmp::Eq for Shape {}
impl ::std::cmp::Ord for Shape {
    fn cmp(&self, other: &Shape) -> ::std::cmp::Ordering {
        self.partial_cmp(other)
            .unwrap_or(::std::cmp::Ordering::Less)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Figure {
    pub shapes: Vec<Shape>,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Scene {
    pub unit: String,
    pub simplify: bool,

    pub figures: Vec<Figure>,
}
