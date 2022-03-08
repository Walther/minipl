use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use anyhow::{anyhow, Error};
use tracing::debug;

use crate::tokens::RawToken::*;
use crate::tokens::Token;

mod colon;
mod identifier;
mod number;
mod range;
mod slash;
mod string;
mod whitespace;
use colon::scan_colon;
use identifier::scan_identifier;
use number::scan_number;
use range::scan_range;
use slash::scan_slash;
use string::scan_string;
use whitespace::scan_whitespace;

/// Main entrypoint of the lexer. Given an input string, parses it into a Vec of [Token]s.
///
/// # Errors
/// The Error case of this Result will only occur when an **unrecoverable** runtime error occurs in the parser itself.
/// Any parse errors for the source code will be returned as [Token]s with type [`RawToken::Error`] in order to recover error locations etc.
/// for use in error reporting for the user.
pub fn scan(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let length = input.len();
    let mut iter: Peekable<Enumerate<Chars>> = input.chars().enumerate().peekable();
    while iter.peek().is_some() {
        match scan_token(&mut iter) {
            Ok(token) => tokens.push(token),
            Err(error) => return Err(error),
        }
    }

    tokens.push(Token {
        token: EOF,
        location: (length, length),
    });
    Ok(tokens)
}

/// The main helper function of the lexer, the function that `parse()` calls in a loop.
///
/// # Errors
/// The Error case of this Result will only occur when an **unrecoverable** runtime error occurs in the parser itself.
/// Any parse errors for the source code will be returned as [Token]s with type [`RawToken::Error`] in order to recover error locations etc.
/// for use in error reporting for the user.
pub fn scan_token(iter: &mut Peekable<Enumerate<Chars>>) -> Result<Token, Error> {
    let &(location, char) = match iter.peek() {
        Some(it) => it,
        None => return Err(anyhow!("Tried to scan a token with no characters left")),
    };

    let token: Token = match char {
        // Single-character tokens
        '&' => Token::new(And, (location, location + 1)),
        '!' => Token::new(Bang, (location, location + 1)),
        '<' => Token::new(Less, (location, location + 1)),
        '-' => Token::new(Minus, (location, location + 1)),
        '(' => Token::new(ParenLeft, (location, location + 1)),
        ')' => Token::new(ParenRight, (location, location + 1)),
        '+' => Token::new(Plus, (location, location + 1)),
        ';' => Token::new(Semicolon, (location, location + 1)),
        '*' => Token::new(Star, (location, location + 1)),
        '=' => Token::new(Equal, (location, location + 1)),
        // NOTE: we consume the char for these ^ at the end with a glob match in order to reduce line noise

        // Colon: possibly an Assign, or just a Colon
        ':' => scan_colon(iter),

        // Range
        '.' => scan_range(iter),

        // Slash: possibly a Comment, or just a Slash
        '/' => scan_slash(iter),

        // Number literal
        '0'..='9' => scan_number(iter),

        // Text i.e. String literal
        '"' => scan_string(iter),

        // Whitespace - note https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_whitespace
        ' ' | '\t' | '\n' | '\u{000C}' | '\r' => scan_whitespace(iter),

        // Identifier or keyword
        'a'..='z' | 'A'..='Z' => scan_identifier(iter),

        // Unknown token
        _ => {
            // Consume and report
            iter.next();
            Token::new(
                Error(format!("Unknown token {char}")),
                (location, location + 1),
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
        iter.next();
    }

    debug!("Lexed: {token:?}");
    Ok(token)
}

#[cfg(test)]
mod tests {
    use crate::lexing::*;

    #[test]
    fn single_character_tokens() {
        let token = &scan("&").unwrap()[0];
        let expected = Token::new(And, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan("!").unwrap()[0];
        let expected = Token::new(Bang, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan(":").unwrap()[0];
        let expected = Token::new(Colon, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan("=").unwrap()[0];
        let expected = Token::new(Equal, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan("<").unwrap()[0];
        let expected = Token::new(Less, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan("-").unwrap()[0];
        let expected = Token::new(Minus, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan("(").unwrap()[0];
        let expected = Token::new(ParenLeft, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan(")").unwrap()[0];
        let expected = Token::new(ParenRight, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan("+").unwrap()[0];
        let expected = Token::new(Plus, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan(";").unwrap()[0];
        let expected = Token::new(Semicolon, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan("/").unwrap()[0];
        let expected = Token::new(Slash, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan("*").unwrap()[0];
        let expected = Token::new(Star, (0, 1));
        assert_eq!(token, &expected);
    }

    #[test]
    fn simple_math1() {
        let tokens = &scan("1+2").unwrap();
        let expected = vec![
            Token::new(Number(1), (0, 1)),
            Token::new(Plus, (1, 2)),
            Token::new(Number(2), (2, 3)),
            Token::new(EOF, (3, 3)),
        ];
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn simple_math2() {
        let tokens = &scan("2*2/2=2").unwrap();
        let expected = vec![
            Token::new(Number(2), (0, 1)),
            Token::new(Star, (1, 2)),
            Token::new(Number(2), (2, 3)),
            Token::new(Slash, (3, 4)),
            Token::new(Number(2), (4, 5)),
            Token::new(Equal, (5, 6)),
            Token::new(Number(2), (6, 7)),
            Token::new(EOF, (7, 7)),
        ];
        assert_eq!(tokens, &expected);
    }
}
