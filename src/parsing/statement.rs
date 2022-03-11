use crate::span::StartEndSpan;

use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(Expression),
    Print(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub stmt: Stmt,
    pub span: StartEndSpan,
}

impl Statement {
    pub fn new(stmt: Stmt) -> Self {
        let span = match &stmt {
            Stmt::Expr(e) => e.span,
            Stmt::Print(e) => e.span,
        };
        Self { stmt, span }
    }
}
