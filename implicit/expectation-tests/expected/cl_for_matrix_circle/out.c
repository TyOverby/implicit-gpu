__kernel void apply(__global float* buffer, ulong width) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;
  float x_s = (float) x;
  float y_s = (float) y;

// Circle _circle_0
float _dx_1 = (x_s * 0.5 + y_s * 0 + 0) - 0;
float _dy_2 = (x_s * 0 + y_s * 1 + 0) - 5;
float _circle_0 = sqrt(_dx_1 * _dx_1 + _dy_2 * _dy_2) - 10;
// End Circle _circle_0
buffer[pos] = _circle_0;
}
