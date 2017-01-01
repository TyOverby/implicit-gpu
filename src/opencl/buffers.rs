use ocl::Buffer;

#[derive(Debug, Clone)]
pub struct FieldBuffer {
    pub(crate) dims: (usize, usize),
    pub(crate) internal: Buffer<f32>,
}

#[derive(Debug, Clone)]
pub struct LineBuffer {
    pub(crate) size: usize,
    pub(crate) internal: Buffer<f32>,
}

impl FieldBuffer {
    pub fn width(&self) -> usize {
        self.dims.0
    }

    pub fn height(&self) -> usize {
        self.dims.1
    }

    pub fn values(&self) -> Vec<f32> {
        let mut out = vec![0.0; self.width() * self.height()];
        self.internal.read(&mut out).enq().unwrap();
        out
    }

    pub fn buffer(&self) -> &Buffer<f32> {
        &self.internal
    }
}

impl LineBuffer {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn values(&self) -> Vec<f32> {
        let mut out = vec![0.0; self.size()];
        self.internal.read(&mut out).enq().unwrap();
        out
    }

    pub fn buffer(&self) -> &Buffer<f32> {
        &self.internal
    }
}
