use ::nodes::{StaticNode, Node, PolyGroup};

#[derive(Debug, PartialEq)]
pub enum NodeGroup {
    Basic(StaticNode),
    Freeze(StaticNode),
    Polygon(PolyGroup),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct GroupId(pub usize);

#[derive(Debug)]
pub struct Nest {
    groups: Vec<NodeGroup>
}

impl GroupId {
    pub fn number(&self) -> usize {
        let &GroupId(ret) = self;
        ret
    }
}

impl Nest {
    pub fn new() -> Nest {
        Nest {
            groups: vec![],
        }
    }

    fn add(&mut self, group: NodeGroup) -> GroupId {
        if let Some(pos) = self.groups.iter().position(|g| g == &group) {
            return GroupId(pos);
        }

        let idx = self.groups.len();
        self.groups.push(group);
        GroupId(idx)
    }

    pub fn group<'a>(&mut self, node: &'a Node<'a>) -> GroupId {
        let group = match node {
            &Node::Polygon(ref poly) => NodeGroup::Polygon((*poly).clone()),
            &Node::Freeze(ref ch) => {
                let s_node: StaticNode = create_node!(a, {
                    let node: &Node = do_group(ch, self, &a);
                    node
                });
                NodeGroup::Freeze(s_node)
            }
            other => {
                let s_node: StaticNode = create_node!(a, {
                    let node: &Node = do_group(other, self, &a);
                    node
                });
                NodeGroup::Basic(s_node)
            }
        };

        self.add(group)
    }

    pub fn get(&self, id: GroupId) -> &NodeGroup {
        let GroupId(id) = id;
        &self.groups[id]
    }
}

fn do_group<'a, 'b, F>(node: &'a Node<'a>, nest: &mut Nest, a: &F) -> &'b Node<'b>
where F: Fn(Node<'b>) -> &'b Node<'b> {
    match node {
        &Node::Polygon(_) => {
            let og = nest.group(node);
            a(Node::OtherGroup(og))
        }
        &Node::Break(o) => {
            let og = nest.group(o);
            a(Node::OtherGroup(og))
        }
        &Node::Freeze(_) => {
            let og = nest.group(node);
            a(Node::OtherGroup(og))
        }
        &Node::Circle {x, y, r} => a(Node::Circle{x, y, r}),
        &Node::Rect {x, y, w, h} => a(Node::Rect{x, y, w, h}),
        &Node::And(ref ch) => a(Node::And(ch.iter().map(|c| do_group(c, nest, a)).collect())),
        &Node::Or(ref ch) => a(Node::Or(ch.iter().map(|c| do_group(c, nest, a)).collect())),
        &Node::Not(ref ch) => do_group(ch, nest, a),
        &Node::Modulate(how_much, ch) => a(Node::Modulate(how_much, do_group(ch, nest, a))),
        &Node::OtherGroup(_) => panic!("OtherGroup found while grouping"),
    }
}
