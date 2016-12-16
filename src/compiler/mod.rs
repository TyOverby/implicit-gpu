use super::nodes::NodePtr;

pub struct CompilationContext {
    identifier_id: usize,
    lines: Vec<String>,
}

pub fn compile(node: &NodePtr) -> String {
    let mut cc = CompilationContext::new();
    let (stage, _) = node.compile(&mut cc);
    let final_result = stage.result.clone();
    cc.add_stage(stage);

    let preamble = r#"
__kernel void apply(__global float* buffer, size_t width) {
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t pos = x + y * width;

    float x_s = (float) x;
    float y_s = (float) y;
"#;

    let postamble = format!(r#"
    buffer[pos] = {};
}}
"#, final_result);

    let mut buffer = String::new();
    buffer.push_str(&preamble);
    for line in cc.lines {
        buffer.push_str(&line);
        buffer.push('\n');
    }
    buffer.push_str(&postamble);

    buffer
}


pub struct Stage {
    pub result: String,
    pub lines: Vec<String>,
}

impl Stage {
    pub fn new(result: String) -> Stage {
        Stage {
            result: result,
            lines: vec![]
        }
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }
}

impl CompilationContext {
    pub fn new() -> CompilationContext {
        CompilationContext {
            identifier_id: 0,
            lines: vec![],
        }
    }

    pub fn get_x(&self) -> &'static str { "x_s" }
    pub fn get_y(&self) -> &'static str { "y_s" }

    pub fn get_id(&mut self, prefix: &str) -> String {
        let r = format!("{}_{}", prefix, self.identifier_id);
        self.identifier_id += 1;
        r
    }

    pub fn add_stage(&mut self, stage: Stage) {
        self.lines.push("".into());
        self.lines.push(format!("    float {};", stage.result));
        self.lines.push("    {".into());
        for line in stage.lines {
            self.lines.push(format!("        {}", line));
        }
        self.lines.push("    }".into());
    }
}
