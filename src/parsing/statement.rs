use crate::{span::StartEndSpan, visitors::Visitable};

use super::{expression::Expression, variable::Variable};

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expression(Expression),
    Print(Expression),
    Variable(Variable),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub stmt: Stmt,
    pub span: StartEndSpan,
}

impl Statement {
    pub fn new(stmt: Stmt) -> Self {
        let span = match &stmt {
            Stmt::Expression(e) => e.span,
            Stmt::Print(e) => e.span,
            Stmt::Variable(v) => v.span,
        };
        Self { stmt, span }
    }
}

impl From<Statement> for Visitable {
    fn from(val: Statement) -> Self {
        Visitable::Statement(val)
    }
}
