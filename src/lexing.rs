use std::{iter::Peekable, str::Chars, string::String};

use anyhow::{anyhow, Error};
use derive_more::Display;

/// All tokens of the Mini-PL programming language.
#[derive(Debug, Display, Clone, PartialEq)]
pub enum Token {
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
    Whitespace,

    // End of file marker
    EOF,
}

use Token::*;

pub fn parse(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter: Peekable<Chars> = input.chars().peekable();
    while iter.peek().is_some() {
        match scan_token(&mut iter) {
            Ok(token) => tokens.push(token),
            Err(error) => return Err(error),
        }
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}

pub fn scan_token(iter: &mut Peekable<Chars>) -> Result<Token, Error> {
    let char = match iter.next() {
        Some(it) => it,
        None => return Err(anyhow!("Tried to scan a token with no characters left")),
    };

    return match char {
        // Single-character tokens
        '&' => Ok(And),
        '-' => Ok(Minus),
        '(' => Ok(ParenLeft),
        ')' => Ok(ParenRight),
        '+' => Ok(Plus),
        ':' => Ok(Colon),
        ';' => Ok(Semicolon),
        '/' => {
            // Two-slash comments: skip until end of line
            if let Some(next) = iter.peek() {
                if next == &'/' {
                    while let Some(next) = iter.peek() {
                        if next == &'\n' {
                            break;
                        } else {
                            iter.next();
                        }
                    }
                    return Ok(Comment);
                }
            }
            // TODO: Handle multiline comments

            // Otherwise, it's just a Slash
            return Ok(Slash);
        }
        '*' => Ok(Star),
        '=' => Ok(Equal),

        // TODO: handle multi-character tokens, identifiers, etc
        // Multi-character tokens
        '"' => scan_string(iter),

        // Ignore whitespace
        ' ' | '\n' | '\r' | '\t' => {
            // TODO: non-token whitespace
            Ok(Whitespace)
        }

        // TODO: better error handling; show source of errors etc
        _ => Err(anyhow!("Unrecognized token {char}")),
    };
}

fn scan_string(iter: &mut Peekable<Chars>) -> Result<Token, Error> {
    let mut contents = String::new();
    while let Some(next) = iter.peek() {
        // TODO: technically we might need to ban literal multiline strings i.e. error on newline chars unless escaped?
        if next == &'"' {
            iter.next(); // Consume the ending quote
            break;
        } else {
            let c = iter.next().unwrap(); // TODO: remove unwrap
            contents.push(c);
        }
    }

    Ok(Text(contents))
}

#[cfg(test)]
mod tests {
    use crate::lexing::*;

    #[test]
    fn single_character_tokens() {
        let token = &parse("&").unwrap()[0];
        assert_eq!(token, &And);

        let token = &parse(":").unwrap()[0];
        assert_eq!(token, &Colon);

        let token = &parse("=").unwrap()[0];
        assert_eq!(token, &Equal);

        let token = &parse("-").unwrap()[0];
        assert_eq!(token, &Minus);

        let token = &parse("(").unwrap()[0];
        assert_eq!(token, &ParenLeft);

        let token = &parse(")").unwrap()[0];
        assert_eq!(token, &ParenRight);

        let token = &parse("+").unwrap()[0];
        assert_eq!(token, &Plus);

        let token = &parse(";").unwrap()[0];
        assert_eq!(token, &Semicolon);

        let token = &parse("/").unwrap()[0];
        assert_eq!(token, &Slash);

        let token = &parse("*").unwrap()[0];
        assert_eq!(token, &Star);
    }

    #[test]
    fn comments() {
        let token = &parse("//").unwrap()[0];
        assert_eq!(token, &Comment);

        let token = &parse("// I am a comment").unwrap()[0];
        assert_eq!(token, &Comment);
    }

    #[test]
    fn whitespace() {
        let token = &parse(" ").unwrap()[0];
        assert_eq!(token, &Whitespace);

        let token = &parse("\n").unwrap()[0];
        assert_eq!(token, &Whitespace);

        let token = &parse("\r").unwrap()[0];
        assert_eq!(token, &Whitespace);

        let token = &parse("\t").unwrap()[0];
        assert_eq!(token, &Whitespace);
    }

    #[test]
    fn string() {
        // NOTE: original source code will have the literal quotes
        let token = &parse("\"I am a string of text\"").unwrap()[0];
        let expected = Text("I am a string of text".into());
        assert_eq!(token, &expected);

        // FIXME: should probably be prohibited? Unclear spec...
        let token = &parse("\"multi\nline\"").unwrap()[0];
        let expected = Text("multi\nline".into());
        assert_eq!(token, &expected);
    }

    #[test]
    fn longer_code() {
        let tokens = &parse("// This is a comment").unwrap();
        assert_eq!(tokens, &vec![Comment, EOF]);

        let tokens = &parse("(( ))").unwrap();
        assert_eq!(
            tokens,
            &vec![ParenLeft, ParenLeft, Whitespace, ParenRight, ParenRight, EOF]
        );
    }
}
