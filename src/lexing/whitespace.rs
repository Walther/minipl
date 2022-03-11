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
    fn whitespace() {
        let token = &scan(" ").unwrap()[0];
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("\n").unwrap()[0];
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("\r").unwrap()[0];
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("\t").unwrap()[0];
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan(" \n \n ").unwrap()[0];
        let expected = Token::new(Whitespace, StartEndSpan::new(0, 5));
        assert_eq!(token, &expected);
    }
}
