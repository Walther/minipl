use crate::span::StartEndSpan;

use super::Error;
use super::Range;
use super::Token;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

/// Internal helper function for scanning a lexeme that starts with a dot. This could be a [Range], or a lexing error.
pub(crate) fn scan_range(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // Consume this token to peek the next
    let (start, _) = iter.next().unwrap();
    // Is this a Range operator?
    if let Some((_, _)) = iter.next_if(|&(_, char)| char == '.') {
        Token::new(Range, StartEndSpan::new(start, start + 2))
    } else {
        // Otherwise, we have a parse error
        Token::new(
            Error("Expected another '.' for Range operator".into()),
            StartEndSpan::new(start, start + 1),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexing::*, span::StartEndSpan};

    #[test]
    fn range_singledot() {
        let token = &scan(".").unwrap()[0];
        let expected = Token::new(
            Error("Expected another '.' for Range operator".into()),
            StartEndSpan::new(0, 1),
        );
        assert_eq!(token, &expected);
    }

    #[test]
    fn range_empty() {
        let token = &scan("..").unwrap()[0];
        let expected = Token::new(Range, StartEndSpan::new(0, 2));
        assert_eq!(token, &expected);
    }

    // TODO: valid range test, 1..10
}
