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

    /// Generate admin password hash
    GenerateAdminHash,

    /// Test email delivery
    TestEmail { email_recipient: String },

    /// Print the default configuration to std out
    PrintDefaultConfig,
    /// Print the default logging configuration to std out
    PrintDefaultLogConfig,

    /// Shell completion
    Completion { shell: Shell }
}

impl Cli {
    pub fn print_completions<G: Generator>(shell: G) {
        let mut cmd = Self::command();
        let name = cmd.get_name().to_string();
        generate(shell, &mut cmd, name, &mut io::stdout());
    }
}
