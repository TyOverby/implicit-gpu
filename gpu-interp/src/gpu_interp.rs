use super::bytecode::CompilationResult;
use ocl::{Buffer, Context, Kernel, Program, Queue};

pub fn execute(compilation: CompilationResult, width: u32, height: u32) -> Buffer<f32> {
    let context = Context::builder().build().unwrap();
    let device = context.devices().into_iter().next().unwrap();
    let queue = Queue::new(&context, device, None).unwrap();

    let bytecode = Buffer::builder()
        .len([compilation.code.len()])
        .copy_host_slice(&compilation.code)
        .queue(queue.clone())
        .build()
        .unwrap();

    let stack = Buffer::<f32>::builder()
        .len([width * height * compilation.max_stack])
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
        .len([width, height])
        .queue(queue.clone())
        .build()
        .unwrap();

    let program = Program::builder()
        .source(include_str!("./interp.c"))
        .build(&context)
        .unwrap();

    let mut kernel_builder = Kernel::builder();
    let kernel = kernel_builder
        .program(&program)
        .name("apply")
        .queue(queue.clone())
        .global_work_size([width, height])
        .arg(output.clone())
        .arg(constants)
        .arg(bytecode)
        .arg(stack)
        .arg(compilation.max_stack as u64)
        .arg(width as u64)
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
    let b = execute(c, 1, 1);
    let mut out = vec![0.0f32];
    b.read(&mut out).enq().unwrap();
    assert_eq!(out[0], 10.0);
}

#[test]
fn interpret_x() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::X);
    let b = execute(c, 3, 1);
    let mut out = vec![0.0f32; 3];
    b.read(&mut out).enq().unwrap();
    assert_eq!(out, vec![0.0, 1.0, 2.0]);
}

#[test]
fn max_of_a_few_constants() {
    use super::bytecode::*;
    use super::*;
    let c = compile(&Ast::Max(&[Ast::Constant(1.0), Ast::X]));
    let b = execute(c, 3, 1);
    let mut out = vec![0.0f32; 3];
    b.read(&mut out).enq().unwrap();
    assert_eq!(out, vec![1.0, 1.0, 2.0]);
}
