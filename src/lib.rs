#![feature(pub_restricted)]

#[macro_use]
extern crate gc;
#[macro_use]
extern crate lazy_static;

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
pub mod image;
pub mod polygon;
pub mod marching;
pub mod evaluator;

