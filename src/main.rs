use anyhow::{anyhow, Result};
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use tracing::{debug, error, Level};
use tracing_subscriber::fmt::time;

mod commands;
use commands::*;

/// Interpreter & Compiler for the Mini-PL programming language.
/// Written for the Spring 2022 Compilers course at University of Helsinki
#[derive(Debug, Parser)]
#[clap(name = "minipl", version)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print the abstract syntax tree for the given file
    Ast {
        /// The path to the file to run
        path: Utf8PathBuf,
    },

    /// Run the lexer on the given file
    Lex {
        /// The path to the file to scan
        path: Utf8PathBuf,
        #[clap(long, short)]
        /// Verbose mode prints all lexemes, including e.g. whitespace and so on
        verbose: bool,
    },

    /// Run a given file with the interpreter
    Run {
        /// The path to the file to run
        path: Utf8PathBuf,
    },

    /// Build a given file with the compiler. Not implemented yet.
    Build {
        /// The path to the file to build
        path: Utf8PathBuf,
    },
}

#[derive(Debug, Args)]
struct GlobalOpts {
    /// Debug tracing of the application flow
    #[clap(long, short, global = true)]
    debug: bool,
}

fn main() -> Result<()> {
    let app = App::parse();

    if app.global_opts.debug {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .with_timer(time::UtcTime::rfc_3339())
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::ERROR)
            .with_timer(time::UtcTime::rfc_3339())
            .init();
    }

    match app.command {
        Command::Ast { path } => {
            debug!("AST subcommand called");
            debug!("File path: {}", path);
            match ast(path) {
                Ok(_) => (),
                Err(error) => error!("{error}"),
            }
        }
        Command::Lex { path, verbose } => {
            debug!("Lex subcommand called");
            debug!("File path: {}", path);
            match lex(path, verbose) {
                Ok(_) => (),
                Err(error) => error!("{error}"),
            }
        }
        Command::Run { path } => {
            debug!("Run subcommand called");
            debug!("File path: {}", path);
            match run(path) {
                Ok(_) => (),
                Err(error) => error!("{error}"),
            }
        }
        Command::Build { path } => {
            debug!("Build subcommand called");
            debug!("File path: {}", path);
            error!("Compiler mode not implemented yet");
            return Err(anyhow!("Compiler mode not implemented yet"));
        }
    }

    Ok(())
}
