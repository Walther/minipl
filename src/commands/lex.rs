use std::fs;

use minipl::lexing::*;
use minipl::rawtoken::RawToken;

use anyhow::Result;
use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use camino::Utf8PathBuf;

pub fn lex(path: Utf8PathBuf, verbose: bool) -> Result<()> {
    let source: String = fs::read_to_string(&path)?;
    let mut tokens = scan(&source)?;

    // Ignore certain elements when printing the lexing report, unless in verbose mode
    if !verbose {
        tokens.retain(|token| {
            !matches!(
                token.token,
                RawToken::Whitespace | RawToken::Semicolon | RawToken::EOF
            )
        });
    }

    let mut report =
        Report::build(ReportKind::Advice, &path, 0).with_message("Lexing report".to_string());
    let mut colors = ColorGenerator::new();

    for token in tokens {
        // Print all tokens as the lexer sees it
        let token_name = &token.token;
        report = report.with_label(
            Label::new((&path, (token.location.0)..(token.location.1)))
                .with_message(format!("{token_name:?}"))
                .with_color(colors.next()),
        );
    }

    report
        .finish()
        .print((&path, Source::from(&source)))
        .unwrap();

    Ok(())
}
