use std::fs;

use minipl::lexing::*;
use minipl::tokens::RawToken;

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use camino::Utf8PathBuf;
use miette::{IntoDiagnostic, Result};
use tracing::info;

pub fn lex(path: Utf8PathBuf, verbose: bool) -> Result<()> {
    let source: String = fs::read_to_string(&path).into_diagnostic()?;
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

    if tokens.is_empty() || tokens[0].tokentype() == RawToken::EOF {
        info!("Nothing to lex. Source code was empty.");
        return Ok(());
    }

    // Using `ariadne` for printing the lexing report

    let mut report =
        Report::build(ReportKind::Advice, &path, 0).with_message("Lexing report".to_string());
    let mut colors = ColorGenerator::new();

    for token in tokens {
        let token_name = &token.token;
        let start = token.span.0;
        let end = token.span.0 + token.span.1;
        report = report.with_label(
            Label::new((&path, start..end))
                .with_message(format!("{token_name:?}"))
                .with_color(colors.next()),
        );
    }

    report
        .finish()
        .print((&path, Source::from(&source)))
        .unwrap();

    // TODO: should we print miette errors on lexing errors?

    Ok(())
}
