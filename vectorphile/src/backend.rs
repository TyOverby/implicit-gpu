pub struct DrawOptions {
    pub fill_color: Option<(u8, u8, u8)>,
    pub stroke_color: Option<(u8, u8, u8)>,
    pub stroke_size: f64,
}

impl DrawOptions {
    pub fn default() -> DrawOptions {
        DrawOptions {
            fill_color: None,
            stroke_color: Some((0, 0, 0)),
            stroke_size: 2.0,
        }
    }

    pub fn stroked((r, g, b): (u8, u8, u8), size: f64) -> DrawOptions {
        DrawOptions {
            fill_color: None,
            stroke_color: Some((r, g, b)),
            stroke_size: size,
        }
    }

    pub fn filled((r, g, b): (u8, u8, u8)) -> DrawOptions {
        DrawOptions {
            fill_color: Some((r, g, b)),
            stroke_color: None,
            stroke_size: 0.0,
        }
    }
}

pub enum Command {
    StartShape(DrawOptions),
    MoveTo {
        x: f64,
        y: f64
    },
    LineTo {
        x: f64,
        y: f64
    },
    CubicCurveTo {
        // Control Point 1
        cx1: f64,
        cy1: f64,
        // Control Point 2
        cx2: f64,
        cy2: f64,
        // End Point
        x: f64,
        y: f64,
    },
    QuadraticCurveTo {
        // Control Point
        cx: f64,
        cy: f64,
        // End Point
        x: f64,
        y: f64,
    },
    // https://www.w3.org/TR/SVG/implnote.html#ArcImplementationNotes
    ArcTo {
        // Radius of of Elipse
        rx: f64,
        ry: f64,
        // x axis rotation
        rotation: f64,
        // Which path to use
        large_arc: bool,
        sweep: bool,
        // End Point
        x: f64,
        y: f64,
    },
    CloseShape,
    EndShape,
}

pub trait DrawBackend {
    type Error;

    fn apply(&mut self, command: Command) -> Result<(), Self::Error>;
    fn apply_all<I: Iterator<Item=Command>>(&mut self, commands: I) -> Result<(), Self::Error> {
        for command in commands {
            self.apply(command)?;
        }
        Ok(())
    }
    fn close(self) -> Result<(), Self::Error>;
}
