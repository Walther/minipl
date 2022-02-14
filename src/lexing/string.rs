use super::Error;
use super::Text;
use super::Token;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;
use std::string::String;

static ERROR_STRING_UNTERMINATED_OR_NEWLINE: &str = "Unterminated string or unescaped newline";
static ERROR_STRING_UNKNOWN_ESCAPE: &str =
    "Unknown character escape sequence or unescaped backslash";

/// Internal helper function for scanning a string literal. Returns a [Token] with [`RawToken::Text(String)`]
pub(crate) fn scan_string(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // TODO: remove unwraps where possible
    // TODO: parse / evaluate escape characters

    // Consume the first quote
    let (start, _) = iter.next().unwrap();
    let mut end = start + 1;

    // Consume and collect all characters within the string
    let mut contents = String::new();
    while let Some((location, char)) = iter.next_if(|&(_, char)| char != '"') {
        end = location + 1;

        // Specification forbids unescaped newlines
        if char == '\n' {
            return Token::new(
                Error(ERROR_STRING_UNTERMINATED_OR_NEWLINE.into()),
                (start, location),
            );
        }
        // Parse escape characters, following https://doc.rust-lang.org/std/primitive.char.html
        else if char == '\\' {
            if let Some(&(_, next)) = iter.peek() {
                iter.next();
                end += 1;
                match next {
                    't' => contents.push('\t'),
                    'r' => contents.push('\r'),
                    'n' => contents.push('\n'),
                    '\'' => contents.push('\''),
                    '\"' => contents.push('\"'),
                    '\\' => contents.push('\\'),
                    _ => {
                        return Token::new(Error(ERROR_STRING_UNKNOWN_ESCAPE.into()), (start, end))
                    }
                }
            } else {
                // Ran out of string to parse
                // TODO: better error message for this case?
                return Token::new(Error(ERROR_STRING_UNKNOWN_ESCAPE.into()), (start, end));
            }
        } else {
            // Normal character, push as-is
            contents.push(char);
        }
    }

    // Check if we have an ending quote
    if let Some((location, _)) = iter.next_if(|&(_, char)| char == '\"') {
        end = location + 1;
        Token::new(Text(contents), (start, end))
    } else {
        Token::new(
            Error(ERROR_STRING_UNTERMINATED_OR_NEWLINE.into()),
            (start, end),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::lexing::string::*;
    use crate::lexing::*;

    #[test]
    fn empty() {
        // NOTE: original source code will have the literal quotes
        let token = &scan("\"\"").unwrap()[0];
        let expected = Token::new(Text("".into()), (0, 2));
        assert_eq!(token, &expected);
    }

    #[test]
    fn unterminated() {
        let token = &scan("\"").unwrap()[0];
        let expected = Token::new(Error(ERROR_STRING_UNTERMINATED_OR_NEWLINE.into()), (0, 1));
        assert_eq!(token, &expected);
    }

    #[test]
    fn raw_newline() {
        // NOTE: Specification forbids raw newlines within a string
        let token = &scan("\"multi\nline\"").unwrap()[0];
        let expected = Token::new(Error(ERROR_STRING_UNTERMINATED_OR_NEWLINE.into()), (0, 6));
        assert_eq!(token, &expected);
    }

    #[test]
    fn unused_escape() {
        let token = &scan(r#""\"#).unwrap()[0];
        let expected = Token::new(Error(ERROR_STRING_UNKNOWN_ESCAPE.into()), (0, 2));
        assert_eq!(token, &expected);
    }

    #[test]
    fn unknown_escape() {
        let token = &scan(r#""\ä""#).unwrap()[0];
        let expected = Token::new(Error(ERROR_STRING_UNKNOWN_ESCAPE.into()), (0, 3));
        assert_eq!(token, &expected);
    }

    #[test]
    fn escaped_tab() {
        let token = &scan(r#""\t""#).unwrap()[0];
        let expected = Token::new(Text("\t".into()), (0, 4));
        assert_eq!(token, &expected);
    }

    #[test]
    fn escaped_carriage_return() {
        let token = &scan(r#""\r""#).unwrap()[0];
        let expected = Token::new(Text("\r".into()), (0, 4));
        assert_eq!(token, &expected);
    }

    #[test]
    fn escaped_line_feed() {
        let token = &scan(r#""\n""#).unwrap()[0];
        let expected = Token::new(Text("\n".into()), (0, 4));
        assert_eq!(token, &expected);
    }

    #[test]
    fn escaped_single_quote() {
        let token = &scan(r#""\'""#).unwrap()[0];
        let expected = Token::new(Text("\'".into()), (0, 4));
        assert_eq!(token, &expected);
    }

    #[test]
    fn escaped_double_quote() {
        let token = &scan(r#""\"""#).unwrap()[0];
        let expected = Token::new(Text("\"".into()), (0, 4));
        assert_eq!(token, &expected);
    }

    #[test]
    fn escaped_backslash() {
        let token = &scan(r#""\\""#).unwrap()[0];
        let expected = Token::new(Text("\\".into()), (0, 4));
        assert_eq!(token, &expected);
    }
}
