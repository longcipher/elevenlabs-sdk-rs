//! Speech-to-speech CLI subcommands.

use clap::{Args, Subcommand};

/// Speech-to-speech conversion operations.
#[derive(Debug, Args)]
pub(crate) struct SpeechToSpeechArgs {
    #[command(subcommand)]
    pub command: SpeechToSpeechCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum SpeechToSpeechCommands {
    /// Convert speech audio using a target voice.
    Convert {
        /// Voice ID to use for conversion.
        #[arg(long)]
        voice_id: String,

        /// Path to the input audio file.
        #[arg(long)]
        input: String,

        /// Model ID to use.
        #[arg(long)]
        model_id: Option<String>,

        /// Output file path for the converted audio.
        #[arg(short, long)]
        output: Option<String>,
    },
}

/// Execute a speech-to-speech subcommand.
pub(crate) async fn execute(args: &SpeechToSpeechArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        SpeechToSpeechCommands::Convert { voice_id, input, model_id, output } => {
            let audio_data = tokio::fs::read(input).await?;
            let filename = std::path::Path::new(input)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("audio.mp3");
            let mut request = elevenlabs_sdk::types::SpeechToSpeechRequest::default();
            if let Some(id) = model_id {
                request.model_id = id.clone();
            }
            let audio = client
                .speech_to_speech()
                .convert(voice_id, &request, &audio_data, filename, "audio/mpeg", None)
                .await?;
            if let Some(path) = output {
                tokio::fs::write(path, &audio).await?;
                eprintln!("Audio written to {path}");
            } else {
                use tokio::io::AsyncWriteExt;
                let mut stdout = tokio::io::stdout();
                stdout.write_all(&audio).await?;
            }
        }
    }
    Ok(())
}
