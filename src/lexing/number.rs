use crate::span::StartEndSpan;

use super::Number;
use super::Token;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;
use std::string::String;

/// Internal helper function for scanning a number literal.
pub(crate) fn scan_number(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // TODO: remove unwraps where possible

    let mut number = String::new();
    let &(start, _) = iter.peek().unwrap();
    let mut length = 0;

    while let Some((_, char)) = iter.next_if(|(_, char)| char.is_ascii_digit()) {
        length += 1;
        number.push(char);
    }

    let number: i64 = number.parse().unwrap();
    let end = start + length;
    Token::new(Number(number), StartEndSpan::new(start, end))
}

#[cfg(test)]
mod tests {
    use crate::{lexing::*, span::StartEndSpan};
    #[test]
    fn number() {
        let token = &scan("1").unwrap()[0];
        let expected = Token::new(Number(1), StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);

        let token = &scan("1234567890").unwrap()[0];
        let expected = Token::new(Number(1234567890), StartEndSpan::new(0, 10));
        assert_eq!(token, &expected);
    }
}
