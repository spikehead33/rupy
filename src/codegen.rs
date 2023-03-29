use inkwell::{
    builder::Builder, context::Context, module::Module, passes::PassManager, values::FunctionValue
};
use rustpython_ast::Stmt;
use rustpython_parser::ast::StmtKind;

struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGenerator<'ctx> {
    fn new(context: &'ctx Context, module_name: &str) -> Self {
        Self {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
        }
    }

    pub fn compile_module(&self) {
        todo!()
    }

    pub fn compile_function(&self) -> Option<FunctionValue> {
        todo!()
    }

    pub fn compile_var_def(&self) {
        todo!()
    }
}
