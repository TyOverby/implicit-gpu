__kernel void apply(__global float* buffer, ulong width, __global float* buffer_0) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;
float other_group_0 = buffer_0[pos];

  float modulate_1 = other_group_0 + -20;

  buffer[pos] = modulate_1; 
}
