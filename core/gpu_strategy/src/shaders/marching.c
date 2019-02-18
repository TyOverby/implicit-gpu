__constant float2 A = (float2)(-0.5f, -0.5f);
__constant float2 B = (float2)(0.5f, -0.5f);
__constant float2 C = (float2)(0.5f, 0.5f);
__constant float2 D = (float2)(-0.5f, 0.5f);

__constant float2 N = (float2)(0.0f, -0.5f);
__constant float2 S = (float2)(0.0f, 0.5f);
__constant float2 E = (float2)(0.5f, 0.0f);
__constant float2 W = (float2)(-0.5f, 0.0f);

static float lerp(float fa, float fb, float dist)
{
    return -dist / 2.0f + dist * ((-fa) / (fb - fa));
}

static float2 n(float distance, float2 point, float how_much)
{
    float2 result = N * distance + point;
    result.x += how_much;
    return result;
}

static float2 s(float distance, float2 point, float how_much)
{
    float2 result = S * distance + point;
    result.x += how_much;
    return result;
}

static float2 e(float distance, float2 point, float how_much)
{
    float2 result = E * distance + point;
    result.y += how_much;
    return result;
}

static float2 w(float distance, float2 point, float how_much)
{
    float2 result = W * distance + point;
    result.y += how_much;
    return result;
}

static void write_line(float2 o1, float2 o2, __global float *out, size_t out_pos)
{
    out[out_pos + 0] = o1.x;
    out[out_pos + 1] = o1.y;
    out[out_pos + 2] = o2.x;
    out[out_pos + 3] = o2.y;
}

static void march(
    float sra, float srb, float src, float srd,
    float2 p,
    float dist,
    __global float *out,
    volatile __global unsigned int *atomic)
{

    size_t a_on = sra <= 0.0f;
    size_t b_on = srb <= 0.0f;
    size_t c_on = src <= 0.0f;
    size_t d_on = srd <= 0.0f;

    size_t which = (a_on << 3) + (b_on << 2) + (c_on << 1) + (d_on << 0);

    float2 o1 = (float2)(NAN, NAN);
    float2 o2 = (float2)(NAN, NAN);

    float2 o3 = (float2)(NAN, NAN);
    float2 o4 = (float2)(NAN, NAN);

    switch (which)
    {
    // 0000
    // 00
    // 00
    case 0:
        // Don't do anything
        break;

    // 0001
    // 00
    // 10
    case 1:
        o1 = w(dist, p, lerp(sra, srd, dist));
        o2 = s(dist, p, -lerp(src, srd, dist));
        break;

    // 0010
    // 00
    // 01
    case 2:
        o1 = s(dist, p, lerp(srd, src, dist));
        o2 = e(dist, p, -lerp(src, srb, dist));
        break;

    // 0011
    // 00
    // 11
    case 3:
        o1 = w(dist, p, lerp(sra, srd, dist));
        o2 = e(dist, p, lerp(srb, src, dist));
        break;

    // 0100
    // 01
    // 00
    case 4:
        o2 = n(dist, p, lerp(sra, srb, dist));
        o1 = e(dist, p, lerp(srb, src, dist));
        break;

    // 0101
    // 01
    // 10
    case 5:
        o2 = n(dist, p, lerp(sra, srb, dist));
        o1 = e(dist, p, lerp(srb, src, dist));

        o3 = w(dist, p, lerp(sra, srd, dist));
        o4 = s(dist, p, -lerp(src, srd, dist));
        // WEW LADS
        break;

    // 0110
    // 01
    // 01
    case 6:
        o2 = n(dist, p, -lerp(srb, sra, dist));
        o1 = s(dist, p, -lerp(src, srd, dist));
        break;

    // 0111
    // 01
    // 11
    case 7:
        o1 = w(dist, p, lerp(sra, srd, dist));
        o2 = n(dist, p, lerp(sra, srb, dist));
        break;

    // 1000
    // 10
    // 00
    case 8:
        o2 = w(dist, p, lerp(sra, srd, dist));
        o1 = n(dist, p, lerp(sra, srb, dist));
        break;

    // 1001
    // 10
    // 10
    case 9:
        o1 = n(dist, p, -lerp(srb, sra, dist));
        o2 = s(dist, p, -lerp(src, srd, dist));
        break;

    // 1010
    // 10
    // 01
    case 10:
        o1 = s(dist, p, lerp(srd, src, dist));
        o2 = e(dist, p, -lerp(src, srb, dist));

        o4 = w(dist, p, lerp(sra, srd, dist));
        o3 = n(dist, p, lerp(sra, srb, dist));
        // PUNT
        break;

    // 1011
    // 10
    // 11
    case 11:
        o1 = n(dist, p, lerp(sra, srb, dist));
        o2 = e(dist, p, -lerp(src, srb, dist));
        break;

    // 1100
    // 11
    // 00
    case 12:
        o2 = w(dist, p, lerp(sra, srd, dist));
        o1 = e(dist, p, lerp(srb, src, dist));
        break;

    // 1101
    // 11
    // 10
    case 13:
        /*
            let db = lerp(srb, src, dist);
            let dd = lerp(srd, src, dist);
            MarchResult::One(Line(s(dist, p, dd), e(dist, p, db)))
            */
        o2 = s(dist, p, lerp(srd, src, dist));
        o1 = e(dist, p, lerp(srb, src, dist));
        break;

    // 1110
    // 11
    // 01
    case 14:
        o2 = w(dist, p, lerp(sra, srd, dist));
        o1 = s(dist, p, lerp(srd, src, dist));
        break;

    // 1111
    // 11
    // 11
    case 15:
        // do nothing
        break;
    }

    if (!isnan(o1.x))
    {
        int p = atomic_inc(atomic);
        write_line(o1, o2, out, p * 4);
    }
    if (!isnan(o4.x))
    {
        int p = atomic_inc(atomic);
        write_line(o3, o4, out, p * 4);
    }
}

__kernel void apply(
    __global float *buffer,
    ulong width, ulong height,
    __global float *out,
    volatile __global unsigned int *atomic)
{
    size_t x = get_global_id(0);
    size_t y = get_global_id(1);

    size_t pos = x + y * width;

    if (x == width - 1 || y == height - 1)
    {
        return;
    }

    size_t a = pos;
    size_t b = pos + 1;
    size_t c = pos + 1 + width;
    size_t d = pos + width;

    float sra = buffer[a];
    float srb = buffer[b];
    float src = buffer[c];
    float srd = buffer[d];

    float2 p = (float2)(x + 0.5f, y + 0.5f);
    march(sra, srb, src, srd, p, 1.0f, out, atomic);
}
