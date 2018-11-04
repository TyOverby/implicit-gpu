use super::bytecode::CompilationResult;
use super::Buffer;
use ocl::{Buffer as OclBuffer, Context, Device, Kernel, Program, Queue};

#[derive(Clone)]
pub struct Triad {
    context: Context,
    device: Device,
    queue: Queue,
}

impl Triad {
    pub fn default() -> Triad {
        let context = Context::builder().build().unwrap();
        let device = context.devices().into_iter().next().unwrap();
        let queue = Queue::new(&context, device, None).unwrap();
        Triad {
            context,
            device,
            queue,
        }
    }
}

pub fn execute(
    compilation: CompilationResult,
    width: u32,
    height: u32,
    depth: u32,
    Triad { context, queue, .. }: Triad,
) -> Buffer {
    assert!(compilation.buffers.len() <= ::bytecode::ops::BUFFER_COUNT);

    let bytecode = OclBuffer::builder()
        .len([compilation.code.len()])
        .copy_host_slice(&compilation.code)
        .queue(queue.clone())
        .build()
        .unwrap();

    let stack = OclBuffer::<f32>::builder()
        .len([width * height * depth * compilation.max_stack])
        .queue(queue.clone())
        .build()
        .unwrap();

    let position_stack = if compilation.transform_depth == 0 {
        OclBuffer::builder()
            .len([1])
            .copy_host_slice(&[0.0])
            .queue(queue.clone())
            .build()
            .unwrap()
    } else {
        OclBuffer::<f32>::builder()
            .len([3 * width * height * depth * ::std::cmp::max(compilation.transform_depth, 1)])
            .queue(queue.clone())
            .build()
            .unwrap()
    };

    let constants = if compilation.constants.len() == 0 {
        OclBuffer::builder()
            .len([1])
            .copy_host_slice(&[0.0])
            .queue(queue.clone())
            .build()
            .unwrap()
    } else {
        OclBuffer::builder()
            .len([compilation.constants.len()])
            .copy_host_slice(&compilation.constants)
            .queue(queue.clone())
            .build()
            .unwrap()
    };

    let output = OclBuffer::<f32>::builder()
        .len([width, height, depth])
        .queue(queue.clone())
        .build()
        .unwrap();

    let program = Program::builder()
        .source(concat!(
            include_str!(concat!(env!("OUT_DIR"), "/opcodes.c")),
            include_str!("./interp.c")
        )).build(&context)
        .unwrap();

    let mut kernel_builder = Kernel::builder();
    let kernel = kernel_builder
        .program(&program)
        .name("apply")
        .queue(queue.clone())
        .global_work_size([width, height, depth])
        .arg(output.clone())
        .arg(constants)
        .arg(bytecode)
        .arg(stack)
        .arg(position_stack);

    let buffer_count = compilation.buffers.len();
    for mut buffer in compilation.buffers {
        kernel.arg(buffer.to_opencl(&queue).clone());
    }
    for _ in 0..(::bytecode::ops::BUFFER_COUNT - buffer_count) {
        kernel.arg(
            OclBuffer::builder()
                .len([1])
                .copy_host_slice(&[0.0])
                .queue(queue.clone())
                .build()
                .unwrap(),
        );
    }

    let kernel = kernel
        .arg(compilation.max_stack as u64)
        .arg(compilation.transform_depth as u64)
        .arg(width as u64)
        .arg(height as u64)
        .arg(compilation.code.len() as u64)
        .build()
        .unwrap();

    unsafe {
        kernel.enq().unwrap();
    }

    Buffer::from_opencl(output, width, height, depth)
}

#[test]
fn interpret_constant() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Constant(10.0));
    let mut b = execute(c, 1, 1, 1, Triad::default());
    assert_eq!(b.to_memory()[0], 10.0);
}

#[test]
fn x_plus_y() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Add(&[Ast::X, Ast::Y]));
    let mut b = execute(c, 3, 3, 1, Triad::default());
    assert_eq!(
        b.to_memory(),
        &[0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0,]
    );
}

#[test]
fn x_plus_z() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Add(&[Ast::X, Ast::Z]));
    let mut b = execute(c, 3, 1, 3, Triad::default());
    assert_eq!(
        b.to_memory(),
        &[0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0,]
    );
}

#[test]
fn y_plus_z() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Add(&[Ast::Y, Ast::Z]));
    let mut b = execute(c, 1, 3, 3, Triad::default());
    assert_eq!(
        b.to_memory(),
        &[0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0,]
    );
}

#[test]
fn x_in_2_dimensions() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::X);
    let mut b = execute(c, 3, 3, 1, Triad::default());
    assert_eq!(
        b.to_memory(),
        &[0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0]
    );
}

#[test]
fn interpret_x() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::X);
    let mut b = execute(c, 3, 1, 1, Triad::default());
    assert_eq!(b.to_memory(), &[0.0, 1.0, 2.0]);
}

#[test]
fn max_of_a_few_constants() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Max(&[Ast::Constant(1.0), Ast::X]));
    let mut b = execute(c, 3, 1, 1, Triad::default());
    assert_eq!(b.to_memory(), &[1.0, 1.0, 2.0]);
}

#[test]
fn scaled_add_xy() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Transform {
        target: &Ast::Add(&[Ast::X, Ast::Y]),
        matrix: ::euclid::Transform3D::create_scale(2.0, 2.0, 2.0),
    });
    let mut b = execute(c, 3, 3, 1, Triad::default());
    assert_eq!(
        b.to_memory(),
        &[
            // y = 0
            (0.0 * 2.0 + 0.0 * 2.0),
            (1.0 * 2.0 + 0.0 * 2.0),
            (2.0 * 2.0 + 0.0 * 2.0),
            // y = 1
            (0.0 * 2.0 + 1.0 * 2.0),
            (1.0 * 2.0 + 1.0 * 2.0),
            (2.0 * 2.0 + 1.0 * 2.0),
            // y = 2
            (0.0 * 2.0 + 2.0 * 2.0),
            (1.0 * 2.0 + 2.0 * 2.0),
            (2.0 * 2.0 + 2.0 * 2.0),
        ]
    );
}

#[test]
fn max_of_a_few_buffers() {
    use super::bytecode::*;
    use super::*;
    let triad = Triad::default();
    let ones = {
        let mut buf = execute(compile(&Ast::Constant(1.0)), 3, 1, 1, triad.clone());
        assert_eq!(buf.to_memory(), &[1.0, 1.0, 1.0]);
        buf
    };
    let xs = {
        let mut buf = execute(compile(&Ast::X), 3, 1, 1, triad.clone());
        assert_eq!(buf.to_memory(), &[0.0, 1.0, 2.0]);
        buf
    };

    let c = compile(&Ast::Max(&[Ast::Buffer(ones), Ast::Buffer(xs)]));

    let mut buf = execute(c, 3, 1, 1, triad.clone());
    assert_eq!(buf.to_memory(), &[1.0, 1.0, 2.0]);
}
