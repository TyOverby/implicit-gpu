// Local crates
extern crate aabb_quadtree;
extern crate gpu_interp;
extern crate line_stitch;
extern crate vectorphile;

// My crates
extern crate expectation;
extern crate expectation_plugin;
extern crate expectation_shared;

// External Crates
extern crate euclid;
extern crate flame;
extern crate image;
extern crate itertools;
extern crate lazy_static;
extern crate ocl;
extern crate serde;
extern crate typed_arena;
extern crate vecmath;

#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate serde_json;

extern crate num_traits;

mod compiler;
pub mod debug;
mod geometry;
pub mod inspector;
mod lines;
mod marching;
mod old_compiler;
mod opencl;
mod polygon;

pub mod exec;
pub mod ocaml;
