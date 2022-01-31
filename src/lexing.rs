use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
    string::String,
};

use anyhow::{anyhow, Error};
use derive_more::Display;

/// All raw tokens of the Mini-PL programming language.
#[derive(Debug, Display, Clone, PartialEq)]
pub enum RawToken {
    // Single-character tokens
    And,
    Colon,
    Equal,
    Minus,
    ParenLeft,
    ParenRight,
    Plus,
    Semicolon,
    Slash,
    Star,

    // Literals
    Identifier,
    Number,
    Text(String),

    // Keywords
    Assert,
    Bool,
    Do,
    End,
    For,
    In,
    Int,
    Print,
    Read,
    String,
    Var,

    // Ignorables // TODO: is this necessary?
    Comment,
    Error(String),
    Whitespace,

    // End of file marker
    EOF,
}

use RawToken::*;

/// A richer [Token] type that wraps the [RawToken] type, and holds more metadata.
#[derive(Debug, PartialEq)]
pub struct Token {
    pub token: RawToken,
    pub location: (usize, usize),
}

impl Token {
    pub fn new(token: RawToken, location: (usize, usize)) -> Self {
        Self { token, location }
    }

    pub fn is_error(&self) -> bool {
        matches!(self.token, RawToken::Error(_))
    }
}

/// Main entrypoint of the lexer. Given an input string, parses it into a Vec of [Token]s.
///
/// The Error case of this Result will only occur when an **unrecoverable** runtime error occurs in the parser itself.
/// Any parse errors for the source code will be returned as [Token]s with type [RawToken::Error] in order to recover error locations etc.
/// for use in error reporting for the user.
pub fn parse(input: &str) -> Result<Vec<Token>, Error> {
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

pub fn scan_token(iter: &mut Peekable<Enumerate<Chars>>) -> Result<Token, Error> {
    let (location, char) = match iter.next() {
        Some(it) => it,
        None => return Err(anyhow!("Tried to scan a token with no characters left")),
    };

    let token: Token = match char {
        // Single-character tokens
        '&' => Token::new(And, (location, location + 1)),
        '-' => Token::new(Minus, (location, location + 1)),
        '(' => Token::new(ParenLeft, (location, location + 1)),
        ')' => Token::new(ParenRight, (location, location + 1)),
        '+' => Token::new(Plus, (location, location + 1)),
        ':' => Token::new(Colon, (location, location + 1)),
        ';' => Token::new(Semicolon, (location, location + 1)),
        '/' => {
            // Two-slash comments: skip until end of line
            let mut end = location;
            if let Some((_, next)) = iter.peek() {
                if next == &'/' {
                    while let Some((_, next)) = iter.peek() {
                        if next == &'\n' {
                            let (eol, _) = iter.next().unwrap();
                            end = eol;
                            break;
                        } else {
                            iter.next();
                        }
                    }
                    return Ok(Token::new(Comment, (location, end)));
                }
            }
            // TODO: Handle multiline comments

            // Otherwise, it's just a Slash
            Token::new(Slash, (location, location + 1))
        }
        '*' => Token::new(Star, (location, location + 1)),
        '=' => Token::new(Equal, (location, location + 1)),

        // TODO: handle multi-character tokens, identifiers, etc
        // Multi-character tokens
        '"' => scan_string(iter),

        // Ignore whitespace
        ' ' | '\n' | '\r' | '\t' => {
            // TODO: non-token whitespace
            Token::new(Whitespace, (location, location + 1))
        }

        // TODO: better error handling; show source of errors etc
        _ => Token::new(
            Error(format!("Unknown token {char}")),
            (location, location + 1),
        ),
    };

    Ok(token)
}

/// Internal helper function for scanning a string literal. Returns a [Token] with [RawToken::Text(String)]
fn scan_string(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // TODO: remove unwraps where possible
    // TODO: parse / evaluate escape characters

    let mut contents = String::new();
    let (start, _) = iter.peek().unwrap();
    let start = *start - 1; // String starts with the quote that we consumed in the match before arriving here
    let mut end = start;
    while let Some((_, next)) = iter.peek() {
        // TODO: technically we might need to ban literal multiline strings i.e. error on newline chars unless escaped?
        if next == &'"' {
            let (location, _) = iter.next().unwrap(); // Consume the ending quote
            end = location;
            break;
        } else {
            let (_, c) = iter.next().unwrap();
            contents.push(c);
        }
    }

    Token::new(Text(contents), (start, end))
}

#[cfg(test)]
mod tests {
    use crate::lexing::*;

    #[test]
    fn single_character_tokens() {
        let token = &parse("&").unwrap()[0];
        let expected = Token::new(And, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse(":").unwrap()[0];
        let expected = Token::new(Colon, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse("=").unwrap()[0];
        let expected = Token::new(Equal, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse("-").unwrap()[0];
        let expected = Token::new(Minus, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse("(").unwrap()[0];
        let expected = Token::new(ParenLeft, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse(")").unwrap()[0];
        let expected = Token::new(ParenRight, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse("+").unwrap()[0];
        let expected = Token::new(Plus, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse(";").unwrap()[0];
        let expected = Token::new(Semicolon, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse("/").unwrap()[0];
        let expected = Token::new(Slash, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse("*").unwrap()[0];
        let expected = Token::new(Star, (0, 1));
        assert_eq!(token, &expected);
    }

    #[test]
    fn comments() {
        let token = &parse("//\n").unwrap()[0];
        let expected = Token::new(Comment, (0, 2));
        assert_eq!(token, &expected);

        let token = &parse("// I am a comment\n").unwrap()[0];
        let expected = Token::new(Comment, (0, 17));
        assert_eq!(token, &expected);
    }

    #[test]
    fn whitespace() {
        let token = &parse(" ").unwrap()[0];
        let expected = Token::new(Whitespace, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse("\n").unwrap()[0];
        let expected = Token::new(Whitespace, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse("\r").unwrap()[0];
        let expected = Token::new(Whitespace, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse("\t").unwrap()[0];
        let expected = Token::new(Whitespace, (0, 1));
        assert_eq!(token, &expected);
    }

    #[test]
    fn string() {
        // NOTE: original source code will have the literal quotes
        let token = &parse("\"I am a string of text\"").unwrap()[0];
        let expected = Token::new(Text("I am a string of text".into()), (0, 22));
        assert_eq!(token, &expected);

        // FIXME: should probably be prohibited? Unclear spec...
        let token = &parse("\"multi\nline\"").unwrap()[0];
        let expected = Token::new(Text("multi\nline".into()), (0, 11));
        assert_eq!(token, &expected);
    }
}
