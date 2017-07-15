__kernel void apply(__global float* buffer, ulong width, __global float* buffer_0) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;

  float circle_0;
  {
    float dx_1 = x_s - 51;
    float dy_2 = y_s - 51;
    circle_0 = sqrt(dx_1 * dx_1 + dy_2 * dy_2) - 50;
  }

  float circle_3;
  {
    float dx_4 = x_s - 51;
    float dy_5 = y_s - 51;
    circle_3 = sqrt(dx_4 * dx_4 + dy_5 * dy_5) - 25;
  }
float other_group_6 = buffer_0[pos];

  float or_7 = min(circle_3, other_group_6);

  float not_8 = -or_7;

  float and_9 = max(circle_0, not_8);

  buffer[pos] = and_9; 
}
