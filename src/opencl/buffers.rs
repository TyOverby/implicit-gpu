use ocl::Buffer;

pub struct FieldBuffer {
    pub(crate) dims: (usize, usize),
    pub(crate) internal: Buffer<f32>,
}

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
        let mut out = Vec::with_capacity(self.width() * self.height());
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
        let mut out = Vec::with_capacity(self.size());
        self.internal.read(&mut out).enq().unwrap();
        out
    }

    pub fn buffer(&self) -> &Buffer<f32> {
        &self.internal
    }
}
