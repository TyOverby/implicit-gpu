#![feature(pub_restricted)]

#[macro_use]
extern crate lazy_static;
extern crate latin;
extern crate typed_arena;
extern crate vecmath;
extern crate ocl;
extern crate flame;
extern crate fnv;
extern crate itertools;
extern crate image as image_crate;

#[macro_export]
macro_rules! create_node {
    ($alloc: ident, $code: block) => {
        {
            let anchor = $crate::nodes::Anchor::new();
            let arena = ::typed_arena::Arena::new();
            let result: &'static $crate::nodes::Node<'static> = {
                let $alloc = |a| {
                    let r: &'static $crate::nodes::Node<'static> = unsafe {
                        ::std::mem::transmute(arena.alloc(a))
                    };

                    anchor.hold(r)
                };

                let result = $code;

                unsafe { ::std::mem::transmute(result) }
            };

            unsafe {
                $crate::nodes::StaticNode::new(arena, result)
            }
        }
    };
}

pub mod nodes;
pub mod compiler;
pub mod opencl;
pub mod debug;
pub mod polygon;
pub mod marching;
pub mod evaluator;

pub fn run_single(node: &nodes::Node, width: usize, height: usize) -> ::opencl::FieldBuffer {
    use compiler::Nest;
    use evaluator::Evaluator;

    let ctx = opencl::OpenClContext::default();

    let mut nest = Nest::new();
    let target = nest.group(node);

    // Create a new Execution Context
    let evaluator = Evaluator::new(nest, width, height, None);
    let result = evaluator.evaluate(target, &ctx);
    result
}
