//! Variables in the Mini-PL programming language.
//!
//! This is a run-time construct. // TODO: better docss

use crate::span::StartEndSpan;

use super::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
/// Enum of the possible data types in the Mini-PL programming language. Boolean, Integer and Text.
pub enum VarType {
    /// Boolean value, `true` or `false`
    Boolean,
    /// Integer value, internally represented as an [i64]
    Number,
    /// Text value, internally represented as a [String]
    Text,
}

#[derive(Debug, Clone, PartialEq)]
/// A rich [Variable] type.
pub struct Variable {
    /// Name of the variable, the identifier used for it in the original source code
    pub name: String,
    /// The type of the variable
    pub kind: VarType,
    /// Optional initializer [Expression] that is evaluated in order to set the initial value of the variable
    pub initializer: Option<Expression>,
    /// Location of the variable in the source code
    pub span: StartEndSpan,
}

impl Variable {
    #[must_use]
    /// Creates a new [Variable]
    pub fn new(
        name: &str,
        kind: VarType,
        initializer: Option<Expression>,
        span: StartEndSpan,
    ) -> Self {
        Self {
            name: name.to_owned(),
            kind,
            initializer,
            span,
        }
    }
}
