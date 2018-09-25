#[cfg(test)]
use debug::*;
#[cfg(test)]
use expectation::{extensions::*, Provider};
use expectation_plugin::expectation_test;

pub use super::super::noise::get_noise;

#[expectation_test]
fn exec_noise(provider: Provider) {
    use ocaml::Matrix;
    use opencl::*;

    let ctx = OpenClContext::default();
    let buffer = get_noise(&ctx, 20, 20, 0.5, Matrix::identity());

    let w_color = provider.png_writer("out.color.png");
    save_field_buffer(&buffer, w_color, ColorMode::Debug);

    let w_bw = provider.png_writer("out.bw.png");
    save_field_buffer(&buffer, w_bw, ColorMode::BlackAndWhite);
}
