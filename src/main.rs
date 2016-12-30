#[macro_use]
extern crate implicit_gpu;
extern crate typed_arena;
use implicit_gpu::nodes::*;
use implicit_gpu::compiler::*;


fn build<'a, F>(a: &F) -> &'a Node<'a>
where F: Fn(Node<'a>) -> &'a Node <'a> {
    a(Node::Circle{ x: 0.0, y: 0.0, r: 10.0 })
}

type F = for<'a> Fn(Node<'a>) -> &'a Node<'a>;

fn main() {
    let stat = create_node!(a, {
        a(Node::And(vec![
            build(&a),
            a(Node::Circle{ x: 5.0, y: 5.0, r: 10.0 }),
        ]))
    });

    println!("{:?}", stat);
    println!("{}", compile(stat.node()));
}
