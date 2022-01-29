use anyhow::{anyhow, Error};

/// All tokens of the Mini-PL programming language.
#[derive(Debug, Clone, PartialEq)]
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
    Text,

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
    let _iter = input.chars().peekable();

    tokens.push(Token::EOF);
    Ok(tokens)
}

pub fn scan_token(str: &str) -> Result<Token, Error> {
    let mut iter = str.chars().peekable();
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

        // Ignore whitespace
        ' ' | '\n' | '\r' | '\t' => {
            // TODO: non-token whitespace
            Ok(Whitespace)
        }

        // TODO: better error handling; show source of errors etc
        _ => Err(anyhow!("Unrecognized token {char}")),
    };
}

#[cfg(test)]
mod tests {
    use crate::lexing::*;

    #[test]
    fn single_character_tokens() {
        let token = scan_token("&").unwrap();
        assert_eq!(token, And);

        let token = scan_token(":").unwrap();
        assert_eq!(token, Colon);

        let token = scan_token("=").unwrap();
        assert_eq!(token, Equal);

        let token = scan_token("-").unwrap();
        assert_eq!(token, Minus);

        let token = scan_token("(").unwrap();
        assert_eq!(token, ParenLeft);

        let token = scan_token(")").unwrap();
        assert_eq!(token, ParenRight);

        let token = scan_token("+").unwrap();
        assert_eq!(token, Plus);

        let token = scan_token(";").unwrap();
        assert_eq!(token, Semicolon);

        let token = scan_token("/").unwrap();
        assert_eq!(token, Slash);

        let token = scan_token("*").unwrap();
        assert_eq!(token, Star);
    }

    #[test]
    fn comments() {
        let token = scan_token("//").unwrap();
        assert_eq!(token, Comment);

        let token = scan_token("// I am a comment").unwrap();
        assert_eq!(token, Comment);
    }

    #[test]
    fn whitespace() {
        let token = scan_token(" ").unwrap();
        assert_eq!(token, Whitespace);

        let token = scan_token("\n").unwrap();
        assert_eq!(token, Whitespace);

        let token = scan_token("\r").unwrap();
        assert_eq!(token, Whitespace);

        let token = scan_token("\t").unwrap();
        assert_eq!(token, Whitespace);
    }
}
