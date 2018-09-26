float sample(
    __global float *input,
    float x,
    float y,
    long width,
    long height
) {
    if (x < 0.0 || y < 0.0 || x >= width || y >= height) {
        return INFINITY;
    }


    float x_floor;
    float x_rem = fract(x, &x_floor);

    float y_floor;
    float y_rem = fract(y, &y_floor);

    /*
    float x_rem = 0.0;
    float y_rem = 0.0;
    float x_floor = floor(x);
    float y_floor = floor(y);
    */

    float nw = input[(long)(x_floor) + (long)(y_floor) * width];
    float ne = input[((long)(x_floor) + 1) + (long)(y_floor) * width];
    float n_d = (nw * (1 - x_rem)) + (ne * x_rem);

    float sw = input[(long)(x_floor) + ((long)(y_floor) + 1) * width];
    float se = input[((long)(x_floor) + 1) + ((long)(y_floor) + 1) * width];
    float s_d = (sw * (1 - x_rem)) + (se * x_rem);

    float avg = (n_d * (1 - y_rem)) + (s_d * y_rem);
    return avg;
}

__kernel void apply(
    __global float *buffer,
    __global float *input,
    float dx,
    float dy,
    long width,
    long height
)
{
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t pos = x + y * width;
    float x_s = (float) x;
    float y_s = (float) y;

    int travel = (int) ceil(max(fabs(dx), fabs(dy)));
    float best = INFINITY;
    for (int i = 0; i < travel; i++) {
        float dist_traveled = ((float) i) / ((float) travel);
        float q_x = x_s - dx * dist_traveled;
        float q_y = y_s - dy * dist_traveled;
        best = min(best, sample(input, q_x, q_y, width, height));
    }

    buffer[pos] = best;
}
