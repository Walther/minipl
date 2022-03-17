#![allow(clippy::unwrap_used)] // TODO: remove clippy allow

use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use miette::{miette, Result};
use tracing::debug;

use crate::tokens::Token;
use crate::{span::StartEndSpan, tokens::RawToken::*};

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
/// Any parse errors for the source code will be returned as [Token]s with type [`crate::tokens::RawToken::Error`] in order to recover error locations for use in error reporting for the user.
pub fn scan(input: &str) -> Result<Vec<Token>> {
    // Use the verbose version
    let mut tokens = scan_verbose(input)?;
    // Then remove ignorables
    tokens.retain(|token| !matches!(token.token, Whitespace | Comment));
    Ok(tokens)
}

/// Alternative for [scan], but does not delete ignorable tokens
pub fn scan_verbose(input: &str) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let length = input.len();
    let mut iter: Peekable<Enumerate<Chars>> = input.chars().enumerate().peekable();
    while iter.peek().is_some() {
        match scan_token(&mut iter) {
            Ok(token) => tokens.push(token),
            Err(error) => return Err(error),
        }
    }

    tokens.push(Token::new(EOF, StartEndSpan::new(length, length)));
    Ok(tokens)
}

/// The main helper function of the lexer, the function that `parse()` calls in a loop.
///
/// # Errors
/// The Error case of this Result will only occur when an **unrecoverable** runtime error occurs in the parser itself.
/// Any parse errors for the source code will be returned as [Token]s with type [`crate::tokens::RawToken::Error`] in order to recover error locations for use in error reporting for the user.
pub fn scan_token(iter: &mut Peekable<Enumerate<Chars>>) -> Result<Token> {
    let &(start, char) = match iter.peek() {
        Some(it) => it,
        None => return Err(miette!("Tried to scan a token with no characters left")),
    };

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
        let expected = Token::new(And, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("!").unwrap()[0];
        let expected = Token::new(Bang, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan(":").unwrap()[0];
        let expected = Token::new(Colon, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("=").unwrap()[0];
        let expected = Token::new(Equal, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("<").unwrap()[0];
        let expected = Token::new(Less, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("-").unwrap()[0];
        let expected = Token::new(Minus, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("(").unwrap()[0];
        let expected = Token::new(ParenLeft, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan(")").unwrap()[0];
        let expected = Token::new(ParenRight, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("+").unwrap()[0];
        let expected = Token::new(Plus, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan(";").unwrap()[0];
        let expected = Token::new(Semicolon, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("/").unwrap()[0];
        let expected = Token::new(Slash, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("*").unwrap()[0];
        let expected = Token::new(Star, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);
    }

    #[test]
    fn simple_math1() {
        let tokens = &scan("1+2").unwrap();
        let expected = vec![
            Token::new(Number(1), StartEndSpan::new(0, 1)),
            Token::new(Plus, StartEndSpan::new(1, 2)),
            Token::new(Number(2), StartEndSpan::new(2, 3)),
            Token::new(EOF, StartEndSpan::new(3, 3)),
        ];
        assert_eq!(tokens, &expected);
    }

    #[test]
    fn simple_math2() {
        let tokens = &scan("2*2/2=2").unwrap();
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
        assert_eq!(tokens, &expected);
    }
}
