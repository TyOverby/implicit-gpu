use opencl::{FieldBuffer, IndexBuffer, OpenClContext};

const PROGRAM: &'static str = include_str!("shaders/surfacenet.c");

pub fn run_surface_net(
    input: &mut FieldBuffer,
    ctx: &OpenClContext,
) -> (IndexBuffer, u32, FieldBuffer, FieldBuffer) {
    let (mut centers, normal_buffer) = run_surface_net_phase_1(input, ctx);
    let (idx, count) = run_surface_net_phase_2(input, &mut centers, ctx);
    (idx, count, centers, normal_buffer)
}

pub fn run_surface_net_phase_1(
    input: &mut FieldBuffer,
    ctx: &OpenClContext,
) -> (FieldBuffer, FieldBuffer) {
    let _guard = ::flame::start_guard("opencl surface net phase 1 [run_surface_net]");

    let width = input.width;
    let height = input.height;
    let depth = input.depth;

    let mut phase_1_kernel = ctx.compile("phase_1", PROGRAM, |register| {
        register.buffer("buffer");
        register.long("width");
        register.long("height");
        register.long("depth");
        register.buffer("out");
        register.buffer("normals");
    });

    let mut center_buffer = ctx.field_buffer(width, height, depth * 3, None);
    let mut normal_buffer = ctx.field_buffer(width, height, depth * 3, None);

    ::flame::start("setup phase_1_kernel");
    phase_1_kernel.set_default_global_work_size(::ocl::SpatialDims::Three(
        width as usize,
        height as usize,
        depth as usize,
    ));
    phase_1_kernel
        .set_arg("buffer", input.to_opencl(ctx.queue()))
        .unwrap();
    phase_1_kernel.set_arg("width", width as u64).unwrap();
    phase_1_kernel.set_arg("height", height as u64).unwrap();
    phase_1_kernel.set_arg("depth", depth as u64).unwrap();
    phase_1_kernel
        .set_arg("out", center_buffer.to_opencl(ctx.queue()))
        .unwrap();
    phase_1_kernel
        .set_arg("normals", normal_buffer.to_opencl(ctx.queue()))
        .unwrap();
    ::flame::end("setup phase_1_kernel");

    unsafe {
        ::flame::span_of("opencl surface_net phase_1 [execution]", || {
            phase_1_kernel.enq().unwrap()
        });
    }

    (center_buffer, normal_buffer)
}
pub fn run_surface_net_phase_2(
    input: &mut FieldBuffer,
    center_buffer: &mut FieldBuffer,
    ctx: &OpenClContext,
) -> (IndexBuffer, u32) {
    let _guard = ::flame::start_guard("opencl surface net [run_surface_net]");

    let width = input.width;
    let height = input.height;
    let depth = input.depth;
    let mut phase_1_kernel = ctx.compile("phase_2", PROGRAM, |register| {
        register.buffer("buffer");
        register.buffer("centers");
        register.long("width");
        register.long("height");
        register.long("depth");
        register.buffer("out");
        register.buffer("atomic");
    });

    let index_buffer =
        ctx.index_buffer_uninit(width as usize * height as usize * depth as usize * 6);
    let sync_buffer = ctx.sync_buffer();

    ::flame::start("setup phase_1_kernel");
    phase_1_kernel.set_default_global_work_size(::ocl::SpatialDims::Three(
        width as usize,
        height as usize,
        depth as usize,
    ));
    phase_1_kernel
        .set_arg("buffer", input.to_opencl(ctx.queue()))
        .unwrap();
    phase_1_kernel
        .set_arg("centers", center_buffer.to_opencl(ctx.queue()))
        .unwrap();
    phase_1_kernel.set_arg("width", width as u64).unwrap();
    phase_1_kernel.set_arg("height", height as u64).unwrap();
    phase_1_kernel.set_arg("depth", depth as u64).unwrap();
    phase_1_kernel
        .set_arg("out", index_buffer.buffer())
        .unwrap();
    phase_1_kernel
        .set_arg("atomic", sync_buffer.buffer())
        .unwrap();
    ::flame::end("setup phase_1_kernel");

    unsafe {
        ::flame::span_of("opencl surface_net [execution]", || {
            phase_1_kernel.enq().unwrap()
        });
    }

    // divide by 4 because the implementation of value() is bullshit
    let count = sync_buffer.value() / 4;
    (index_buffer, count * 3 * 2)
}
