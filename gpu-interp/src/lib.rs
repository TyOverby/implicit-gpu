extern crate ocl;

pub mod bytecode;
pub mod gpu_interp;
pub mod walk_interp;

pub type AstPtr<'a> = &'a Ast<'a>;
pub type AstSlice<'a> = &'a [Ast<'a>];

#[derive(Debug, Copy, Clone)]
pub enum Ast<'a> {
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
}
