use crate::span::StartEndSpan;

use super::Assign;
use super::Colon;
use super::Token;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

/// Internal helper function for scanning a lexeme that starts with a colon. This could be an [Assign], or just a [Colon].
pub(crate) fn scan_colon(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // Consume this token to peek the next
    let (start, _) = iter.next().unwrap();
    // Is this an Assign operator?
    if let Some((_end, _)) = iter.next_if(|&(_, char)| char == '=') {
        Token::new(Assign, StartEndSpan::new(start, start + 2))
    } else {
        // Otherwise, it's just a Colon
        Token::new(Colon, StartEndSpan::new(start, start + 1))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use crate::{lexing::*, span::StartEndSpan};

    #[test]
    fn colon() {
        let token = &scan(":").unwrap()[0];
        let expected = Token::new(Colon, StartEndSpan::new(0, 1));
        assert_eq!(token, &expected);
    }

    #[test]
    fn assign() {
        let token = &scan(":=").unwrap()[0];
        let expected = Token::new(Assign, StartEndSpan::new(0, 2));
        assert_eq!(token, &expected);
    }
}
