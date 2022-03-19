use std::fs;

use minipl::lexing::*;
use minipl::tokens::RawToken;

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use camino::Utf8PathBuf;
use miette::{IntoDiagnostic, Result};
use tracing::info;

pub fn lex(path: Utf8PathBuf, verbose: bool) -> Result<()> {
    let source: String = fs::read_to_string(&path).into_diagnostic()?;
    let mut lexer = Lexer::new(&source);
    let tokens = if verbose {
        lexer.scan_verbose()?
    } else {
        lexer.scan()?
    };

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
        report = report.with_label(
            Label::new((&path, token.span.start..token.span.end))
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
