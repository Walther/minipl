use super::Text;
use super::Token;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;
use std::string::String;

/// Internal helper function for scanning a string literal. Returns a [Token] with [`RawToken::Text(String)`]
pub(crate) fn scan_string(iter: &mut Peekable<Enumerate<Chars>>) -> Token {
    // TODO: remove unwraps where possible
    // TODO: parse / evaluate escape characters
    // TODO: technically we might need to ban literal multiline strings i.e. error on newline chars unless escaped?

    // Consume the first quote
    let (start, _) = iter.next().unwrap();

    // Consume and collect all characters within the string
    let mut contents = String::new();
    while let Some((_, char)) = iter.next_if(|&(_, char)| char != '"') {
        contents.push(char);
    }
    // Consume the ending quote too
    let (location, _) = iter.next().unwrap();
    let end = location + 1;

    Token::new(Text(contents), (start, end))
}

#[cfg(test)]
mod tests {
    use crate::lexing::*;

    #[test]
    fn string() {
        // NOTE: original source code will have the literal quotes
        let token = &scan("\"\"").unwrap()[0];
        let expected = Token::new(Text("".into()), (0, 2));
        assert_eq!(token, &expected);

        // FIXME: should probably be prohibited? Unclear spec...
        let token = &scan("\"multi\nline\"").unwrap()[0];
        let expected = Token::new(Text("multi\nline".into()), (0, 12));
        assert_eq!(token, &expected);
    }
}
