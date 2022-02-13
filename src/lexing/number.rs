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
    let mut end = start;

    while let Some((location, char)) = iter.next_if(|(_, char)| char.is_ascii_digit()) {
        end = location + 1;
        number.push(char);
    }

    let number: i64 = number.parse().unwrap();

    Token::new(Number(number), (start, end))
}

#[cfg(test)]
mod tests {
    use crate::lexing::*;
    #[test]
    fn number() {
        let token = &scan("1").unwrap()[0];
        let expected = Token::new(Number(1), (0, 1));
        assert_eq!(token, &expected);

        let token = &scan("1234567890").unwrap()[0];
        let expected = Token::new(Number(1234567890), (0, 10));
        assert_eq!(token, &expected);
    }
}
