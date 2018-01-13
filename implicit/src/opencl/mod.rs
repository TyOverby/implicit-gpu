use ocl::{Buffer, Context, Device, Kernel, Platform, Program, Queue};
use ocl::enums::{DeviceInfo, DeviceInfoResult};
use ocl::flags::{MEM_COPY_HOST_PTR, MEM_READ_WRITE};
use std::sync::Mutex;

mod buffers;

pub use self::buffers::*;

pub struct OpenClContext {
    platform: Platform,
    device: Device,
    context: Context,
    queue: Queue,
    program_cache: Mutex<Vec<(String, Program)>>,
}


pub fn all_devices() -> Vec<(Platform, Device)> {
    let mut out = vec![];
    for plat in Platform::list() {
        if let Ok(all_devices) = Device::list_all(&plat) {
            for dev in all_devices {
                out.push((plat.clone(), dev));
            }
        }
    }

    // Prefer GPU
    out.sort_by(|&(_, ref a), &(_, ref b)| {
        let a_type = a.info(DeviceInfo::Type);
        let b_type = b.info(DeviceInfo::Type);
        if let (DeviceInfoResult::Type(a_type), DeviceInfoResult::Type(b_type)) = (a_type, b_type) {
            b_type.cmp(&a_type)
        } else {
            (0).cmp(&0)
        }
    });

    out
}

impl OpenClContext {
    pub fn new(platform: Platform, device: Device) -> OpenClContext {
        let context = Context::builder()
            .platform(platform)
            .devices(device)
            .build()
            .unwrap();
        let queue = Queue::new(&context, device, None).unwrap();

        OpenClContext {
            platform: platform,
            device: device,
            context: context,
            queue: queue,
            program_cache: Mutex::new(vec![]),
        }
    }

    pub fn max_workgroup_size(&self) -> usize { self.device.max_wg_size().unwrap() }

    pub fn default() -> OpenClContext {
        let (pt, dv) = all_devices().into_iter().nth(0).unwrap();
        OpenClContext::new(pt, dv)
    }

    // TODO(tyoverby): You should use a Kernel Cache instead of
    // Program Cache once Kernels
    // implement Clone.
    pub fn compile<S2: Into<String>, S1: Into<String>>(&self, name: S1, source: S2) -> Kernel {
        let _guard = ::flame::start_guard("OpenClContext::compile");
        let name = name.into();
        let source = source.into();

        {
            let program_cache = self.program_cache.lock().unwrap();
            if let Some(&(_, ref p)) = program_cache
                .iter()
                .filter(|&&(ref s, _)| s == &source)
                .next()
            {
                let _guard = ::flame::start_guard("Kernel::new");
                return Kernel::new(name, p).unwrap();
            }
        }

        let program = Program::builder()
            .src(source.clone())
            .devices(self.device)
            .build(&self.context)
            .unwrap();

        {
            let mut program_cache = self.program_cache.lock().unwrap();
            program_cache.push((source, program.clone()));
        }

        Kernel::new(name, &program).unwrap()
    }

    pub fn field_buffer(&self, width: usize, height: usize, fill: Option<&[f32]>) -> FieldBuffer {
        let _guard = ::flame::start_guard("OpenClContext::field_buffer");
        let buffer = if let Some(fill) = fill {
            assert_eq!(fill.len(), width * height);
            Buffer::new(
                self.queue.clone(),
                Some(MEM_COPY_HOST_PTR | MEM_READ_WRITE),
                &[width, height],
                Some(fill),
            ).unwrap()
        } else {
            Buffer::new(self.queue.clone(), Some(MEM_READ_WRITE), &[width, height], None).unwrap()
        };

        FieldBuffer {
            dims: (width, height),
            internal: buffer,
        }
    }

    pub fn field_buffer_inf(&self, width: usize, height: usize) -> FieldBuffer {
        let _guard = ::flame::start_guard("OpenClContext::field_buffer_inf");
        let buffer = vec![::std::f32::INFINITY; width * height];
        self.field_buffer(width, height, Some(&buffer))
    }

    pub fn field_buffer_neg_inf(&self, width: usize, height: usize) -> FieldBuffer {
        let _guard = ::flame::start_guard("OpenClContext::field_buffer_neg_inf");
        let buffer = vec![::std::f32::NEG_INFINITY; width * height];
        self.field_buffer(width, height, Some(&buffer))
    }

    pub fn line_buffer(&self, fill: &[f32]) -> LineBuffer {
        let _guard = ::flame::start_guard("OpenClContext::linear_buffer");
        LineBuffer {
            size: fill.len(),
            internal: Buffer::new(self.queue.clone(), Some(MEM_COPY_HOST_PTR), &[fill.len()], Some(fill)).unwrap(),
        }
    }

    pub fn sync_buffer(&self) -> SyncBuffer {
        let _guard = ::flame::start_guard("OpenClContext::sync_buffer");
        SyncBuffer {
            internal: Buffer::new(self.queue.clone(), Some(MEM_COPY_HOST_PTR), &[1], Some(&[0])).unwrap(),
        }
    }

    pub fn mask_buffer(&self, size: usize, fill: Option<&[u32]>) -> MaskBuffer {
        let _guard = ::flame::start_guard("OpenClContext::mask_buffer");
        if let Some(fill) = fill {
            debug_assert!(size == fill.len());
            MaskBuffer {
                size: size,
                internal: Buffer::new(
                    self.queue.clone(),
                    Some(MEM_COPY_HOST_PTR | MEM_READ_WRITE),
                    &[size],
                    Some(fill),
                ).unwrap(),
            }
        } else {
            MaskBuffer {
                size: size,
                internal: Buffer::new(self.queue.clone(), Some(MEM_READ_WRITE), &[size], None).unwrap(),
            }
        }
    }

    pub fn empty_queue(&self) {
        let _guard = ::flame::start_guard("OpenClContext::empty_queue");
        self.queue.finish().unwrap();
    }

    pub fn platform(&self) -> &Platform { &self.platform }

    pub fn device(&self) -> &Device { &self.device }

    pub fn context(&self) -> &Context { &self.context }

    pub fn queue(&self) -> &Queue { &self.queue }
}
