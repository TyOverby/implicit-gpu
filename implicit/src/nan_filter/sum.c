struct ArrayView {
    __global uint* data;
    size_t length;
};

static struct ArrayView array_new(__global uint *data, size_t length) {
    struct ArrayView result;
    result.data = data;
    result.length = length;
    return result;
}

static __global uint* array_index(struct ArrayView array, size_t index) {
    return &array.data[index];
}

static bool array_in_bounds(struct ArrayView array, size_t index) {
    return index < array.length;
}

static void copy_into_shared(struct ArrayView input, __local uint* shared) {
    size_t global_id = get_global_id(0);
    size_t local_id = get_local_id(0);
    size_t local_size = get_local_size(0);
    size_t workgroup_id = global_id / local_size;
    size_t load = workgroup_id * 2 * local_size + local_id;
    if (array_in_bounds(input, load))
    {
        shared[local_id] = *array_index(input, load);
    }
    if (array_in_bounds(input, load + local_size))
    {
        shared[local_id + local_size] = *array_index(input, load + local_size);
    }
    barrier(CLK_LOCAL_MEM_FENCE);
}

static void copy_into_global(struct ArrayView input, __local uint* shared) {
    size_t global_id = get_global_id(0);
    size_t local_id = get_local_id(0);
    size_t local_size = get_local_size(0);
    size_t workgroup_id = global_id / local_size;
    size_t load = workgroup_id * 2 * local_size + local_id;
    if (array_in_bounds(input, load))
    {
        *array_index(input, load) = shared[local_id];
    }
    if (array_in_bounds(input, load + local_size))
    {
        *array_index(input, load + local_size) = shared[local_id + local_size];
    }
}

static void phase_one(__local uint* shared, __global uint* aux) {
    size_t local_id = get_local_id(0);
    size_t buffer_size = get_local_size(0) * 2;
    size_t index = local_id * 2;
    for (size_t offset = 1; offset < buffer_size; offset *= 2) {
        size_t source = (index + 1) * offset - 1;
        size_t dest = (index + 2) * offset - 1;
        if (dest < buffer_size)
        {
            shared[dest] += shared[source];
        }
        barrier(CLK_LOCAL_MEM_FENCE);
    }
    if (local_id == 0) {
        size_t workgroup_id = get_global_id(0) / get_local_size(0);
        aux[workgroup_id] = shared[buffer_size - 1];
        shared[buffer_size - 1] = 0;
    }
    barrier(CLK_LOCAL_MEM_FENCE);
    for (size_t offset = buffer_size / 2; offset > 0; offset /= 2) {
        size_t source = (index + 1) * offset - 1;
        size_t dest = (index + 2) * offset - 1;
        if (dest < buffer_size) {
            int sharedSource = shared[source];
            shared[source] = shared[dest];
            shared[dest] += sharedSource;
        }
        barrier(CLK_LOCAL_MEM_FENCE);
    }
}

static void sum_phase_one(struct ArrayView input, __global uint* aux, __local uint* shared) {
    copy_into_shared(input, shared);
    phase_one(shared, aux);
    copy_into_global(input, shared);
}

static void sum_phase_two(struct ArrayView input, __global uint* aux, __local uint* shared) {
    size_t global_id = get_global_id(0);
    size_t local_id = get_local_id(0);
    size_t local_size = get_local_size(0);
    size_t workgroup_id = global_id / local_size;
    size_t load = workgroup_id * 2 * local_size + local_id;
    uint add_value = 0;
    for (size_t i = 0; i < workgroup_id; i++)
    {
        add_value += aux[i];
    }
    if (array_in_bounds(input, load))
    {
        *array_index(input, load) += add_value;
    }
    if (array_in_bounds(input, load + local_size))
    {
        *array_index(input, load + local_size) += add_value;
    }
}

__kernel void sum(__global uint* input_raw_array, uint n_items, __global uint* aux, int is_phase_two, __local uint* shared) {
    struct ArrayView input = array_new(input_raw_array, n_items);
    if (!is_phase_two)
    {
        sum_phase_one(input, aux, shared);
    }
    else
    {
        sum_phase_two(input, aux, shared);
    }
}
