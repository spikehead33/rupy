use rustpython_ast::{Located, ExprKind};

type Statements = Vec<Located<StmtKind>>;
type Expression = Located<ExprKind>;

struct Arguments<'a> {

}

struct Returns<'a> {

}

struct Module<'a> {
    body: &'a Statements,
}

struct FunctionDef<'a> {
    name: &'a str,
    args: Vec<Arguments>,
    body: &'a Statements,
    returns: Returns
}

struct ClassDef<'a> {
    name: &'a str,
    bases: &'a Vec<Box<ClassDef>>,
    body: &'a Statements,
}

impl<'a> From<&'a Statements> for Module<'a> {
    fn from(value: &'a Statements) -> Self {
        panic!()
    }
}

impl<'a> From<&'a Statements> for FunctionDef<'a> {
    fn from(value: &'a Statements) -> Self {
        panic!()
    }
}

impl<'a> From<&'a Statements> for ClassDef<'a> {
    fn from(value: &'a Statements) -> Self {
        panic!()
    }
}
