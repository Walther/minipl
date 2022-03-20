use std::fs;

use minipl::tokens::RawToken;
use minipl::visitors::Interpreter;
use minipl::{lexing::Lexer, parsing::Parser};

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use camino::Utf8PathBuf;
use miette::{IntoDiagnostic, Result};
use tracing::info;

pub fn run(path: Utf8PathBuf) -> Result<()> {
    // 1. Lexing
    let source: String = fs::read_to_string(&path).into_diagnostic()?;
    let mut lexer = Lexer::new(&source);
    let scan_results = lexer.scan();
    // 2. Error reporting for UnrecoverableLexingError
    let tokens = match scan_results {
        Ok(tokens) => tokens,
        Err(err) => {
            // Print an additional newline to clear possible outputs
            println!();
            let report: miette::Report = err;
            return Err(report.with_source_code(source));
        }
    };

    // 3. Error reporting for RecoverableLexingError
    let mut colors = ColorGenerator::new();
    if tokens
        .iter()
        .any(|token| matches!(token.token, RawToken::Error(_)))
    {
        let mut report =
            Report::build(ReportKind::Error, &path, 0).with_message("Lexing errors found");

        for token in &tokens {
            if let RawToken::Error(message) = token.token.clone() {
                report = report.with_label(
                    Label::new((&path, token.span.start..token.span.end))
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

    // 4. Parsing
    if tokens.is_empty() {
        info!("Nothing to execute. Source contained ignorable tokens only.");
        return Ok(());
    }
    let mut parser = Parser::new(tokens);

    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(err) => {
            let report: miette::Report = err.into();
            return Err(report.with_source_code(source));
        }
    };

    // 5. Evaluation
    let mut interpreter = Interpreter::default();
    match interpreter.eval(&statements) {
        // NOTE: we discard any returned values
        Ok(_result) => {
            // Print an additional newline to clear the output line
            println!();
            // Return ok
            Ok(())
        }
        Err(err) => {
            // Print an additional newline to clear the output line
            println!();
            let report: miette::Report = err.into();
            Err(report.with_source_code(source))
        }
    }
}
