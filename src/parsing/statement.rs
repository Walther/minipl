use crate::parsing::{Expression, Variable};
use crate::span::StartEndSpan;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expression(Expression),
    Print(Expression),
    Read(String),
    VariableDefinition(Variable),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub stmt: Stmt,
    pub span: StartEndSpan,
}

impl Statement {
    pub fn new(stmt: Stmt, span: StartEndSpan) -> Self {
        Self { stmt, span }
    }
}
