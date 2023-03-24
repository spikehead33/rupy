// use rustpython_ast::{
//     Stmt,
//     StmtKind::{
//         FunctionDef,
//         ClassDef,
//         AnnAssign
//     }
// };

// enum DeclarationType {
//     FunctionDef,
//     ClassDef,
//     GlobalConstant,
// }

// struct Declaration {
//     name: String,
//     decl_type: DeclarationType,
//     ast: Vec<Stmt>,
// }

// #[derive(Debug)]
// struct NotDeclarationError(String);

// impl TryFrom<&Stmt> for Declaration {
//     type Error = NotDeclarationError;

//     fn try_from(value: &Stmt) -> Result<Self, Self::Error> {
//         match value.node {
//             FunctionDef { name, args, body, returns, .. } => (),
//             ClassDef { name, bases, keywords, body, decorator_list } => (),
//             AnnAssign { target, annotation, value, simple } => (),
//             _ => return Err(NotDeclarationError("Unsupported Global declaration".to_string())),
//         }
//     }
// }