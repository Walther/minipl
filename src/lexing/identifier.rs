use crate::span::StartEndSpan;
use crate::tokens::RawToken::{
    Assert, Bool, Do, End, False, For, Identifier, In, Int, Print, Read, String, True, Var,
};
use crate::tokens::Token;

use super::{Lexer, LexingError};

impl Lexer<'_> {
    /// Internal helper function for scanning identifiers. Greedy / maximal munch, consumes all consecutive ascii-alphabetic chars.
    pub(crate) fn scan_identifier(&mut self) -> Result<Token, LexingError> {
        // Grab the start location from the current, unconsumed char
        let &(start, _) = self.maybe_peek()?;
        let mut length = 0;
        // Consume all alphabetic characters; [maximal munch](https://craftinginterpreters.com/scanning.html)
        let mut identifier = std::string::String::new();
        while let Some((_start, char)) = self.iter.next_if(|(_, char)| char.is_ascii_alphabetic()) {
            identifier.push(char);
            length += 1;
        }
        let end = start + length;

        let raw_token = match identifier.as_ref() {
            // Is this a keyword?
            "assert" => Assert,
            "bool" => Bool,
            "do" => Do,
            "end" => End,
            "false" => False,
            "for" => For,
            "in" => In,
            "int" => Int,
            "print" => Print,
            "read" => Read,
            "string" => String,
            "true" => True,
            "var" => Var,
            // Otherwise, assume it's a user-defined identifier name
            _ => Identifier(identifier),
        };
        Ok(Token::new(raw_token, StartEndSpan::new(start, end)))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use crate::{lexing::*, span::StartEndSpan};

    #[test]
    fn keyword_assert() {
        let source = "assert";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Assert, StartEndSpan::new(0, 6));
        assert_eq!(token, expected);
    }

    #[test]
    fn keyword_bool() {
        let source = "bool";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Bool, StartEndSpan::new(0, 4));
        assert_eq!(token, expected);
    }

    #[test]
    fn keyword_do() {
        let source = "do";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Do, StartEndSpan::new(0, 2));
        assert_eq!(token, expected);
    }

    #[test]
    fn keyword_end() {
        let source = "end";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(End, StartEndSpan::new(0, 3));
        assert_eq!(token, expected);
    }

    #[test]
    fn keyword_for() {
        let source = "for";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(For, StartEndSpan::new(0, 3));
        assert_eq!(token, expected);
    }

    #[test]
    fn keyword_in() {
        let source = "in";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(In, StartEndSpan::new(0, 2));
        assert_eq!(token, expected);
    }

    #[test]
    fn keyword_int() {
        let source = "int";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Int, StartEndSpan::new(0, 3));
        assert_eq!(token, expected);
    }

    #[test]
    fn keyword_print() {
        let source = "print";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Print, StartEndSpan::new(0, 5));
        assert_eq!(token, expected);
    }

    #[test]
    fn keyword_read() {
        let source = "read";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Read, StartEndSpan::new(0, 4));
        assert_eq!(token, expected);
    }

    #[test]
    fn keyword_string() {
        let source = "string";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(String, StartEndSpan::new(0, 6));
        assert_eq!(token, expected);
    }

    #[test]
    fn keyword_var() {
        let source = "var";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Var, StartEndSpan::new(0, 3));
        assert_eq!(token, expected);
    }
}
