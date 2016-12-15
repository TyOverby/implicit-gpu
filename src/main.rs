extern crate ocl;
extern crate implicit_gpu;

use ocl::ProQue;
use implicit_gpu::image::{ColorMode, save_image};


const PROGRAM: &'static str = r#"
    __kernel void circle(__global float* buffer, size_t width, float size) {
        size_t x = get_global_id(0);
        size_t y = get_global_id(1);
        size_t pos = x + y * width;

        float x_s = (float) x;
        float y_s = (float) y;

        float value;
        {
            float dx = x_s - 50.0;
            float dy = y_s - 50.0;
            value = sqrt(dx * dx + dy * dy) - size;
        }

        buffer[pos] = value;
    }
"#;

const DIM: usize = 100;
const SIZE: f32 = 20.0;

fn main() {
    let proq = ProQue::builder().src(PROGRAM).dims([DIM, DIM]).build().unwrap();
    let buf = proq.create_buffer::<f32>().unwrap();
    let kernel = proq.create_kernel("circle").unwrap();

    kernel.arg_buf(&buf).arg_scl(DIM).arg_scl(SIZE).enq().unwrap();
    let mut vec = vec![5.0f32; buf.len()];
    buf.read(&mut vec).enq().unwrap();

    save_image(&vec, DIM, "out.png", ColorMode::BlackAndWhite);
}
