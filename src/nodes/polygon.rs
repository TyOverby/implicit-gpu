use super::*;
use compiler::{CompilationContext, Stage};

#[derive(PartialEq, Debug)]
pub struct Polygon {
    pub points: Vec<f32>
}


impl NodeTrait for Polygon {
    fn compile(&self, _: &mut CompilationContext) -> (Stage, InputInfo) {
        unimplemented!();
    }

    fn is_break(&self) -> bool { true }
}

unsafe impl Trace for Polygon {
    unsafe_empty_trace!();
}
