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

  float circle_5;
  {
    float dx_6 = x_0 - 50;
    float dy_7 = y_1 - 50;
    circle_5 = sqrt(dx_6 * dx_6 + dy_7 * dy_7) - 25;
  }

  float not_8 = -circle_5;

  float and_9 = max(circle_2, not_8);

  float circle_10;
  {
    float dx_11 = x_0 - 50;
    float dy_12 = y_1 - 50;
    circle_10 = sqrt(dx_11 * dx_11 + dy_12 * dy_12) - 10;
  }

  float or_13 = min(and_9, circle_10);

  buffer[pos] = or_13; 
}
