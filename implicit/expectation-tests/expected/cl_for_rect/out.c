float2 dist_to_line_comp(float x, float y, float x1, float y1, float x2, float y2)
{
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

    float pos = (x2 - x1) * (y - y1) - (y2 - y1) * (x - x1);
    if (pos == 0) {
        pos = -1.0;
    }

    if (param < 0)
    {
        xx = x1;
        yy = y1;
    }
    else if (param > 1)
    {
        xx = x2;
        yy = y2;
    }
    else
    {
        xx = x1 + param * C;
        yy = y1 + param * D;
    }

    float dx = x - xx;
    float dy = y - yy;

    return (float2)(sqrt(dx * dx + dy * dy), sign(pos));
}

float dist_to_line(float x, float y, float x1, float y1, float x2, float y2)
{
    return dist_to_line_comp(x, y, x1, y1, x2, y2).x;
}

float position(float x, float y, float x1, float y1, float x2, float y2)
{
    return dist_to_line_comp(x, y, x1, y1, x2, y2).y;
}

__kernel void apply(__global float* buffer, ulong width) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;
  float x_s = (float) x;
  float y_s = (float) y;

// Rect _rect_0
float _rect_0 = INFINITY;
_rect_0 = min(_rect_0, dist_to_line(x_s, y_s, 0, 5, 10, 5));
_rect_0 = min(_rect_0, dist_to_line(x_s, y_s, 10, 5, 10, 25));
_rect_0 = min(_rect_0, dist_to_line(x_s, y_s, 10, 25, 0, 25));
_rect_0 = min(_rect_0, dist_to_line(x_s, y_s, 0, 25, 0, 5));
if (x_s > 0 && y_s > 5 && x_s < (0 + 10) && y_s < (5 + 20))
_rect_0 = -_rect_0;
// End Rect _rect_0
buffer[pos] = _rect_0;
}
