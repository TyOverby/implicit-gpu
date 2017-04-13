__kernel void scan(__global float *a,
                   __global float *r,
                   __local float *b,
                   ulong n_items)
{
    ulong gid = get_global_id(0);
    ulong lid = get_local_id(0);
    ulong dp = 1;

    b[2*lid] = a[2*gid];
    [2*lid+1] = a[2*gid+1];

    for(ulong s = n_items>>1; s > 0; s >>= 1) {
        barrier(CLK_LOCAL_MEM_FENCE);
        if(lid < s) {
            uint i = dp*(2*lid+1)-1;
            uint j = dp*(2*lid+2)-1;
            b[j] += b[i];
        }

        dp <<= 1;
    }

    if(lid == 0) b[n_items - 1] = 0;

    for(ulong s = 1; s < n_items; s <<= 1) {
        dp >>= 1;
        barrier(CLK_LOCAL_MEM_FENCE);

        if(lid < s) {
            ulong i = dp*(2*lid+1)-1;
            ulong j = dp*(2*lid+2)-1;

            float t = b[j];
            b[j] += b[i];
            b[i] = t;
        }
    }

    barrier(CLK_LOCAL_MEM_FENCE);

    r[2*gid] = b[2*lid];
    r[2*gid+1] = b[2*lid+1];
}
