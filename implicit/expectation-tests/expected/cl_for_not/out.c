__kernel void apply(__global float* buffer, ulong width, __global float* field__5) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;
  float x_s = (float) x;
  float y_s = (float) y;

// Not _negate_0

float _field_1 = field__5[x][y];
float _negate_0 = -_field_1;
// End Not _negate_0
buffer[pos] = _negate_0;
}
