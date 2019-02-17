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
        let variables = declare_variables(float, &mut builder, entry_ebb, expr);
        let x_var = variables.get("x").unwrap().clone();
        let y_var = variables.get("y").unwrap().clone();
        let z_var = variables.get("z").unwrap().clone();

        let mut trans = FunctionTranslator {
            float,
            builder,
            variables,
            module: &mut self.module,
        };

        let return_value = trans.translate_expr(expr, x_var, y_var, z_var, 0);

        trans.builder.ins().return_(&[return_value]);
        trans.builder.finalize();
        Ok(())
    }
}

impl<'a> FunctionTranslator<'a> {
    fn translate_binary_slice<F>(
        &mut self,
        slice: &[Ast],
        xvar: Variable,
        yvar: Variable,
        zvar: Variable,
        transform_depth: usize,
        f: F,
    ) -> Value
    where
        F: Fn(&mut FunctionBuilder<'a>, Value, Value) -> Value,
    {
        assert!(slice.len() > 0);
        let first = &slice[0];
        let rest = &slice[1..];
        rest.iter().fold(
            self.translate_expr(first, xvar, yvar, zvar, transform_depth),
            |a, b| {
                let b = self.translate_expr(b, xvar, yvar, zvar, transform_depth);
                f(&mut self.builder, a, b)
            },
        )
    }
    fn translate_expr(
        &mut self,
        expr: AstPtr,
        xvar: Variable,
        yvar: Variable,
        zvar: Variable,
        transform_depth: usize,
    ) -> Value {
        match expr {
            Ast::Constant(f) => self.builder.ins().f32const(Ieee32::with_float(*f)),
            Ast::X => self.builder.use_var(xvar),
            Ast::Y => self.builder.use_var(yvar),
            Ast::Z => self.builder.use_var(zvar),

            Ast::Add(slice) => {
                self.translate_binary_slice(slice, xvar, yvar, zvar, transform_depth, |b, x, y| {
                    b.ins().fadd(x, y)
                })
            }
            Ast::Mul(slice) => {
                self.translate_binary_slice(slice, xvar, yvar, zvar, transform_depth, |b, x, y| {
                    b.ins().fmul(x, y)
                })
            }
            Ast::Min(slice) => {
                self.translate_binary_slice(slice, xvar, yvar, zvar, transform_depth, |b, x, y| {
                    b.ins().fmin(x, y)
                })
            }
            Ast::Max(slice) => {
                self.translate_binary_slice(slice, xvar, yvar, zvar, transform_depth, |b, x, y| {
                    b.ins().fmax(x, y)
                })
            }

            Ast::Sub(a, b) => {
                let a = self.translate_expr(a, xvar, yvar, zvar, transform_depth);
                let b = self.translate_expr(b, xvar, yvar, zvar, transform_depth);
                self.builder.ins().fsub(a, b)
            }
            Ast::Neg(a) => {
                let a = self.translate_expr(a, xvar, yvar, zvar, transform_depth);
                self.builder.ins().fneg(a)
            }
            Ast::Sqrt(a) => {
                let a = self.translate_expr(a, xvar, yvar, zvar, transform_depth);
                self.builder.ins().sqrt(a)
            }
            Ast::Abs(a) => {
                let a = self.translate_expr(a, xvar, yvar, zvar, transform_depth);
                self.builder.ins().fabs(a)
            }
            Ast::Square(a) => {
                let a = self.translate_expr(a, xvar, yvar, zvar, transform_depth);
                self.builder.ins().fmul(a, a)
            }

            Ast::Transform { target, matrix } => {
                let (new_x, new_y, new_z, new_w) = (
                    self.variables
                        .get(&format!("x_{}", transform_depth))
                        .unwrap()
                        .clone(),
                    self.variables
                        .get(&format!("y_{}", transform_depth))
                        .unwrap()
                        .clone(),
                    self.variables
                        .get(&format!("z_{}", transform_depth))
                        .unwrap()
                        .clone(),
                    self.variables
                        .get(&format!("w_{}", transform_depth))
                        .unwrap()
                        .clone(),
                );
                let (x, y, z, w) = {
                    let mut form = |a, b, c, d| {
                        let xx = self.builder.use_var(xvar);
                        let yy = self.builder.use_var(yvar);
                        let zz = self.builder.use_var(zvar);

                        let ac = self.builder.ins().f32const(Ieee32::with_float(a));
                        let av = self.builder.ins().fmul(xx, ac);

                        let bc = self.builder.ins().f32const(Ieee32::with_float(b));
                        let bv = self.builder.ins().fmul(yy, bc);

                        let cc = self.builder.ins().f32const(Ieee32::with_float(c));
                        let cv = self.builder.ins().fmul(zz, cc);

                        let f1 = self.builder.ins().fadd(av, bv);
                        let dc = self.builder.ins().f32const(Ieee32::with_float(d));
                        let f2 = self.builder.ins().fadd(cv, dc);
                        self.builder.ins().fadd(f1, f2)
                    };

                    /*
                    let x = x_s * m11 + y_s * m21 + z_s * m31 + m41;
                    let y = x_s * m12 + y_s * m22 + z_s * m32 + m42;
                    let z = x_s * m13 + y_s * m23 + z_s * m33 + m43;
                    let w = x_s * m14 + y_s * m24 + z_s * m34 + m44;
                    */

                    let x = form(matrix.m11, matrix.m21, matrix.m31, matrix.m41);
                    let y = form(matrix.m12, matrix.m22, matrix.m32, matrix.m42);
                    let z = form(matrix.m13, matrix.m23, matrix.m33, matrix.m43);
                    let w = form(matrix.m14, matrix.m24, matrix.m34, matrix.m44);
                    (x, y, z, w)
                };

                self.builder.def_var(new_w, w);

                let w_used = self.builder.use_var(new_w);

                let x = self.builder.ins().fdiv(x, w_used);
                let y = self.builder.ins().fdiv(y, w_used);
                let z = self.builder.ins().fdiv(z, w_used);

                self.builder.def_var(new_x, x);
                self.builder.def_var(new_y, y);
                self.builder.def_var(new_z, z);

                self.translate_expr(target, new_x, new_y, new_z, transform_depth + 1)
            }
            Ast::Buffer(_) | Ast::DistToPoly(_) => unimplemented!(),
        }
    }
}

fn declare_variables(
    float: types::Type,
    builder: &mut FunctionBuilder,
    entry_ebb: Ebb,
    ast: AstPtr,
) -> HashMap<String, Variable> {
    let mut variables = HashMap::new();
    let mut index = 0;

    // x, y, z
    for (i, name) in ["x", "y", "z"].iter().enumerate() {
        let val = builder.ebb_params(entry_ebb)[i];
        let var = declare_variable(float, builder, &mut variables, &mut index, name);
        builder.def_var(var, val);
    }

    for i in 0..max_depth_transforms(ast) {
        for dim in ["x", "y", "z", "w"].iter() {
            let var = format!("{}_{}", dim, i);
            declare_variable(float, builder, &mut variables, &mut index, &var);
        }
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

fn max_depth_transforms(expr: AstPtr) -> usize {
    match expr {
        Ast::Constant(_) | Ast::X | Ast::Y | Ast::Z | Ast::Buffer(_) | Ast::DistToPoly(_) => 0,
        Ast::Neg(a) | Ast::Sqrt(a) | Ast::Square(a) | Ast::Abs(a) => max_depth_transforms(a),

        Ast::Add(slice) | Ast::Mul(slice) | Ast::Max(slice) | Ast::Min(slice) => {
            slice.iter().map(max_depth_transforms).max().unwrap_or(0)
        }

        Ast::Sub(a, b) => {
            let a = max_depth_transforms(a);
            let b = max_depth_transforms(b);
            a.max(b)
        }

        Ast::Transform { target, .. } => max_depth_transforms(target) + 1,
    }
}
