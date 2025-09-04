use clap::Parser as _;

use crate::cli::{print_completions, Cli};

pub mod cli;

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::CliCommands::Serve => todo!(),
        cli::CliCommands::PrintDefaultConfig => todo!(),
        cli::CliCommands::PrintDefaultLogConfig => todo!(),
        cli::CliCommands::Completion { shell } => print_completions(shell)
    }

    println!("Hello, world!");
}

