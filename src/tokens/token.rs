use crate::tokens::rawtoken::RawToken;
use crate::tokens::rawtoken::RawToken::Error;

/// A richer [Token] type that wraps the [`RawToken`] type, and holds more metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The raw token itself
    pub token: RawToken,
    /// A tuple of `(start, length)`. An example single-letter token `x` would have span `(0,1)`, a string token `"abc"` would have span `(0,5). Used for [miette]-compatible code spans in error reporting.
    pub span: (usize, usize),
}

impl Token {
    #[must_use]
    /// Creates a new [Token] type, when given a [`RawToken`] and a span `(start, length)`
    pub fn new(token: RawToken, span: (usize, usize)) -> Self {
        Self { token, span }
    }

    #[must_use]
    /// Helper method for filtering [`Error`] types for error message reporting.
    pub fn is_error(&self) -> bool {
        matches!(self.token, Error(_))
    }

    #[must_use]
    /// Helper method for returning the [`RawToken`] type of the [Token]
    pub fn tokentype(&self) -> RawToken {
        self.token.clone()
    }
}
