#[cfg(test)]
use expectation::{extensions::*, Provider};
use expectation_plugin::expectation_test;
use ocaml::*;
use std::cell::Cell;
use std::collections::BTreeSet;
use std::io::{Result as IoResult, Write};
use std::rc::Rc;

const DIST_TO_LINE: &'static str = include_str!("shaders/dist_to_line.c");

pub struct CompileResult<W> {
    pub dependencies: Vec<Id>,
    pub text: W,
}

struct NameGen {
    id: Rc<Cell<u32>>,
}

impl NameGen {
    fn new() -> Self {
        NameGen {
            id: Rc::new(Cell::new(0)),
        }
    }

    fn gen(&self, n: &str) -> String {
        let id = self.id.get();
        self.id.set(id + 1);
        format!("_{}_{}", n, id)
    }

    fn gen_3(&self, n1: &str, n2: &str, n3: &str) -> (String, String, String) {
        (self.gen(n1), self.gen(n2), self.gen(n3))
    }

    fn gen_2(&self, n1: &str, n2: &str) -> (String, String) {
        (self.gen(n1), self.gen(n2))
    }
}

pub fn compile<W: Write>(shape: &Shape, mut writer: W) -> IoResult<CompileResult<W>> {
    let mut deps = BTreeSet::new();
    let mut program_body: Vec<u8> = vec![];
    let mut uses_dist_to_line = false;
    let result = compile_impl(
        shape,
        &mut program_body,
        &mut uses_dist_to_line,
        &mut deps,
        &NameGen::new(),
        ("x_s".into(), "y_s".into()),
    )?;
    let deps = deps.into_iter().collect();

    if uses_dist_to_line {
        writeln!(writer, "{}", DIST_TO_LINE)?;
    }
    write!(
        writer,
        "{}",
        r"__kernel void apply(__global float* buffer, ulong width"
    )?;

    for b in &deps {
        write!(writer, ", __global float* field__{}", b)?;
    }
    write!(
        writer,
        r#") {{
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;
  float x_s = (float) x;
  float y_s = (float) y;
"#
    )?;

    writer.write_all(&program_body)?;
    writeln!(writer, "buffer[pos] = {};", result)?;
    writeln!(writer, "}}")?;

    Ok(CompileResult {
        text: writer,
        dependencies: deps,
    })
}

fn get_xy(matrix: &Matrix, current_xy: (String, String)) -> (String, String) {
    let (old_x, old_y) = current_xy;
    if matrix.approx_eq(&Matrix::identity()) {
        return (old_x.into(), old_y.into());
    }
    let inverted = matrix.inverse().unwrap();
    // x = point.x * self.m11 + point.y * self.m21 + self.m31
    let x = format!(
        "({x} * {m11} + {y} * {m21} + {m31})",
        x = old_x,
        y = old_y,
        m11 = inverted.m11,
        m21 = inverted.m21,
        m31 = inverted.m31
    );
    // y = point.x * self.m12 + point.y * self.m22 + self.m32
    let y = format!(
        "({x} * {m12} + {y} * {m22} + {m32})",
        x = old_x,
        y = old_y,
        m12 = inverted.m12,
        m22 = inverted.m22,
        m32 = inverted.m32
    );
    (x, y)
}

fn compile_impl<W: Write>(
    shape: &Shape,
    out: &mut W,
    uses_dist_to_line: &mut bool,
    deps: &mut BTreeSet<Id>,
    namegen: &NameGen,
    current_xy: (String, String),
) -> IoResult<String> {
    writeln!(out, "")?;
    match shape {
        Shape::Transform(target, matrix) => {
            let (x, y) = get_xy(matrix, current_xy);
            let (nx, ny) = namegen.gen_2("nx", "ny");
            writeln!(out, "// Transform {:?}", matrix)?;
            writeln!(out, "float {} = {};", nx, x)?;
            writeln!(out, "float {} = {};", ny, y)?;
            compile_impl(target, out, uses_dist_to_line, deps, namegen, (nx, ny))
        }
        Shape::Terminal(Terminal::Circle(Circle { x, y, r })) => {
            let (res, dx, dy) = namegen.gen_3("circle", "dx", "dy");
            let (mx, my) = current_xy;
            writeln!(out, "// Circle {}", res)?;
            writeln!(out, "float {} = {} - {};", dx, mx, x)?;
            writeln!(out, "float {} = {} - {};", dy, my, y)?;
            writeln!(
                out,
                "float {} = sqrt({dx} * {dx} + {dy} * {dy}) - {r};",
                res,
                dx = dx,
                dy = dy,
                r = r,
            )?;
            writeln!(out, "// End Circle {}", res)?;

            Ok(res)
        }
        Shape::Terminal(Terminal::Rect(Rect { x, y, w, h })) => {
            *uses_dist_to_line = true;
            let (x, y, w, h) = (*x, *y, *w, *h);
            let res = namegen.gen("rect");
            let (mx, my) = current_xy;
            writeln!(out, "// Rect {}", res)?;
            writeln!(out, "float {} = INFINITY;", res)?;
            {
                let mut dist_to_line = |ax: f32, ay: f32, bx: f32, by: f32| {
                    writeln!(
                        out,
                        "{res} = min({res}, dist_to_line({mx}, {my}, {ax}, {ay}, {bx}, {by}));",
                        res = res,
                        mx = mx,
                        my = my,
                        ax = ax,
                        ay = ay,
                        bx = bx,
                        by = by
                    )
                };
                dist_to_line(x, y, x + w, y)?;
                dist_to_line(x + w, y, x + w, y + h)?;
                dist_to_line(x + w, y + h, x, y + h)?;
                dist_to_line(x, y + h, x, y)?;
            }
            write!(
                out,
                "if ({mx} > {rx} && {my} > {ry} && ",
                mx = mx,
                my = my,
                rx = x,
                ry = y
            )?;
            writeln!(
                out,
                "{mx} < ({rx} + {w}) && {my} < ({ry} + {h}))",
                mx = mx,
                my = my,
                rx = x,
                ry = y,
                w = w,
                h = h,
            )?;
            writeln!(out, "{res} = -{res};", res = res)?;
            writeln!(out, "// End Rect {}", res)?;

            Ok(res)
        }
        Shape::Terminal(Terminal::Field(id)) => {
            deps.insert(*id);
            let res = namegen.gen("field");
            writeln!(out, "float {res} = field__{id}[pos];", res = res, id = id)?;
            Ok(res)
        }
        Shape::Intersection(shapes) => {
            if shapes.is_empty() {
                panic!("empty intersection");
            }

            let result = namegen.gen("intersection");
            writeln!(out, "// Intersection {}", result)?;

            writeln!(out, "float {} = -INFINITY;", result)?;
            for shape in shapes {
                let intermediate = compile_impl(
                    shape,
                    out,
                    uses_dist_to_line,
                    deps,
                    namegen,
                    current_xy.clone(),
                )?;
                writeln!(
                    out,
                    "{res} = max({res}, {int});",
                    res = result,
                    int = intermediate
                )?;
            }
            writeln!(out, "// End Intersection {}", result)?;
            Ok(result)
        }
        Shape::Union(shapes) => {
            if shapes.is_empty() {
                panic!("empty union");
            }
            let result = namegen.gen("union");
            writeln!(out, "// Union {}", result)?;
            writeln!(out, "float {} = INFINITY;", result)?;
            for shape in shapes {
                let intermediate = compile_impl(
                    shape,
                    out,
                    uses_dist_to_line,
                    deps,
                    namegen,
                    current_xy.clone(),
                )?;
                writeln!(
                    out,
                    "{res} = min({res}, {int});",
                    res = result,
                    int = intermediate
                )?;
            }
            writeln!(out, "// End Union {}", result)?;
            Ok(result)
        }
        Shape::Not(shape) => {
            let result = namegen.gen("negate");
            writeln!(out, "// Not {}", result)?;
            let intermediate =
                compile_impl(shape, out, uses_dist_to_line, deps, namegen, current_xy)?;
            writeln!(out, "float {} = -{};", result, intermediate)?;
            writeln!(out, "// End Not {}", result)?;
            Ok(result)
        }
        Shape::Modulate(shape, how_much) => {
            let result = namegen.gen("modulate");
            writeln!(out, "// Modulate {}", result)?;
            let intermediate =
                compile_impl(shape, out, uses_dist_to_line, deps, namegen, current_xy)?;
            writeln!(out, "float {} = {} - {};", result, intermediate, how_much)?;
            writeln!(out, "// End Modulate {}", result)?;
            Ok(result)
        }
    }
}

#[expectation_test]
fn cl_for_circle(provider: Provider) {
    let w = provider.text_writer("out.c");
    let shape = Shape::Terminal(Terminal::Circle(Circle {
        x: 0.0,
        y: 5.0,
        r: 10.0,
    }));
    compile(&shape, w).unwrap();
}

#[expectation_test]
fn cl_for_matrix_circle(provider: Provider) {
    use euclid::Transform2D;
    let w = provider.text_writer("out.c");
    let shape = Shape::Transform(
        Box::new(Shape::Terminal(Terminal::Circle(Circle {
            x: 0.0,
            y: 5.0,
            r: 10.0,
        }))),
        Transform2D::create_scale(2.0, 1.0),
    );
    compile(&shape, w).unwrap();
}

#[expectation_test]
fn cl_for_rect(provider: Provider) {
    use ocaml::Rect;
    let w = provider.text_writer("out.c");
    let shape = Shape::Terminal(Terminal::Rect(Rect {
        x: 0.0,
        y: 5.0,
        w: 10.0,
        h: 20.0,
    }));
    compile(&shape, w).unwrap();
}

#[expectation_test]
fn cl_for_field(provider: Provider) {
    let w = provider.text_writer("out.c");
    let shape = Shape::Terminal(Terminal::Field(5));
    compile(&shape, w).unwrap();
}

#[expectation_test]
fn cl_for_intersection(provider: Provider) {
    let w = provider.text_writer("out.c");
    let shape = Shape::Intersection(vec![
        Shape::Terminal(Terminal::Field(5)),
        Shape::Terminal(Terminal::Field(6)),
    ]);
    compile(&shape, w).unwrap();
}

#[expectation_test]
fn cl_for_union(provider: Provider) {
    let w = provider.text_writer("out.c");
    let shape = Shape::Union(vec![
        Shape::Terminal(Terminal::Field(5)),
        Shape::Terminal(Terminal::Field(6)),
    ]);
    compile(&shape, w).unwrap();
}

#[expectation_test]
fn cl_for_not(provider: Provider) {
    let w = provider.text_writer("out.c");
    let shape = Shape::Not(Box::new(Shape::Terminal(Terminal::Field(5))));
    compile(&shape, w).unwrap();
}

#[expectation_test]
fn cl_for_modulate(provider: Provider) {
    let w = provider.text_writer("out.c");
    let shape = Shape::Modulate(Box::new(Shape::Terminal(Terminal::Field(5))), 23.53);
    compile(&shape, w).unwrap();
}
