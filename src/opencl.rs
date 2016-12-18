use ocl::{Platform, Device, Context, Queue, Program, Kernel, Buffer};
use ocl::enums::{DeviceInfo, DeviceInfoResult};
use ocl::traits::{MemLen, OclPrm};
use ocl::flags::{MEM_COPY_HOST_PTR};

pub struct OpenClContext {
    platform: Platform,
    device: Device,
    context: Context,
    queue: Queue,
}

pub fn all_devices() -> Vec<(Platform, Device)> {
    let mut out = vec![];
    for plat in Platform::list() {
        for dev in Device::list(&plat, None) {
            out.push((plat.clone(), dev));
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
        let context = Context::builder().platform(platform).devices(device).build().unwrap();
        let queue = Queue::new(&context, device).unwrap();

        OpenClContext {
            platform: platform,
            device: device,
            context: context,
            queue: queue,
        }
    }

    pub fn default() -> OpenClContext {
        let (pt, dv) = all_devices().into_iter().nth(0).unwrap();
        OpenClContext::new(pt, dv)
    }

    pub fn compile<S2: Into<String>, S1: Into<String>>(&self, name: S1, source: S2) -> Kernel {
        let program = Program::builder().src(source).devices(self.device).build(&self.context).unwrap();
        Kernel::new(name, &program, &self.queue).unwrap()
    }

    pub fn output_buffer<D: MemLen, O: OclPrm>(&self, dims: D) -> Buffer<O> {
        Buffer::new(&self.queue, None, &dims, None).unwrap()
    }

    pub fn input_buffer<D: MemLen, O: OclPrm>(&self, dims: D, input: &[O]) -> Buffer<O> {
        let buffer = Buffer::new(&self.queue, Some(MEM_COPY_HOST_PTR), &dims, Some(input)).unwrap();
        buffer
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
