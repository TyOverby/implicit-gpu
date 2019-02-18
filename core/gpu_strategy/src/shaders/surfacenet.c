// This place is not a place of honor.
// No highly esteemed deed is commemorated here.
// Nothing valued is here.
// What is here is dangerous and repulsive to us.
// This message is a warning about danger.

__constant long3 OFFSETS[24] = {
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

void printf3(__constant char* prefix, float3 v) {
    printf("%s: %f %f %f\n", prefix, v.x, v.y, v.z);
}

void printul3(__constant char* prefix, long3 v) {
    printf("%s: %ld %ld %ld\n", prefix, v.x, v.y, v.z);
}

float grid_values(__global float* buffer, long3 position, long3 dims) {
    size_t pos = position.x + (position.y * dims.x) + (position.z * dims.x * dims.y);
    return buffer[pos];
}

long center_coord(long3 position, long3 dims) {
    return position.x + (position.y * dims.x) + (position.z * dims.x * dims.y);
}

float3 center_values(__global float* centers, long3 position, long3 dims) {
    long pos = center_coord(position, dims);
    long pos_3 = pos * 3;
    float3 res = (float3)(
         centers[pos_3 + 0],
         centers[pos_3 + 1],
         centers[pos_3 + 2]);
    return res;
}

void write_float3(float3 v, __global float* out, ulong start, ulong offset) {
    out[start + offset * 3 + 0] = v.x;
    out[start + offset * 3 + 1] = v.y;
    out[start + offset * 3 + 2] = v.z;
}

float3 find_edge(
    __global float* buffer,
    long3 coord,
    long3 offset1,
    long3 offset2,
    long3 dims
) {
    float value1 = grid_values(buffer, coord + offset1, dims);
    float value2 = grid_values(buffer, coord + offset2, dims);
    if ((value1 < 0.0) == (value2 < 0.0)) {
        return (float3)(NAN, NAN, NAN);
    }
    float interp = value1 / (value1 - value2);
    float3 point = (float3)(
        ((float)offset1.x) * (1.0 - interp) + ((float) offset2.x) * interp + ((float)coord.x),
        ((float)offset1.y) * (1.0 - interp) + ((float) offset2.y) * interp + ((float)coord.y),
        ((float)offset1.z) * (1.0 - interp) + ((float) offset2.z) * interp + ((float)coord.z)
    );
    return point;
}

long3 normal_offset(long3 coord, long x) {
    return (long3) (
        coord.x + (x & 1),
        coord.y + ((x >> 1) & 1),
        coord.z + ((x >> 2) & 1)
    );
}

float3 find_center(__global float* buffer, long3 coord, long3 dims, float3* normal) {
    float n_000 = grid_values(buffer, normal_offset(coord, 0), dims);
    float n_001 = grid_values(buffer, normal_offset(coord, 1), dims);
    float n_010 = grid_values(buffer, normal_offset(coord, 2), dims);
    float n_011 = grid_values(buffer, normal_offset(coord, 3), dims);
    float n_100 = grid_values(buffer, normal_offset(coord, 4), dims);
    float n_101 = grid_values(buffer, normal_offset(coord, 5), dims);
    float n_110 = grid_values(buffer, normal_offset(coord, 6), dims);
    float n_111 = grid_values(buffer, normal_offset(coord, 7), dims);


    float normal_x = (n_001 + n_011 + n_101 + n_111)
            - (n_000 + n_010 + n_100 + n_110);
    float normal_y = (n_010 + n_011 + n_110 + n_111)
            - (n_000 + n_001 + n_100 + n_101);
    float normal_z = (n_100 + n_101 + n_110 + n_111)
            - (n_000 + n_001 + n_010 + n_011);
    float normal_len = sqrt(normal_x * normal_x + normal_y * normal_y + normal_z * normal_z);

    long count = 0;
    float3 sum = (float3)(0.0, 0.0, 0.0);
    for (int i = 0; i < 24; i+=2) {
        long3 a = OFFSETS[i];
        long3 b = OFFSETS[i+1];
        float3 edge = find_edge(buffer, coord, a, b, dims);
        if (!isnan(edge.x)) {
            count += 1;
            sum += edge;
        }
    }
    if (count == 0) {
        return (float3)(NAN, NAN, NAN);
    } else {
        *normal = (float3)(
            normal_x / normal_len,
            normal_y / normal_len,
            normal_z / normal_len
        );

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
};

enum FaceResult is_face(
    __global float* buffer,
    long3 coord,
    long3 offset,
    long3 dims
    )
{
    long3 other = coord + offset;
    bool a = grid_values(buffer, coord, dims) < 0.0;
    bool b = grid_values(buffer, other, dims) < 0.0;
    if (a && !b) {
        return FacePositive;
    } else if (!a && b) {
        return FaceNegative;
    } else {
        return NoFace;
    }
}

float dist(float3 a, float3 b) {
    float3 d = a - b;
    return d.x * d.x + d.y * d.y + d.z * d.z;
}

void make_triangle(
    __global float* buffer,
    __global float* centers,
    __global long* out,
    long3 coord,
    long3 offset,
    long3 axis1,
    long3 axis2,
    long3 dims,
    volatile __global unsigned int *atomic
) {
    enum FaceResult fr = is_face(buffer, coord, offset, dims);
    if (fr == NoFace) {
        return;
    }

    long p = atomic_inc(atomic);
    long insert_pos = p * 3 * 2;

    long v1 = center_coord(coord, dims);
    long v2 = center_coord(coord - axis1, dims);
    long v3 = center_coord(coord - axis2, dims);
    long v4 = center_coord(coord - axis1 - axis2, dims);

    float3 p1 = center_values(centers, coord, dims);
    float3 p2 = center_values(centers, coord - axis1, dims);
    float3 p3 = center_values(centers, coord - axis2, dims);
    float3 p4 = center_values(centers, coord - axis1 - axis2, dims);

    float d14 = dist(p1, p4);
    float d23 = dist(p2, p3);
    if (d14 < d23) {
        if (fr == FacePositive) {
            out[insert_pos + 0] = v1;
            out[insert_pos + 1] = v2;
            out[insert_pos + 2] = v4;

            out[insert_pos + 3] = v1;
            out[insert_pos + 4] = v4;
            out[insert_pos + 5] = v3;
        } else {
            out[insert_pos + 0] = v1;
            out[insert_pos + 1] = v4;
            out[insert_pos + 2] = v2;

            out[insert_pos + 3] = v1;
            out[insert_pos + 4] = v3;
            out[insert_pos + 5] = v4;
        }
    } else {
        if (fr == FacePositive) {
            out[insert_pos + 0] = v2;
            out[insert_pos + 1] = v4;
            out[insert_pos + 2] = v3;

            out[insert_pos + 3] = v2;
            out[insert_pos + 4] = v3;
            out[insert_pos + 5] = v1;
        } else {
            out[insert_pos + 0] = v2;
            out[insert_pos + 1] = v3;
            out[insert_pos + 2] = v4;

            out[insert_pos + 3] = v2;
            out[insert_pos + 4] = v1;
            out[insert_pos + 5] = v3;
        }
    }
}

__kernel void phase_1(
    __global float* buffer,
    ulong width,
    ulong height,
    ulong depth,
    __global float* out,
    __global float* normals
) {
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t z = get_global_id(2);

    size_t pos = x + (y * width) + (z * width * height);
    float3 normal;
    float3 center = find_center(buffer, (long3)(x, y, z), (long3)(width, height, depth), &normal);
    size_t pos_out = pos * 3;
    out[pos_out + 0] = center.x;
    out[pos_out + 1] = center.y;
    out[pos_out + 2] = center.z;
    normals[pos_out + 0] = normal.x;
    normals[pos_out + 1] = normal.y;
    normals[pos_out + 2] = normal.z;
}

__kernel void phase_2(
    __global float* buffer,
    __global float* centers,
    ulong width,
    ulong height,
    ulong depth,
    __global long* out,
    volatile __global unsigned int *atomic)
{
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);
    size_t z = get_global_id(2);

    if (y != 0 && z != 0) {
        make_triangle(
            buffer,
            centers,
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
            centers,
            out,
            (long3)(x, y, z),
            (long3)(0, 1, 0),
            (long3)(0, 0, 1),
            (long3)(1, 0, 0),
            (long3)(width, height, depth),
            atomic
        );
    }
    if (x != 0 && y != 0) {
        make_triangle(
            buffer,
            centers,
            out,
            (long3)(x, y, z),
            (long3)(0, 0, 1),
            (long3)(1, 0, 0),
            (long3)(0, 1, 0),
            (long3)(width, height, depth),
            atomic
        );
    }
}
