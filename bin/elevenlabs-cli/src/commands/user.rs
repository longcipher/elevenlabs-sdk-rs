//! User CLI subcommands.

use clap::{Args, Subcommand};

use crate::{cli::Cli, context::build_client, output::print_json};

/// User account operations.
#[derive(Debug, Args)]
pub(crate) struct UserArgs {
    #[command(subcommand)]
    pub command: UserCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum UserCommands {
    /// Get current user info.
    Info,

    /// Get subscription details.
    Subscription,
}

/// Execute a user subcommand.
pub(crate) async fn execute(args: &UserArgs, cli: &Cli) -> eyre::Result<()> {
    let client = build_client(cli)?;

    match &args.command {
        UserCommands::Info => {
            let response = client.user().get().await?;
            print_json(&response, cli.format)?;
        }
        UserCommands::Subscription => {
            let response = client.user().get_subscription().await?;
            print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
