use ocl::builders::{BufferBuilder, KernelBuilder};
use ocl::enums::{DeviceInfo, DeviceInfoResult};
use ocl::{Context, Device, Kernel, Platform, Program, Queue};
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
    use std::cmp::Ordering;
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
        match (a_type, b_type) {
            (Ok(DeviceInfoResult::Type(a)), Ok(DeviceInfoResult::Type(b))) => b.cmp(&a),
            (Ok(_), Err(_)) => Ordering::Less,
            (Err(_), Ok(_)) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    });

    out
}

pub struct Register<'a, 'b: 'a> {
    b: &'a mut KernelBuilder<'b>,
}

impl<'a, 'b> Register<'a, 'b> {
    pub fn register_buffer<'c, S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.b
            .arg_named::<_, _, Option<&::ocl::Buffer<f32>>>(name, None);
    }
    pub fn register_float<'c, S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.b.arg_named(name, &0.0f32);
    }
    pub fn register_long<'c, S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.b.arg_named(name, &0u64);
    }
    pub fn register_matrix(&mut self) {
        self.register_float("m11");
        self.register_float("m12");
        self.register_float("m21");
        self.register_float("m22");
        self.register_float("m31");
        self.register_float("m32");
    }
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

    pub fn max_workgroup_size(&self) -> usize {
        self.device.max_wg_size().unwrap()
    }

    pub fn default() -> OpenClContext {
        let (pt, dv) = all_devices().into_iter().nth(0).unwrap();
        OpenClContext::new(pt, dv)
    }

    // TODO(tyoverby): You should use a Kernel Cache instead of
    // Program Cache once Kernels
    // implement Clone.
    pub fn compile<S1, S2, F>(&self, name: S1, source: S2, f: F) -> Kernel
    where
        S2: Into<String>,
        S1: Into<String>,
        F: for<'a, 'b, 'c> FnOnce(&'c mut Register<'a, 'b>),
    {
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
                let mut builder = KernelBuilder::new();
                builder.queue(self.queue.clone()).name(name).program(p);
                f(&mut Register { b: &mut builder });
                return builder.build_unfinished().unwrap();
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

        {
            let _guard = ::flame::start_guard("Kernel::new");
            let mut builder = KernelBuilder::new();
            builder
                .queue(self.queue.clone())
                .name(name)
                .program(&program);
            f(&mut Register { b: &mut builder });
            return builder.build_unfinished().unwrap();
        }
    }

    pub fn field_buffer(&self, width: usize, height: usize, fill: Option<&[f32]>) -> FieldBuffer {
        let _guard = ::flame::start_guard("OpenClContext::field_buffer");

        let builder: BufferBuilder<f32> = BufferBuilder::new()
            .queue(self.queue.clone())
            .len(&[width, height]);

        let internal = if let Some(fill) = fill {
            if fill.len() == 1 {
                builder.fill_val(fill[0]).build().unwrap()
            } else {
                let built = builder.build().unwrap();
                built.write(fill).enq().unwrap();
                built
            }
        } else {
            builder.build().unwrap()
        };

        FieldBuffer {
            dims: (width, height),
            internal,
        }
    }

    pub fn field_buffer_nan(&self, width: usize, height: usize) -> FieldBuffer {
        let _guard = ::flame::start_guard("OpenClContext::field_buffer_inf");
        let buffer = &[::std::f32::NAN][..];
        self.field_buffer(width, height, Some(buffer))
    }

    pub fn field_buffer_inf(&self, width: usize, height: usize) -> FieldBuffer {
        let _guard = ::flame::start_guard("OpenClContext::field_buffer_inf");
        let buffer = &[::std::f32::INFINITY][..];
        self.field_buffer(width, height, Some(buffer))
    }

    pub fn field_buffer_neg_inf(&self, width: usize, height: usize) -> FieldBuffer {
        let _guard = ::flame::start_guard("OpenClContext::field_buffer_neg_inf");
        let buffer = &[::std::f32::NEG_INFINITY][..];
        self.field_buffer(width, height, Some(buffer))
    }

    pub fn line_buffer_uninit(&self, len: usize) -> LineBuffer {
        let _guard = ::flame::start_guard("OpenClContext::linear_buffer");
        let internal = BufferBuilder::new()
            .queue(self.queue.clone())
            .len(&[len])
            .build()
            .unwrap();
        LineBuffer {
            size: len,
            internal,
        }
    }

    pub fn line_buffer(&self, fill: &[f32]) -> LineBuffer {
        let _guard = ::flame::start_guard("OpenClContext::linear_buffer");
        let internal = BufferBuilder::new()
            .queue(self.queue.clone())
            .len(&[fill.len()])
            .build()
            .unwrap();
        internal.write(fill).enq().unwrap();
        LineBuffer {
            size: fill.len(),
            internal,
        }
    }

    pub fn sync_buffer(&self) -> SyncBuffer {
        let _guard = ::flame::start_guard("OpenClContext::sync_buffer");
        SyncBuffer {
            internal: BufferBuilder::new()
                .queue(self.queue.clone())
                .len(&[1])
                .fill_val(0u32)
                .build()
                .unwrap(),
        }
    }

    pub fn empty_queue(&self) {
        let _guard = ::flame::start_guard("OpenClContext::empty_queue");
        self.queue.finish().unwrap();
    }

    pub fn platform(&self) -> &Platform {
        &self.platform
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
