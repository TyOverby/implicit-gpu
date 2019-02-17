extern crate cranelift;
extern crate cranelift_module;
extern crate cranelift_simplejit;
extern crate euclid;
extern crate ocl;

pub mod ast_walk;
mod buffer;
pub mod gpu;
pub mod jit;

pub use buffer::*;

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
    Square(AstPtr<'a>),
    DistToPoly(Vec<(f32, f32, f32, f32)>),
    Transform {
        target: AstPtr<'a>,
        matrix: euclid::Transform3D<f32>,
    },
}
