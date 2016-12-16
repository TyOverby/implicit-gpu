use gc::{Gc, Trace};
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};

use compiler::{CompilationContext, Stage};

mod id;
mod circle;
mod not;
mod and;

#[derive(Clone)]
pub struct NodePtr(Gc<Node>);

pub struct InputInfo;

pub trait Node: Trace {
    fn id(&self) -> usize {
        panic!("wrap all ops with a IdOp");
    }

    fn compile(&self, &mut CompilationContext) -> (Stage, InputInfo);

    fn is_break(&self) -> bool { false }
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
        and(self, other)
    }

    pub fn invert(&self) -> NodePtr {
        not(self)
    }
}

pub fn construct<T: Node + 'static>(v: T) -> NodePtr {
    NodePtr(Gc::new(id::Id::new(v)))
}


pub fn circle(x: f32, y: f32, r: f32) -> NodePtr {
    let circle = circle::Circle{
        position: (x, y),
        radius: r
    };

    construct(circle)
}

fn not(inner: &NodePtr) -> NodePtr {
    construct(not::Not {inner: inner.clone()})
}

fn and(left: &NodePtr, right: &NodePtr) -> NodePtr {
    construct(and::And {
        left: left.clone(),
        right: right.clone(),
    })
}
