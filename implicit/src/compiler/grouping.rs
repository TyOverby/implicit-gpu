use nodes::{Node, PolyGroup};
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub enum NodeGroup {
    Basic(Arc<Node>),
    Freeze(Arc<Node>),
    Polygon(PolyGroup),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd)]
pub struct GroupId(pub usize);

#[derive(Debug)]
pub struct Nest {
    groups: Vec<NodeGroup>,
}

impl GroupId {
    pub fn number(&self) -> usize {
        let &GroupId(ret) = self;
        ret
    }
}

impl Nest {
    pub fn new() -> Nest { Nest { groups: vec![] } }

    fn add(&mut self, group: NodeGroup) -> GroupId {
        if let Some(pos) = self.groups.iter().position(|g| g == &group) {
            return GroupId(pos);
        }

        let idx = self.groups.len();
        self.groups.push(group);
        GroupId(idx)
    }

    pub fn group(&mut self, node: Arc<Node>) -> GroupId {
        let group = match &*node {
            &Node::Polygon(ref poly) => NodeGroup::Polygon((*poly).clone()),
            &Node::Freeze(ref ch) => {
                NodeGroup::Freeze(do_group(ch.clone(), self))
            }
            other => {
                NodeGroup::Basic(do_group(Arc::new(other.clone()), self))
            }
        };

        self.add(group)
    }

    pub fn get(&self, id: GroupId) -> &NodeGroup {
        let GroupId(id) = id;
        &self.groups[id]
    }
}

fn do_group(node: Arc<Node>, nest: &mut Nest) -> Arc<Node> {
    let n = node.clone();
    match &*node {
        &Node::Polygon(_) => {
            let og = nest.group(n);
            Arc::new(Node::OtherGroup(og))
        }
        &Node::Break(ref o) => {
            let og = nest.group(o.clone());
            Arc::new(Node::OtherGroup(og))
        }
        &Node::Freeze(_) => {
            let og = nest.group(n);
            Arc::new(Node::OtherGroup(og))
        }
        &Node::Circle { .. } => n,
        &Node::Rect { .. } => n,
        &Node::And(ref ch) => Arc::new(Node::And(ch.iter().map(|c| do_group(c.clone(), nest)).collect())),
        &Node::Or(ref ch) => Arc::new(Node::Or(ch.iter().map(|c| do_group(c.clone(), nest)).collect())),
        &Node::Not(ref ch) => Arc::new(Node::Not(do_group(ch.clone(), nest))),
        &Node::Modulate(how_much, ref ch) => Arc::new(Node::Modulate(how_much, do_group(ch.clone(), nest))),
        &Node::OtherGroup(_) => panic!("OtherGroup found while grouping"),
    }
}
