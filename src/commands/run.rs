use std::fs;

use minipl::lexing::scan;
use minipl::parsing::parse;
use minipl::tokens::RawToken;
use minipl::visitors::Interpreter;

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use camino::Utf8PathBuf;
use miette::{IntoDiagnostic, Result};
use tracing::info;

pub fn run(path: Utf8PathBuf) -> Result<()> {
    // 1. Lexing
    let source: String = fs::read_to_string(&path).into_diagnostic()?;
    let mut tokens = scan(&source)?;
    let mut colors = ColorGenerator::new();

    // 2. Error reporting for lexing
    if tokens
        .iter()
        .any(|token| matches!(token.token, RawToken::Error(_)))
    {
        let mut report =
            Report::build(ReportKind::Error, &path, 0).with_message("Lexing errors found");

        for token in &tokens {
            if let RawToken::Error(message) = token.token.clone() {
                let start = token.span.0;
                let end = token.span.0 + token.span.1;
                report = report.with_label(
                    Label::new((&path, start..end))
                        .with_message(message)
                        .with_color(colors.next()),
                );
            }
        }

        report
            .finish()
            .print((&path, Source::from(&source)))
            .unwrap();
    }

    // 3. Parsing
    // remove ignorables
    tokens.retain(|token| {
        !matches!(
            token.token,
            RawToken::Comment | RawToken::Error(_) | RawToken::Whitespace | RawToken::EOF
        )
    });

    if tokens.is_empty() {
        info!("Nothing to execute. Source contained ignorable tokens only.");
        return Ok(());
    }

    let ast = match parse(tokens) {
        Ok(ast) => ast,
        Err(err) => {
            let report: miette::Report = err.into();
            return Err(report.with_source_code(source));
        }
    };

    // 4. Evaluation
    let mut interpreter = Interpreter::default();
    match interpreter.eval(&ast) {
        Ok(result) => println!("{:?}", result),
        Err(err) => {
            let report: miette::Report = err;
            return Err(report.with_source_code(source));
        }
    };

    Ok(())
}
