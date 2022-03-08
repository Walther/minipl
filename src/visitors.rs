use crate::parsing::expression::*;

/// Prettyprinter for the abstract syntax tree
pub mod astprinter;
pub use astprinter::ASTPrinter;

/// Interpreter i.e. evaluator for the language
pub mod interpreter;
pub use interpreter::Interpreter;

/// The main visitor trait for the visitor pattern. See e.g. [Rust Design Patterns: Visitor](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
pub trait Visitor<T> {
    /// Visit function for the [Binary] type
    fn visit_binary(&mut self, b: &Binary) -> T;
    /// Visit function for the [Grouping] type
    fn visit_grouping(&mut self, g: &Grouping) -> T;
    /// Visit function for the [Literal] type
    fn visit_literal(&mut self, l: &Literal) -> T;
    /// Visit function for the [Unary] type
    fn visit_unary(&mut self, u: &Unary) -> T;
}
