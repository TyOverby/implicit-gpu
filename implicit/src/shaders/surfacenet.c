// This place is not a place of honor.
// No highly esteemed deed is commemorated here.
// Nothing valued is here.
// What is here is dangerous and repulsive to us.
// This message is a warning about danger.

long3* OFFSETS = {
    (long3)(0, 0, 0),(long3) (0, 0, 1),
    (long3)(0, 0, 0),(long3) (0, 1, 0),
    (long3)(0, 0, 0), (long3)(1, 0, 0),
    (long3)(0, 0, 1), (long3)(0, 1, 1),
    (long3)(0, 0, 1), (long3)(1, 0, 1),
    (long3)(0, 1, 0), (long3)(0, 1, 1),
    (long3)(0, 1, 0), (long3)(1, 1, 0),
    (long3)(0, 1, 1), (long3)(1, 1, 1),
    (long3)(1, 0, 0), (long3)(1, 0, 1),
    (long3)(1, 0, 0), (long3)(1, 1, 0),
    (long3)(1, 0, 1), (long3)(1, 1, 1),
    (long3)(1, 1, 0), (long3)(1, 1, 1),
};

float grid_values(float* buffer, long3 position, long3 dims) {
    size_t pos = position.x + (position.y * dims.x) + (position.z * dims.x * dims.y);
    return buffer[pos];
}

float3 find_edge(
    float* buffer,
    long3 coord,
    long3 offset1,
    long3 offset2,
    long3 dims,
) {
    float value1 = grid_values(buffer, coord + offset1, dims);
    float value2 = grid_values(buffer, coord + offset2, dims);
    if ((value 1 < 0.0) == (value2 < 0.0)) {
        return (float3)(NAN, NAN, NAN);
    }
    float interp = value1 / (value1 - value2);
    float3 point = (float3)(
        ((float)coord.x) * (1.0 - interp) + ((float) offset2.x) * interp + ((float)coord.x),
        ((float)coord.y) * (1.0 - interp) + ((float) offset2.y) * interp + ((float)coord.z),
        ((float)coord.z) * (1.0 - interp) + ((float) offset2.z) * interp + ((float)coord.z)
    );
    return point;
}

float3 find_center(float* buffer, long3 coord, long3 dims) {
    long count = 0;
    float3 sum = (float3)(0.0, 0.0, 0.0);
    for (int i = 0; i < 24, i+=2) {
        long3 a = OFFSETS[i];
        long3 b = OFFSETS[i+1];
        float3 edge = find_edge(buffer, coord, a, b, dims);
        if (edge.x != NAN) {
            count += 1;
            sum += edge;
        }
    }
    if (count == 0) {
        return (float3)(NAN, NAN, NAN);
    } else {
        float c = (float) count;
        return (float3)(
            sum.x / c,
            sum.y / c,
            sum.z / c
        );
    }
}

enum FaceResult {
    NoFace,
    FacePositive,
    FaceNegative,
}

FaceResult is_face(
    float* buffer,
    long3 coord,
    long3 offset,
    long3 dims,
    )
{
    long3 other = coord + offset;
    bool a = grid_values(buffer, coord, dims);
    bool b = grid_values(buffer, other, dims);
    if (a && !b) {
        return FacePositive;
    } else if (!a && b) {
        return FaceNegative;
    } else {
        return NoFace;
    }
}

float dist(float3 a, float3 b) {
    let d = a - b;
    return d.x * d.x + d.y * d.y + d.z * d.z;
}

void make_triangle(
    float* buffer,
    float* out,
    long3 coord,
    long3 offset,
    long3 axis1,
    long3 axis2,
    long3 dims,
    volatile __global unsigned int *atomic,
) {
    FaceResult fr = is_face(buffer, coord, offset, dims;
    if (fr == NoFace) {
        return;
    }

    int p = atomic_inc(atomic);
    long insert_pos = p * 6;

    float p1 = grid_values(buffer, coord, dims);
    float p2 = grid_values(buffer, coord - axis1, dims);
    float p3 = grid_values(buffer, coord - axis2, dims);
    float p4 = grid_values(buffer, coord - axis1 - axis2, dims);

    float d14 = dist(p1, p4);
    float d23 = dist(p2, p3);
    if (d14 < d23) {
        if (fr == FacePositive) {
            out[0 + insert_pos] = v1;
            out[1 + insert_pos] = v2;
            out[2 + insert_pos] = v4;

            out[3 + insert_pos] = v1;
            out[4 + insert_pos] = v4;
            out[5 + insert_pos] = v3;
        } else {
            out[0 + insert_pos] = v1;
            out[1 + insert_pos] = v4;
            out[2 + insert_pos] = v2;

            out[3 + insert_pos] = v1;
            out[4 + insert_pos] = v3;
            out[5 + insert_pos] = v4;
        }
    } else {
        if (fr == FacePositive) {
            out[0 + insert_pos] = v2;
            out[1 + insert_pos] = v4;
            out[2 + insert_pos] = v3;

            out[3 + insert_pos] = v2;
            out[4 + insert_pos] = v3;
            out[5 + insert_pos] = v1;
        } else {
            out[0 + insert_pos] = v2;
            out[1 + insert_pos] = v3;
            out[2 + insert_pos] = v4;

            out[3 + insert_pos] = v2;
            out[4 + insert_pos] = v1;
            out[5 + insert_pos] = v3;
        }
    }
}

__kernel void apply(
    __global float *buffer,
    ulong width,
    ulong height,
    ulong depth,
    __global float *out,
    volatile __global unsigned int *atomic)
{
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t z = get_global_id(2);

    if (x == 0 || y == 0 || z == 0 || x == width - 1 || y == height - 1 || z == depth -1) {
        return;
    }

    if (y != 0 && z != 0) {
        make_triangle(
            buffer,
            out,
            (long3)(x, y, z),
            (long3)(1, 0, 0),
            (long3)(0, 1, 0),
            (long3)(0, 0, 1),
            (long3)(width, height, depth),
            atomic
        );
    }
    if (x != 0 && z != 0) {
        make_triangle(
            buffer,
            out,
            (long3)(x, y, z),
            (long3)(1, 0, 0),
            (long3)(0, 1, 0),
            (long3)(0, 0, 1),
            (long3)(width, height, depth),
            atomic
        );
    }
    if (x != 0 && y != 0) {
        make_triangle(
            buffer,
            out,
            (long3)(x, y, z),
            (long3)(1, 0, 0),
            (long3)(0, 1, 0),
            (long3)(0, 0, 1),
            (long3)(width, height, depth),
            atomic
        );
    }
}
