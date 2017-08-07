float dist_to_line(float x, float y, float x1, float y1, float x2, float y2)
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

    return sqrt(dx * dx + dy * dy);
}

float position(float x, float y, float x1, float y1, float x2, float y2)
{
    return (x2 - x1) * (y - y1) - (y2 - y1) * (x - x1);
}

__kernel void apply(__global float* buffer, ulong width) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;

  float x_0 = x_s - 2;
  float y_1 = y_s - 2;

  float circle_2;
  {
    float dx_3 = x_0 - 50;
    float dy_4 = y_1 - 50;
    circle_2 = sqrt(dx_3 * dx_3 + dy_4 * dy_4) - 50;
  }

  buffer[pos] = circle_2; 
}
