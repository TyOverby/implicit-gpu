__kernel void apply(__global float* buffer, ulong width, __global float* buffer_0) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;
float other_group_0 = buffer_0[pos];

  buffer[pos] = other_group_0; 
}
