use super::Error;
use super::Range;
use super::Token;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

/// Internal helper function for scanning a lexeme that starts with a dot. This could be a [Range], or a lexing error.
pub(crate) fn scan_range(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // Consume this token to peek the next
    let (location, _) = iter.next().unwrap();
    // Is this a Range operator?
    if let Some((end, _)) = iter.next_if(|&(_, char)| char == '.') {
        Token::new(Range, (location, end + 1))
    } else {
        // Otherwise, we have a parse error
        Token::new(
            Error("Expected another '.' for Range operator".into()),
            (location, location + 2),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::lexing::*;
    #[test]
    fn range() {
        let token = &scan(".").unwrap()[0];
        let expected = Token::new(
            Error("Expected another '.' for Range operator".into()),
            (0, 2),
        );
        assert_eq!(token, &expected);

        let token = &scan("..").unwrap()[0];
        let expected = Token::new(Range, (0, 2));
        assert_eq!(token, &expected);
    }
}