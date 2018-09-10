use opencl::{FieldBuffer, OpenClContext};

const PROGRAM: &'static str = include_str!("./shaders/simplex.c");

pub fn get_noise(ctx: &OpenClContext, width: usize, height: usize) -> FieldBuffer {
    let out = ctx.field_buffer(width, height, None);
    let mut kernel = ctx.compile("apply", PROGRAM, |register| {
        register.buffer("buffer");
        register.long("width");
    });

    kernel.set_default_global_work_size(::ocl::SpatialDims::Two(width, height));
    kernel.set_arg("buffer", out.buffer()).unwrap();
    kernel.set_arg("width", width as u64).unwrap();
    unsafe {
        kernel.enq().unwrap();
    }
    out
}
