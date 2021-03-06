use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct Buffer {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    kind: BufferKind,
}

#[derive(Debug, Clone, PartialEq)]
enum BufferKind {
    Opencl(OpenclBuffer),
    Memory(Vec<f32>),
    Both(OpenclBuffer, Vec<f32>),
    #[cfg(test)]
    Debug,
}

#[derive(Debug, Clone)]
struct OpenclBuffer(::ocl::Buffer<f32>);

impl PartialEq for OpenclBuffer {
    fn eq(&self, other: &OpenclBuffer) -> bool {
        self.0.as_core().as_ptr() == other.0.as_core().as_ptr()
    }
}

impl Buffer {
    #[cfg(test)]
    pub fn debug() -> Self {
        Buffer {
            width: 0,
            height: 0,
            depth: 0,
            kind: BufferKind::Debug,
        }
    }
    pub fn from_opencl(b: ::ocl::Buffer<f32>, width: u32, height: u32, depth: u32) -> Buffer {
        Buffer {
            width,
            height,
            depth,
            kind: BufferKind::Opencl(OpenclBuffer(b)),
        }
    }

    pub fn from_memory(v: Vec<f32>, width: u32, height: u32, depth: u32) -> Buffer {
        Buffer {
            width,
            height,
            depth,
            kind: BufferKind::Memory(v),
        }
    }

    pub fn to_opencl(&mut self, queue: &::ocl::Queue) -> &::ocl::Buffer<f32> {
        let mut contents = BufferKind::Memory(vec![]);
        ::std::mem::swap(&mut self.kind, &mut contents);
        self.kind = match contents {
            BufferKind::Opencl(o) => BufferKind::Opencl(o),
            BufferKind::Both(o, c) => BufferKind::Both(o, c),
            BufferKind::Memory(c) => {
                let o = ::ocl::Buffer::<f32>::builder()
                    .len([self.width, self.height, self.depth])
                    .queue(queue.clone())
                    .copy_host_slice(&c[..])
                    .build()
                    .unwrap();
                BufferKind::Both(OpenclBuffer(o), c)
            }

            #[cfg(test)]
            BufferKind::Debug => panic!("to_memory called on a BufferKind::Debug"),
        };

        match &self.kind {
            BufferKind::Opencl(OpenclBuffer(o)) => o,
            BufferKind::Both(OpenclBuffer(o), _) => o,
            _ => unreachable!(),
        }
    }

    pub fn to_memory(&mut self) -> &[f32] {
        let mut contents = BufferKind::Memory(vec![]);
        ::std::mem::swap(&mut self.kind, &mut contents);
        self.kind = match contents {
            BufferKind::Memory(c) => BufferKind::Memory(c),
            BufferKind::Both(o, c) => BufferKind::Both(o, c),
            BufferKind::Opencl(o) => {
                let mut buffer = vec![0.0; (self.width * self.height * self.depth) as usize];
                o.0.read(&mut buffer).enq().unwrap();
                BufferKind::Both(o, buffer)
            }

            #[cfg(test)]
            BufferKind::Debug => panic!("to_memory called on a BufferKind::Debug"),
        };

        match &self.kind {
            BufferKind::Memory(c) => c,
            BufferKind::Both(_, c) => c,
            _ => unreachable!(),
        }
    }
}

impl strategy::FieldBuffer for Buffer {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn depth(&self) -> u32 {
        self.depth
    }

    fn values(&mut self) -> Cow<[f32]> {
        Cow::Borrowed(self.to_memory())
    }
}
