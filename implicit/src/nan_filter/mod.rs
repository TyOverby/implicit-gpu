use super::opencl::{MaskBuffer, OpenClContext, LineBuffer};

const MASK_PROG: &'static str = include_str!("./mask.c");

pub fn create_mask(ctx: &OpenClContext, line: &LineBuffer) -> MaskBuffer {
    let size = line.size();

    let out = ctx.mask_buffer(size);
    let kernel = ctx.compile("apply", MASK_PROG);

    kernel.gws([size])
        .arg_buf(line.buffer())
        .arg_buf(out.buffer())
        .arg_scl(size)
        .enq()
        .unwrap();

    out
}

pub fn sum_mask(ctx: &OpenClContext, mask: &MaskBuffer) -> MaskBuffer {
    /*
    let size = mask.size();
    let out = ctx.mask_buffer(size);
    */
    unimplemented!();
}

#[test]
fn test_create_mask() {
    let ctx = OpenClContext::default();
    let mut input_cpu = vec![0.0f32; 8_000_000];

    for _ in 0..1_000_000 {
        let idx: usize = ::rand::random::<usize>() % 8_000_000;
        input_cpu[idx] = ::std::f32::NAN;
    }

    let input = ctx.line_buffer(&input_cpu);

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
    let mut input_cpu = vec![0.0f32; 8_000_000];

    for _ in 0 .. 1_000_000 {
        let idx: usize = ::rand::random::<usize>() % 8_000_000;
        input_cpu[idx] = ::std::f32::NAN;
    }

    let input = ctx.line_buffer(&input_cpu);

    let masked = create_mask(&ctx, &input);
    let summed = sum_mask(&ctx, &masked);

    let masked_expected = input.values().into_iter().map(|a| { if a.is_nan() { 1 } else { 0 } });
    let summed_expected = masked_actual.scan(0, |a, b| {
        let r = *a;
        *a = r + b;
        Some(r)
    });

    //for (a, t) in summed_expected.zip(summed.values())
}
