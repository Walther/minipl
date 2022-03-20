#![allow(missing_docs)] // TODO: document

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Runtime error")]
#[diagnostic()]
/// Runtime errors of the interpreter
pub enum RuntimeError {
    #[diagnostic(help = "Expected a numeric value, got: {0}")]
    AsNumericFailed(String), // TODO: span
    #[diagnostic(help = "Expected a boolean value, got: {0}")]
    AsBooleanFailed(String), // TODO: span
    #[diagnostic(help = "Expected a text value, got: {0}")]
    AsTextFailed(String), // TODO: span
    #[diagnostic(
        help = "Equal operator can only be used for Number=Number or Text=Text, got: {0} = {1}"
    )]
    EqualTypeMismatch(
        String,
        String,
        #[label = "{0}"] SourceSpan,
        #[label = "{1}"] SourceSpan,
    ),
    #[diagnostic(
        help = "Less operator can only be used for Number=Number or Text=Text, got: {0} < {1}"
    )]
    LessTypeMismatch(
        String,
        String,
        #[label = "{0}"] SourceSpan,
        #[label = "{1}"] SourceSpan,
    ),
    #[diagnostic(help = "Unexpected operator for a binary expression: {0}")]
    UnexpectedBinaryOperator(String, #[label = "{0}"] SourceSpan),
    #[diagnostic(help = "Unexpected value for a literal expression: {0}")]
    UnexpectedLiteral(String, #[label = "{0}"] SourceSpan),
    #[diagnostic(help = "Unexpected value for a logical operator: {0}")]
    UnexpectedLogicalOperator(String, #[label = "{0}"] SourceSpan),
    #[diagnostic(help = "Unexpected value for a unary operator: {0}")]
    UnexpectedUnaryOperator(String, #[label = "{0}"] SourceSpan),
    #[diagnostic(help = "Assertion statement must evaluate to true or false")]
    AssertExprNotTruthy(#[label = "not a truthy statement"] SourceSpan),
    #[diagnostic(help = "Assertion failed")]
    AssertionFailed(#[label = "false"] SourceSpan),
    #[diagnostic(help = "Variable assignment failed during for loop")]
    ForBadAssignment(String, #[label = "the variable"] SourceSpan),
    #[diagnostic(help = "End of the for loop should be larger than the start")]
    ForEndLarger(
        #[label = "larger"] SourceSpan,
        #[label = "smaller"] SourceSpan,
    ),
    ForEndNonNumeric(String, #[label = "{0}"] SourceSpan),
    ForStartNonNumeric(String, #[label = "{0}"] SourceSpan),
    #[diagnostic(
        help = "Plus operator can only be used for Number+Number or Text+Text, got: {0} + {1}"
    )]
    PlusTypeMismatch(
        String,
        String,
        #[label = "{0}"] SourceSpan,
        #[label = "{1}"] SourceSpan,
    ),
    #[diagnostic(help = "Failed to flush stdout after print")]
    PrintCouldNotFlush,
    #[diagnostic(help = "Failed to read a variable from stdin")]
    ReadLineFailed,
    #[diagnostic(
        help = "Internal compiler error. Tried to read a variable into a Nothing object."
    )]
    ReadNothing,
    #[diagnostic(help = "Failed to parse a variable from stdin as a boolean")]
    ReadParseBoolFailed,
    #[diagnostic(help = "Failed to parse a variable from stdin as an integer (i64)")]
    ReadParseIntFailed,
    #[diagnostic(help(
        "Try removing the latter `var` to reassign, or use a different identifier"
    ))]
    VariableReDeclaration(#[label = "Attempted to re-declare existing variable name"] SourceSpan),
    #[diagnostic(help("Use the keyword `var` to declare the variable"))]
    VariableAssignToUndeclared(
        #[label = "Attempted to assign to a variable that has not been declared"] SourceSpan,
    ),
    #[diagnostic(help = "Unable to find variable with name: {0}")]
    VariableGetFailed(String), // TODO: span
}
