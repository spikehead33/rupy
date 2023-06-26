use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{AnyValueEnum, FloatValue, IntValue},
};
use num_bigint::Sign;
use rustpython_ast::{Arguments, ExprKind, Located};
use rustpython_parser::ast::StmtKind;

type Variable = String;
type Type = String;

/// Note!
/// Statement will only occur in the place with 'body' attributes
/// 1. Module's body
/// 2. FunctionDef's body
/// 3. ClassDef's body

trait StmtCodeGenerator {
    fn gen(&self);
}

trait ExprCodeGenerator<'a> {
    fn gen(&self) -> AnyValueEnum<'a>;
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    environment: Vec<(Variable, Type, Option<AnyValueEnum<'ctx>>)>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>, builder: Builder<'ctx>) -> Self {
        Self {
            context,
            module,
            builder,
            environment: vec![],
        }
    }

    pub fn codegen_stmt(&self, stmt: &Located<StmtKind>) {
    }

    pub fn codegen_module(&self, stmts: &Vec<Located<StmtKind>>) {
        println!("{:#?}", stmts);
        for stmt in stmts.iter() {
            match &stmt.node {
                StmtKind::Expr { value } => self.codegen_expr(value),
                StmtKind::AnnAssign {
                    target,
                    annotation,
                    value,
                    ..
                } => self.codegen_assign(target, annotation, value),
                StmtKind::FunctionDef {
                    name,
                    args,
                    body,
                    decorator_list,
                    returns,
                    type_comment,
                } => self.codegen_funcdef(name, args, body, returns),
                StmtKind::ClassDef {
                    name,
                    bases,
                    keywords,
                    body,
                    decorator_list,
                } => self.codegen_classdef(bases, body),
                _ => println!("Skip codegen for StmtKind: {:?}", &stmt.node),
            }
        }
    }

    pub fn codegen_assign(
        &mut self,
        target: &Box<Located<ExprKind>>,
        annotation: &Box<Located<ExprKind>>,
        value: &Option<Box<Located<ExprKind>>>,
    ) {
        let name = if let ExprKind::Name { id, .. } = &target.node {
            id.clone()
        } else {
            unreachable!()
        };

        let ty = if let ExprKind::Name { id, .. } = &annotation.node {
            id.clone()
        } else {
            unreachable!()
        };

        value.as_ref().map_or_else(|| self.environment.push((name, ty, None)), |x| {
            
        })
    }

    pub fn codegen_funcdef(
        &self,
        name: &str,
        args: &Box<Arguments>,
        body: &Vec<Located<StmtKind>>,
        returns: &Option<Box<Located<ExprKind>>>,
    ) {
    }

    pub fn codegen_classdef(&self, bases: &Vec<Located<ExprKind>>, body: &Vec<Located<StmtKind>>) {}

    pub fn codegen_expr(&self, expr: &Box<Located<ExprKind>>) {
        match &expr.node {
            ExprKind::Constant { value, .. } => {}
            ExprKind::BinOp { left, op, right } => {}
            ExprKind::Call {
                func,
                args,
                keywords,
            } => {}
            _ => unimplemented!(),
        }
    }

    pub fn codegen_int(&self, sign: Sign, val: u64) -> IntValue {
        let i64_type = self.context.i64_type();
        match sign {
            Sign::NoSign | Sign::Plus => i64_type.const_int(val as u64, false),
            Sign::Minus => i64_type.const_int(val as u64, true),
        }
    }

    pub fn codegen_float(&self, val: f64) -> FloatValue {
        let f64_type = self.context.f64_type();
        f64_type.const_float(val)
    }

    pub fn codegen_binop(&self, bino)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
}
