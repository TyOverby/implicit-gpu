use lines::LineType;
use lines::util::geom;

pub fn output_svg_linetype<'a, I>(file: ::std::fs::File, lines: I)
where I: Iterator<Item = &'a LineType> {
    use vectorphile::Canvas;
    use vectorphile::backend::{Command, DrawBackend, DrawOptions};
    use vectorphile::svg::SvgBackend;
    let mut canvas = Canvas::new(SvgBackend::new(file).unwrap());

    for line in lines {
        let (pts, restitch) = match line {
            &LineType::Joined(ref pts) => {
                canvas
                    .apply(Command::StartShape(DrawOptions::stroked((0, 0, 0), 0.2)))
                    .unwrap();
                (&pts[..], true)
            }
            &LineType::Unjoined(ref pts) => {
                canvas
                    .apply(Command::StartShape(DrawOptions::stroked((255, 0, 0), 0.2)))
                    .unwrap();
                (&pts[..], false)
            }
        };
        if pts.len() > 0 {
            canvas
                .apply(Command::MoveTo {
                    x: pts[0].x as f64,
                    y: pts[0].y as f64,
                })
                .unwrap();
            canvas
                .apply_all(
                    pts.iter()
                        .skip(1)
                        .map(|pt| Command::LineTo { x: pt.x as f64, y: pt.y as f64 }),
                )
                .unwrap();

            if restitch {
                canvas
                    .apply(Command::LineTo {
                        x: pts[0].x as f64,
                        y: pts[0].y as f64,
                    })
                    .unwrap();
            }
        }
        canvas.apply(Command::EndShape).unwrap();
    }
    canvas.close().unwrap();
}

pub fn output_svg_lines<'a, I>(file: ::std::fs::File, lines: I)
where I: Iterator<Item = geom::Line> {
    use vectorphile::Canvas;
    use vectorphile::backend::{Command, DrawBackend, DrawOptions};
    use vectorphile::svg::SvgBackend;
    let mut canvas = Canvas::new(SvgBackend::new(file).unwrap());

    canvas
        .apply(Command::StartShape(DrawOptions::stroked((0, 0, 0), 0.2)))
        .unwrap();

    for geom::Line(p1, p2) in lines {
        canvas
            .apply(Command::MoveTo { x: p1.x as f64, y: p1.y as f64 })
            .unwrap();
        canvas
            .apply(Command::LineTo { x: p2.x as f64, y: p2.y as f64 })
            .unwrap();
    }

    canvas.apply(Command::EndShape).unwrap();

    canvas.close().unwrap();
}
