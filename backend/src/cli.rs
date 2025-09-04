use std::io;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};


#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: CliCommands
}

#[derive(Clone, Debug, Subcommand)]
pub enum CliCommands {
    /// Start the server
    Serve,

    /// Print the default configuration to std out
    PrintDefaultConfig,
    /// Print the default logging configuration to std out
    PrintDefaultLogConfig,

    // /// User manipulation
    // User {
    //     #[clap(subcommand)]
    //     user_command: UserCommands,
    // },
    /// Shell completion
    Completion {
        shell: Shell
    }
}

pub fn print_completions<G: Generator>(r#gen: G) {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(r#gen, &mut cmd, name, &mut io::stdout());
}
