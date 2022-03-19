use crate::span::StartEndSpan;

use super::Lexer;
use super::LexingError;
use super::Number;
use super::Token;

use std::string::String;

impl Lexer<'_> {
    /// Internal helper function for scanning a number literal.
    pub(crate) fn scan_number(&mut self) -> Result<Token, LexingError> {
        let mut number = String::new();
        let &(start, _) = self.maybe_peek()?;
        let mut length = 0;

        while let Some((_, char)) = self.iter.next_if(|(_, char)| char.is_ascii_digit()) {
            length += 1;
            number.push(char);
        }

        let number: i64 = match number.parse() {
            Ok(n) => n,
            Err(_) => return Err(LexingError::ParseIntError((start, length).into())),
        };
        let end = start + length;
        Ok(Token::new(Number(number), StartEndSpan::new(start, end)))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use crate::{lexing::*, span::StartEndSpan};

    #[test]
    fn number_singledigit() {
        let source = "1";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Number(1), StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn number_multidigit() {
        let source = "1234567890";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Number(1_234_567_890), StartEndSpan::new(0, 10));
        assert_eq!(token, expected);
    }
}
