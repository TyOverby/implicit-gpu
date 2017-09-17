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

__kernel void apply(__global float* buffer, ulong width, __global float* buffer_0) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;

  float circle_0;
  {
    float dx_1 = x_s - 52;
    float dy_2 = y_s - 52;
    circle_0 = sqrt(dx_1 * dx_1 + dy_2 * dy_2) - 50;
  }

  float circle_3;
  {
    float dx_4 = x_s - 52;
    float dy_5 = y_s - 52;
    circle_3 = sqrt(dx_4 * dx_4 + dy_5 * dy_5) - 25;
  }
float other_group_6 = buffer_0[pos];

  float or_7 = min(circle_3, other_group_6);

  float not_8 = -or_7;

  float and_9 = max(circle_0, not_8);

  buffer[pos] = and_9; 
}