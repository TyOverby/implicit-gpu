use geometry::Point;
use line_stitch::PathSegment;

pub fn output_svg_linetype<'a, I, S: 'static>(file: ::std::fs::File, lines: I)
where
    I: Iterator<Item = &'a PathSegment<S>>,
{
    use vectorphile::backend::{Command, DrawBackend, DrawOptions};
    use vectorphile::svg::SvgBackend;
    use vectorphile::Canvas;
    let mut canvas = Canvas::new(SvgBackend::new(file).unwrap());

    for &PathSegment {
        ref path, closed, ..
    } in lines
    {
        if closed {
            canvas
                .apply(Command::StartShape(DrawOptions::stroked((0, 0, 0), 0.2)))
                .unwrap();
        } else {
            canvas
                .apply(Command::StartShape(DrawOptions::stroked((255, 0, 0), 0.2)))
                .unwrap();
        }

        if path.len() > 0 {
            canvas
                .apply(Command::MoveTo {
                    x: path[0].x as f64,
                    y: path[0].y as f64,
                })
                .unwrap();
            canvas
                .apply_all(path.iter().skip(1).map(|pt| Command::LineTo {
                    x: pt.x as f64,
                    y: pt.y as f64,
                }))
                .unwrap();

            if closed {
                canvas
                    .apply(Command::LineTo {
                        x: path[0].x as f64,
                        y: path[0].y as f64,
                    })
                    .unwrap();
            }
        }
        canvas.apply(Command::EndShape).unwrap();
    }
    canvas.close().unwrap();
}

pub fn output_svg_lines<'a, I>(file: ::std::fs::File, lines: I)
where
    I: Iterator<Item = (Point, Point)>,
{
    use vectorphile::backend::{Command, DrawBackend, DrawOptions};
    use vectorphile::svg::SvgBackend;
    use vectorphile::Canvas;
    let mut canvas = Canvas::new(SvgBackend::new(file).unwrap());

    canvas
        .apply(Command::StartShape(DrawOptions::stroked((0, 0, 0), 0.2)))
        .unwrap();

    for (p1, p2) in lines {
        canvas
            .apply(Command::MoveTo {
                x: p1.x as f64,
                y: p1.y as f64,
            })
            .unwrap();
        canvas
            .apply(Command::LineTo {
                x: p2.x as f64,
                y: p2.y as f64,
            })
            .unwrap();
    }

    canvas.apply(Command::EndShape).unwrap();

    canvas.close().unwrap();
}
