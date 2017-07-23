__kernel void apply(__global float* buffer, ulong width) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;

  float x_0 = x_s - 1;
  float y_1 = y_s - 1;

  float circle_2;
  {
    float dx_3 = x_0 - 50;
    float dy_4 = y_1 - 50;
    circle_2 = sqrt(dx_3 * dx_3 + dy_4 * dy_4) - 50;
  }

  buffer[pos] = circle_2; 
}
