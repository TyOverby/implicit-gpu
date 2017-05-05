use super::opencl::{MaskBuffer, OpenClContext, LineBuffer};
use ocl::Buffer;

const MASK_PROG: &'static str = include_str!("./mask.c");
const SUM_PROG: &'static str = include_str!("./sum.c");
const REORDER_PROG: &'static str = include_str!("./reorder.c");

pub fn filter_nans(ctx: &OpenClContext, line: &LineBuffer) -> LineBuffer {
    let mask = create_mask(ctx, line);
    sum_mask(ctx, &mask);
    let filtered = reorder(ctx, &mask, line);
    filtered
}

fn create_mask(ctx: &OpenClContext, line: &LineBuffer) -> MaskBuffer {
    let size = line.size();

    let out = ctx.mask_buffer(size, None);
    let kernel = ctx.compile("apply", MASK_PROG);

    kernel.gws([size])
        .arg_buf(line.buffer())
        .arg_buf(out.buffer())
        .arg_scl(size)
        .enq()
        .unwrap();

    out
}

fn sum_mask(ctx: &OpenClContext, mask: &MaskBuffer) {
    const WORKGROUP_SIZE:usize = 512;

    let array_size = mask.size();
    let num_workgroups = ((array_size + 1) / 2 + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
    let launch_size = num_workgroups * WORKGROUP_SIZE;

    let aux_temp: Buffer<u32> = Buffer::new(ctx.queue().clone(), None, &[num_workgroups], None).unwrap();

    let kernel = ctx.compile("sum", SUM_PROG);
    kernel.gws([launch_size])
        .lws([WORKGROUP_SIZE])
        .arg_buf(mask.buffer())
        .arg_scl(array_size)
        .arg_buf(&aux_temp)
        .arg_scl(0i32)
        .arg_loc::<u32>(WORKGROUP_SIZE * 2)
        .enq()
        .unwrap();

    let kernel = ctx.compile("sum", SUM_PROG);
    kernel.gws([launch_size])
        .lws([WORKGROUP_SIZE])
        .arg_buf(mask.buffer())
        .arg_scl(array_size)
        .arg_buf(&aux_temp)
        .arg_scl(1i32)
        .arg_loc::<u32>(WORKGROUP_SIZE * 2)
        .enq()
        .unwrap();
}

fn reorder(ctx: &OpenClContext, sum: &MaskBuffer, input: &LineBuffer) -> LineBuffer {
    let size = input.size();
    let nans = vec![::std::f32::NAN; size];
    let out = ctx.line_buffer(&nans);

    let kernel = ctx.compile("apply", REORDER_PROG);

    kernel
        .gws([size])
        .arg_buf(input.buffer())
        .arg_buf(sum.buffer())
        .arg_buf(out.buffer())
        .arg_scl(size)
        .enq().unwrap();
    out
}


#[cfg(test)]
fn get_rand_array(ctx: &OpenClContext, size: usize) -> (Vec<f32>, LineBuffer) {
    let mut input_cpu = vec![0.0f32; size];

    for _ in 0 .. (size / 2) {
        let idx: usize = ::rand::random::<usize>() % size;
        input_cpu[idx] = ::std::f32::NAN;
    }

    let input = ctx.line_buffer(&input_cpu);
    (input_cpu, input)
}

#[test]
fn test_create_mask() {
    let ctx = OpenClContext::default();
    let (input_cpu, input) = get_rand_array(&ctx, 8_000_000);

    let masked = create_mask(&ctx, &input);
    let out = masked.values();

    for (v, m) in input_cpu.into_iter().zip(out.into_iter()) {
        if m == 1 {
            assert!(!v.is_nan())
        } else {
            assert!(v.is_nan())
        }
    }
}

#[test]
fn test_sum_mask() {
    let ctx = OpenClContext::default();
    let test_size = |size: usize| {
        let (input_cpu, input) = get_rand_array(&ctx, size);

        let masked = create_mask(&ctx, &input);

        let masked_expected: Vec<u32> = input_cpu.into_iter().map(|a| {
            if a.is_nan() { 0 } else { 1 }
        }).collect();

        sum_mask(&ctx, &masked);

        let summed_expected: Vec<u32> = masked_expected.into_iter().scan(0u32, |a: &mut u32, b: u32| {
            let r: u32 = *a;
            *a = r + b;
            let r: Option<u32> = Some(r);
            r
        }).collect();

        assert!(masked.values() == summed_expected);
    };

    test_size(1);
    test_size(10);
    test_size(512);
    test_size(512 - 1);
    test_size(512 + 1);
    test_size(8_000_000);
    test_size(15625 * 512);
    test_size(15625 * 512 - 1);
    test_size(15625 * 512 + 1);
}

#[test]
fn test_filter() {
    let ctx = OpenClContext::default();
    let (_, input) = get_rand_array(&ctx, 8_000_000);
    let filtered = filter_nans(&ctx, &input);
    let mut seen_nan = false;
    assert!(filtered.non_nans_at_front());
}
