use std::fs::{self};

use minipl::lexing::*;

use anyhow::Result;
use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use camino::Utf8PathBuf;

pub(crate) fn run(path: Utf8PathBuf) -> Result<()> {
    // 1. Lexing
    let source: String = fs::read_to_string(&path)?;
    let tokens = parse(&source)?;
    let mut colors = ColorGenerator::new();

    // 2. Error reporting for lexing
    if tokens
        .iter()
        .any(|token| matches!(token.token, RawToken::Error(_)))
    {
        let mut report =
            Report::build(ReportKind::Error, &path, 0).with_message("Lexing errors found");

        for token in tokens {
            if let RawToken::Error(message) = token.token {
                report = report.with_label(
                    Label::new((&path, (token.location.0)..(token.location.1)))
                        .with_message(message)
                        .with_color(colors.next()),
                )
            }
        }

        report
            .finish()
            .print((&path, Source::from(&source)))
            .unwrap();
    }

    // TODO: n. Semantic analysis
    // TODO: n. Execution

    Ok(())
}

pub(crate) fn lex(path: Utf8PathBuf, verbose: bool) -> Result<()> {
    let source: String = fs::read_to_string(&path)?;
    let mut tokens = parse(&source)?;

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
