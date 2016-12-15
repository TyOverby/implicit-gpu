#[macro_use]
extern crate gc;
#[macro_use]
extern crate lazy_static;

extern crate image as image_crate;

mod nodes;
mod compiler;
pub mod image;

pub use nodes::{circle, not, and};
pub use compiler::compile;
