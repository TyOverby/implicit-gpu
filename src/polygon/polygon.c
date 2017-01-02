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

    if (count < 2) {
        buffer[pos] = NAN;
        return;
    }

    float minimum = INFINITY;

    for(size_t i = 0; i < count; i += 2) {
        if (pos == 0) {
            printf("(%f, %f) -> (%f, %f)\n",
                    xs[i], ys[i],
                    xs[i + 1], ys[i +1]);
        }

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
