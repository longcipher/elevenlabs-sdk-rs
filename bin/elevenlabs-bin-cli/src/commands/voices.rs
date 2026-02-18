//! Voices CLI subcommands.

use clap::{Args, Subcommand};

use crate::{cli::Cli, context::build_client, output::print_json};

/// Voice management operations.
#[derive(Debug, Args)]
pub(crate) struct VoicesArgs {
    #[command(subcommand)]
    pub command: VoicesCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum VoicesCommands {
    /// List all voices.
    List,

    /// Get details about a voice.
    Get {
        /// Voice ID to retrieve.
        #[arg(long)]
        voice_id: String,
    },

    /// Delete a voice.
    Delete {
        /// Voice ID to delete.
        #[arg(long)]
        voice_id: String,
    },

    /// Get voice settings.
    GetSettings {
        /// Voice ID to get settings for.
        #[arg(long)]
        voice_id: String,
    },

    /// Edit voice settings.
    EditSettings {
        /// Voice ID to edit settings for.
        #[arg(long)]
        voice_id: String,

        /// Stability value (0.0 to 1.0).
        #[arg(long)]
        stability: f64,

        /// Similarity boost value (0.0 to 1.0).
        #[arg(long)]
        similarity_boost: f64,
    },

    /// Browse shared/library voices.
    GetShared,
}

/// Execute a voices subcommand.
pub(crate) async fn execute(args: &VoicesArgs, cli: &Cli) -> eyre::Result<()> {
    let client = build_client(cli)?;

    match &args.command {
        VoicesCommands::List => {
            let response = client.voices().list(None).await?;
            print_json(&response, cli.format)?;
        }
        VoicesCommands::Get { voice_id } => {
            let response = client.voices().get(voice_id, None).await?;
            print_json(&response, cli.format)?;
        }
        VoicesCommands::Delete { voice_id } => {
            let response = client.voices().delete(voice_id).await?;
            print_json(&response, cli.format)?;
        }
        VoicesCommands::GetSettings { voice_id } => {
            let response = client.voices().get_settings(voice_id).await?;
            print_json(&response, cli.format)?;
        }
        VoicesCommands::EditSettings { voice_id, stability, similarity_boost } => {
            let settings = elevenlabs_sdk::types::VoiceSettings {
                stability: Some(*stability),
                similarity_boost: Some(*similarity_boost),
                style: None,
                use_speaker_boost: None,
                speed: None,
            };
            let response = client.voices().edit_settings(voice_id, &settings).await?;
            print_json(&response, cli.format)?;
        }
        VoicesCommands::GetShared => {
            let response = client
                .voices()
                .get_shared_voices(None, None, None, None, None, None, None, None)
                .await?;
            print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
