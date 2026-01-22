use super::super::provider::{Provider, WriteRequester};
use super::super::*;
use expectation_shared::filesystem::ReadSeek;

use std::io::{BufReader, Result as IoResult};
use std::path::Path;

use image::*;

pub trait ImageDiffExtension {
    fn png_writer<N>(&self, filename: N) -> Writer
    where
        N: AsRef<Path>;

    fn rgb_image<N>(&self, filename: N, image: RgbImage) -> IoResult<()>
    where
        N: AsRef<Path>,
    {
        let mut w = self.png_writer(filename);
        let dyn_image = DynamicImage::ImageRgb8(image);
        dyn_image.write_to(&mut w, ImageOutputFormat::PNG).unwrap();
        Ok(())
    }

    fn rgba_image<N>(&self, filename: N, image: RgbaImage) -> IoResult<()>
    where
        N: AsRef<Path>,
    {
        let mut w = self.png_writer(filename);
        let dyn_image = DynamicImage::ImageRgba8(image);
        dyn_image.write_to(&mut w, ImageOutputFormat::PNG).unwrap();
        Ok(())
    }
}

impl ImageDiffExtension for Provider {
    fn png_writer<S>(&self, filename: S) -> Writer
    where
        S: AsRef<Path>,
    {
        self.custom_test(
            filename,
            |a, b| image_eq(a, b),
            |a, b, c, d| image_diff(a, b, c, d),
        )
    }
}

fn image_eq<R1: ReadSeek, R2: ReadSeek>(r1: R1, r2: R2) -> IoResult<bool> {
    let mut r1 = BufReader::new(r1);
    let mut r2 = BufReader::new(r2);

    let i1 = load(&mut r1, ImageFormat::PNG).unwrap();
    let i2 = load(&mut r2, ImageFormat::PNG).unwrap();

    match (i1, i2) {
        (DynamicImage::ImageRgb8(i1), DynamicImage::ImageRgb8(i2)) => {
            if i1.width() != i2.width() || i1.height() != i2.height() {
                return Ok(false);
            }
            for x in 0..i1.width() {
                for y in 0..i1.height() {
                    if i1.get_pixel(x, y) != i2.get_pixel(x, y) {
                        return Ok(false);
                    }
                }
            }
        }
        (DynamicImage::ImageRgba8(i1), DynamicImage::ImageRgba8(i2)) => {
            if i1.width() != i2.width() || i1.height() != i2.height() {
                return Ok(false);
            }
            for x in 0..i1.width() {
                for y in 0..i1.height() {
                    if i1.get_pixel(x, y) != i2.get_pixel(x, y) {
                        return Ok(false);
                    }
                }
            }
        }
        (_, _) => return Ok(false),
    }

    Ok(true)
}

fn _add_extension(p: &Path, new_ext: &str) -> PathBuf {
    let old_ext = match p.extension() {
        Some(e) => e.to_string_lossy().into_owned(),
        None => "".to_owned(),
    };
    p.with_extension(format!("{}{}", old_ext, new_ext))
}

fn image_diff<R1: ReadSeek, R2: ReadSeek>(
    r1: R1,
    r2: R2,
    path: &Path,
    write_requester: &mut WriteRequester,
) -> IoResult<()> {
    let mut r1 = BufReader::new(r1);
    let mut r2 = BufReader::new(r2);

    let i1 = load(&mut r1, ImageFormat::PNG).unwrap();
    let i2 = load(&mut r2, ImageFormat::PNG).unwrap();

    match (i1, i2) {
        (DynamicImage::ImageRgb8(i1), DynamicImage::ImageRgb8(i2)) => {
            if i1.width() != i2.width() || i1.height() != i2.height() {
                return write_requester.request(path.join("img-size.txt"), |w| {
                    writeln!(w, "image dimensions are different")?;
                    writeln!(w, "actual:   width: {} height: {}", i1.width(), i1.height())?;
                    writeln!(w, "expected: width: {} height: {}", i2.width(), i2.height())?;
                    Ok(())
                });
            }
            // TODO: implement image diffing
            Ok(())
            //panic!("images arent pixel-by-pixel equal");
        }
        (DynamicImage::ImageRgba8(i1), DynamicImage::ImageRgba8(i2)) => {
            if i1.width() != i2.width() || i1.height() != i2.height() {
                return write_requester.request(path.join("img-size.txt"), |w| {
                    writeln!(w, "image dimensions are different")?;
                    writeln!(w, "actual:   width: {} height: {}", i1.width(), i1.height())?;
                    writeln!(w, "expected: width: {} height: {}", i2.width(), i2.height())?;
                    Ok(())
                });
            }
            // TODO: implement image diffing
            Ok(())
            //panic!("images arent pixel-by-pixel equal");
        }
        (DynamicImage::ImageRgb8(_), DynamicImage::ImageRgba8(_)) => {
            return write_requester.request(path.join("img-format.txt"), |w| {
                writeln!(w, "image formats are different");
                writeln!(w, "actual:   RGB8");
                writeln!(w, "expected: RGBA8 (Alpha)");
                Ok(())
            });
        }
        (DynamicImage::ImageRgba8(_), DynamicImage::ImageRgb8(_)) => {
            return write_requester.request(path.join("img-format.txt"), |w| {
                writeln!(w, "image formats are different");
                writeln!(w, "actual:   RGBA8 (Alpha)");
                writeln!(w, "expected: RGB8");
                Ok(())
            });
        }
        (_, _) => panic!(),
    }
}
