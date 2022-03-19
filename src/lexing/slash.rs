use crate::span::StartEndSpan;

use super::Comment;
use super::Lexer;
use super::LexingError;
use super::Slash;
use super::Token;

impl Lexer<'_> {
    /// Internal helper function for scanning a lexeme that starts with a slash. This could be a [Comment], or just a [Slash].
    pub(crate) fn scan_slash(&mut self) -> Result<Token, LexingError> {
        // TODO: remove unwraps
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
        // TODO: multi-line comments with nesting support

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
}
