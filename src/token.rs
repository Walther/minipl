use crate::rawtoken::RawToken;
use crate::rawtoken::RawToken::Error;

/// A richer [Token] type that wraps the [`RawToken`] type, and holds more metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The raw token itself
    pub token: RawToken,
    /// Location, a tuple of start and end. An example single-letter token would have location `(0,1)`.
    pub location: (usize, usize),
}

impl Token {
    #[must_use]
    /// Creates a new [Token] type, when given a [`RawToken`] and a location `(start, end)`
    pub fn new(token: RawToken, location: (usize, usize)) -> Self {
        Self { token, location }
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
