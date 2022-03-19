use crate::span::StartEndSpan;

use super::Lexer;
use super::LexingError;
use super::Token;
use super::Whitespace;

/// Internal helper function for scanning and skipping over whitespace. Greedy / maximal munch.
impl Lexer<'_> {
    pub(crate) fn scan_whitespace(&mut self) -> Result<Token, LexingError> {
        let &(start, _) = self.maybe_peek()?;
        let mut length = 0;

        while let Some((_, _)) = self.iter.next_if(|(_, char)| char.is_ascii_whitespace()) {
            length += 1;
        }

        Ok(Token::new(
            Whitespace,
            StartEndSpan::new(start, start + length),
        ))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use crate::{lexing::*, span::StartEndSpan};

    #[test]
    fn space() {
        let source = " ";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_verbose().unwrap()[0].clone();
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn newline() {
        let source = "\n";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_verbose().unwrap()[0].clone();
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn carriage_return() {
        let source = "\r";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_verbose().unwrap()[0].clone();
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn horizontal_tab() {
        let source = "\t";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_verbose().unwrap()[0].clone();
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn mixed_whitespace() {
        let source = " \n \r \t ";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_verbose().unwrap()[0].clone();
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 7));
        assert_eq!(token, expected);
    }
}
