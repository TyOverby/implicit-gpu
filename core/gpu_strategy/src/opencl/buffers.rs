#![allow(dead_code)]

use ocl::Buffer;

pub type FieldBuffer = gpu_interp::Buffer;

#[derive(Debug, Clone)]
pub struct LineBuffer {
    pub size: usize,
    pub internal: Buffer<f32>,
}

#[derive(Debug, Clone)]
pub struct IndexBuffer {
    pub size: usize,
    pub internal: Buffer<i64>,
}

#[derive(Debug, Clone)]
pub struct SyncBuffer {
    pub internal: Buffer<u32>,
}

impl SyncBuffer {
    pub fn buffer(&self) -> &Buffer<u32> {
        &self.internal
    }
    pub fn value(&self) -> u32 {
        let _guard = ::flame::start_guard("sync buffer value");
        let mut out = vec![0];
        self.internal.read(&mut out).enq().unwrap();
        // TODO: FUCKING REMOVE THIS MULTIPLICATION
        out[0] * 4 // Multiply by 4 because there are 4 floats in a line
    }
}
impl LineBuffer {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn values(&self, _count: Option<u32>) -> Vec<f32> {
        let _guard = ::flame::start_guard("line buffer values");
        let count = self.size();
        let mut out = vec![0.0; count];
        self.internal.read(&mut out).enq().unwrap();
        if let Some(count) = _count {
            let count = count as usize;
            out.drain(count..);
        }
        out
    }

    pub fn non_nans_at_front(&self) -> bool {
        let _guard = ::flame::start_guard("line buffer non-nans-at-front");
        let mut seen_nan = false;
        for v in self.values(None) {
            if v.is_nan() {
                seen_nan = true;
            } else if seen_nan {
                return false;
            }
        }
        return true;
    }

    pub fn buffer(&self) -> &Buffer<f32> {
        &self.internal
    }
}

impl IndexBuffer {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn values(&self, _count: Option<u32>) -> Vec<i64> {
        let _guard = ::flame::start_guard("line buffer values");
        let count = self.size();
        let mut out = vec![-1i64; count];
        self.internal.read(&mut out).enq().unwrap();
        if let Some(count) = _count {
            let count = count as usize;
            out.drain(count..);
        }
        out
    }

    pub fn buffer(&self) -> &Buffer<i64> {
        &self.internal
    }
}
