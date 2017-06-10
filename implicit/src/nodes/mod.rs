mod poly;

use std::sync::Arc;
pub use self::poly::*;
use compiler::GroupId;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NodeRef {
    node: Arc<Node>
}

unsafe impl Sync for NodeRef {}
unsafe impl Send for NodeRef {}

impl ::std::ops::Deref for NodeRef {
    type Target = Node;
    fn deref(&self) -> &Node {
        &*self.node
    }
}

impl NodeRef {
    pub fn new(node: Node) -> NodeRef {
        NodeRef {
            node: Arc::new(node),
        }
    }
}

impl <'de> ::serde::Deserialize<'de> for NodeRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: ::serde::Deserializer<'de> {
        <Node as ::serde::Deserialize>::deserialize(deserializer).map(|res| {
            NodeRef { node: Arc::new(res)}
        })
    }
}

impl <'de> ::serde::Serialize for NodeRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: ::serde::Serializer {
        self.node.serialize(serializer)
    }
}

// IF YOU ADD AN ENUM HERE, UPDATE `eq_ignore_group`
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all="snake_case")]
pub enum Node {
    Circle {
        x: f32,
        y: f32,
        r: f32
    },
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32
    },
    And {
        children: Vec<NodeRef>,
    },
    Or {
        children: Vec<NodeRef>,
    },
    Not {
        target: NodeRef,
    },
    Polygon {
        group: PolyGroup,
    },
    Modulate {
        how_much: f32,
        target: NodeRef,
    },
    Break {
        target: NodeRef
    },
    OtherGroup {
        group_id: GroupId
    },
    Freeze {
        target: NodeRef
    },
}

impl Node {
    pub fn eq_ignore_group(&self, other: &Node) -> bool {
        match (self, other) {
            (&Node::Circle { x: mx, y: my, r: mr }, &Node::Circle { x: ox, y: oy, r: or }) =>
                mx == ox && my == oy && mr == or,
            (&Node::And{children: ref mch}, &Node::And{children: ref och}) =>
                mch.iter().zip(och.iter()).all(|(a, b)| a.eq_ignore_group(&*b)),
            (&Node::Or{children: ref mch}, &Node::Or{children: ref och}) =>
                mch.iter().zip(och.iter()).all(|(a, b)| a.eq_ignore_group(&*b)),
            (&Node::Not{target: ref mc}, &Node::Not{target: ref oc}) => mc.eq_ignore_group(&*oc),
            (&Node::Polygon{group: ref mpg}, &Node::Polygon{group: ref opg}) => mpg == opg,
            (&Node::Modulate{how_much: ref mhm, target: ref mc}, &Node::Modulate{how_much: ref ohm, target: ref oc})
                => mhm == ohm && mc.eq_ignore_group(&*oc),
            (&Node::Break{target: ref mc}, &Node::Break{target: ref oc}) => mc.eq_ignore_group(&*oc),
            (&Node::Freeze{target: ref mc}, &Node::Freeze{target: ref oc}) => mc.eq_ignore_group(&*oc),
            (&Node::OtherGroup{..}, &Node::OtherGroup{..}) => true,
            (_, _) => false,
        }
    }
}

#[doc(hidden)]
pub struct Anchor<'a, T: 'a> {
    _p: ::std::marker::PhantomData<&'a T>,
}

impl<'a, T: 'a> Anchor<'a, T> {
    pub fn new() -> Anchor<'a, T> { Anchor { _p: ::std::marker::PhantomData } }

    pub fn hold<'b: 'a>(&'a self, obj: &'b T) -> &'a T { obj }
}

#[test]
fn ser_de() {
    use serde_json::*;

    let node = Node::Circle{x: 10.0, y: 30.0, r: 10.0};
    let as_str = to_string_pretty(&node).unwrap();
    assert_eq!(as_str, r#"{
  "kind": "circle",
  "x": 10.0,
  "y": 30.0,
  "r": 10.0
}"#);
}
