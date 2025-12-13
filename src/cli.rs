//! The cli module provides the (very small) CLI for monkey.

use std::path::PathBuf;

use crate::app::App;
use anyhow::Error;
use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable debug information
    #[arg(short, long, global = true)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a monkey script
    Run {
        /// The monkey script file to run
        file: PathBuf,

        /// Only tokenise the file and output the tokens.
        #[arg(short, long, default_value_t = false)]
        tokenize: bool,
    },

    /// Launch the monkey repl
    Repl,
}

/// run is the entrypoint for the CLI.
pub fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    let level = if cli.debug { Level::DEBUG } else { Level::INFO };

    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let app = App::new();

    match cli.command {
        Commands::Run { file, tokenize } => app.run(&file, tokenize)?,
        Commands::Repl => println!("Launching a repl"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_smoke_test() {
        Cli::command().debug_assert();
    }
}
