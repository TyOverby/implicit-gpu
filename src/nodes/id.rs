use super::*;
use compiler::{CompilationContext, Stage};


lazy_static! {
    static ref ID_POOL: AtomicUsize = AtomicUsize::new(0);
}

pub struct Id<T: Node> {
    id: usize,
    op: T
}

unsafe impl <T: Node> Trace for Id<T> {
    custom_trace!(this, {
        mark(&this.op)
    });
}

impl <T: Node> Id<T> {
    pub fn new(inner: T) -> Id<T> {
        Id {
            id: ID_POOL.fetch_add(1, Ordering::SeqCst),
            op: inner,
        }
    }
}

impl <T: Node> Node for Id<T> {
    fn id(&self) -> usize { self.id }
    fn compile(&self, cc: &mut CompilationContext) -> (Stage, InputInfo) {
        self.op.compile(cc)
    }
    fn is_break(&self) -> bool {
        self.op.is_break()
    }
}
