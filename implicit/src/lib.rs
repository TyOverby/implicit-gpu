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

pub mod compiler;
pub mod debug;
pub mod geometry;
pub mod inspector;
pub mod lines;
pub mod marching;
pub mod opencl;
pub mod polygon;
pub mod surface_net;

pub mod exec;
pub mod ocaml;
