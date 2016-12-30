use typed_arena::Arena;
use std::mem::transmute;

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Circle { x: f32, y: f32, r: f32 },
    And(Vec<&'a Node<'a>>),
    Or(Vec<&'a Node<'a>>),
    Not(&'a Node<'a>),
    Polygon(Vec<f32>, Vec<f32>),
    Modulate(f32, &'a Node<'a>),
}

pub struct StaticNode {
    _arena: Arena<Node<'static>>,
    node: &'static Node<'static>,
}

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

impl StaticNode {
    pub unsafe fn new<'a>(arena: Arena<Node<'a>>, node: &'static Node<'static>) -> StaticNode {
        StaticNode {
            _arena: transmute(arena),
            node: transmute(node),
        }
    }

    pub fn node<'a>(&'a self) -> &'a Node<'a> {
        &self.node
    }
}

impl ::std::fmt::Debug for StaticNode {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        self.node.fmt(formatter)
    }
}

#[doc(hidden)]
pub struct Anchor <'a, T: 'a> {
    _p: ::std::marker::PhantomData<&'a T>,
}

impl <'a, T: 'a > Anchor<'a, T> {
    pub fn new() -> Anchor<'a, T> {
        Anchor {
            _p: ::std::marker::PhantomData,
        }
    }

    pub fn hold<'b: 'a>(&'a self, obj: &'b T) -> &'a T {
        obj
    }
}

