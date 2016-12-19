use gc::{Gc, Trace};
use std::ops::Deref;
use std::collections::HashMap;
use ::nodes::{
    NodePtr,
    Node,
    NodeTrait,
    NodeId
};

#[derive(PartialEq, Debug, Clone)]
pub struct NodeGroupPtr(Gc<NodeGroup>);

#[derive(PartialEq, Debug, Clone)]
pub enum NodeGroup {
    CoreNodes {
        root: NodePtr,
        input_buffers: HashMap<NodeId, NodeGroupPtr>,
    },

    Polygon {
        poly: NodePtr
    },
}

impl NodeGroupPtr {
    pub fn group(node: &NodePtr) -> NodeGroupPtr {
        fn group_internal(current: &NodePtr, mapping: &mut HashMap<NodeId, NodeGroupPtr>) {
            if current.is_break() {
                mapping.insert(
                    current.id(),
                    NodeGroupPtr::group(current));
            } else {
                match &***current {
                    &Node::And(_, )
                }
            }
        }

        if let &Node::Poly(_, _) = &***node {
            return NodeGroupPtr(Gc::new(NodeGroup::Polygon{
                poly: node.clone()
            }))
        }

        let mut mapping = HashMap::new();
        group_internal(node, &mut mapping);

        NodeGroupPtr(Gc::new(NodeGroup::CoreNodes {
            root: node.clone(),
            input_buffers: mapping,
        }))
    }
}

impl Deref for NodeGroupPtr {
    type Target = Gc<NodeGroup>;

    fn deref(&self) -> &Gc<NodeGroup> {
        &self.0
    }
}

unsafe impl Trace for NodeGroupPtr {
    custom_trace!(this, {
        mark(&this.0)
    });
}

unsafe impl Trace for NodeGroup {
    custom_trace!(this, {
        match this {
            &NodeGroup::CoreNodes{ref root, ref input_buffers} => {
                mark(root);
                for (_, n) in input_buffers {
                    mark(n);
                }
            },
            &NodeGroup::Polygon{ref poly} => mark(poly),
        }
    });
}
