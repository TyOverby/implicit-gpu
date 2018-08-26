__kernel void apply(__global float* buffer, ulong width) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;
  float x_s = (float) x;
  float y_s = (float) y;

// Transform [2.0, 0.0, 0.0, 1.0, 0.0, 0.0]
float _nx_0 = (x_s * 0.5 + y_s * 0 + 0);
float _ny_1 = (x_s * 0 + y_s * 1 + 0);

// Circle _circle_2
float _dx_3 = _nx_0 - 0;
float _dy_4 = _ny_1 - 5;
float _circle_2 = sqrt(_dx_3 * _dx_3 + _dy_4 * _dy_4) - 10;
// End Circle _circle_2
buffer[pos] = _circle_2;
}
