use ocl::{ProQue, Buffer};

const PROGRAM: &'static str = r#"
float dist_to_point(float x1, float y1, float x2, float y2) {
    float dx = x1 - x2;
    float dy = y1 - y2;
    return sqrt(dx * dx + dy * dy);
}

float dist_to_line(float x, float y, float p1x, float p1y, float p2x, float p2y) {
    float l2 = dist_to_point(p1x, p1y, p2x, p2y);
    if (l2 == 0.0) {
        return dist_to_point(x, y, p1x, p1y);
    }

    float temp1 = p2y - p1y;
    float temp2 = p2x - p1x;
    float temp3 = y - p1y;
    float temp4 = x - p1x;

    float t = (temp4 * temp2 + temp3 * temp1) / l2;
    float s =  temp2 * temp3 - temp4 * temp1;

    float invert = -1.0;
    if (s < 0.0) {
        invert = 1.0;
    }

    if (t < 0.0) {
        return dist_to_point(x, y, p1x, p1y) * invert;
    } else if (t > 1.0) {
        return dist_to_point(x, y, p2x, p2y) * invert;
    } else {
        float npx = p1x + t * (p2x - p1x);
        float npy = p1y + t * (p2y - p1y);
        return dist_to_point(x, y, npx, npy) * invert;
    }
}

__kernel void apply(__global float* buffer, size_t width, __global float* xs, __global float* ys, size_t count) {
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t pos = x + y * width;

    float x_s = (float) x;
    float y_s = (float) y;

    float minimum = dist_to_line(
        x_s, y_s,
        xs[count - 1], ys[count-1],
        xs[0], ys[0]);

    for(size_t i = 0; i < count - 1; i++) {
        float new = dist_to_line(
            x_s, y_s,
            xs[i], ys[i],
            xs[i + 1], ys[i + 1]);
        float new_abs = fabs(new);
        float min_abs = fabs(minimum);
        if (new_abs < min_abs) {
            minimum = new;
        }
    }

    buffer[pos] = minimum;
}
"#;

const DIM: usize = 100;

pub fn run_poly(xs: &[f32], ys: &[f32], width: usize, height: usize) -> Buffer<f32> {
    assert_eq!(xs.len(), ys.len());
    let len = xs.len();

    let proq = ProQue::builder().src(PROGRAM).dims([width, height]).build().unwrap();
    let buf = proq.create_buffer::<f32>().unwrap();
    let kernel = proq.create_kernel("apply").unwrap();

    let copy = ::ocl::flags::MEM_COPY_HOST_PTR;
    let xs_buf = Buffer::new(proq.queue(), Some(copy), [len], Some(xs)).unwrap();
    let ys_buf = Buffer::new(proq.queue(), Some(copy), [len], Some(ys)).unwrap();

    kernel
        .arg_buf(&buf)
        .arg_scl(DIM)
        .arg_buf(&xs_buf)
        .arg_buf(&ys_buf)
        .arg_scl(len)
        .enq().unwrap();

    buf
}
