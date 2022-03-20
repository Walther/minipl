use std::fmt::Display;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Lexing error")]
#[diagnostic()]
/// The unrecoverable error enum for the [Lexer]
pub enum UnrecoverableLexingError {
    /// Ran out of tokens while scanning
    OutOfChars(#[label = "Out of characters error. Lexer expected further input."] SourceSpan),
    /// Unable to parse into an integer
    ParseIntError(#[label = "Could not parse this into a number (i64)"] SourceSpan),
}

/// The recoverable error enum for the [Lexer], these will go in [RawToken::Error](crate::tokens::RawToken::Error)
#[derive(Debug, Clone, PartialEq)]
pub enum RecoverableLexingError {
    /// Unknown escape code. Supported escape characters are: \\r \\t \\n \\' \\\"
    UnknownEscape,
    /// Unterminated string or unescaped newline. Raw newlines in are not supported in strings in Mini-PL.
    Unterminated,
    /// Encountered a single dot. Only use of dots in Mini-PL is the Range operator `..` which requires two dots
    SingleDot,
    /// Encountered an unknown character - something that is not an operator and not valid as a start of an identifier
    UnknownChar(char),
}

use RecoverableLexingError::*;

// TODO: possibly use miette even for these? how?
impl Display for RecoverableLexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self {
            UnknownEscape => write!(f,"Unknown escape character or unescaped backslash. Supported escape characters are: \\r \\t \\n \\' \\\""),
            Unterminated => write!(f, "Unterminated string or unescaped newline. If you need newlines, use \\n"),
            SingleDot => write!(f, "Expected another '.' for Range operator"),
            UnknownChar(t) => write!(f, "Unknown character: {}", t)
        }
    }
}
