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

    match identifier.as_ref() {
        // Is this a keyword?
        // TODO: const hashmap and a simple get method?
        "assert" => Token::new(Assert, StartEndSpan::new(start, end)),
        "bool" => Token::new(Bool, StartEndSpan::new(start, end)),
        "do" => Token::new(Do, StartEndSpan::new(start, end)),
        "end" => Token::new(End, StartEndSpan::new(start, end)),
        "false" => Token::new(False, StartEndSpan::new(start, end)),
        "for" => Token::new(For, StartEndSpan::new(start, end)),
        "in" => Token::new(In, StartEndSpan::new(start, end)),
        "int" => Token::new(Int, StartEndSpan::new(start, end)),
        "print" => Token::new(Print, StartEndSpan::new(start, end)),
        "read" => Token::new(Read, StartEndSpan::new(start, end)),
        "string" => Token::new(String, StartEndSpan::new(start, end)),
        "true" => Token::new(True, StartEndSpan::new(start, end)),
        "var" => Token::new(Var, StartEndSpan::new(start, end)),
        // Otherwise, assume it's a user-defined identifier name
        _ => Token::new(Identifier(identifier), StartEndSpan::new(start, end)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexing::*, span::StartEndSpan};

    #[test]
    fn keywords() {
        let token = &scan("assert").unwrap()[0];
        let expected = Token::new(Assert, StartEndSpan::new(0, 6));
        assert_eq!(token, &expected);

        let token = &scan("bool").unwrap()[0];
        let expected = Token::new(Bool, StartEndSpan::new(0, 4));
        assert_eq!(token, &expected);

        let token = &scan("do").unwrap()[0];
        let expected = Token::new(Do, StartEndSpan::new(0, 2));
        assert_eq!(token, &expected);

        let token = &scan("end").unwrap()[0];
        let expected = Token::new(End, StartEndSpan::new(0, 3));
        assert_eq!(token, &expected);

        let token = &scan("for").unwrap()[0];
        let expected = Token::new(For, StartEndSpan::new(0, 3));
        assert_eq!(token, &expected);

        let token = &scan("in").unwrap()[0];
        let expected = Token::new(In, StartEndSpan::new(0, 2));
        assert_eq!(token, &expected);

        let token = &scan("int").unwrap()[0];
        let expected = Token::new(Int, StartEndSpan::new(0, 3));
        assert_eq!(token, &expected);

        let token = &scan("print").unwrap()[0];
        let expected = Token::new(Print, StartEndSpan::new(0, 5));
        assert_eq!(token, &expected);

        let token = &scan("read").unwrap()[0];
        let expected = Token::new(Read, StartEndSpan::new(0, 4));
        assert_eq!(token, &expected);

        let token = &scan("string").unwrap()[0];
        let expected = Token::new(String, StartEndSpan::new(0, 6));
        assert_eq!(token, &expected);

        let token = &scan("var").unwrap()[0];
        let expected = Token::new(Var, StartEndSpan::new(0, 3));
        assert_eq!(token, &expected);
    }
}
