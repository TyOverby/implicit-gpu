#![allow(dead_code)]

#[cfg(test)]
mod test;

use super::*;
use cranelift::prelude::*;
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};
use std::collections::HashMap;

pub struct JIT {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: Module<SimpleJITBackend>,
}

struct FunctionTranslator<'a> {
    float: types::Type,
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut Module<SimpleJITBackend>,
}

impl JIT {
    /// Create a new `JIT` instance.
    pub fn new() -> Self {
        // Windows calling conventions are not supported yet.
        if cfg!(windows) {
            unimplemented!();
        }

        let builder = SimpleJITBuilder::new();
        let module = Module::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }

    fn compile(&mut self, name: &str, expr: AstPtr) -> Result<*const u8, String> {
        self.translate(expr).map_err(|e| e.to_string())?;
        let id = self
            .module
            .declare_function(&name, Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;
        self.module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())?;
        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions();
        let code = self.module.get_finalized_function(id);

        Ok(code)
    }

    fn translate(&mut self, expr: AstPtr) -> Result<(), String> {
        let float = types::F32;

        // X, Y, and Z
        self.ctx.func.signature.params.push(AbiParam::new(float));
        self.ctx.func.signature.params.push(AbiParam::new(float));
        self.ctx.func.signature.params.push(AbiParam::new(float));

        self.ctx.func.signature.returns.push(AbiParam::new(float));
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        let entry_ebb = builder.create_ebb();
        builder.append_ebb_params_for_function_params(entry_ebb);
        builder.switch_to_block(entry_ebb);
        builder.seal_block(entry_ebb);
        let variables = declare_variables(float, &mut builder, entry_ebb);

        let mut trans = FunctionTranslator {
            float,
            builder,
            variables,
            module: &mut self.module,
        };

        let return_value = trans.translate_expr(expr);

        trans.builder.ins().return_(&[return_value]);
        trans.builder.finalize();
        Ok(())
    }
}

impl<'a> FunctionTranslator<'a> {
    fn translate_binary_slice<F>(&mut self, slice: &[Ast], f: F) -> Value
    where
        F: Fn(&mut FunctionBuilder<'a>, Value, Value) -> Value,
    {
        assert!(slice.len() > 0);
        let first = &slice[0];
        let rest = &slice[1..];
        rest.iter().fold(self.translate_expr(first), |a, b| {
            let b = self.translate_expr(b);
            f(&mut self.builder, a, b)
        })
    }
    fn translate_expr(&mut self, expr: AstPtr) -> Value {
        match expr {
            Ast::Constant(f) => self.builder.ins().f32const(Ieee32::with_float(*f)),
            Ast::X => self.builder.use_var(*self.variables.get("x").unwrap()),
            Ast::Y => self.builder.use_var(*self.variables.get("y").unwrap()),
            Ast::Z => self.builder.use_var(*self.variables.get("z").unwrap()),

            Ast::Add(slice) => self.translate_binary_slice(slice, |b, x, y| b.ins().fadd(x, y)),
            Ast::Mul(slice) => self.translate_binary_slice(slice, |b, x, y| b.ins().fmul(x, y)),
            Ast::Min(slice) => self.translate_binary_slice(slice, |b, x, y| b.ins().fmin(x, y)),
            Ast::Max(slice) => self.translate_binary_slice(slice, |b, x, y| b.ins().fmax(x, y)),

            Ast::Sub(a, b) => {
                let a = self.translate_expr(a);
                let b = self.translate_expr(b);
                self.builder.ins().fsub(a, b)
            }
            Ast::Neg(a) => {
                let a = self.translate_expr(a);
                self.builder.ins().fneg(a)
            }
            Ast::Sqrt(a) => {
                let a = self.translate_expr(a);
                self.builder.ins().sqrt(a)
            }
            Ast::Abs(a) => {
                let a = self.translate_expr(a);
                self.builder.ins().fabs(a)
            }
            Ast::Square(a) => {
                let a = self.translate_expr(a);
                self.builder.ins().fmul(a, a)
            }

            Ast::Buffer(_) | Ast::DistToPoly(_) | Ast::Transform { .. } => unimplemented!(),
        }
    }
}

fn declare_variables(
    int: types::Type,
    builder: &mut FunctionBuilder,
    entry_ebb: Ebb,
) -> HashMap<String, Variable> {
    let mut variables = HashMap::new();
    let mut index = 0;

    // x, y, z
    for (i, name) in ["x", "y", "z"].iter().enumerate() {
        // TODO: cranelift_frontend should really have an API to make it easy to set
        // up param variables.
        let val = builder.ebb_params(entry_ebb)[i];
        let var = declare_variable(int, builder, &mut variables, &mut index, name);
        builder.def_var(var, val);
    }

    variables
}
fn declare_variable(
    float: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    name: &str,
) -> Variable {
    let var = Variable::new(*index);
    if !variables.contains_key(name) {
        variables.insert(name.into(), var);
        builder.declare_var(var, float);
        *index += 1;
    }
    var
}
