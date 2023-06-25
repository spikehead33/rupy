use rustpython_ast::{Located, Mod, StmtKind};

pub struct TypeChecker<'a> {
    mode: &'a Mod,
}

#[derive(Debug)]
pub struct TypeCheckError(String);

impl std::fmt::Display for TypeCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for TypeCheckError {
    fn description(&self) -> &str {
        ""
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
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
            StmtKind::Assign { .. } => false,
            StmtKind::For { body, .. } | StmtKind::ClassDef { body, .. } => {
                body.iter().all(TypeChecker::completely_annotated)
            }
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
            _ => true,
        }
    }
}
