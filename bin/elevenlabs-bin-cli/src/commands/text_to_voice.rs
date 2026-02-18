//! Text-to-voice CLI subcommands.

use clap::{Args, Subcommand};

/// Text-to-voice generation operations.
#[derive(Debug, Args)]
pub(crate) struct TextToVoiceArgs {
    #[command(subcommand)]
    pub command: TextToVoiceCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum TextToVoiceCommands {
    /// Create voice previews from a text description.
    CreatePreviews {
        /// Text description of the desired voice.
        #[arg(long)]
        text: String,
    },

    /// Create a voice from a text description.
    CreateVoice {
        /// Text description of the desired voice.
        #[arg(long)]
        text: String,

        /// Name for the new voice.
        #[arg(long)]
        voice_name: String,
    },
}

/// Execute a text-to-voice subcommand.
pub(crate) async fn execute(args: &TextToVoiceArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        TextToVoiceCommands::CreatePreviews { text } => {
            let request = elevenlabs_sdk::types::VoicePreviewsRequest {
                voice_description: text.clone(),
                text: None,
                auto_generate_text: None,
                loudness: None,
                quality: None,
                seed: None,
                guidance_scale: None,
                should_enhance: None,
            };
            let response = client.text_to_voice().create_previews(&request).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        TextToVoiceCommands::CreateVoice { text, voice_name } => {
            let request = elevenlabs_sdk::types::CreateVoiceFromPreviewRequest {
                voice_name: voice_name.clone(),
                voice_description: text.clone(),
                generated_voice_id: String::new(),
                labels: None,
                played_not_selected_voice_ids: None,
            };
            let response = client.text_to_voice().create_voice(&request).await?;
            crate::output::print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
