use crate::span::StartEndSpan;

use super::Error;
use super::Lexer;
use super::LexingError;
use super::Text;
use super::Token;
use std::string::String;

static ERROR_STRING_UNTERMINATED_OR_NEWLINE: &str = "Unterminated string or unescaped newline";
static ERROR_STRING_UNKNOWN_ESCAPE: &str =
    "Unknown character escape sequence or unescaped backslash";

impl<'a> Lexer<'a> {
    /// Internal helper function for scanning a string literal. Returns a [Token] with [`RawToken::Text(String)`]
    pub(crate) fn scan_string(&mut self) -> Result<Token, LexingError> {
        // Consume the first quote
        let (start, _) = self.maybe_next()?;
        let mut length = 1;

        // Consume and collect all characters within the string
        let mut contents = String::new();
        while let Some((_, char)) = self.iter.next_if(|&(_, char)| char != '"') {
            length += 1;

            // Specification forbids unescaped newlines
            if char == '\n' {
                return Ok(Token::new(
                    Error(ERROR_STRING_UNTERMINATED_OR_NEWLINE.into()),
                    StartEndSpan::new(start, length),
                ));
            }
            // Parse escape characters, following https://doc.rust-lang.org/std/primitive.char.html
            else if char == '\\' {
                let (_, next) = self.maybe_next()?;
                length += 1;
                match next {
                    't' => contents.push('\t'),
                    'r' => contents.push('\r'),
                    'n' => contents.push('\n'),
                    '\'' => contents.push('\''),
                    '\"' => contents.push('\"'),
                    '\\' => contents.push('\\'),
                    _ => {
                        return Ok(Token::new(
                            Error(ERROR_STRING_UNKNOWN_ESCAPE.into()),
                            StartEndSpan::new(start, start + length),
                        ))
                    }
                }
            } else {
                // Normal character, push as-is
                contents.push(char);
            }
        }

        // Check if we have an ending quote
        if let Some((_, _)) = self.iter.next_if(|&(_, char)| char == '\"') {
            length += 1;
            Ok(Token::new(
                Text(contents),
                StartEndSpan::new(start, start + length),
            ))
        } else {
            Ok(Token::new(
                Error(ERROR_STRING_UNTERMINATED_OR_NEWLINE.into()),
                StartEndSpan::new(start, start + length),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use crate::lexing::string::*;
    use crate::lexing::*;

    #[test]
    fn empty() {
        // NOTE: original source code will have the literal quotes
        let source = "\"\"";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Text("".into()), StartEndSpan::new(0, 2));
        assert_eq!(token, expected);
    }

    #[test]
    fn unterminated() {
        let source = "\"";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(
            Error(ERROR_STRING_UNTERMINATED_OR_NEWLINE.into()),
            StartEndSpan::new(0, 1),
        );
        assert_eq!(token, expected);
    }

    #[test]
    fn raw_newline() {
        // NOTE: Specification forbids raw newlines within a string
        let source = "\"multi\nline\"";
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(
            Error(ERROR_STRING_UNTERMINATED_OR_NEWLINE.into()),
            StartEndSpan::new(0, 7),
        );
        assert_eq!(token, expected);
    }

    #[test]
    fn unknown_escape() {
        let source = r#""\ä""#;
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(
            Error(ERROR_STRING_UNKNOWN_ESCAPE.into()),
            StartEndSpan::new(0, 3),
        );
        assert_eq!(token, expected);
    }

    #[test]
    fn escaped_tab() {
        let source = r#""\t""#;
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Text("\t".into()), StartEndSpan::new(0, 4));
        assert_eq!(token, expected);
    }

    #[test]
    fn escaped_carriage_return() {
        let source = r#""\r""#;
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Text("\r".into()), StartEndSpan::new(0, 4));
        assert_eq!(token, expected);
    }

    #[test]
    fn escaped_line_feed() {
        let source = r#""\n""#;
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Text("\n".into()), StartEndSpan::new(0, 4));
        assert_eq!(token, expected);
    }

    #[test]
    fn escaped_single_quote() {
        let source = r#""\'""#;
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Text("\'".into()), StartEndSpan::new(0, 4));
        assert_eq!(token, expected);
    }

    #[test]
    fn escaped_double_quote() {
        let source = r#""\"""#;
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Text("\"".into()), StartEndSpan::new(0, 4));
        assert_eq!(token, expected);
    }

    #[test]
    fn escaped_backslash() {
        let source = r#""\\""#;
        let mut lexer = Lexer::new(source);
        let token = lexer.scan().unwrap()[0].clone();
        let expected = Token::new(Text("\\".into()), StartEndSpan::new(0, 4));
        assert_eq!(token, expected);
    }
}
