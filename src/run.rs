use anyhow::Result;
use camino::Utf8PathBuf;
use std::fs::{self};

use minipl::lexing::*;

pub(crate) fn run(path: Utf8PathBuf) -> Result<()> {
    let file = fs::read_to_string(path)?;
    let tokens = parse(&file)?;

    // TODO: currently, we just print the tokens. This exists only for error handling testing.
    println!("{tokens:?}");
    Ok(())
}
