
__kernel void apply_no_sign(
    __global float *buffer,
    ulong width,
    __global float *lines,
    ulong count,
    float m11, float m12,
    float m21, float m22,
    float m31, float m32
)
{
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t pos = x + y * width;

    float x_s = (float)x * m11 + (float)y * m21 + m31;
    float y_s = (float)x * m12 + (float)y * m22 + m32;

    if (count < 2)
    {
        buffer[pos] = NAN;
        return;
    }

    float minimum = INFINITY;
    float sign_of_min = 0.0;

    for (size_t i = 0; i < count; i += 4)
    {
        float x1 = lines[i + 0];
        float y1 = lines[i + 1];
        float x2 = lines[i + 2];
        float y2 = lines[i + 3];

        if (x1 == x2 && y1 == y2) {
            continue;
        }

        if (isnan(x1) ||
            isnan(x2) ||
            isnan(y1) ||
            isnan(y2)) {
            break;
        }

        float new = dist_to_line(x_s, y_s, x1, y1, x2, y2);
        float pos = sign(position(x_s, y_s, x1, y1, x2, y2));

        float new_abs = fabs(new);
        float min_abs = fabs(minimum);

        if (new_abs < min_abs)
        {
            minimum = copysign(new, pos);
            sign_of_min = pos;
        }

        if (new_abs == min_abs && sign_of_min != pos)
        {
            minimum = copysign(minimum, -1);
        }
    }

    buffer[pos] = -minimum;
}

__kernel void apply_with_sign(
    __global float *buffer,
    __global float *signbuffer,
    ulong width,
    __global float *lines,
    ulong count,
    float m11, float m12,
    float m21, float m22,
    float m31, float m32
)
{
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t pos = x + y * width;

    float x_s = (float)x;
    float y_s = (float)y;

    if (count < 2)
    {
        buffer[pos] = NAN;
        return;
    }

    float minimum_abs = INFINITY;

    for (size_t i = 0; i < count; i += 4)
    {
        float x1 = lines[i + 0];
        float y1 = lines[i + 1];
        float x2 = lines[i + 2];
        float y2 = lines[i + 3];

        if (x1 == x2 && y1 == y2) {
            continue;
        }

        if (isnan(x1) ||
            isnan(x2) ||
            isnan(y1) ||
            isnan(y2)) {
            break;
        }

        float new = dist_to_line(x_s, y_s, x1, y1, x2, y2);
        float new_abs = fabs(new);

        if (new_abs < minimum_abs) {
            minimum_abs = new_abs;
        }
    }

    buffer[pos] = copysign(minimum_abs, signbuffer[pos]);
}
