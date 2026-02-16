//! Dubbing CLI subcommands.

use clap::{Args, Subcommand};

/// Dubbing operations.
#[derive(Debug, Args)]
pub(crate) struct DubbingArgs {
    #[command(subcommand)]
    pub command: DubbingCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum DubbingCommands {
    /// Create a new dubbing project.
    Create {
        /// Source language code.
        #[arg(long)]
        source_lang: Option<String>,

        /// Target language code.
        #[arg(long)]
        target_lang: String,
    },

    /// List all dubbing projects.
    List,

    /// Get details about a dubbing project.
    Get {
        /// Dubbing project ID.
        #[arg(long)]
        dubbing_id: String,
    },

    /// Delete a dubbing project.
    Delete {
        /// Dubbing project ID.
        #[arg(long)]
        dubbing_id: String,
    },

    /// Get audio for a dubbed project.
    GetAudio {
        /// Dubbing project ID.
        #[arg(long)]
        dubbing_id: String,

        /// Language code for the audio.
        #[arg(long)]
        language_code: String,

        /// Output file path for the audio.
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Get transcript for a dubbed project.
    GetTranscript {
        /// Dubbing project ID.
        #[arg(long)]
        dubbing_id: String,

        /// Language code for the transcript.
        #[arg(long)]
        language_code: String,
    },
}

/// Execute a dubbing subcommand.
pub(crate) async fn execute(args: &DubbingArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        DubbingCommands::Create { source_lang, target_lang } => {
            let request = elevenlabs_sdk::types::CreateDubbingRequest {
                name: None,
                source_url: None,
                source_lang: source_lang.clone(),
                target_lang: Some(target_lang.clone()),
                target_accent: None,
                num_speakers: None,
                watermark: None,
                start_time: None,
                end_time: None,
                highest_resolution: None,
                drop_background_audio: None,
                use_profanity_filter: None,
                dubbing_studio: None,
                disable_voice_cloning: None,
                mode: None,
                csv_fps: None,
            };
            let response = client.dubbing().create(&request, None).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        DubbingCommands::List => {
            let response = client.dubbing().list(None, None).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        DubbingCommands::Get { dubbing_id } => {
            let response = client.dubbing().get(dubbing_id).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        DubbingCommands::Delete { dubbing_id } => {
            let response = client.dubbing().delete(dubbing_id).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        DubbingCommands::GetAudio { dubbing_id, language_code, output } => {
            let audio = client.dubbing().get_audio(dubbing_id, language_code).await?;
            if let Some(path) = output {
                tokio::fs::write(path, &audio).await?;
                eprintln!("Audio written to {path}");
            } else {
                use tokio::io::AsyncWriteExt;
                let mut stdout = tokio::io::stdout();
                stdout.write_all(&audio).await?;
            }
        }
        DubbingCommands::GetTranscript { dubbing_id, language_code } => {
            let response = client.dubbing().get_transcript(dubbing_id, language_code).await?;
            crate::output::print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
