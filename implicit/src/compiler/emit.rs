use ::nodes::{Node};
use ::compiler::GroupId;
use std::fmt::Write;

pub struct CompilationContext {
    identifier_id: usize,
    dependencies: Vec<GroupId>,
    dep_strings: Vec<String>,
}

pub fn compile(node: &Node) -> (String, CompilationContext) {
    let mut cc = CompilationContext::new();

    let mut buffer = "".into(); //preamble.into();
    let final_result = comp(node, &mut cc, &mut buffer);
    buffer.push('\n');
    writeln!(&mut buffer, "  buffer[pos] = {}; \n}}", final_result).unwrap();

    let mut preamble = r"__kernel void apply(__global float* buffer, ulong width".to_string();
    for b in &cc.dep_strings {
        preamble.push_str(&format!(", __global float* {}", b));
    }
    preamble.push_str( r#") {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;
"#);


    (format!("{}{}", preamble, buffer), cc)
}

fn comp(node: &Node, cc: &mut CompilationContext, buff: &mut String) -> String {
    match *node {
        Node::Rect{x, y, w, h} => {
            let (res, _dx, _dy, _out) = (cc.get_id("rect"), cc.get_id("dx"), cc.get_id("dy"), cc.get_id("out"));

            buff.push('\n');
            writeln!(buff, "  float {result};", result = res).unwrap();
            writeln!(buff, "  {{").unwrap();
            writeln!(buff, "    {result} = ({x}-{a}) * ({x} - {c}) * ({y} - {b}) * ({y} - {d});",
                                result = res, x = cc.get_x(), y = cc.get_y(), a = x, b = y, c = x + w, d = y + h).unwrap();
            writeln!(buff, "  }}").unwrap();

            res
        }
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
            writeln!(buff, "  float {result} = -{val};", result = res, val = child_result).unwrap();
            res
        }

        Node::Modulate(v, ref child) => {
            let child_result = comp(child, cc, buff);
            let res = cc.get_id("modulate");
            buff.push('\n');
            writeln!(buff, "  float {result} = {other} + {value};", result = res, other = child_result, value = v).unwrap();
            res
        }

        Node::OtherGroup(group_id) => {
            let buffer_ref = cc.buffer_ref(group_id);
            let res = cc.get_id("other_group");

            writeln!(buff, "float {result} = {buffer_ref}[pos];", result = res, buffer_ref = buffer_ref).unwrap();
            res
        }

        ref o => panic!("unexpected {:?}", o),
    }
}

impl CompilationContext {
    pub fn new() -> CompilationContext {
        CompilationContext {
            identifier_id: 0,
            dependencies: vec![],
            dep_strings: vec![],
        }
    }

    pub fn buffer_ref(&mut self, group_id: GroupId) -> String {
        if !self.dependencies.contains(&group_id) {
            self.dependencies.push(group_id);
        }

        let s = format!("buffer_{}", group_id.number());
        self.dep_strings.push(s.clone());
        return s;
    }

    pub fn get_x(&self) -> &'static str { "x_s" }

    pub fn get_y(&self) -> &'static str { "y_s" }

    pub fn get_id(&mut self, prefix: &str) -> String {
        let r = format!("{}_{}", prefix, self.identifier_id);
        self.identifier_id += 1;
        r
    }

    pub fn deps(&self) -> &[GroupId] {
        &self.dependencies
    }
}
