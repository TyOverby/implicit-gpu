extern crate ocl;

pub mod bytecode;
pub mod gpu_interp;
pub mod walk_interp;

pub type AstPtr<'a> = &'a Ast<'a>;
pub type AstSlice<'a> = &'a [Ast<'a>];

#[derive(Debug, Clone)]
pub struct Buffer {
    buffer: ocl::Buffer<f32>,
}

impl PartialEq for Buffer {
    fn eq(&self, other: &Buffer) -> bool {
        self.buffer.as_core().as_ptr() == other.buffer.as_core().as_ptr()
    }
}

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
}
