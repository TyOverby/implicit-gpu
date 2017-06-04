use std::path::Path;
use output::*;
use svg::Document;
use svg::node::element::Path as SvgPath;
use svg::node::element::path::Data;

fn add_line(doc: &mut Document, line: Vec<(f32, f32)>) {
    use svg::Node;
    let mut points = line.into_iter();
    let first = points.next().unwrap();
    let data = Data::new();
    let data = data.move_to(first);
    let data = points.fold(data, |data, pt| data.line_to(pt));
    let data = data.line_to(first);
    let path = SvgPath::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);
    doc.append(path);
}

pub fn write_out<P: AsRef<Path>>(path: P, mut out: OutputScene) {
    assert!(out.figures.len() == 1, "only support 1 figure per scene");
    let figure = out.figures.pop().unwrap();

    let mut document = Document::new();

    for shape in figure.shapes {
        match shape.lines {
            LineGroup::Polygon { filled, additive, subtractive } => {
                assert!(subtractive.len() == 0, "no support for subtractive poly lines");
                assert!(!filled, "no support for filled");
                for line in additive.into_iter() {
                    add_line(&mut document, line);
                }
            }
            LineGroup::Lines(lines) => {
                for line in lines {
                    add_line(&mut document, line);
                }
            }
        }
    }

    ::svg::save(path, &document).unwrap();
}
