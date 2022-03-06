use crate::rawtoken::RawToken::{
    Assert, Bool, Do, End, False, For, Identifier, In, Int, Print, Read, String, True, Var,
};
use crate::token::Token;

use std::iter::{Enumerate, Peekable};
use std::str::Chars;

/// Internal helper function for scanning identifiers. Greedy / maximal munch, consumes all consecutive ascii-alphabetic chars.
pub(crate) fn scan_identifier(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // Grab the start location from the current, unconsumed char
    let &(location, _) = iter.peek().unwrap();
    let mut end = location;
    // Consume all alphabetic characters; [maximal munch](https://craftinginterpreters.com/scanning.html)
    let mut identifier = std::string::String::new();
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
        "false" => Token::new(False, (location, end)),
        "for" => Token::new(For, (location, end)),
        "in" => Token::new(In, (location, end)),
        "int" => Token::new(Int, (location, end)),
        "print" => Token::new(Print, (location, end)),
        "read" => Token::new(Read, (location, end)),
        "string" => Token::new(String, (location, end)),
        "true" => Token::new(True, (location, end)),
        "var" => Token::new(Var, (location, end)),
        // Otherwise, assume it's a user-defined identifier name
        _ => Token::new(Identifier(identifier), (location, end)),
    }
}

#[cfg(test)]
mod tests {
    use crate::lexing::*;

    #[test]
    fn keywords() {
        let token = &scan("assert").unwrap()[0];
        let expected = Token::new(Assert, (0, 6));
        assert_eq!(token, &expected);

        let token = &scan("bool").unwrap()[0];
        let expected = Token::new(Bool, (0, 4));
        assert_eq!(token, &expected);

        let token = &scan("do").unwrap()[0];
        let expected = Token::new(Do, (0, 2));
        assert_eq!(token, &expected);

        let token = &scan("end").unwrap()[0];
        let expected = Token::new(End, (0, 3));
        assert_eq!(token, &expected);

        let token = &scan("for").unwrap()[0];
        let expected = Token::new(For, (0, 3));
        assert_eq!(token, &expected);

        let token = &scan("in").unwrap()[0];
        let expected = Token::new(In, (0, 2));
        assert_eq!(token, &expected);

        let token = &scan("int").unwrap()[0];
        let expected = Token::new(Int, (0, 3));
        assert_eq!(token, &expected);

        let token = &scan("print").unwrap()[0];
        let expected = Token::new(Print, (0, 5));
        assert_eq!(token, &expected);

        let token = &scan("read").unwrap()[0];
        let expected = Token::new(Read, (0, 4));
        assert_eq!(token, &expected);

        let token = &scan("string").unwrap()[0];
        let expected = Token::new(String, (0, 6));
        assert_eq!(token, &expected);

        let token = &scan("var").unwrap()[0];
        let expected = Token::new(Var, (0, 3));
        assert_eq!(token, &expected);
    }
}
