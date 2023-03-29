// mod structures;
mod codegen;
mod typechecker;

use anyhow::Result;
use rustpython_parser::{mode, parser};
use std::env;
use std::fs::read_to_string;

use crate::codegen::CodeGenerator;
use inkwell::context::Context;

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

    let context = Context::create();
    let module = context.create_module("firstmodule");
    let builder = context.create_builder();

    // println!("context {:#?}", context);
    // println!();
    // println!("module {:#?}", module);
    // println!();
    // println!("builder {:#?}", builder);

    Ok(())
}
