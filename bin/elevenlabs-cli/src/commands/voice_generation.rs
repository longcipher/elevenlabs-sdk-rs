//! Voice generation CLI subcommands.

use clap::{Args, Subcommand};

/// Voice generation operations.
#[derive(Debug, Args)]
pub(crate) struct VoiceGenerationArgs {
    #[command(subcommand)]
    pub command: VoiceGenerationCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum VoiceGenerationCommands {
    /// Get available voice generation parameters.
    GetParameters,

    /// Generate a random voice.
    GenerateRandom {
        /// Gender for the generated voice.
        #[arg(long)]
        gender: String,

        /// Accent for the generated voice.
        #[arg(long)]
        accent: String,

        /// Age for the generated voice.
        #[arg(long)]
        age: String,

        /// Text to use for the voice preview.
        #[arg(long)]
        text: String,
    },
}

/// Execute a voice-generation subcommand.
pub(crate) async fn execute(args: &VoiceGenerationArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        VoiceGenerationCommands::GetParameters => {
            let response = client.voice_generation().get_parameters().await?;
            crate::output::print_json(&response, cli.format)?;
        }
        VoiceGenerationCommands::GenerateRandom { gender, accent, age, text } => {
            let gender = match gender.to_lowercase().as_str() {
                "female" => elevenlabs_sdk::types::GenerateVoiceGender::Female,
                "male" => elevenlabs_sdk::types::GenerateVoiceGender::Male,
                _ => return Err(eyre::eyre!("Invalid gender: {gender}. Use 'male' or 'female'")),
            };
            let age = match age.to_lowercase().as_str() {
                "young" => elevenlabs_sdk::types::GenerateVoiceAge::Young,
                "middle_aged" | "middle-aged" => {
                    elevenlabs_sdk::types::GenerateVoiceAge::MiddleAged
                }
                "old" => elevenlabs_sdk::types::GenerateVoiceAge::Old,
                _ => {
                    return Err(eyre::eyre!(
                        "Invalid age: {age}. Use 'young', 'middle_aged', or 'old'"
                    ))
                }
            };
            let request = elevenlabs_sdk::types::GenerateRandomVoiceRequest {
                gender,
                accent: accent.clone(),
                age,
                accent_strength: 1.0,
                text: text.clone(),
            };
            let audio = client.voice_generation().generate_random(&request).await?;
            use tokio::io::AsyncWriteExt;
            let mut stdout = tokio::io::stdout();
            stdout.write_all(&audio).await?;
        }
    }
    Ok(())
}
