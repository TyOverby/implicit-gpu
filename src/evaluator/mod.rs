use std::collections::HashMap;
use std::sync::Mutex;

use ::opencl::OpenClContext;
use ::compiler::*;
use ::opencl::FieldBuffer;


#[derive(Debug)]
pub struct Evaluator {
    finished: Mutex<HashMap<GroupId, FieldBuffer>>,
    width: usize,
    height: usize,
    nest: Nest,
}


impl Evaluator {
    pub fn new(nest: Nest, width: usize, height: usize, _prev: Option<Evaluator>) -> Evaluator {
        Evaluator {
            finished: Mutex::new(HashMap::new()),
            width: width,
            height: height,
            nest: nest,
        }
    }

    pub fn evaluate(&self, which: GroupId, ctx: &OpenClContext) -> FieldBuffer {
        let _guard = ::flame::start_guard(format!("evaluate {:?}", which));
        {
            let finished = self.finished.lock().unwrap();
            if let Some(buff) = finished.get(&which) {
                return buff.clone();
            }
        }

        let group = self.nest.get(which);
        match group {
            &NodeGroup::Basic(ref root) => {
                let (program, compilation) = ::compiler::compile(root.node());
                let deps: Vec<FieldBuffer> = compilation.deps().iter().map(|&g| self.evaluate(g, ctx)).collect();

                let out = ctx.field_buffer(self.width, self.height, None);
                let kernel = ctx.compile("apply", program);

                let mut kc = kernel.gws([self.width, self.height])
                      .arg_buf(out.buffer())
                      .arg_scl(self.width);

                for dep in &deps {
                    kc = kc.arg_buf(dep.buffer());
                }

                ::flame::span_of("eval", || kc.enq().unwrap());

                out
            }
            &NodeGroup::Polygon(ref _poly) => {
                unimplemented!()
            }
        }
    }
}
