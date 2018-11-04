use super::bytecode::CompilationResult;
use ocl::{Buffer, Context, Device, Kernel, Program, Queue};

#[derive(Clone)]
pub struct Triad {
    context: Context,
    device: Device,
    queue: Queue,
}

impl Triad {
    fn default() -> Triad {
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
) -> Buffer<f32> {
    assert!(compilation.buffers.len() <= ::bytecode::ops::BUFFER_COUNT);

    let bytecode = Buffer::builder()
        .len([compilation.code.len()])
        .copy_host_slice(&compilation.code)
        .queue(queue.clone())
        .build()
        .unwrap();

    let stack = Buffer::<f32>::builder()
        .len([width * height * depth * compilation.max_stack])
        .queue(queue.clone())
        .build()
        .unwrap();

    let constants = if compilation.constants.len() == 0 {
        Buffer::builder()
            .len([1])
            .copy_host_slice(&[0.0])
            .queue(queue.clone())
            .build()
            .unwrap()
    } else {
        Buffer::builder()
            .len([compilation.constants.len()])
            .copy_host_slice(&compilation.constants)
            .queue(queue.clone())
            .build()
            .unwrap()
    };

    let output = Buffer::<f32>::builder()
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
        .arg(stack);
    let buffer_count = compilation.buffers.len();
    for buffer in compilation.buffers {
        kernel.arg(buffer.buffer);
    }
    for _ in 0..(::bytecode::ops::BUFFER_COUNT - buffer_count) {
        kernel.arg(
            Buffer::builder()
                .len([1])
                .copy_host_slice(&[0.0])
                .queue(queue.clone())
                .build()
                .unwrap(),
        );
    }

    let kernel = kernel
        .arg(compilation.max_stack as u64)
        .arg(width as u64)
        .arg(height as u64)
        .arg(compilation.code.len() as u64)
        .build()
        .unwrap();

    unsafe {
        kernel.enq().unwrap();
    }

    output
}

#[test]
fn interpret_constant() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Constant(10.0));
    let b = execute(c, 1, 1, 1, Triad::default());
    let mut out = vec![0.0f32];
    b.read(&mut out).enq().unwrap();
    assert_eq!(out[0], 10.0);
}

#[test]
fn x_plus_y() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Add(&[Ast::X, Ast::Y]));
    let b = execute(c, 3, 3, 1, Triad::default());
    let mut out = vec![0.0f32; 9];
    b.read(&mut out).enq().unwrap();
    assert_eq!(out, vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0,]);
}

#[test]
fn x_plus_z() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Add(&[Ast::X, Ast::Z]));
    let b = execute(c, 3, 1, 3, Triad::default());
    let mut out = vec![0.0f32; 9];
    b.read(&mut out).enq().unwrap();
    assert_eq!(out, vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0,]);
}

#[test]
fn y_plus_z() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Add(&[Ast::Y, Ast::Z]));
    let b = execute(c, 1, 3, 3, Triad::default());
    let mut out = vec![0.0f32; 9];
    b.read(&mut out).enq().unwrap();
    assert_eq!(out, vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0,]);
}

#[test]
fn x_in_2_dimensions() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::X);
    let b = execute(c, 3, 3, 1, Triad::default());
    let mut out = vec![0.0f32; 9];
    b.read(&mut out).enq().unwrap();
    assert_eq!(out, vec![0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0]);
}

#[test]
fn interpret_x() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::X);
    let b = execute(c, 3, 1, 1, Triad::default());
    let mut out = vec![0.0f32; 3];
    b.read(&mut out).enq().unwrap();
    assert_eq!(out, vec![0.0, 1.0, 2.0]);
}

#[test]
fn max_of_a_few_constants() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Max(&[Ast::Constant(1.0), Ast::X]));
    let b = execute(c, 3, 1, 1, Triad::default());
    let mut out = vec![0.0f32; 3];
    b.read(&mut out).enq().unwrap();
    assert_eq!(out, vec![1.0, 1.0, 2.0]);
}

#[test]
fn max_of_a_few_buffers() {
    use super::bytecode::*;
    use super::*;
    let triad = Triad::default();
    let ones = {
        let buf = execute(compile(&Ast::Constant(1.0)), 3, 1, 1, triad.clone());
        let mut out = vec![0.0f32; 3];
        buf.read(&mut out).enq().unwrap();
        assert_eq!(out, vec![1.0, 1.0, 1.0]);
        buf
    };
    let xs = {
        let buf = execute(compile(&Ast::X), 3, 1, 1, triad.clone());
        let mut out = vec![0.0f32; 3];
        buf.read(&mut out).enq().unwrap();
        assert_eq!(out, vec![0.0, 1.0, 2.0]);
        buf
    };

    let c = compile(&Ast::Max(&[
        Ast::Buffer(Buffer { buffer: ones }),
        Ast::Buffer(Buffer { buffer: xs }),
    ]));

    let buf = execute(c, 3, 1, 1, triad.clone());
    let mut out = vec![0.0f32; 3];
    buf.read(&mut out).enq().unwrap();
    assert_eq!(out, vec![1.0, 1.0, 2.0]);
}
