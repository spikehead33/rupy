use rustpython_ast::{Located, Mod, StmtKind};

pub struct TypeChecker<'a> {
    mode: &'a Mod,
}

impl<'a> TypeChecker<'a> {
    pub fn new(mode: &'a Mod) -> Self {
        Self { mode }
    }

    /// TODO:
    /// complete the type checker
    pub fn run(&self) -> bool {
        let stmts = if let Mod::Module { body, .. } = self.mode {
            body
        } else {
            std::process::exit(1);
        };

        stmts.iter().all(TypeChecker::completely_annotated)
    }

    /// check if all entity is annotated
    fn completely_annotated(stmt: &Located<StmtKind>) -> bool {
        match &stmt.node {
            StmtKind::AnnAssign { .. } => true,
            StmtKind::FunctionDef {
                args,
                body,
                returns,
                ..
            } => {
                args.args.iter().all(|arg| arg.node.annotation.is_some())
                && returns.is_some()
                && body.iter().all(TypeChecker::completely_annotated)
            }
            StmtKind::Assign { .. } => {
                eprintln!("unannotated statement found!");
                std::process::exit(1)
            }
            StmtKind::ClassDef {
                body,
                ..
            } => {
                body.iter().all(TypeChecker::completely_annotated)
            }
            _ => true
        }
    }
}
