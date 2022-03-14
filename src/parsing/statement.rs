use crate::parsing::{Expression, Variable};
use crate::span::StartEndSpan;

use super::Forloop;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Assert(Expression),
    Expression(Expression),
    Forloop(Forloop),
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
