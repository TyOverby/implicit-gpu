use super::opencl::{MaskBuffer, OpenClContext, LineBuffer};

const MASK_PROG: &'static str = include_str!("./mask.c");
const _SUM_PROG: &'static str = include_str!("./sum.c");
const REORDER_PROG: &'static str = include_str!("./reorder.c");

pub fn filter_nans(ctx: &OpenClContext, line: &LineBuffer) -> LineBuffer {
    let mask = create_mask(ctx, line);
    let sum = sum_mask(ctx, &mask);
    let filtered = reorder(ctx, &sum, line);
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

fn sum_mask(ctx: &OpenClContext, mask: &MaskBuffer) -> MaskBuffer {
    let mut values = mask.values();
    let mut sum = 0;
    for v in &mut values {
        let c = *v;
        *v = sum;
        sum = sum + c;
    }
    ctx.mask_buffer(values.len(), Some(&values))
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
fn get_rand_array(ctx: &OpenClContext) -> (Vec<f32>, LineBuffer) {
    let mut input_cpu = vec![0.0f32; 8_000_000];

    for _ in 0..1_000_000 {
        let idx: usize = ::rand::random::<usize>() % 8_000_000;
        input_cpu[idx] = ::std::f32::NAN;
    }

    let input = ctx.line_buffer(&input_cpu);
    (input_cpu, input)
}

#[test]
fn test_create_mask() {
    let ctx = OpenClContext::default();
    let (input_cpu, input) = get_rand_array(&ctx);

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
    let (input_cpu, input) = get_rand_array(&ctx);

    let masked = create_mask(&ctx, &input);
    let summed = sum_mask(&ctx, &masked);


    let masked_expected: Vec<u32> = input_cpu.into_iter().map(|a| {
        if a.is_nan() { 0 } else { 1 }
    }).collect();
    let summed_expected: Vec<u32> = masked_expected.into_iter().scan(0u32, |a: &mut u32, b: u32| {
        let r: u32 = *a;
        *a = r + b;
        let r: Option<u32> = Some(r);
        r
    }).collect();

    assert_eq!(summed.values(), summed_expected);
}

