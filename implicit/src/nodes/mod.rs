mod poly;

use std::sync::Arc;
pub use self::poly::*;
use compiler::GroupId;

// IF YOU ADD AN ENUM HERE, UPDATE `eq_ignore_group`
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Node {
    Circle { x: f32, y: f32, r: f32 },
    Rect { x: f32, y: f32, w: f32, h: f32 },
    And(Vec<Arc<Node>>),
    Or(Vec<Arc<Node>>),
    Not(Arc<Node>),
    Polygon(PolyGroup),
    Modulate(f32, Arc<Node>),
    Break(Arc<Node>),
    OtherGroup(GroupId),
    Freeze(Arc<Node>),
}

pub fn take_node(node: Arc<Node>) -> Node {
    match Arc::try_unwrap(node) {
        Ok(t) => t,
        Err(n) => (*n).clone()
    }
}

impl Node {
    pub fn eq_ignore_group(&self, other: &Node) -> bool {
        match (self, other) {
            (&Node::Circle { x: mx, y: my, r: mr }, &Node::Circle { x: ox, y: oy, r: or }) =>
                mx == ox && my == oy && mr == or,
            (&Node::And(ref mch), &Node::And(ref och)) =>
                mch.iter().zip(och.iter()).all(|(a, b)| a.eq_ignore_group(&*b)),
            (&Node::Or(ref mch), &Node::Or(ref och)) =>
                mch.iter().zip(och.iter()).all(|(a, b)| a.eq_ignore_group(&*b)),
            (&Node::Not(ref mc), &Node::Not(ref oc)) => mc.eq_ignore_group(&*oc),
            (&Node::Polygon(ref mpg), &Node::Polygon(ref opg)) => mpg == opg,
            (&Node::Modulate(ref mhm, ref mc), &Node::Modulate(ref ohm, ref oc))
                => mhm == ohm && mc.eq_ignore_group(&*oc),
            (&Node::Break(ref mc), &Node::Break(ref oc)) => mc.eq_ignore_group(&*oc),
            (&Node::Freeze(ref mc), &Node::Freeze(ref oc)) => mc.eq_ignore_group(&*oc),
            (&Node::OtherGroup(_), &Node::OtherGroup(_)) => true,
            (_, _) => false,
        }
    }
}

#[doc(hidden)]
pub struct Anchor<'a, T: 'a> {
    _p: ::std::marker::PhantomData<&'a T>,
}

impl<'a, T: 'a> Anchor<'a, T> {
    pub fn new() -> Anchor<'a, T> { Anchor { _p: ::std::marker::PhantomData } }

    pub fn hold<'b: 'a>(&'a self, obj: &'b T) -> &'a T { obj }
}
