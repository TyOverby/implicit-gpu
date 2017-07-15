__kernel void apply(__global float* buffer, ulong width, __global float* buffer_0, __global float* buffer_1) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;
float other_group_0 = buffer_0[pos];
float other_group_1 = buffer_1[pos];

  float modulate_2 = other_group_1 + -50;

  float not_3 = -modulate_2;

  float and_4 = max(other_group_0, not_3);

  buffer[pos] = and_4; 
}
