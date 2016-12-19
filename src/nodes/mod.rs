use gc::{Gc, Trace};
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};

use compiler::{CompilationContext, Stage};

mod circle;
mod not;
mod and;
mod polygon;

pub use circle::Circle;
pub use not::Not;
pub use and::And;
pub use polygon::Polygon;

lazy_static! {
    static ref ID_POOL: AtomicUsize = AtomicUsize::new(0);
}

#[derive(PartialEq, Debug, Clone)]
pub struct NodePtr(Gc<Node>);

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct NodeId(usize);

#[derive(PartialEq, Debug)]
pub enum Node {
    And(NodeId, and::And),
    Circle(NodeId, circle::Circle),
    Not(NodeId, not::Not),
    Poly(NodeId, polygon::Polygon),
}

impl Node {
    pub fn id(&self) -> NodeId {
        match self {
            &Node::And(id, _) => id,
            &Node::Circle(id, _) => id,
            &Node::Not(id, _) => id,
            &Node::Poly(id, _) => id,
        }
    }
}

impl Deref for NodePtr {
    type Target = Gc<Node>;

    fn deref(&self) -> &Gc<Node> {
        &self.0
    }
}

unsafe impl Trace for NodePtr {
    custom_trace!(this, {
        mark(&this.0)
    });
}

impl NodePtr {
    pub fn and(&self, other: &NodePtr) -> NodePtr {
        NodePtr(Gc::new(Node::And(NodeId::new(), and::And {
            left: self.clone(),
            right: other.clone(),
        })))
    }

    pub fn invert(&self) -> NodePtr {
        NodePtr(Gc::new(Node::Not(NodeId::new(), not::Not {
            inner: self.clone()
        })))
    }
}

impl NodeTrait for Node {
    fn compile(&self, cc: &mut CompilationContext) -> (Stage, InputInfo) {
        match self {
            &Node::And(_, ref a) => a.compile(cc),
            &Node::Circle(_, ref a) => a.compile(cc),
            &Node::Not(_, ref a) => a.compile(cc),
            &Node::Poly(_, ref a) => a.compile(cc),
        }
    }

    fn is_break(&self) -> bool {
        match self {
            &Node::And(_, ref a) => a.is_break(),
            &Node::Circle(_, ref a) => a.is_break(),
            &Node::Not(_, ref a) => a.is_break(),
            &Node::Poly(_, ref a) => a.is_break(),
        }
    }
}

unsafe impl Trace for Node {
    custom_trace!(this, {
        match this {
            &Node::And(_, ref a) => mark(a),
            &Node::Circle(_, ref a) => mark(a),
            &Node::Not(_, ref a) => mark(a),
            &Node::Poly(_, ref a) => mark(a),
        }
    });
}

impl NodeId {
    pub fn new() -> NodeId {
         NodeId(ID_POOL.fetch_add(1, Ordering::SeqCst))
    }
}

pub struct InputInfo;

pub trait NodeTrait: Trace {
    fn compile(&self, &mut CompilationContext) -> (Stage, InputInfo);
    fn is_break(&self) -> bool { false }
}

pub fn circle(x: f32, y: f32, r: f32) -> NodePtr {
    NodePtr(Gc::new(Node::Circle(NodeId::new(), circle::Circle{position: (x, y), radius: r})))
}
