use clap::Parser as _;

use crate::cli::{print_completions, Cli};
use crate::server::start_server;

pub mod cli;
pub mod server;
pub mod structs;

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::CliCommands::Serve => start_server().await?,
        cli::CliCommands::PrintDefaultConfig => todo!(),
        cli::CliCommands::PrintDefaultLogConfig => todo!(),
        cli::CliCommands::Completion { shell } => print_completions(shell)
    }
}

