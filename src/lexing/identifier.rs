use crate::span::StartEndSpan;
use crate::tokens::RawToken::{
    Assert, Bool, Do, End, False, For, Identifier, In, Int, Print, Read, String, True, Var,
};
use crate::tokens::Token;

use std::iter::{Enumerate, Peekable};
use std::str::Chars;

/// Internal helper function for scanning identifiers. Greedy / maximal munch, consumes all consecutive ascii-alphabetic chars.
pub(crate) fn scan_identifier(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // Grab the start location from the current, unconsumed char
    let &(start, _) = iter.peek().unwrap();
    let mut length = 0;
    // Consume all alphabetic characters; [maximal munch](https://craftinginterpreters.com/scanning.html)
    let mut identifier = std::string::String::new();
    while let Some((_start, char)) = iter.next_if(|(_, char)| char.is_ascii_alphabetic()) {
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
    Token::new(raw_token, StartEndSpan::new(start, end))
}

#[cfg(test)]
mod tests {
    use crate::{lexing::*, span::StartEndSpan};

    #[test]
    fn keyword_assert() {
        let token = &scan("assert").unwrap()[0];
        let expected = Token::new(Assert, StartEndSpan::new(0, 6));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keyword_bool() {
        let token = &scan("bool").unwrap()[0];
        let expected = Token::new(Bool, StartEndSpan::new(0, 4));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keyword_do() {
        let token = &scan("do").unwrap()[0];
        let expected = Token::new(Do, StartEndSpan::new(0, 2));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keyword_end() {
        let token = &scan("end").unwrap()[0];
        let expected = Token::new(End, StartEndSpan::new(0, 3));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keyword_for() {
        let token = &scan("for").unwrap()[0];
        let expected = Token::new(For, StartEndSpan::new(0, 3));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keyword_in() {
        let token = &scan("in").unwrap()[0];
        let expected = Token::new(In, StartEndSpan::new(0, 2));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keyword_int() {
        let token = &scan("int").unwrap()[0];
        let expected = Token::new(Int, StartEndSpan::new(0, 3));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keyword_print() {
        let token = &scan("print").unwrap()[0];
        let expected = Token::new(Print, StartEndSpan::new(0, 5));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keyword_read() {
        let token = &scan("read").unwrap()[0];
        let expected = Token::new(Read, StartEndSpan::new(0, 4));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keyword_string() {
        let token = &scan("string").unwrap()[0];
        let expected = Token::new(String, StartEndSpan::new(0, 6));
        assert_eq!(token, &expected);
    }

    #[test]
    fn keyword_var() {
        let token = &scan("var").unwrap()[0];
        let expected = Token::new(Var, StartEndSpan::new(0, 3));
        assert_eq!(token, &expected);
    }
}
