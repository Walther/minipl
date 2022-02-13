use super::Assign;
use super::Colon;
use super::Token;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

/// Internal helper function for scanning a lexeme that starts with a colon. This could be an [Assign], or just a [Colon].
pub(crate) fn scan_colon(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
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

#[cfg(test)]
mod tests {
    use crate::lexing::*;
    #[test]
    fn colon() {
        let token = &scan(":").unwrap()[0];
        let expected = Token::new(Colon, (0, 1));
        assert_eq!(token, &expected);

        let token = &scan(":=").unwrap()[0];
        let expected = Token::new(Assign, (0, 2));
        assert_eq!(token, &expected);
    }
}
