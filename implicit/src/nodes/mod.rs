mod poly;

pub use self::poly::*;
use super::lines::util::geom::Rect;
use compiler::GroupId;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NodeRef {
    node: Arc<Node>,
}

unsafe impl Sync for NodeRef {}
unsafe impl Send for NodeRef {}

impl ::std::ops::Deref for NodeRef {
    type Target = Node;
    fn deref(&self) -> &Node { &*self.node }
}

impl NodeRef {
    pub fn new(node: Node) -> NodeRef { NodeRef { node: Arc::new(node) } }

    pub fn take(self) -> Node {
        use std::ops::Deref;
        match Arc::try_unwrap(self.node) {
            Ok(n) => n,
            Err(a) => (a.deref()).clone(),
        }
    }
}

impl<'de> ::serde::Deserialize<'de> for NodeRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        <Node as ::serde::Deserialize>::deserialize(deserializer).map(|res| NodeRef { node: Arc::new(res) })
    }
}

impl<'de> ::serde::Serialize for NodeRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.node.serialize(serializer)
    }
}

// IF YOU ADD AN ENUM HERE, UPDATE `eq_ignore_group`
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Node {
    Circle { x: f32, y: f32, r: f32 },
    Rect { x: f32, y: f32, w: f32, h: f32 },
    And { children: Vec<NodeRef> },
    Or { children: Vec<NodeRef> },
    Not { target: NodeRef },
    Polygon { dx: f32, dy: f32, group: PolyGroup },
    Modulate { how_much: f32, target: NodeRef },
    Translate { dx: f32, dy: f32, target: NodeRef },
    Break { target: NodeRef },
    OtherGroup { group_id: GroupId },
    Freeze { target: NodeRef },
}

impl Node {
    pub fn eq_ignore_group(&self, other: &Node) -> bool {
        match (self, other) {
            (&Node::Circle { x: mx, y: my, r: mr }, &Node::Circle { x: ox, y: oy, r: or }) => mx == ox && my == oy && mr == or,
            (&Node::And { children: ref mch }, &Node::And { children: ref och }) => mch.iter().zip(och.iter()).all(|(a, b)| a.eq_ignore_group(&*b)),
            (&Node::Or { children: ref mch }, &Node::Or { children: ref och }) => mch.iter().zip(och.iter()).all(|(a, b)| a.eq_ignore_group(&*b)),
            (&Node::Not { target: ref mc }, &Node::Not { target: ref oc }) => mc.eq_ignore_group(&*oc),
            (&Node::Polygon {
                 dx: dx1,
                 dy: dy1,
                 group: ref mpg,
             },
             &Node::Polygon {
                 dx: dx2,
                 dy: dy2,
                 group: ref opg,
             }) => dx1 == dx2 && dy1 == dy2 && mpg == opg,
            (&Node::Modulate {
                 how_much: ref mhm,
                 target: ref mc,
             },
             &Node::Modulate {
                 how_much: ref ohm,
                 target: ref oc,
             }) => mhm == ohm && mc.eq_ignore_group(&*oc),
            (&Node::Break { target: ref mc }, &Node::Break { target: ref oc }) => mc.eq_ignore_group(&*oc),
            (&Node::Freeze { target: ref mc }, &Node::Freeze { target: ref oc }) => mc.eq_ignore_group(&*oc),
            (&Node::OtherGroup { .. }, &Node::OtherGroup { .. }) => true,
            (&Node::Translate {
                 dx: dx1,
                 dy: dy1,
                 target: ref t1,
             },
             &Node::Translate {
                 dx: dx2,
                 dy: dy2,
                 target: ref t2,
             }) => dx1 == dx2 && dy1 == dy2 && t1.eq_ignore_group(t2),
            (_, _) => false,
        }
    }

    pub fn propagate_translates(self, dx: f32, dy: f32) -> Self {
        use self::Node::*;
        match self {
            Circle { x, y, r } => Circle { x: x + dx, y: y + dy, r },
            Rect { x, y, w, h } => Rect { x: x + dx, y: y + dy, w, h },
            Polygon { group, dx: pdx, dy: pdy } => Polygon {
                dx: pdx + dx,
                dy: pdy + dy,
                group,
            },
            Break { target } => Break { target: NodeRef::new(target.take().propagate_translates(dx, dy)) },
            Freeze { target } => Freeze { target: NodeRef::new(target.take().propagate_translates(dx, dy)) },
            And { children } => And {
                children: children
                    .into_iter()
                    .map(|child| NodeRef::new(child.take().propagate_translates(dx, dy)))
                    .collect(),
            },
            Or { children } => Or {
                children: children
                    .into_iter()
                    .map(|child| NodeRef::new(child.take().propagate_translates(dx, dy)))
                    .collect(),
            },
            Not { target } => Not { target: NodeRef::new(target.take().propagate_translates(dx, dy)) },
            Modulate { how_much, target } => Modulate {
                how_much,
                target: NodeRef::new(target.take().propagate_translates(dx, dy)),
            },
            Translate { target, dx: tdx, dy: tdy } => target.take().propagate_translates(tdx + dx, tdy + dy),
            OtherGroup { .. } => panic!("OtherGroup in propagate_translates"),
        }
    }

    pub fn contains_break(&self) -> bool {
        use self::Node::*;
        match *self {
            Circle { .. } => false,
            Rect { .. } => false,
            Polygon { .. } => true,
            Break { .. } => true,
            Freeze { .. } => true,
            And { ref children } => children.iter().any(|n| n.contains_break()),
            Or { ref children } => children.iter().any(|n| n.contains_break()),
            Not { ref target } => target.contains_break(),
            Modulate { ref target, .. } => target.contains_break(),
            Translate { ref target, .. } => target.contains_break(),
            OtherGroup { .. } => panic!("contains_break on OtherGroup"),
        }
    }

    pub fn bounding_box(&self) -> (Option<Rect>, Option<Rect>) {
        use super::lines::util::geom::{Point, Rect, Vector};

        fn union<I: Iterator<Item = Option<Rect>>>(mut i: I) -> Option<Rect> {
            let mut res = i.next().and_then(|a| a);
            for r in i {
                res = match (res, r) {
                    (None, None) => None,
                    (Some(_), None) | (None, Some(_)) => None,
                    (Some(a), Some(b)) => Some(a.union_with(&b)),
                };
            }
            res
        }

        fn intersection<I: Iterator<Item = Option<Rect>>>(mut i: I) -> Option<Rect> {
            let mut res = i.next().and_then(|a| a);
            for r in i {
                res = match (res, r) {
                    (None, None) => None,
                    (Some(a), None) | (None, Some(a)) => Some(a),
                    (Some(a), Some(b)) => {
                        let r = a.intersect_with(&b);
                        if r.is_null() { None } else { Some(r) }
                    }
                };
            }
            res
        }

        match self {
            &Node::Circle { x, y, r } => (Some(Rect::centered_with_radius(&Point { x, y }, r)), None),
            &Node::Rect { x, y, w, h } => (Some(Rect::from_point_and_size(&Point { x, y }, &Vector { x: w, y: h })), None),
            &Node::Polygon { dx, dy, ref group } => {
                let mut rect = Rect::null();
                for polygon in &group.additive {
                    for (&x, &y) in polygon.xs.iter().zip(polygon.ys.iter()) {
                        rect.expand_to_include(&Point { x: x + dx, y: y + dy });
                    }
                }
                (Some(rect), None)
            }
            &Node::Modulate { how_much, ref target } => {
                let v = how_much;
                let (a, s) = target.bounding_box();
                (a.map(|b| b.expand(-v, -v, -v, -v)), s.map(|b| b.expand(v, v, v, v)))
            }
            &Node::Translate { dx, dy, ref target } => {
                let (a, s) = target.bounding_box();
                (a.map(|b| b.expand(-dx, -dy, dx, dy)), s.map(|b| b.expand(-dx, -dy, dx, dy)))
            }
            &Node::And { ref children } => {
                (
                    intersection(children.iter().map(|n| n.bounding_box().0)),
                    union(children.iter().map(|n| n.bounding_box().1)),
                )
            }
            &Node::Or { ref children } => {
                (
                    union(children.iter().map(|n| n.bounding_box().0)),
                    intersection(children.iter().map(|n| n.bounding_box().1)),
                )
            }
            &Node::Not { ref target } => {
                let (a, s) = target.bounding_box();
                (s, a)
            }
            &Node::OtherGroup { .. } => panic!("other group in find bounding box"),
            &Node::Freeze { ref target } => target.bounding_box(),
            &Node::Break { ref target } => target.bounding_box(),
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

    let node = Node::Circle { x: 10.0, y: 30.0, r: 10.0 };
    let as_str = to_string_pretty(&node).unwrap();
    assert_eq!(
        as_str,
        r#"{
  "kind": "circle",
  "x": 10.0,
  "y": 30.0,
  "r": 10.0
}"#
    );
}
