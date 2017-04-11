__kernel void apply(__global float* buffer, __global uint* mask, ulong count) {
    size_t i = get_global_id(0);
    mask[i] = !isnan(buffer[i]);
}
