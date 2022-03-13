use crate::parsing::{Expression, Variable};
use crate::span::StartEndSpan;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expression(Expression),
    Print(Expression),
    VariableDefinition(Variable),
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
            Stmt::VariableDefinition(v) => v.span,
        };
        Self { stmt, span }
    }
}
