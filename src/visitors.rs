use crate::parsing::expression::*;

/// Prettyprinter for the abstract syntax tree
pub mod astprinter;
pub use astprinter::ASTPrinter;

/// Interpreter i.e. evaluator for the language
pub mod interpreter;
pub use interpreter::Interpreter;

use miette::Result;

/// The main visitor trait for the visitor pattern. See e.g. [Rust Design Patterns: Visitor](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
pub trait Visitor<T, E> {
    /// Visit function for the [Binary] type
    fn visit_binary(&mut self, b: &Binary) -> Result<T, E>;
    /// Visit function for the [Grouping] type
    fn visit_grouping(&mut self, g: &Grouping) -> Result<T, E>;
    /// Visit function for the [Literal] type
    fn visit_literal(&mut self, l: &Literal) -> Result<T, E>;
    /// Visit function for the [Unary] type
    fn visit_unary(&mut self, u: &Unary) -> Result<T, E>;
}
