
__kernel void apply(__global float *buffer, ulong width, __global float *lines, ulong count, float dx, float dy)
{
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t pos = x + y * width;

    if (pos == 0)
    {
        //printf("OPENCL: pos: %d | width: %d | count: %d\n", pos, width, count);
    }

    float x_s = (float)x - dx;
    float y_s = (float)y - dy;

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
