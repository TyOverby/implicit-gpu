extern crate euclid;
extern crate ocl;

mod buffer;
pub mod bytecode;
mod gpu_interp;
pub mod walk_interp;

pub use buffer::*;
pub use bytecode::compile;
pub use gpu_interp::{execute, Triad};

pub type AstPtr<'a> = &'a Ast<'a>;
pub type AstSlice<'a> = &'a [Ast<'a>];

#[derive(Debug, Clone)]
pub enum Ast<'a> {
    Buffer(Buffer),
    Constant(f32),
    X,
    Y,
    Z,
    Add(AstSlice<'a>),
    Mul(AstSlice<'a>),
    Sub(AstPtr<'a>, AstPtr<'a>),
    Max(AstSlice<'a>),
    Min(AstSlice<'a>),
    Abs(AstPtr<'a>),
    Neg(AstPtr<'a>),
    Sqrt(AstPtr<'a>),
    Transform {
        target: AstPtr<'a>,
        matrix: euclid::Transform3D<f32>,
    },
}
