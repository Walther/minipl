use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
    string::String,
};

use anyhow::{anyhow, Error};

/// All raw tokens of the Mini-PL programming language.
#[derive(Debug, Clone, PartialEq)]
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

    // Multi-character tokens
    Assign,
    Range,

    // Literals
    Identifier(String),
    Number(i64),
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

    // Ignorables
    Comment,
    Error(String),
    Whitespace,

    // End of file marker
    EOF,
}

use tracing::debug;
use RawToken::*;

/// A richer [Token] type that wraps the [RawToken] type, and holds more metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token: RawToken,
    pub location: (usize, usize),
}

impl Token {
    pub fn new(token: RawToken, location: (usize, usize)) -> Self {
        Self { token, location }
    }

    pub fn is_error(&self) -> bool {
        matches!(self.token, Error(_))
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
    let &(location, char) = match iter.peek() {
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
    if matches!(char, '&' | '-' | '(' | ')' | '+' | ';' | '*' | '=') {
        iter.next();
    }

    debug!("Lexed: {token:?}");
    Ok(token)
}

/// Internal helper function for scanning a lexeme that starts with a colon. This could be an [Assign], or just a [Colon].
fn scan_colon(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // Consume this token to peek the next
    let (location, _) = iter.next().unwrap();
    // Is this an Assign operator?
    if let Some((end, _)) = iter.next_if(|&(_, char)| char == '=') {
        Token::new(Assign, (location, end + 1))
    } else {
        // Otherwise, it's just a Colon
        Token::new(Colon, (location, location + 1))
    }
}

/// Internal helper function for scanning a lexeme that starts with a dot. This could be a [Range], or a lexing error.
fn scan_range(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // Consume this token to peek the next
    let (location, _) = iter.next().unwrap();
    // Is this a Range operator?
    if let Some((end, _)) = iter.next_if(|&(_, char)| char == '.') {
        Token::new(Range, (location, end + 1))
    } else {
        // Otherwise, we have a parse error
        Token::new(
            Error("Expected another '.' for Range operator".into()),
            (location, location + 2),
        )
    }
}

/// Internal helper function for scanning a lexeme that starts with a slash. This could be a [Comment], or just a [Slash].
fn scan_slash(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // TODO: remove unwraps
    // Consume the first slash & grab the location
    let (location, _) = iter.next().unwrap();
    let mut end = location;
    // Do we have a second slash?
    if let Some((_, _)) = iter.next_if(|&(_, char)| char == '/') {
        // Second slash found, consume until end of line
        for (location, next) in iter {
            if next == '\n' {
                end = location;
                break;
            }
        }
        return Token::new(Comment, (location, end));
    }
    // TODO: multi-line comments with nesting support

    // Not a comment, just a slash
    Token::new(Slash, (location, location + 1))
}

/// Internal helper function for scanning a number literal.
fn scan_number(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // TODO: remove unwraps where possible

    let mut number = String::new();
    let &(start, _) = iter.peek().unwrap();
    let mut end = start;

    while let Some((location, char)) = iter.next_if(|(_, char)| char.is_ascii_digit()) {
        end = location + 1;
        number.push(char);
    }

    let number: i64 = number.parse().unwrap();

    Token::new(Number(number), (start, end))
}

/// Internal helper function for scanning a string literal. Returns a [Token] with [RawToken::Text(String)]
fn scan_string(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // TODO: remove unwraps where possible
    // TODO: parse / evaluate escape characters
    // TODO: technically we might need to ban literal multiline strings i.e. error on newline chars unless escaped?

    // Consume the first quote
    let (start, _) = iter.next().unwrap();

    // Consume and collect all characters within the string
    let mut contents = String::new();
    while let Some((_, char)) = iter.next_if(|&(_, char)| char != '"') {
        contents.push(char);
    }
    // Consume the ending quote too
    let (location, _) = iter.next().unwrap();
    let end = location + 1;

    Token::new(Text(contents), (start, end))
}

/// Internal helper function for scanning and skipping over whitespace. Greedy / maximal munch.
fn scan_whitespace(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    let &(start, _) = iter.peek().unwrap();
    let mut end = start;

    while let Some((location, _)) = iter.next_if(|(_, char)| char.is_ascii_whitespace()) {
        end = location + 1;
    }

    Token::new(Whitespace, (start, end))
}

fn scan_identifier(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // Grab the start location from the current, unconsumed char
    let &(location, _) = iter.peek().unwrap();
    let mut end = location;
    // Consume all alphabetic characters; [maximal munch](https://craftinginterpreters.com/scanning.html)
    let mut identifier = String::new();
    while let Some((location, char)) = iter.next_if(|(_, char)| char.is_ascii_alphabetic()) {
        identifier.push(char);
        end = location + 1;
    }

    match identifier.as_ref() {
        // Is this a keyword?
        "assert" => Token::new(Assert, (location, end)),
        "bool" => Token::new(Bool, (location, end)),
        "do" => Token::new(Do, (location, end)),
        "end" => Token::new(End, (location, end)),
        "for" => Token::new(For, (location, end)),
        "in" => Token::new(In, (location, end)),
        "int" => Token::new(Int, (location, end)),
        "print" => Token::new(Print, (location, end)),
        "read" => Token::new(Read, (location, end)),
        "string" => Token::new(String, (location, end)),
        "var" => Token::new(Var, (location, end)),
        // Otherwise, assume it's a user-defined identifier name
        _ => Token::new(Identifier(identifier), (location, end)),
    }
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

        let token = &parse(" \n \n ").unwrap()[0];
        let expected = Token::new(Whitespace, (0, 5));
        assert_eq!(token, &expected);
    }

    #[test]
    fn number() {
        let token = &parse("1").unwrap()[0];
        let expected = Token::new(Number(1), (0, 1));
        assert_eq!(token, &expected);

        let token = &parse("1234567890").unwrap()[0];
        let expected = Token::new(Number(1234567890), (0, 10));
        assert_eq!(token, &expected);
    }

    #[test]
    fn colon() {
        let token = &parse(":").unwrap()[0];
        let expected = Token::new(Colon, (0, 1));
        assert_eq!(token, &expected);

        let token = &parse(":=").unwrap()[0];
        let expected = Token::new(Assign, (0, 2));
        assert_eq!(token, &expected);
    }

    #[test]
    fn range() {
        let token = &parse(".").unwrap()[0];
        let expected = Token::new(
            Error("Expected another '.' for Range operator".into()),
            (0, 2),
        );
        assert_eq!(token, &expected);

        let token = &parse("..").unwrap()[0];
        let expected = Token::new(Range, (0, 2));
        assert_eq!(token, &expected);
    }

    #[test]
    fn string() {
        // NOTE: original source code will have the literal quotes
        let token = &parse("\"\"").unwrap()[0];
        let expected = Token::new(Text("".into()), (0, 2));
        assert_eq!(token, &expected);

        // FIXME: should probably be prohibited? Unclear spec...
        let token = &parse("\"multi\nline\"").unwrap()[0];
        let expected = Token::new(Text("multi\nline".into()), (0, 12));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keywords() {
        let token = &parse("assert").unwrap()[0];
        let expected = Token::new(Assert, (0, 6));
        assert_eq!(token, &expected);

        let token = &parse("bool").unwrap()[0];
        let expected = Token::new(Bool, (0, 4));
        assert_eq!(token, &expected);

        let token = &parse("do").unwrap()[0];
        let expected = Token::new(Do, (0, 2));
        assert_eq!(token, &expected);

        let token = &parse("end").unwrap()[0];
        let expected = Token::new(End, (0, 3));
        assert_eq!(token, &expected);

        let token = &parse("for").unwrap()[0];
        let expected = Token::new(For, (0, 3));
        assert_eq!(token, &expected);

        let token = &parse("in").unwrap()[0];
        let expected = Token::new(In, (0, 2));
        assert_eq!(token, &expected);

        let token = &parse("int").unwrap()[0];
        let expected = Token::new(Int, (0, 3));
        assert_eq!(token, &expected);

        let token = &parse("print").unwrap()[0];
        let expected = Token::new(Print, (0, 5));
        assert_eq!(token, &expected);

        let token = &parse("read").unwrap()[0];
        let expected = Token::new(Read, (0, 4));
        assert_eq!(token, &expected);

        let token = &parse("string").unwrap()[0];
        let expected = Token::new(String, (0, 6));
        assert_eq!(token, &expected);

        let token = &parse("var").unwrap()[0];
        let expected = Token::new(Var, (0, 3));
        assert_eq!(token, &expected);
    }

    #[test]
    fn simple_math1() {
        let tokens = &parse("1+2").unwrap();
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
        let tokens = &parse("2*2/2=2").unwrap();
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
