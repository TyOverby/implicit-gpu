use super::*;
use compiler::{CompilationContext, Stage};

pub struct Not {
    pub inner: NodePtr
}


impl Node for Not {
    fn compile(&self, _cc: &mut CompilationContext) -> (Stage, InputInfo) {
        unimplemented!();
    }
}

unsafe impl Trace for Not {
    custom_trace!(this, {
        mark(&this.inner)
    });
}

