use expectation::extensions::*;
use ocaml::*;
use std::cell::Cell;
use std::collections::HashSet;
use std::io::{Result as IoResult, Write};
use std::rc::Rc;

pub struct CompileResult<W> {
    dependencies: Vec<Id>,
    text: W,
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
    fn gen_2(&self, n1: &str, n2: &str) -> (String, String) {
        (self.gen(n1), self.gen(n2))
    }
    fn gen_3(&self, n1: &str, n2: &str, n3: &str) -> (String, String, String) {
        (self.gen(n1), self.gen(n2), self.gen(n3))
    }
}

pub fn compile<W: Write>(shape: &Shape, mut writer: W) -> CompileResult<W> {
    let mut deps = HashSet::new();
    let result = compile_impl(shape, &mut writer, &mut deps, &NameGen::new()).unwrap();
    CompileResult {
        text: writer,
        dependencies: deps.into_iter().collect(),
    }
}

pub fn get_xy(matrix: &Matrix) -> (String, String) {
    if !matrix.approx_eq(&Matrix::identity()) {
        panic!("Only identity matrixes in circles are supported at the moment");
    }
    return ("x".into(), "y".into());
}

fn compile_impl<W: Write>(
    shape: &Shape,
    mut out: W,
    deps: &mut HashSet<Id>,
    namegen: &NameGen,
) -> IoResult<String> {
    use ocaml::Shape::*;
    writeln!(out, "")?;
    match shape {
        Terminal(BasicTerminals::Circle(Circle { x, y, r, mat })) => {
            let (res, dx, dy) = namegen.gen_3("circle", "dx", "dy");
            let (mx, my) = get_xy(mat);
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
        Terminal(BasicTerminals::Rect(Rect { x, y, w, h, mat })) => {
            let (x, y, w, h) = (*x, *y, *w, *h);
            let res = namegen.gen("rect");
            let (mx, my) = get_xy(mat);
            writeln!(out, "// Rect {}", res);
            writeln!(out, "float {} = INFINITY;", res);
            {
                let mut dist_to_line = |ax: f32, ay: f32, bx: f32, by: f32| {
                    writeln!(
                        out,
                        "{res} = min({res}, dist_to_line({mx}, {my}, {ax}, {ay}, {bx}, {by}))",
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
            writeln!(out, "{res} = -{res};", res = res);
            writeln!(out, "// End Rect {}", res);

            Ok(res)
        }
        Terminal(BasicTerminals::Field(id)) => {
            deps.insert(*id);
            let res = namegen.gen("field");
            writeln!(out, "float {res} = field__{id}[x][y];", res = res, id = id)?;
            Ok(res)
        }
        _ => unimplemented!(),
    }
}

expectation_test!{
    fn expectation_test_cl_for_circle(provider: &mut ::expectation::Provider) {
        use euclid::*;
        let w = provider.text_writer("out.c");
        let shape = Shape::Terminal(BasicTerminals::Circle(Circle {
            x: 0.0,
            y: 5.0,
            r: 10.0,
            mat: Transform2D::identity(),
        }));
        compile(&shape, w);
    }
}

expectation_test!{
    fn expectation_test_cl_for_rect(provider: &mut ::expectation::Provider) {
        use euclid::*;
        use ocaml::Rect;
        let w = provider.text_writer("out.c");
        let shape = Shape::Terminal(BasicTerminals::Rect(Rect {
            x: 0.0,
            y: 5.0,
            w: 10.0,
            h: 20.0,
            mat: Transform2D::identity(),
        }));
        compile(&shape, w);
    }
}
