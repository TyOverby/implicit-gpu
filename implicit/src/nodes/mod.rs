mod poly;

pub use self::poly::*;
use compiler::GroupId;
use std::mem::transmute;
use typed_arena::Arena;

// IF YOU ADD AN ENUM HERE, UPDATE `eq_ignore_group`
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Node<'a> {
    Circle { x: f32, y: f32, r: f32 },
    Rect { x: f32, y: f32, w: f32, h: f32 },
    And(Vec<&'a Node<'a>>),
    Or(Vec<&'a Node<'a>>),
    Not(&'a Node<'a>),
    Polygon(PolyGroup),
    Modulate(f32, &'a Node<'a>),
    Break(&'a Node<'a>),
    OtherGroup(GroupId),
    Freeze(&'a Node<'a>),
}

pub struct StaticNode {
    _arena: Arena<Node<'static>>,
    node: &'static Node<'static>,
}

impl<'a> Node<'a> {
    pub fn eq_ignore_group<'o>(&self, other: &'o Node<'o>) -> bool {
        match (self, other) {
            (&Node::Circle { x: mx, y: my, r: mr }, &Node::Circle { x: ox, y: oy, r: or }) => mx == ox && my == oy && mr == or,
            (&Node::And(ref mch), &Node::And(ref och)) => mch.iter().zip(och.iter()).all(|(&a, &b)| a.eq_ignore_group(b)),
            (&Node::Or(ref mch), &Node::Or(ref och)) => mch.iter().zip(och.iter()).all(|(&a, &b)| a.eq_ignore_group(b)),
            (&Node::Not(mc), &Node::Not(oc)) => mc.eq_ignore_group(oc),
            (&Node::Polygon(ref mpg), &Node::Polygon(ref opg)) => mpg == opg,
            (&Node::Modulate(mhm, mc), &Node::Modulate(ohm, oc)) => mhm == ohm && mc.eq_ignore_group(oc),
            (&Node::Break(mc), &Node::Break(oc)) => mc.eq_ignore_group(oc),
            (&Node::Freeze(mc), &Node::Freeze(oc)) => mc.eq_ignore_group(oc),
            (&Node::OtherGroup(_), &Node::OtherGroup(_)) => true,
            (_, _) => false,
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

    pub fn node<'a>(&'a self) -> &'a Node<'a> { &self.node }
}

impl ::std::fmt::Debug for StaticNode {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> { self.node.fmt(formatter) }
}

impl ::std::cmp::PartialOrd for StaticNode {
    fn partial_cmp(&self, other: &StaticNode) -> Option<::std::cmp::Ordering> {
        self.node.partial_cmp(other.node)
    }
}
impl ::std::cmp::Eq for StaticNode { }
impl ::std::cmp::PartialEq for StaticNode {
    fn eq(&self, other: &StaticNode) -> bool { self.node == other.node }
}

impl Clone for StaticNode {
    fn clone(&self) -> StaticNode {
        fn clone_node<'i, 'o, F>(input: &'i Node<'i>, a: &F) -> &'o Node<'o> where F: Fn(Node<'o>) -> &'o Node<'o> {
            match input {
                &Node::Circle { x, y, r } => a(Node::Circle { x, y, r }),
                &Node::Rect { x, y, w, h } => a(Node::Rect { x, y, w, h }),
                &Node::And(ref ch) => a(Node::And(ch.iter().map(|c| clone_node(c, a)).collect())),
                &Node::Or(ref ch) => a(Node::Or(ch.iter().map(|c| clone_node(c, a)).collect())),
                &Node::Not(c) => a(Node::Not(clone_node(c, a))),
                &Node::Polygon(ref pg) => a(Node::Polygon(pg.clone())),
                &Node::Modulate(how_much, c) => a(Node::Modulate(how_much, clone_node(c, a))),
                &Node::Break(c) => a(Node::Break(clone_node(c, a))),
                &Node::Freeze(c) => a(Node::Freeze(clone_node(c, a))),
                &Node::OtherGroup(gid) => a(Node::OtherGroup(gid.clone())),
            }
        }

        create_node!(
            a, {
                clone_node(self.node(), &a)
            }
        )
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
