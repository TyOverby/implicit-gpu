__kernel void apply(__global float* buffer, ulong width) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;

  float x_0 = x_s - -49;
  float y_1 = y_s - -49;

  float circle_2;
  {
    float dx_3 = x_0 - 50;
    float dy_4 = y_1 - 50;
    circle_2 = sqrt(dx_3 * dx_3 + dy_4 * dy_4) - 50;
  }

  float circle_5;
  {
    float dx_6 = x_0 - 100;
    float dy_7 = y_1 - 100;
    circle_5 = sqrt(dx_6 * dx_6 + dy_7 * dy_7) - 50;
  }

  float and_8 = max(circle_2, circle_5);

  buffer[pos] = and_8; 
}
