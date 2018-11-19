#![cfg_attr(not(test), allow(dead_code))]

use euclid::TypedPoint2D;
use geometry::PathSegment;
use image::{DynamicImage, ImageBuffer, ImageRgb8, Rgb, PNG};

use opencl::FieldBuffer;
use std::f32::{INFINITY, NEG_INFINITY};
use std::io::{Result as IoResult, Write};

#[derive(Copy, Clone)]
pub enum ColorMode {
    BlackAndWhite,
    Debug,
}

pub fn save_field_buffer<W: Write>(buffer: &FieldBuffer, writer: W, color_mode: ColorMode) {
    let _guard = ::flame::start_guard("save_field_buffer");
    let samples = ::flame::span_of("fetch values", || buffer.values());
    save_image(&samples, buffer.width(), writer, color_mode);
}

fn save_image<W: Write>(samples: &[f32], width: usize, mut writer: W, color_mode: ColorMode) {
    let _guard = ::flame::start_guard("save_image");
    let mut min = INFINITY;
    let mut max = NEG_INFINITY;

    for &sample in samples {
        min = min.min(sample);
        max = max.max(sample);
    }

    let mut buf = ImageBuffer::new(width as u32, (samples.len() / width) as u32);
    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let sample = samples[x as usize + y as usize * width];
        let sample_abs = sample.abs();
        let color = match (color_mode, sample >= 0.0, sample_abs < 1.0) {
            (ColorMode::BlackAndWhite, true, true) => {
                let v = 127 - (sample_abs * 127.0) as u8;
                [v, v, v]
            }
            (ColorMode::BlackAndWhite, false, true) => {
                let v = 127 + (sample_abs * 127.0) as u8;
                [v, v, v]
            }
            (ColorMode::BlackAndWhite, true, false) => [0, 0, 0],
            (ColorMode::BlackAndWhite, false, false) => [255, 255, 255],
            (ColorMode::Debug, _, _) if sample == 0.0 => [0, 255, 0],
            (ColorMode::Debug, true, _) => {
                let compressed = sample / max;
                let rounded = (compressed * 255.0) as u8;
                [0, 0, 255 - rounded]
            }
            (ColorMode::Debug, false, _) => {
                let compressed = sample / min;
                let rounded = (compressed * 255.0) as u8;
                [255 - rounded, 0, 0]
            }
        };

        *pixel = Rgb(color);
    }

    let d: DynamicImage = ImageRgb8(buf);
    d.write_to(&mut writer, PNG).unwrap();
}

pub fn svg_path_segments<W: Write>(out: W, extracted: &[PathSegment]) -> IoResult<()> {
    use vectorphile::{backend::*, svg::*, *};
    let svgb = SvgBackend::new(out)?;
    let mut canvas = Canvas::new(svgb);

    let mut additive: Vec<Vec<_>> = vec![];
    let mut subtractive: Vec<Vec<_>> = vec![];
    let mut non_filled: Vec<Vec<_>> = vec![];
    for segment in extracted.iter().cloned() {
        if !segment.closed {
            non_filled.push(segment.into_iter().collect());
        } else if is_clockwise(&segment.path) {
            additive.push(segment.into_iter().collect());
        } else {
            subtractive.push(segment.into_iter().collect());
        }
    }

    canvas.draw_holy_polygon(additive, subtractive, DrawOptions::filled((0, 0, 0)))?;
    for segment in non_filled {
        let mut is_first = true;
        canvas.apply(Command::StartShape(DrawOptions::default()))?;
        for point in segment {
            if is_first {
                canvas.apply(Command::MoveTo {
                    x: point.x as f64,
                    y: point.y as f64,
                })?;
                is_first = false;
            } else {
                canvas.apply(Command::LineTo {
                    x: point.x as f64,
                    y: point.y as f64,
                })?;
            }
        }
        canvas.apply(Command::CloseShape)?;
    }
    canvas.close()
}

pub fn is_clockwise<K>(pts: &[TypedPoint2D<f32, K>]) -> bool {
    assert!(pts.len() > 0);
    let mut total = 0.0f32;
    for slice in pts.windows(2) {
        let a = slice[0];
        let b = slice[1];
        total += (b.x - a.x) * (b.y + a.y);
    }
    {
        let a = pts[0];
        let b = pts[pts.len() - 1];
        total += (b.x - a.x) * (b.y + a.y);
    }
    total > 0.0
}
pub fn print_path_segments<W: Write>(mut out: W, extracted: &[PathSegment]) {
    writeln!(out, "{} line segments", extracted.len()).unwrap();
    for (i, segment) in extracted.iter().enumerate() {
        writeln!(out).unwrap();
        writeln!(out, "Line Segment {} ", i).unwrap();
        writeln!(out, "{} points", segment.path.len()).unwrap();
        writeln!(out, "Clockwise? {}", is_clockwise(&segment.path[..])).unwrap();
        for point in &segment.path[..] {
            writeln!(out, "{:?}", point).unwrap();
        }
    }
}
