use crate::parsing::{Expression, Statement};

/// Prettyprinter for the abstract syntax tree
mod astprinter;
pub use astprinter::ASTPrinter;

/// Interpreter i.e. evaluator for the language
mod interpreter;
pub use interpreter::Interpreter;

use miette::Result;

/// The main visitor trait for the visitor pattern. See e.g. [Rust Design Patterns: Visitor](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
pub trait Visitor<T, E> {
    /// Expressions
    fn visit_expression(&mut self, expression: &Expression) -> Result<T, E>;
    /// Statements
    fn visit_statement(&mut self, statement: &Statement) -> Result<T, E>;
}
