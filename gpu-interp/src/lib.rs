extern crate euclid;
extern crate ocl;

mod buffer;
pub mod bytecode;
pub mod gpu_interp;
pub mod walk_interp;

use buffer::*;

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
    Sub(AstPtr<'a>, AstPtr<'a>),
    Max(AstSlice<'a>),
    Min(AstSlice<'a>),
    Abs(AstPtr<'a>),
    Sqrt(AstPtr<'a>),
    Transform {
        target: AstPtr<'a>,
        matrix: euclid::Transform3D<f32>,
    },
}
