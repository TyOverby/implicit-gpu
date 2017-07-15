__kernel void apply(__global float* buffer, ulong width, __global float* buffer_0) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;

  float circle_0;
  {
    float dx_1 = x_s - 76;
    float dy_2 = y_s - 31;
    circle_0 = sqrt(dx_1 * dx_1 + dy_2 * dy_2) - 75;
  }
float other_group_3 = buffer_0[pos];

  float and_4 = max(circle_0, other_group_3);

  buffer[pos] = and_4; 
}
