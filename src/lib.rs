#[macro_use]
extern crate gc;
#[macro_use]
extern crate lazy_static;

extern crate vecmath;
extern crate ocl;
extern crate flame;
extern crate fnv;
extern crate itertools;
extern crate image as image_crate;

pub mod nodes;
pub mod compiler;
pub mod opencl;
pub mod image;
pub mod polygon;
pub mod marching;
pub mod node_group;

pub use nodes::{circle, NodePtr};
pub use compiler::compile;
