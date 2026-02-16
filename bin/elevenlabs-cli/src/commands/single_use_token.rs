//! Single-use token CLI subcommands.

use clap::{Args, Subcommand};

use crate::{cli::Cli, context::build_client, output::print_json};

/// Single-use token operations.
#[derive(Debug, Args)]
pub(crate) struct SingleUseTokenArgs {
    #[command(subcommand)]
    pub command: SingleUseTokenCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum SingleUseTokenCommands {
    /// Create a single-use token.
    Create,
}

/// Execute a single-use-token subcommand.
pub(crate) async fn execute(args: &SingleUseTokenArgs, cli: &Cli) -> eyre::Result<()> {
    let client = build_client(cli)?;

    match &args.command {
        SingleUseTokenCommands::Create => {
            let response = client.single_use_token().create("tts").await?;
            print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
