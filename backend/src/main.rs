use clap::Parser as _;

use crate::cli::{Cli, CliCommands};
use crate::server::admin::login::generate_admin_hash;
use crate::server::meeting::login::mail::test_email;
use crate::server::start_server;

pub mod cli;
pub mod server;
pub mod structs;
pub mod config;
pub mod db;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CliCommands::Serve => start_server().await?,
        CliCommands::GenerateAdminHash => generate_admin_hash(),
        CliCommands::PrintDefaultConfig => todo!(),
        CliCommands::PrintDefaultLogConfig => todo!(),
        CliCommands::TestEmail { email_address } => test_email(&email_address).await,
        CliCommands::Completion { shell } => Cli::print_completions(shell)
    }

    Ok(())
}

