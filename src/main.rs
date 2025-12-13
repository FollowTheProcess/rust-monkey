#![doc = include_str!("../README.md")]

use std::process::ExitCode;

use colored::Colorize;

mod app;
mod cli;
mod syntax;

fn main() -> ExitCode {
    if let Err(e) = cli::run() {
        eprintln!("{}: {}", "Error".red().bold(), e.to_string().bold());
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
