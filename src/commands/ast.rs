use std::fs;

use minipl::lexing::scan;
use minipl::parsing::parse;
use minipl::tokens::RawToken;
use minipl::visitors::ASTPrinter;

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use camino::Utf8PathBuf;
use miette::{IntoDiagnostic, Result};
use tracing::info;

pub fn ast(path: Utf8PathBuf) -> Result<()> {
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

    // 3. Parsing
    // remove ignorables
    tokens.retain(|token| {
        !matches!(
            token.token,
            RawToken::Comment | RawToken::Error(_) | RawToken::Whitespace | RawToken::EOF
        )
    });

    if tokens.is_empty() {
        info!("Nothing to parse. Source contained ignorable tokens only.");
        return Ok(());
    }

    let statements = match parse(tokens) {
        Ok(statements) => statements,
        Err(err) => {
            let report: miette::Report = err.into();
            return Err(report.with_source_code(source));
        }
    };

    // 4. AST prettyprinting
    let mut astprinter = ASTPrinter::default();
    for statement in statements {
        let prettyprint = astprinter.print(&statement.into())?;
        println!("{}", prettyprint);
    }

    Ok(())
}
