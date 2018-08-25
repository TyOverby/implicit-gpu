use euclid::{Point2D, Transform2D};
use serde::Deserializer;

pub type Id = u32;
pub type Matrix = Transform2D<f32>;
pub type Point = Point2D<f32>;

#[derive(Deserialize)]
struct PointDef {
    x: f32,
    y: f32,
}

#[derive(Deserialize)]
#[serde(remote = "Matrix")]
struct MatrixDef {
    #[serde(getter = "garbage")]
    m11: f32,
    m12: f32,
    m21: f32,
    m22: f32,
    m31: f32,
    m32: f32,
}

// Provide a conversion to construct the remote type.
impl From<PointDef> for Point {
    fn from(def: PointDef) -> Point {
        ::euclid::point2(def.x, def.y)
    }
}

impl From<MatrixDef> for Matrix {
    fn from(def: MatrixDef) -> Matrix {
        Transform2D::row_major(def.m11, def.m12, def.m21, def.m22, def.m31, def.m32)
    }
}

#[derive(Deserialize, Debug)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,

    #[serde(default = "Transform2D::identity")]
    #[serde(with = "MatrixDef")]
    pub mat: Matrix,
}

#[derive(Deserialize, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    #[serde(default = "Transform2D::identity")]
    #[serde(with = "MatrixDef")]
    pub mat: Matrix,
}

#[derive(Deserialize, Debug)]
pub struct Polygon {
    #[serde(deserialize_with = "transcode_point")]
    pub points: Vec<Point>,

    #[serde(default = "Transform2D::identity")]
    #[serde(with = "MatrixDef")]
    pub mat: Matrix,
}

#[derive(Deserialize, Debug)]
pub enum Terminal {
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

#[derive(Deserialize, Debug)]
pub struct Bbox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Deserialize, Debug)]
pub enum Bounding {
    Everything,
    Nothing,
    Positive(Bbox),
    Negative(Bbox),
}

fn transcode_point<'de, D>(deserializer: D) -> Result<Vec<Point>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{SeqAccess, Visitor};
    use std::fmt;
    use std::marker::PhantomData;

    struct PointVisitor(PhantomData<fn() -> Vec<Point>>);
    impl<'de> Visitor<'de> for PointVisitor {
        type Value = Vec<Point>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a list of points")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Vec<Point>, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut out = vec![];
            while let Some(value) = seq.next_element()? {
                let v: PointDef = value;
                out.push(v.into());
            }

            Ok(out)
        }
    }

    let visitor = PointVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}
