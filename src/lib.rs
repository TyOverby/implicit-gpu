#[macro_use]
extern crate gc;
#[macro_use]
extern crate lazy_static;

extern crate ocl;
extern crate image as image_crate;

mod nodes;
mod compiler;
pub mod image;
pub mod polygon;

pub use nodes::{circle, construct, NodePtr};
pub use compiler::compile;
