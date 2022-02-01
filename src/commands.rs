use std::fs::{self};

use minipl::lexing::*;

use anyhow::Result;
use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use camino::Utf8PathBuf;

pub(crate) fn run(path: Utf8PathBuf) -> Result<()> {
    // 1. Lexing
    let source: String = fs::read_to_string(&path)?;
    let tokens = parse(&source)?;

    for token in tokens {
        // Print all error reports
        if let RawToken::Error(message) = token.token {
            Report::build(ReportKind::Error, &path, token.location.0)
                .with_message(message.clone())
                .with_label(
                    Label::new((&path, (token.location.0)..(token.location.1)))
                        .with_message(message),
                )
                .finish()
                .print((&path, Source::from(&source)))
                .unwrap();
        }
    }

    // TODO: 2. Semantic analysis
    // TODO: 3. Execution

    Ok(())
}

pub(crate) fn lex(path: Utf8PathBuf) -> Result<()> {
    let source: String = fs::read_to_string(&path)?;
    let tokens = parse(&source)?;

    // Ignore certain elements when printing the lexing report
    let tokens = tokens
        .iter()
        .filter(|token| !matches!(token.token, RawToken::Whitespace | RawToken::Semicolon));

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
