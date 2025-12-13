//! The app module exposes the implementation of the monkey program, public methods
//! here map directly to the commands exposed via the cli.

// TODO(@FollowTheProcess): Use a proper error type rather than just anyhow

use crate::syntax::lexer::Lexer;
use crate::syntax::token::Token;
use std::{
    fs::{self},
    io::{self, Write},
    path::PathBuf,
};

use anyhow::Result;

#[derive(Debug)]
pub struct App {}

impl App {
    /// Create a new App.
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Run handles the run subcommand.
    #[tracing::instrument]
    pub(crate) fn run(&self, file: &PathBuf, tokenize: bool) -> Result<()> {
        let contents = fs::read_to_string(file)?;
        let mut lexer = Lexer::new(&contents);

        let mut tokens: Vec<Token> = vec![];

        loop {
            let token = lexer.lex();
            tokens.push(token);
            if token.is_eof() || token.is_error() {
                break;
            }
        }

        let mut stdout = io::stdout().lock();
        for token in tokens {
            writeln!(stdout, "{token}")?;
        }
        stdout.flush()?;

        Ok(())
    }
}
