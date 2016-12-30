use super::nodes::{Node};
use std::fmt::Write;

pub struct CompilationContext {
    identifier_id: usize,
}

pub fn compile(node: &Node) -> String {
    let mut cc = CompilationContext::new();

    let preamble = r#"
__kernel void apply(__global float* buffer, size_t width) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;
"#;

    let mut buffer = preamble.into();
    let final_result = comp(node, &mut cc, &mut buffer);
    buffer.push('\n');
    writeln!(&mut buffer, "  buffer[pos] = {}; \n}}", final_result);
    buffer
}

fn comp(node: &Node, cc: &mut CompilationContext, buff: &mut String) -> String {
    match *node {
        Node::Circle{x, y, r} => {
            let (res, dx, dy) = (cc.get_id("circle"), cc.get_id("dx"), cc.get_id("dy"));

            buff.push('\n');
            writeln!(buff, "  float {result};", result = res).unwrap();
            writeln!(buff, "  {{").unwrap();
            writeln!(buff, "    float {dx} = {x} - {cx};", dx = dx, x = cc.get_x(), cx = x).unwrap();
            writeln!(buff, "    float {dy} = {y} - {cy};", dy = dy, y = cc.get_y(), cy = y).unwrap();
            writeln!(buff, "    {result} = sqrt({dx} * {dx} + {dy} * {dy}) - {radius};", result = res, dx = dx, dy = dy, radius = r).unwrap();
            writeln!(buff, "  }}").unwrap();

            res
        }
        Node::And(ref children) => {
            match children.len() {
                0 => panic!("And([])"),
                1 => comp(&children[0], cc, buff),
                n => {
                    let mut left = children.clone();
                    let right = left.split_off(n / 2);

                    let res_left = comp(&Node::And(left), cc, buff);
                    let res_right = comp(&Node::And(right), cc, buff);

                    let res = cc.get_id("and");

                    buff.push('\n');
                    writeln!(buff, "  float {result} = max({a}, {b});", result = res, a = res_left, b = res_right).unwrap();

                    res
                }
            }
        }
        Node::Or(ref children) => {
            match children.len() {
                0 => panic!("Or([])"),
                1 => comp(&children[0], cc, buff),
                n => {
                    let mut left = children.clone();
                    let right = left.split_off(n / 2);

                    let res_left = comp(&Node::Or(left), cc, buff);
                    let res_right = comp(&Node::Or(right), cc, buff);

                    let res = cc.get_id("or");

                    buff.push('\n');
                    writeln!(buff, "  float {result} = min({a}, {b});", result = res, a = res_left, b = res_right).unwrap();

                    res
                }
            }
        }
        Node::Not(ref child) => {
            let child_result = comp(child, cc, buff);
            let res = cc.get_id("not");
            buff.push('\n');
            writeln!(buff, "  float {result} = - {val};", result = res, val = child_result).unwrap();
            res
        }

        ref o => panic!("unexpected {:?}", o),
    }
}

impl CompilationContext {
    pub fn new() -> CompilationContext {
        CompilationContext {
            identifier_id: 0,
        }
    }

    pub fn get_x(&self) -> &'static str { "x_s" }

    pub fn get_y(&self) -> &'static str { "y_s" }

    pub fn get_id(&mut self, prefix: &str) -> String {
        let r = format!("{}_{}", prefix, self.identifier_id);
        self.identifier_id += 1;
        r
    }
}
