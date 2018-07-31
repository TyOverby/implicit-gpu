use nodes::{Node, NodeRef, PolyGroup};

#[derive(Debug, PartialEq)]
pub enum NodeGroup {
    Basic(NodeRef),
    Freeze(NodeRef),
    Polygon { group: PolyGroup, dx: f32, dy: f32 },
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Deserialize, Serialize)]
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
    pub fn new() -> Nest {
        Nest { groups: vec![] }
    }

    fn add(&mut self, group: NodeGroup) -> GroupId {
        if let Some(pos) = self.groups.iter().position(|g| g == &group) {
            return GroupId(pos);
        }

        let idx = self.groups.len();
        self.groups.push(group);
        GroupId(idx)
    }

    pub fn group(&mut self, node: NodeRef) -> GroupId {
        let _guard = ::flame::start_guard("Nest::group");
        let node = if node.contains_break() {
            NodeRef::new(node.take().propagate_translates(0.0, 0.0))
        } else {
            node
        };

        let group = match &*node {
            &Node::Polygon { ref group, dx, dy } => NodeGroup::Polygon {
                group: group.clone(),
                dx,
                dy,
            },
            &Node::Freeze { ref target } => {
                // TODO: track dx and dy here too
                NodeGroup::Freeze(do_group(target.clone(), self))
            }
            other => NodeGroup::Basic(do_group(NodeRef::new(other.clone()), self)),
        };

        self.add(group)
    }

    pub fn get(&self, id: GroupId) -> &NodeGroup {
        let GroupId(id) = id;
        &self.groups[id]
    }
}

fn do_group(node: NodeRef, nest: &mut Nest) -> NodeRef {
    let n = node.clone();
    match &*node {
        &Node::Polygon { .. } => {
            let group_id = nest.group(n);
            NodeRef::new(Node::OtherGroup { group_id })
        }
        &Node::Break { ref target } => {
            let group_id = nest.group(target.clone());
            NodeRef::new(Node::OtherGroup { group_id })
        }
        &Node::Freeze { .. } => {
            let group_id = nest.group(n);
            NodeRef::new(Node::OtherGroup { group_id })
        }
        &Node::Circle { .. } => n,
        &Node::Rect { .. } => n,
        &Node::Translate { dx, dy, ref target } => NodeRef::new(Node::Translate {
            dx,
            dy,
            target: do_group(target.clone(), nest),
        }),
        &Node::Scale { dx, dy, ref target } => NodeRef::new(Node::Scale {
            dx,
            dy,
            target: do_group(target.clone(), nest),
        }),
        &Node::And { ref children } => NodeRef::new(Node::And {
            children: children.iter().map(|c| do_group(c.clone(), nest)).collect(),
        }),
        &Node::Or { ref children } => NodeRef::new(Node::Or {
            children: children.iter().map(|c| do_group(c.clone(), nest)).collect(),
        }),
        &Node::Not { ref target } => NodeRef::new(Node::Not {
            target: do_group(target.clone(), nest),
        }),
        &Node::Modulate { how_much, ref target } => NodeRef::new(Node::Modulate {
            how_much,
            target: do_group(target.clone(), nest),
        }),
        &Node::OtherGroup { .. } => panic!("OtherGroup found while grouping"),
    }
}
