use image_crate::{ImageBuffer, ImageRgb8, Rgb, PNG, DynamicImage};

use opencl::FieldBuffer;
use std::f32::{INFINITY, NEG_INFINITY};
use std::io::Write;

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

fn save_image<W: Write>(
    samples: &[f32],
    width: usize,
    mut writer: W,
    color_mode: ColorMode,
) {
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
