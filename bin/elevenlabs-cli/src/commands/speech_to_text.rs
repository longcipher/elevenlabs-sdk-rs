//! Speech-to-text CLI subcommands.

use clap::{Args, Subcommand};

/// Speech-to-text transcription operations.
#[derive(Debug, Args)]
pub(crate) struct SpeechToTextArgs {
    #[command(subcommand)]
    pub command: SpeechToTextCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum SpeechToTextCommands {
    /// Transcribe an audio file.
    Transcribe {
        /// Path to the audio file to transcribe.
        #[arg(long)]
        input: String,

        /// Model ID to use for transcription.
        #[arg(long)]
        model_id: Option<String>,
    },

    /// Get an existing transcript.
    GetTranscript {
        /// Transcript ID to retrieve.
        #[arg(long)]
        transcript_id: String,
    },

    /// Delete a transcript.
    DeleteTranscript {
        /// Transcript ID to delete.
        #[arg(long)]
        transcript_id: String,
    },
}

/// Execute a speech-to-text subcommand.
pub(crate) async fn execute(args: &SpeechToTextArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        SpeechToTextCommands::Transcribe { input, model_id } => {
            let audio_data = tokio::fs::read(input).await?;
            let filename = std::path::Path::new(input)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("audio.mp3");
            let mut request = elevenlabs_sdk::types::SpeechToTextRequest::default();
            if let Some(id) = model_id {
                request.model_id = match id.as_str() {
                    "scribe_v1" => elevenlabs_sdk::types::SpeechToTextModelId::ScribeV1,
                    _ => elevenlabs_sdk::types::SpeechToTextModelId::ScribeV2,
                };
            }
            let response = client
                .speech_to_text()
                .transcribe(&request, Some((&audio_data, filename, "audio/mpeg")))
                .await?;
            crate::output::print_json(&response, cli.format)?;
        }
        SpeechToTextCommands::GetTranscript { transcript_id } => {
            let response = client.speech_to_text().get_transcript(transcript_id).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        SpeechToTextCommands::DeleteTranscript { transcript_id } => {
            client.speech_to_text().delete_transcript(transcript_id).await?;
            eprintln!("Transcript {transcript_id} deleted");
        }
    }
    Ok(())
}
