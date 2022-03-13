use crate::parsing::{expression::*, statement::Statement, variable::Variable};

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
    /// Variables
    fn visit_variable(&mut self, variable: &Variable) -> Result<T>;
}

// TODO: does this make any sense
/// Enum of all the Visitable objects in the language implementation
#[derive(Debug, Clone, PartialEq)]
pub enum Visitable {
    /// An expression
    Expression(Expression),
    /// A statement
    Statement(Statement),
    /// A variable
    Variable(Variable),
}
