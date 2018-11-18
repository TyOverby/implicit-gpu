use opencl::{FieldBuffer, LineBuffer, OpenClContext};

const PROGRAM: &'static str = include_str!("shaders/surfacenet.c");

pub fn run_surface_net(input: &FieldBuffer, ctx: &OpenClContext) -> (LineBuffer, u32) {
    let _guard = ::flame::start_guard("opencl marching [run_marching]");

    let (width, height) = (input.width(), input.height());
    let mut kernel = ctx.compile("apply", PROGRAM, |register| {
        register.buffer("buffer");
        register.long("width");
        register.long("height");
        register.buffer("out");
        register.buffer("atomic");
    });

    let line_buffer = ctx.line_buffer_uninit(width * height * 4);
    let sync_buffer = ctx.sync_buffer();

    ::flame::start("setup kernel");
    kernel.set_default_global_work_size(::ocl::SpatialDims::Two(width, height));
    kernel.set_arg("buffer", input.buffer()).unwrap();
    kernel.set_arg("width", width as u64).unwrap();
    kernel.set_arg("height", height as u64).unwrap();
    kernel.set_arg("out", line_buffer.buffer()).unwrap();
    kernel.set_arg("atomic", sync_buffer.buffer()).unwrap();
    ::flame::end("setup kernel");

    unsafe {
        ::flame::span_of("opencl marching [execution]", || kernel.enq().unwrap());
    }

    let count = sync_buffer.value();
    (line_buffer, count)
}
