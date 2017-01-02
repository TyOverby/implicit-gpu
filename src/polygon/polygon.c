float position(float x, float y, float x1, float y1, float x2, float y2) {
    return sign((x2 - x1) * (y - y1) - (y2 - y1) * (x - x1));
}

float dist_to_line(float x, float y, float x1, float y1, float x2, float y2) {
    float A = x - x1;
    float B = y - y1;
    float C = x2 - x1;
    float D = y2 - y1;

    float dot = A * C + B * D;
    float len_sq = C * C + D * D;
    float param = -1;

    if (len_sq != 0)
    {
        param = dot / len_sq;
    }

    float xx;
    float yy;

    if (param < 0) {
        xx = x1;
        yy = y1;
    }
    else if (param > 1) {
        xx = x2;
        yy = y2;
    }
    else {
        xx = x1 + param * C;
        yy = y1 + param * D;
    }

    float dx = x - xx;
    float dy = y - yy;

    return sqrt(dx * dx + dy * dy);
}

__kernel void apply(__global float* buffer, size_t width, __global float* xs, __global float* ys, size_t count) {
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t pos = x + y * width;

    float x_s = (float) x;
    float y_s = (float) y;

    if (count < 2) {
        buffer[pos] = NAN;
        return;
    }

    float minimum = INFINITY;
    float sign_of_min = 0.0;

    for(size_t i = 0; i < count; i += 2) {
        float x1 = xs[i];
        float y1 = ys[i];
        float x2 = xs[i + 1];
        float y2 = ys[i + 1];

        float new = dist_to_line(x_s, y_s, x1, y1, x2, y2);
        float pos = position(x_s, y_s, x1, y1, x2, y2);

        if (x == 200 && y == 800) {
            printf("(%f, %f) -> (%f, %f): %f by %f\n", x1, y1, x2, y2, new, pos);
        }

        float new_abs = fabs(new);
        float min_abs = fabs(minimum);

        if (new_abs < min_abs) {
            minimum = copysign(new, pos);
            sign_of_min = pos;
        }

        if (new_abs == min_abs && sign_of_min != pos) {
            minimum = copysign(minimum, -1);
        }
    }

    buffer[pos] = minimum;
}
