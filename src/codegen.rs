use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{AnyValueEnum, FloatValue, IntValue},
};
use num_bigint::Sign;
use rustpython_ast::{Arguments, ExprKind, Located};
use rustpython_parser::ast::StmtKind;

type Name = String;
type Args = Box<Arguments>;
type Body = Vec<Located<StmtKind>>;
type Variable = String;
type Type = String;

/// Note!
/// Statement will only occur in the place with 'body' attributes
/// 1. Module's body
/// 2. FunctionDef's body
/// 3. ClassDef's body

trait StmtsCodeGenerator {
    fn gen(&self, stmts: &Vec<Located<StmtKind>>);
}

trait ExprCodeGenerator<'a> {
    fn gen(&self, expr: Located<ExprKind>) -> AnyValueEnum<'a>;
}

struct ModuleCodeGenerator();
struct ClassDefCodeGenerator();
struct FunctionDefCodeGenerator<'a>(Name, Args, &'a Body, );


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
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
}
