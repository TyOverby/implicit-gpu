#[macro_use]
extern crate implicit_gpu;
extern crate typed_arena;

mod util;

use implicit_gpu::nodes::Node;

#[test]
fn main() {
    let node = create_node!(a, {
        a(Node::Circle{ x: 50.0, y: 50.0, r: 50.0 })
    });

    util::run_test("circles", node.node());
}
