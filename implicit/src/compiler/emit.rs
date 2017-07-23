use compiler::GroupId;
use nodes::Node;
use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::Write;
use std::rc::Rc;

#[derive(Clone)]
pub struct SharedInfo {
    pub identifier_id: usize,
    pub dependencies: Vec<GroupId>,
    pub dep_strings: Vec<String>,
}

#[derive(Clone)]
enum CompilationContext {
    Base(Rc<RefCell<SharedInfo>>),
    PositionMod {
        x: String,
        y: String,
        shared: Rc<RefCell<SharedInfo>>,
    },
}

pub fn compile(node: &Node) -> (String, SharedInfo) {
    use std::ops::Deref;

    let (cc, shared) = CompilationContext::new();

    let mut buffer = "".into(); // preamble.into();
    let final_result = comp(node, cc.clone(), &mut buffer);
    buffer.push('\n');
    writeln!(&mut buffer, "  buffer[pos] = {}; \n}}", final_result).unwrap();

    let mut preamble = r"__kernel void apply(__global float* buffer, ulong width".to_string();
    for b in cc.dep_strings() {
        preamble.push_str(&format!(", __global float* {}", b));
    }
    preamble.push_str(
        r#") {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;
"#,
    );

    let shared_ref = shared.deref().borrow();
    (format!("{}{}", preamble, buffer), shared_ref.clone())
}

fn comp(node: &Node, mut cc: CompilationContext, buff: &mut String) -> String {
    match *node {
        Node::Rect { x, y, w, h } => {
            let (res, _dx, _dy, _out) = (cc.get_id("rect"), cc.get_id("dx"), cc.get_id("dy"), cc.get_id("out"));

            buff.push('\n');
            writeln!(buff, "  float {result};", result = res).unwrap();
            writeln!(buff, "  {{").unwrap();
            writeln!(
                buff,
                "    {result} = ({x}-{a}) * ({x} - {c}) * ({y} - {b}) * ({y} - {d});",
                result = res,
                x = cc.get_x(),
                y = cc.get_y(),
                a = x,
                b = y,
                c = x + w,
                d = y + h,
            ).unwrap();
            writeln!(buff, "  }}").unwrap();

            res
        }
        Node::Circle { x, y, r } => {
            let (res, dx, dy) = (cc.get_id("circle"), cc.get_id("dx"), cc.get_id("dy"));

            buff.push('\n');
            writeln!(buff, "  float {result};", result = res).unwrap();
            writeln!(buff, "  {{").unwrap();
            writeln!(buff, "    float {dx} = {x} - {cx};", dx = dx, x = cc.get_x(), cx = x).unwrap();
            writeln!(buff, "    float {dy} = {y} - {cy};", dy = dy, y = cc.get_y(), cy = y).unwrap();
            writeln!(
                buff,
                "    {result} = sqrt({dx} * {dx} + {dy} * {dy}) - {radius};",
                result = res,
                dx = dx,
                dy = dy,
                radius = r,
            ).unwrap();
            writeln!(buff, "  }}").unwrap();

            res
        }
        Node::And { ref children } => {
            match children.len() {
                0 => panic!("And([])"),
                1 => comp(&children[0], cc, buff),
                n => {
                    let mut left = children.clone();
                    let right = left.split_off(n / 2);

                    let res_left = comp(&Node::And { children: left }, cc.clone(), buff);
                    let res_right = comp(&Node::And { children: right }, cc.clone(), buff);

                    let res = cc.get_id("and");

                    buff.push('\n');
                    writeln!(
                        buff,
                        "  float {result} = max({a}, {b});",
                        result = res,
                        a = res_left,
                        b = res_right,
                    ).unwrap();

                    res
                }
            }
        }
        Node::Or { ref children } => {
            match children.len() {
                0 => panic!("Or([])"),
                1 => comp(&children[0], cc, buff),
                n => {
                    let mut left = children.clone();
                    let right = left.split_off(n / 2);

                    let res_left = comp(&Node::Or { children: left }, cc.clone(), buff);
                    let res_right = comp(&Node::Or { children: right }, cc.clone(), buff);

                    let res = cc.get_id("or");

                    buff.push('\n');
                    writeln!(
                        buff,
                        "  float {result} = min({a}, {b});",
                        result = res,
                        a = res_left,
                        b = res_right,
                    ).unwrap();

                    res
                }
            }
        }
        Node::Not { ref target } => {
            let child_result = comp(target, cc.clone(), buff);
            let res = cc.get_id("not");
            buff.push('\n');
            writeln!(buff, "  float {result} = -{val};", result = res, val = child_result).unwrap();
            res
        }

        Node::Translate { dx, dy, ref target } => {
            let (new_x, new_y) = (cc.get_id("x"), cc.get_id("y"));
            buff.push('\n');
            writeln!(
                buff,
                "  float {new_x} = {old_x} - {dx};",
                new_x = new_x,
                old_x = cc.get_x(),
                dx = dx
            ).unwrap();
            writeln!(
                buff,
                "  float {new_y} = {old_y} - {dy};",
                new_y = new_y,
                old_y = cc.get_y(),
                dy = dy
            ).unwrap();

            comp(target, cc.with_xy(new_x, new_y), buff)
        }
        Node::Modulate { how_much, ref target } => {
            let child_result = comp(target, cc.clone(), buff);
            let res = cc.get_id("modulate");
            buff.push('\n');
            writeln!(
                buff,
                "  float {result} = {other} + {value};",
                result = res,
                other = child_result,
                value = how_much
            ).unwrap();
            res
        }

        Node::OtherGroup { group_id } => {
            let buffer_ref = cc.buffer_ref(group_id);
            let res = cc.get_id("other_group");

            writeln!(
                buff,
                "float {result} = {buffer_ref}[pos];",
                result = res,
                buffer_ref = buffer_ref,
            ).unwrap();
            res
        }

        ref o => panic!("unexpected {:?}", o),
    }
}

impl CompilationContext {
    pub fn new() -> (CompilationContext, Rc<RefCell<SharedInfo>>) {
        let shared = Rc::new(RefCell::new(SharedInfo {
            identifier_id: 0,
            dependencies: vec![],
            dep_strings: vec![],
        }));

        (CompilationContext::Base(shared.clone()), shared)
    }
}

impl CompilationContext {
    fn shared(&self) -> Rc<RefCell<SharedInfo>> {
        match self {
            &CompilationContext::Base(ref shared) => shared.clone(),
            &CompilationContext::PositionMod { ref shared, .. } => shared.clone(),
        }
    }

    pub fn with_xy(&self, x: String, y: String) -> CompilationContext { CompilationContext::PositionMod { x, y, shared: self.shared() } }

    pub fn buffer_ref(&mut self, group_id: GroupId) -> String {
        let shared = self.shared();
        let mut shared = shared.borrow_mut();
        let &mut SharedInfo {
            ref mut dependencies,
            ref mut dep_strings,
            ..
        } = &mut *shared;

        if !dependencies.contains(&group_id) {
            dependencies.push(group_id);
        }

        let s = format!("buffer_{}", group_id.number());
        dep_strings.push(s.clone());
        s
    }

    pub fn get_x(&self) -> Cow<'static, str> {
        use self::CompilationContext::*;
        match self {
            &Base { .. } => Cow::Borrowed("x_s"),
            &PositionMod { ref x, .. } => Cow::Owned(x.clone()),
        }
    }

    pub fn get_y(&self) -> Cow<'static, str> {
        use self::CompilationContext::*;
        match self {
            &Base { .. } => Cow::Borrowed("y_s"),
            &PositionMod { ref y, .. } => Cow::Owned(y.clone()),
        }
    }


    pub fn get_id(&mut self, prefix: &str) -> String {
        let shared = self.shared();
        let mut shared = shared.borrow_mut();
        let &mut SharedInfo { ref mut identifier_id, .. } = &mut *shared;

        let r = format!("{}_{}", prefix, identifier_id);
        *identifier_id += 1;
        r
    }

    pub fn dep_strings(&self) -> Vec<String> {
        let shared = self.shared();
        let shared = shared.borrow();
        let &SharedInfo { ref dep_strings, .. } = &*shared;
        dep_strings.clone()
    }
}
