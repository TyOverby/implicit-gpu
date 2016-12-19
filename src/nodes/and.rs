use super::*;
use compiler::{CompilationContext, Stage};

#[derive(PartialEq, Debug, Clone)]
pub struct And {
    pub left: NodePtr,
    pub right: NodePtr,
}

impl NodeTrait for And {
    fn compile(&self, cc: &mut CompilationContext) -> (Stage, InputInfo) {
        let (left_stage, _) = self.left.compile(cc);
        let (right_stage, _) = self.right.compile(cc);

        let res = cc.get_id("and");

        let mut stage = Stage::new(res.clone());
        stage.add_line(format!("{result} = max({a}, {b});", result = res, a = left_stage.result, b = right_stage.result));

        cc.add_stage(left_stage);
        cc.add_stage(right_stage);

        (stage, InputInfo)
    }
}

unsafe impl Trace for And {
    custom_trace!(this, {
        mark(&this.left);
        mark(&this.right);
    });
}

