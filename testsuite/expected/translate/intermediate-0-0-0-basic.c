__kernel void apply(__global float* buffer, ulong width) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;

  float x_0 = x_s - -9;
  float y_1 = y_s - -9;

  float x_2 = x_0 - 10;
  float y_3 = y_1 - 10;

  float circle_4;
  {
    float dx_5 = x_2 - 50;
    float dy_6 = y_3 - 50;
    circle_4 = sqrt(dx_5 * dx_5 + dy_6 * dy_6) - 50;
  }

  buffer[pos] = circle_4; 
}
