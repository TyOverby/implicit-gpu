// Local crates
extern crate aabb_quadtree;
extern crate line_stitch;
extern crate vectorphile;

// My crates
extern crate expectation;
extern crate expectation_plugin;

// External Crates
extern crate euclid;
extern crate flame;
extern crate image;
extern crate itertools;
extern crate lazy_static;
extern crate ocl;
extern crate serde;
extern crate vecmath;

#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate serde_json;

extern crate num_traits;

mod compiler;
mod debug;
mod geometry;
pub mod inspector;
mod lines;
mod noise;
mod marching;
mod opencl;
mod polygon;

pub mod exec;
pub mod ocaml;
