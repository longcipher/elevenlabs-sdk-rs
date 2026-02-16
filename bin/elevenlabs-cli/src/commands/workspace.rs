//! Workspace CLI subcommands.

use clap::{Args, Subcommand};

use crate::{cli::Cli, context::build_client, output::print_json};

/// Workspace management operations.
#[derive(Debug, Args)]
pub(crate) struct WorkspaceArgs {
    #[command(subcommand)]
    pub command: WorkspaceCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum WorkspaceCommands {
    /// Invite a user to the workspace.
    InviteUser {
        /// Email of the user to invite.
        #[arg(long)]
        email: String,
    },

    /// Update a workspace member.
    UpdateMember {
        /// Email of the member to update.
        #[arg(long)]
        email: String,
    },

    /// Get workspace webhooks.
    GetWebhooks,
}

/// Execute a workspace subcommand.
pub(crate) async fn execute(args: &WorkspaceArgs, cli: &Cli) -> eyre::Result<()> {
    let client = build_client(cli)?;

    match &args.command {
        WorkspaceCommands::InviteUser { email } => {
            let request = elevenlabs_sdk::types::InviteWorkspaceMemberRequest {
                email: email.clone(),
                ..Default::default()
            };
            let response = client.workspace().invite_user(&request).await?;
            print_json(&response, cli.format)?;
        }
        WorkspaceCommands::UpdateMember { email } => {
            let request = elevenlabs_sdk::types::UpdateWorkspaceMemberRequest {
                email: email.clone(),
                ..Default::default()
            };
            let response = client.workspace().update_member(&request).await?;
            print_json(&response, cli.format)?;
        }
        WorkspaceCommands::GetWebhooks => {
            let response = client.workspace().get_webhooks().await?;
            print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
