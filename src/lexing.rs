use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use miette::Result;
use tracing::debug;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::tokens::Token;
use crate::{span::StartEndSpan, tokens::RawToken::*};

use LexingError::*;

// implementation split into multiple files for convenience
mod colon;
mod identifier;
mod number;
mod range;
mod slash;
mod string;
mod whitespace;

#[derive(Debug)]
/// The lexer for the Mini-PL programming language
pub struct Lexer<'a> {
    _tokens: Vec<Token>, // TODO: use internal field instead of passing the vec around in returns
    source: std::string::String,
    iter: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    /// Initializes a lexer with the given input [`&str`]
    pub fn new(input: &'a str) -> Self {
        let tokens = Vec::new();
        let source = input.to_string();
        Self {
            _tokens: tokens,
            source,
            iter: input.chars().enumerate().peekable(),
        }
    }

    /// Main entrypoint of the lexer. Given an input string, parses it into a Vec of [Token]s.
    ///
    /// # Errors
    /// The Error case of this Result will only occur when an **unrecoverable** runtime error occurs in the parser itself.
    /// Any parse errors for the source code will be returned as [Token]s with type [`RawToken::Error`](crate::tokens::RawToken::Error) in order to recover error locations for use in error reporting for the user.
    pub fn scan(&mut self) -> Result<Vec<Token>> {
        // Use the verbose version
        let mut tokens = self.scan_verbose()?;
        // Then remove ignorables
        tokens.retain(|token| !matches!(token.token, Whitespace | Comment));
        Ok(tokens)
    }

    /// Alternative for [Lexer::scan], but does not delete ignorable tokens
    ///     
    /// # Errors
    /// The Error case of this Result will only occur when an **unrecoverable** runtime error occurs in the parser itself.
    /// Any parse errors for the source code will be returned as [Token]s with type [`RawToken::Error`](crate::tokens::RawToken::Error) in order to recover error locations for use in error reporting for the user.
    pub fn scan_verbose(&mut self) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        let length = self.source.len();
        while self.iter.peek().is_some() {
            match self.scan_token() {
                Ok(token) => tokens.push(token),
                Err(error) => return Err(error.into()),
            }
        }

        tokens.push(Token::new(EOF, StartEndSpan::new(length, length)));
        Ok(tokens)
    }

    /// The main helper function of the lexer, the function that [Lexer::scan] calls in a loop.
    ///
    /// # Errors
    /// The Error case of this Result will only occur when an **unrecoverable** runtime error occurs in the parser itself.
    /// Any parse errors for the source code will be returned as [Token]s with type [`RawToken::Error`](crate::tokens::RawToken::Error) in order to recover error locations for use in error reporting for the user.
    pub fn scan_token(&mut self) -> Result<Token, LexingError> {
        let &(start, char) = self.maybe_peek()?;

        let token: Token = match char {
            // Single-character tokens
            '&' => Token::new(And, StartEndSpan::new(start, start + 1)),
            '!' => Token::new(Bang, StartEndSpan::new(start, start + 1)),
            '<' => Token::new(Less, StartEndSpan::new(start, start + 1)),
            '-' => Token::new(Minus, StartEndSpan::new(start, start + 1)),
            '(' => Token::new(ParenLeft, StartEndSpan::new(start, start + 1)),
            ')' => Token::new(ParenRight, StartEndSpan::new(start, start + 1)),
            '+' => Token::new(Plus, StartEndSpan::new(start, start + 1)),
            ';' => Token::new(Semicolon, StartEndSpan::new(start, start + 1)),
            '*' => Token::new(Star, StartEndSpan::new(start, start + 1)),
            '=' => Token::new(Equal, StartEndSpan::new(start, start + 1)),
            // NOTE: we consume the char for these ^ at the end with a glob match in order to reduce line noise

            // Colon: possibly an Assign, or just a Colon
            ':' => self.scan_colon()?,

            // Range
            '.' => self.scan_range()?,

            // Slash: possibly a Comment, or just a Slash
            '/' => self.scan_slash()?,

            // Number literal
            '0'..='9' => self.scan_number()?,

            // Text i.e. String literal
            '"' => self.scan_string()?,

            // Whitespace - note https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_whitespace
            ' ' | '\t' | '\n' | '\u{000C}' | '\r' => self.scan_whitespace()?,

            // Identifier or keyword
            'a'..='z' | 'A'..='Z' => self.scan_identifier()?,

            // Unknown token
            _ => {
                // Consume and report
                self.iter.next();
                Token::new(
                    Error(format!("Unknown token {char}")),
                    StartEndSpan::new(start, start + 1),
                )
            }
        };

        // If we peeked a single-character token, other than slash, consume it.
        // This is required because the multi-character token parsing helper functions need the iterator with the first char included.
        // Slash is an exception because the comment parsing handling ends up always consuming the first slash.
        if matches!(
            char,
            '&' | '!' | '<' | '-' | '(' | ')' | '+' | ';' | '*' | '='
        ) {
            self.iter.next();
        }

        debug!("Lexed: {token:?}");
        Ok(token)
    }

    /// Internal helper: returns the peeked next token, or an OutOfTokens error
    fn maybe_peek(&mut self) -> Result<&(usize, char), LexingError> {
        if let Some(next) = self.iter.peek() {
            Ok(next)
        } else {
            // TODO: proper error span!
            Err(OutOfChars((0, 0).into()))
        }
    }

    /// Internal helper: consumes and returns the next token, or an OutOfTokens error
    fn maybe_next(&mut self) -> Result<(usize, char), LexingError> {
        if let Some(next) = self.iter.next() {
            Ok(next)
        } else {
            // TODO: proper error span!
            Err(OutOfChars((0, 0).into()))
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Parse error")]
#[diagnostic()]
/// The error enum for the [Lexer]
pub enum LexingError {
    /// Ran out of tokens while scanning
    OutOfChars(SourceSpan),
    /// Unable to parse into an integer
    ParseIntError(#[label = "Could not parse this into a number (i64)"] SourceSpan),
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use crate::lexing::*;

    #[test]
    fn single_character_token_and() {
        let source = "&";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(And, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_bang() {
        let source = "!";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Bang, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_colon() {
        let source = ":";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Colon, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_equal() {
        let source = "=";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Equal, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_less() {
        let source = "<";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Less, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_minus() {
        let source = "-";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Minus, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_parenleft() {
        let source = "(";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(ParenLeft, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_parenright() {
        let source = ")";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(ParenRight, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_plus() {
        let source = "+";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Plus, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_semicolon() {
        let source = ";";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Semicolon, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_slash() {
        let source = "/";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Slash, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn single_character_token_star() {
        let source = "*";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Star, StartEndSpan::new(0, 1));
        assert_eq!(token, expected);
    }

    #[test]
    fn simple_math1() {
        let source = "1+2";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan().unwrap();
        let expected = vec![
            Token::new(Number(1), StartEndSpan::new(0, 1)),
            Token::new(Plus, StartEndSpan::new(1, 2)),
            Token::new(Number(2), StartEndSpan::new(2, 3)),
            Token::new(EOF, StartEndSpan::new(3, 3)),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn simple_math2() {
        let source = "2*2/2=2";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan().unwrap();
        let expected = vec![
            Token::new(Number(2), StartEndSpan::new(0, 1)),
            Token::new(Star, StartEndSpan::new(1, 2)),
            Token::new(Number(2), StartEndSpan::new(2, 3)),
            Token::new(Slash, StartEndSpan::new(3, 4)),
            Token::new(Number(2), StartEndSpan::new(4, 5)),
            Token::new(Equal, StartEndSpan::new(5, 6)),
            Token::new(Number(2), StartEndSpan::new(6, 7)),
            Token::new(EOF, StartEndSpan::new(7, 7)),
        ];
        assert_eq!(tokens, expected);
    }
}
