//! Models CLI subcommands.

use clap::{Args, Subcommand};

use crate::{cli::Cli, context::build_client, output::print_json};

/// Model operations.
#[derive(Debug, Args)]
pub(crate) struct ModelsArgs {
    #[command(subcommand)]
    pub command: ModelsCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum ModelsCommands {
    /// List available models.
    List,
}

/// Execute a models subcommand.
pub(crate) async fn execute(args: &ModelsArgs, cli: &Cli) -> eyre::Result<()> {
    let client = build_client(cli)?;

    match &args.command {
        ModelsCommands::List => {
            let response = client.models().list().await?;
            print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
