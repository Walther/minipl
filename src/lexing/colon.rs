use crate::span::StartEndSpan;

use super::Assign;
use super::Colon;
use super::Lexer;
use super::LexingError;
use super::Token;

impl Lexer<'_> {
    /// Internal helper function for scanning a lexeme that starts with a colon. This could be an [Assign], or just a [Colon].
    pub(crate) fn scan_colon(&mut self) -> Result<Token, LexingError> {
        // Consume this token to peek the next
        let (start, _) = self.maybe_next()?;
        // Is this an Assign operator?
        if let Some((_end, _)) = self.iter.next_if(|&(_, char)| char == '=') {
            Ok(Token::new(Assign, StartEndSpan::new(start, start + 2)))
        } else {
            // Otherwise, it's just a Colon
            Ok(Token::new(Colon, StartEndSpan::new(start, start + 1)))
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use crate::{lexing::*, span::StartEndSpan};

    #[test]
    fn colon() {
        let source = ":";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Colon, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn assign() {
        let source = ":=";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Assign, StartEndSpan::new(0, 2));
        assert_eq!(token, expected);
    }
}
