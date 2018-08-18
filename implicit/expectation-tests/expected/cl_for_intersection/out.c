__kernel void apply(__global float* buffer, ulong width, __global float* field__5, __global float* field__6) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;
  float x_s = (float) x;
  float y_s = (float) y;

// Intersection _intersection_0
float _intersection_0 = -INFINITY;

float _field_1 = field__5[x][y];
_intersection_0 = max(_intersection_0, _field_1)

float _field_2 = field__6[x][y];
_intersection_0 = max(_intersection_0, _field_2)
// End Intersection _intersection_0
buffer[pos] = _intersection_0;
}
