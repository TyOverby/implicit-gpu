extern crate euclid;
extern crate extern_api;
extern crate flame;
extern crate gpu_interp;
extern crate itertools;
extern crate line_stitch;
extern crate ocl;
extern crate strategy;
extern crate typed_arena;

extern crate expectation;
extern crate expectation_plugin;
extern crate expectation_shared;

extern crate debug_helpers;

use extern_api::*;
use std::borrow::Cow;

mod compiler;
mod impls;
mod opencl;

pub struct GpuStrategy {
    cl_context: opencl::OpenClContext,
}

impl strategy::Strategy for GpuStrategy {
    type FieldBuf = gpu_interp::Buffer;
    type LineBuf = opencl::LineBuffer;

    fn march_2d(&self, mut buf: gpu_interp::Buffer) -> (Self::LineBuf, u32) {
        impls::run_marching(&mut buf, &self.cl_context)
    }

    fn drag_2d(&self, mut buf: gpu_interp::Buffer, dx: f32, dy: f32) -> gpu_interp::Buffer {
        impls::exec_drag(&self.cl_context, &mut buf, dx, dy)
    }

    fn freeze_2d(&self, mut buf: gpu_interp::Buffer) -> gpu_interp::Buffer {
        impls::exec_freeze(&self.cl_context, &mut buf)
    }

    fn noise_2d(
        &self,
        width: u32,
        height: u32,
        cutoff: f32,
        matrix: extern_api::Matrix,
    ) -> gpu_interp::Buffer {
        impls::get_noise(&self.cl_context, width, height, cutoff, matrix)
    }

    fn poly_2d(&self, polygon: Polygon, width: u32, height: u32) -> gpu_interp::Buffer {
        impls::exec_poly(&self.cl_context, polygon, width, height)
    }

    fn shape<F>(&self, shape: Shape, width: u32, height: u32, buffer_find: F) -> gpu_interp::Buffer
    where
        F: Fn(Id) -> gpu_interp::Buffer,
    {
        unimplemented!()
    }
}
