#![doc = include_str!("../README.md")]

use std::process::ExitCode;

use colored::Colorize;

mod cli;

fn main() -> ExitCode {
    if let Err(e) = cli::run() {
        eprintln!("{}: {}", "Error".red().bold(), e.to_string().bold());
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
