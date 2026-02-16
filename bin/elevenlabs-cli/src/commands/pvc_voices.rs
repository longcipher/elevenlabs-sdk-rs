//! PVC voices CLI subcommands.

use clap::{Args, Subcommand};

/// Professional voice cloning operations.
#[derive(Debug, Args)]
pub(crate) struct PvcVoicesArgs {
    #[command(subcommand)]
    pub command: PvcVoicesCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum PvcVoicesCommands {
    /// Create a new PVC voice.
    Create {
        /// Name of the voice.
        #[arg(long)]
        name: String,
    },

    /// Edit an existing PVC voice.
    Edit {
        /// Voice ID to edit.
        #[arg(long)]
        voice_id: String,

        /// New name for the voice.
        #[arg(long)]
        name: Option<String>,
    },
}

/// Execute a PVC voices subcommand.
pub(crate) async fn execute(args: &PvcVoicesArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        PvcVoicesCommands::Create { name } => {
            let request = elevenlabs_sdk::types::CreatePvcVoiceRequest {
                name: name.clone(),
                description: None,
                labels: None,
            };
            let response = client.pvc_voices().create_pvc_voice(&request).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        PvcVoicesCommands::Edit { voice_id, name } => {
            let request = elevenlabs_sdk::types::EditPvcVoiceRequest {
                name: name.clone(),
                description: None,
                labels: None,
            };
            let response = client.pvc_voices().edit_pvc_voice(voice_id, &request).await?;
            crate::output::print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
