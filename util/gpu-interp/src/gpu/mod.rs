pub mod bytecode;
mod gpu_interp;

pub use self::bytecode::compile;
pub use self::gpu_interp::{execute, Triad};
