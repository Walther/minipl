use super::Comment;
use super::Slash;
use super::Token;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

/// Internal helper function for scanning a lexeme that starts with a slash. This could be a [Comment], or just a [Slash].
pub(crate) fn scan_slash(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // TODO: remove unwraps
    // Consume the first slash & grab the start location
    let (start, _) = iter.next().unwrap();
    let mut length = 1;
    // Do we have a second slash?
    if let Some((_, _)) = iter.next_if(|&(_, char)| char == '/') {
        length += 1;
        // Second slash found, consume until end of line
        for (_, next) in iter {
            if next == '\n' {
                break;
            } else {
                length += 1;
            }
        }
        return Token::new(Comment, (start, length));
    }
    // TODO: multi-line comments with nesting support

    // Not a comment, just a slash
    Token::new(Slash, (start, 1))
}

#[cfg(test)]
mod tests {
    use crate::lexing::*;
    #[test]
    fn comments() {
        let token = &scan("//\n").unwrap()[0];
        let expected = Token::new(Comment, (0, 2));
        assert_eq!(token, &expected);

        let token = &scan("// I am a comment\n").unwrap()[0];
        let expected = Token::new(Comment, (0, 17));
        assert_eq!(token, &expected);
    }
}
