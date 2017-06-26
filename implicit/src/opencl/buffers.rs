use ocl::Buffer;

#[derive(Debug, Clone)]
pub struct FieldBuffer {
    pub dims: (usize, usize),
    pub internal: Buffer<f32>,
}

#[derive(Debug, Clone)]
pub struct MaskBuffer {
    pub size: usize,
    pub internal: Buffer<u32>,
}

#[derive(Debug, Clone)]
pub struct LinearBuffer {
    pub size: usize,
    pub internal: Buffer<f32>,
}

#[derive(Debug, Clone)]
pub struct LineBuffer {
    pub size: usize,
    pub internal: Buffer<f32>,
}

#[derive(Debug, Clone)]
pub struct SyncBuffer {
    pub internal : Buffer<u32>,
}

impl SyncBuffer {
    pub fn buffer(&self) -> &Buffer<u32> { &self.internal }
}

impl FieldBuffer {
    pub fn size(&self) -> (usize, usize) { (self.width(), self.height()) }
    pub fn width(&self) -> usize { self.dims.0 }

    pub fn height(&self) -> usize { self.dims.1 }

    pub fn values(&self) -> Vec<f32> {
        let mut out = vec![0.0; self.width() * self.height()];
        self.internal.read(&mut out).enq().unwrap();
        out
    }

    pub fn buffer(&self) -> &Buffer<f32> { &self.internal }
}

impl LinearBuffer {
    pub fn size(&self) -> usize { self.size }

    pub fn values(&self) -> Vec<f32> {
        let mut out = vec![0.0; self.size()];
        self.internal.read(&mut out).enq().unwrap();
        out
    }

    pub fn non_nans_at_front(&self) -> bool {
        let mut seen_nan = false;
        for v in self.values() {
            if v.is_nan() {
                seen_nan = true;
            } else if seen_nan {
                return false;
            }
        }
        return true;
    }

    pub fn buffer(&self) -> &Buffer<f32> { &self.internal }
}

impl LineBuffer {
    pub fn size(&self) -> usize { self.size }

    pub fn values(&self) -> Vec<f32> {
        let mut out = vec![0.0; self.size()];
        self.internal.read(&mut out).enq().unwrap();
        out
    }

    pub fn non_nans_at_front(&self) -> bool {
        let mut seen_nan = false;
        for v in self.values() {
            if v.is_nan() {
                seen_nan = true;
            } else if seen_nan {
                return false;
            }
        }
        return true;
    }

    pub fn buffer(&self) -> &Buffer<f32> { &self.internal }
}

impl MaskBuffer {
    pub fn size(&self) -> usize { self.size }

    pub fn values(&self) -> Vec<u32> {
        let mut out = vec![0; self.size()];
        self.internal.read(&mut out).enq().unwrap();
        out
    }

    pub fn buffer(&self) -> &Buffer<u32> { &self.internal }
}
