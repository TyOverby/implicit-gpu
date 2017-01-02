use typed_arena::Arena;
use std::mem::transmute;
use ::compiler::GroupId;

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Circle { x: f32, y: f32, r: f32 },
    And(Vec<&'a Node<'a>>),
    Or(Vec<&'a Node<'a>>),
    Not(&'a Node<'a>),
    Polygon(PolyGroup),
    Modulate(f32, &'a Node<'a>),
    Break(&'a Node<'a>),
    OtherGroup(GroupId),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Polygon {
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PolyGroup {
    pub additive: Vec<Polygon>,
    pub subtractive: Vec<Polygon>,
}

pub struct StaticNode {
    _arena: Arena<Node<'static>>,
    node: &'static Node<'static>,
}

impl ::std::iter::FromIterator<(f32, f32)> for Polygon {
    fn from_iter<T>(iterator: T) -> Polygon
    where T: IntoIterator<Item=(f32, f32)> {
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

        xs.reverse();
        ys.reverse();


        Polygon {xs: xs, ys: ys}
    }
}

impl PolyGroup {
    pub fn single_additive(xs: Vec<f32>, ys: Vec<f32>) -> PolyGroup {
        PolyGroup {
            additive: vec![ Polygon{ xs: xs, ys: ys } ],
            subtractive: vec![],
        }
    }
}


impl StaticNode {
    pub unsafe fn new<'a>(arena: Arena<Node<'a>>, node: &'static Node<'static>) -> StaticNode {
        StaticNode {
            _arena: transmute(arena),
            node: transmute(node),
        }
    }

    pub fn node<'a>(&'a self) -> &'a Node<'a> {
        &self.node
    }
}

impl ::std::fmt::Debug for StaticNode {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        self.node.fmt(formatter)
    }
}

impl ::std::cmp::PartialEq for StaticNode {
    fn eq(&self, other: &StaticNode) -> bool {
        self.node == other.node
    }
}

#[doc(hidden)]
pub struct Anchor <'a, T: 'a> {
    _p: ::std::marker::PhantomData<&'a T>,
}

impl <'a, T: 'a > Anchor<'a, T> {
    pub fn new() -> Anchor<'a, T> {
        Anchor {
            _p: ::std::marker::PhantomData,
        }
    }

    pub fn hold<'b: 'a>(&'a self, obj: &'b T) -> &'a T {
        obj
    }
}

