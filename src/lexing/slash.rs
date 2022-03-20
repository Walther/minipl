use crate::span::StartEndSpan;

use super::Comment;
use super::Lexer;
use super::Slash;
use super::Token;
use super::UnrecoverableLexingError;

impl Lexer<'_> {
    /// Internal helper function for scanning a lexeme that starts with a slash. This could be a [Comment], or just a [Slash].
    pub(crate) fn scan_slash(&mut self) -> Result<Token, UnrecoverableLexingError> {
        // Consume the first slash & grab the start location
        let (start, _) = self.maybe_next()?;
        let mut length = 1;
        // Do we have a second slash?
        if let Some((_, _)) = self.iter.next_if(|&(_, char)| char == '/') {
            length += 1;
            // Second slash found, consume until end of line
            for (_, next) in &mut self.iter {
                if next == '\n' {
                    break;
                }
                length += 1;
            }
            return Ok(Token::new(
                Comment,
                StartEndSpan::new(start, start + length),
            ));
        }
        // Do we have a multi-line comment?
        if let Some((_, _)) = self.iter.next_if(|&(_, char)| char == '*') {
            length += 1;
            let mut multiline_comment_level = 1;
            while multiline_comment_level > 0 {
                let (_, next) = self.maybe_next()?;
                length += 1;
                // Do we exit one level of multiline comment nesting?
                if next == '*' {
                    let (_, next) = self.maybe_next()?;
                    length += 1;
                    if next == '/' {
                        multiline_comment_level -= 1;
                    }
                }
                // Do we enter another level of multiline comment nesting?
                if next == '/' {
                    let (_, next) = self.maybe_next()?;
                    length += 1;
                    if next == '*' {
                        multiline_comment_level += 1;
                    }
                }
            }
            return Ok(Token::new(
                Comment,
                StartEndSpan::new(start, start + length),
            ));
        }

        // Not a comment, just a slash
        Ok(Token::new(Slash, StartEndSpan::new(start, start + 1)))
    }
}
#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use crate::{lexing::*, span::StartEndSpan};
    #[test]
    fn comment_empty() {
        let source = "//\n";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_verbose().unwrap()[0].clone();
        let expected = Token::new(Comment, StartEndSpan::new(0, 2));
        assert_eq!(token, expected);
    }

    #[test]
    fn comment_singleline() {
        let source = "// I am a comment\n";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_verbose().unwrap()[0].clone();
        let expected = Token::new(Comment, StartEndSpan::new(0, 17));
        assert_eq!(token, expected);
    }

    #[test]
    fn comment_multiline_simple() {
        let source = "/* I am a comment \n */";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_verbose().unwrap()[0].clone();
        let expected = Token::new(Comment, StartEndSpan::new(0, 22));
        assert_eq!(token, expected);
    }

    #[test]
    fn comment_multiline_nested() {
        let source = "/* \n /* \n */ \n */";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_verbose().unwrap()[0].clone();
        let expected = Token::new(Comment, StartEndSpan::new(0, 17));
        assert_eq!(token, expected);
    }

    #[test]
    fn comment_multiline_nested_with_stars_and_slashes() {
        let source = "/* \n /* * / / * */ \n */";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_verbose().unwrap()[0].clone();
        let expected = Token::new(Comment, StartEndSpan::new(0, 23));
        assert_eq!(token, expected);
    }
}
