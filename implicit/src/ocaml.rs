use euclid::{Point2D, Transform2D};

pub type Id = u32;
pub type Matrix = Transform2D<f32>;
pub type Point = Point2D<f32>;

#[derive(Deserialize, Debug)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,

    #[serde(default = "Transform2D::identity")]
    pub mat: Matrix,
}

#[derive(Deserialize, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    #[serde(default = "Transform2D::identity")]
    pub mat: Matrix,
}

#[derive(Deserialize, Debug)]
pub struct Polygon {
    pub points: Vec<Point>,

    #[serde(default = "Transform2D::identity")]
    pub mat: Matrix,
}

#[derive(Deserialize, Debug)]
pub enum Terminal{
    Circle(Circle),
    Rect(Rect),
    Field(Id),
}

#[derive(Deserialize, Debug)]
pub enum Shape {
    Terminal(Terminal),
    Not(Box<Shape>),
    Union(Vec<Shape>),
    Intersection(Vec<Shape>),
    Modulate(Box<Shape>, f32),
}

#[derive(Deserialize, Debug)]
pub enum Value {
    BasicShape(Shape),
    Polygon(Polygon),
}

#[derive(Deserialize, Debug)]
pub enum Command {
    Concurrently(Vec<Command>),
    Serially(Vec<Command>),
    Define(Id, Value),
    Freeze { target: Id, id: Id },
    Export(Id),
}
