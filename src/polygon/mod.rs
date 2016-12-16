use ocl::{ProQue, Buffer};

const PROGRAM: &'static str = r#"
__kernel void apply(__global float* buffer, size_t width) {
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t pos = x + y * width;

    float x_s = (float) x;
    float y_s = (float) y;

    float minimum = INFINITY;


    buffer[pos] = minimum;
}
"#;

const DIM: usize = 256;

pub fn run_poly(points: &[f32]) -> Buffer<f32> {
    let proq = ProQue::builder().src(PROGRAM).dims([DIM, DIM]).build().unwrap();
    let buf = proq.create_buffer::<f32>().unwrap();
    let kernel = proq.create_kernel("apply").unwrap();

    kernel.arg_buf(&buf).arg_scl(DIM).enq().unwrap();

    buf
}
