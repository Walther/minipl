use std::fs::{self};

use minipl::lexing::*;

use anyhow::Result;
use ariadne::{Label, Report, ReportKind, Source};
use camino::Utf8PathBuf;

pub(crate) fn run(path: Utf8PathBuf) -> Result<()> {
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

    Ok(())
}
