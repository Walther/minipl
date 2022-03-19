use crate::span::StartEndSpan;

use super::Error;
use super::Lexer;
use super::LexingError;
use super::Range;
use super::Token;

impl Lexer<'_> {
    /// Internal helper function for scanning a lexeme that starts with a dot. This could be a [Range], or a lexing error.
    pub(crate) fn scan_range(&mut self) -> Result<Token, LexingError> {
        // Consume this token to peek the next
        let (start, _) = self.maybe_next()?;
        // Is this a Range operator?
        if let Some((_, _)) = self.iter.next_if(|&(_, char)| char == '.') {
            Ok(Token::new(Range, StartEndSpan::new(start, start + 2)))
        } else {
            // Otherwise, we have a parse error
            Ok(Token::new(
                Error("Expected another '.' for Range operator".into()),
                StartEndSpan::new(start, start + 1),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use crate::{lexing::*, span::StartEndSpan};

    #[test]
    fn range_singledot() {
        let source = ".";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(
            Error("Expected another '.' for Range operator".into()),
            StartEndSpan::new(0, 1),
        );
        assert_eq!(token, expected);
    }

    #[test]
    fn range_empty() {
        let source = "..";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Range, StartEndSpan::new(0, 2));
        assert_eq!(token, expected);
    }

    // TODO: valid range test, 1..10
}
