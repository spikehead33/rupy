mod codegen;
mod toplevel;
mod typechecker;
mod util;

use codegen::CodeGen;
use inkwell::context::Context;
use inkwell::execution_engine::RemoveModuleError;
use rustpython_ast::Mod;
use toplevel::TopLevels;

use anyhow::Result;
use rustpython_parser::{mode, parser};
use std::env;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).map_or("test.py".to_string(), |x| x.clone());
    let source = read_to_string(filename)?;
    let mode = parser::parse(source.as_str(), mode::Mode::Module, "test.py")?;

    // make sure type annotation are there
    let tychecker = typechecker::TypeChecker::new(&mode);
    if !tychecker.run() {
        eprintln!("Type checks failed");
    }

    // code generation
    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("__main__");
    let codegen = CodeGen::new(&context, module, builder);

    if let Mod::Module { body, .. } = mode {
        codegen.codegen_module(&body);
    }

    Ok(())
}
