use image_crate::{ImageBuffer, ImageRgb8, Rgb, PNG};

use opencl::FieldBuffer;
use std::f32::{INFINITY, NEG_INFINITY};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Copy, Clone)]
pub enum ColorMode {
    BlackAndWhite,
    Debug,
}

pub fn save_field_buffer<P: AsRef<Path>>(buffer: &FieldBuffer, name: P, color_mode: ColorMode) {
    let _guard = ::flame::start_guard("save_field_buffer");
    let samples = ::flame::span_of("fetch values", || buffer.values());
    save_image(&samples, buffer.width(), name, color_mode);
}

pub fn save_image<P: AsRef<Path>>(
    samples: &[f32],
    width: usize,
    file_name: P,
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

    ::std::fs::create_dir_all({
        let mut dir = file_name.as_ref().to_path_buf();
        dir.pop();
        dir
    }).unwrap();
    let fout = File::create(file_name).unwrap();
    let mut fout = BufWriter::new(fout);
    ImageRgb8(buf).save(&mut fout, PNG).unwrap();
}
