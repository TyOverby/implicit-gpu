__kernel void apply(__global float *buffer, __global uint *locations, __global float *out, ulong count)
{
    size_t i = get_global_id(0);
    float value = buffer[i];
    if (!isnan(value))
    {
        out[locations[i]] = value;
    }
}
