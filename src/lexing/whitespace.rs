use crate::span::StartEndSpan;

use super::Token;
use super::Whitespace;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

/// Internal helper function for scanning and skipping over whitespace. Greedy / maximal munch.
pub(crate) fn scan_whitespace(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    let &(start, _) = iter.peek().unwrap();
    let mut length = 0;

    while let Some((_, _)) = iter.next_if(|(_, char)| char.is_ascii_whitespace()) {
        length += 1;
    }

    Token::new(Whitespace, StartEndSpan::new(start, start + length))
}

#[cfg(test)]
mod tests {
    use crate::{lexing::*, span::StartEndSpan};

    #[test]
    fn space() {
        let token = &scan_verbose(" ").unwrap()[0];
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);
    }

    #[test]
    fn newline() {
        let token = &scan_verbose("\n").unwrap()[0];
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);
    }

    #[test]
    fn carriage_return() {
        let token = &scan_verbose("\r").unwrap()[0];
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);
    }

    #[test]
    fn horizontal_tab() {
        let token = &scan_verbose("\t").unwrap()[0];
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);
    }

    #[test]
    fn mixed_whitespace() {
        let token = &scan_verbose(" \n \r \t ").unwrap()[0];
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 7));
        assert_eq!(token, &expected);
    }
}
