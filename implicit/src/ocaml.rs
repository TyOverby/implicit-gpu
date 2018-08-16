use euclid::Transform2D;

pub type Id = u32;
pub type Matrix = Transform2D<f32>;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,

    #[serde(default = "Transform2D::identity")]
    pub mat: Matrix,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    #[serde(default = "Transform2D::identity")]
    pub mat: Matrix,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Polygon {
    pub points: Vec<Point>,

    #[serde(default = "Transform2D::identity")]
    pub mat: Matrix,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub enum BasicTerminals {
    Circle(Circle),
    Rect(Rect),
    Field(Id),
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub enum Shape {
    Terminal(BasicTerminals),
    Not(Box<Shape>),
    Union(Vec<Shape>),
    Intersection(Vec<Shape>),
    Modulate(Box<Shape>, f32),
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub enum Value {
    BasicShape(Shape),
    Polygon(Polygon),
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub enum Command {
    Concurrently(Vec<Command>),
    Serially(Vec<Command>),
    Define(Id, Value),
    Freeze { target: Id, id: Id },
    Export(Id),
}
