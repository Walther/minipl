use anyhow::{Error, Result};
use camino::Utf8PathBuf;
use std::fs::{self, File};
use tracing::{debug, error, Level};

pub(crate) fn run(path: Utf8PathBuf) -> Result<()> {
    let file = fs::read_to_string(path)?;

    // TODO: currently, we just print the file contents. This exists only for error handling testing.
    println!("{file}");
    Ok(())
}
