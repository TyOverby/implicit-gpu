__kernel void apply(__global float* buffer, ulong width, __global float* field__5) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;
  float x_s = (float) x;
  float y_s = (float) y;

float _field_0 = field__5[pos];
buffer[pos] = _field_0;
}
