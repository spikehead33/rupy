use rustpython_ast::{Located, Mod};
use rustpython_parser::ast::StmtKind;

type TopLevel = Located<StmtKind>;
pub struct TopLevels<'a>(pub &'a Vec<TopLevel>);

impl<'a> From<&'a Mod> for TopLevels<'a> {
    fn from(value: &'a Mod) -> Self {
        if let Mod::Module { body, .. } = value {
            return Self(body);
        }
        unimplemented!()
    }
}

impl<'a> TopLevels<'a> {
    pub fn toplevels(&self) -> &'a Vec<TopLevel> {
        self.0
    }
}
