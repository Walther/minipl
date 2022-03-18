//! Statements in the Mini-PL programming language.
//!
//! In the parsing phase, the source code is constructed into [Expression]s and [Statement](crate::parsing::Statement)s.

#![allow(missing_docs)] // TODO: document

use crate::parsing::{Expression, Variable};
use crate::span::StartEndSpan;

use super::Forloop;

#[derive(Clone, Debug, PartialEq)]
/// Low-level enum containing all possible statement variants.
pub enum Stmt {
    Assert(Expression),
    Expression(Expression),
    Forloop(Forloop),
    Print(Expression),
    Read(String),
    VariableDefinition(Variable),
}

#[derive(Debug, Clone, PartialEq)]
/// A richer [Statement] type that wraps the [Stmt] enum, and holds more metadata.
pub struct Statement {
    /// The contents of the [Statement], as a low-level [Stmt]
    pub stmt: Stmt,
    /// The location span `(start, end)` of the [Statement]
    pub span: StartEndSpan,
}

impl Statement {
    #[must_use]
    /// Creates a new [Statement]
    pub fn new(stmt: Stmt, span: StartEndSpan) -> Self {
        Self { stmt, span }
    }
}
