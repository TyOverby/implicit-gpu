use gc::{Gc, Trace};
use std::sync::atomic::{AtomicUsize, Ordering};

use compiler::{CompilationContext, Stage};

mod id;
mod circle;
mod not;
mod and;

pub type NodePtr = Gc<Node>;

pub struct Compilation;
pub struct InputInfo;

pub trait Node: Trace {
    fn id(&self) -> usize {
        panic!("wrap all ops with a IdOp");
    }

    fn compile(&self, &mut CompilationContext) -> (Stage, InputInfo);

    fn is_break(&self) -> bool { false }
}

fn id<T: Node + 'static>(v: T) -> NodePtr {
    Gc::new(id::Id::new(v))
}

pub fn circle(x: f32, y: f32, r: f32) -> NodePtr {
    let circle = circle::Circle{
        position: (x, y),
        radius: r
    };

    id(circle)
}

pub fn not(inner: &NodePtr) -> NodePtr {
    id(not::Not {inner: inner.clone()})
}

pub fn and(left: &NodePtr, right: &NodePtr) -> NodePtr {
    id(and::And {
        left: left.clone(),
        right: right.clone(),
    })
}
