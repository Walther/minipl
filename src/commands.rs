use std::fs::{self};

use minipl::parsing::parse;
use minipl::token::Token;
use minipl::{lexing::*, parsing::astprinter::ASTPrinter};

use anyhow::{anyhow, Result};
use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use camino::Utf8PathBuf;

pub(crate) fn ast(path: Utf8PathBuf) -> Result<()> {
    // 1. Lexing
    let source: String = fs::read_to_string(&path)?;
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

    // 3. Parsing
    // remove ignorables
    tokens.retain(|token| {
        !matches!(
            token.token,
            RawToken::Comment | RawToken::Error(_) | RawToken::Whitespace | RawToken::EOF
        )
    });
    let ast = match parse(tokens) {
        Ok(ast) => ast,
        Err(err) => {
            let token = err
                .token
                .unwrap_or_else(|| Token::new(RawToken::Error("Unknown location".into()), (0, 0))); // TODO: better ergonomics
            let report = Report::build(ReportKind::Error, &path, 0)
                .with_message("Parse errors found")
                .with_label(
                    Label::new((&path, (token.location.0)..(token.location.1)))
                        .with_message(err.message.clone())
                        .with_color(colors.next()),
                );
            report
                .finish()
                .print((&path, Source::from(&source)))
                .unwrap();
            return Err(anyhow!(err.message));
        }
    };
    // TODO: better AST prettyprinting
    let mut astprinter = ASTPrinter::default();
    let prettyprint = astprinter.print(&ast);
    println!("{}", prettyprint);

    Ok(())
}

pub(crate) fn lex(path: Utf8PathBuf, verbose: bool) -> Result<()> {
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

pub(crate) fn run(_path: Utf8PathBuf) -> Result<()> {
    unimplemented!()
}
