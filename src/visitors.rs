use crate::parsing::{Expression, Statement};

/// Prettyprinter for the abstract syntax tree
pub mod astprinter;
pub use astprinter::ASTPrinter;

/// Interpreter i.e. evaluator for the language
pub mod interpreter;
pub use interpreter::Interpreter;

use miette::Result;

/// The main visitor trait for the visitor pattern. See e.g. [Rust Design Patterns: Visitor](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
pub trait Visitor<T> {
    /// Expressions
    fn visit_expression(&mut self, expression: &Expression) -> Result<T>;
    /// Statements
    fn visit_statement(&mut self, statement: &Statement) -> Result<T>;
}
