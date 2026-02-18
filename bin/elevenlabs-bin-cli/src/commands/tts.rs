//! Text-to-speech CLI subcommands.

use clap::{Args, Subcommand};

/// Text-to-speech operations.
#[derive(Debug, Args)]
pub(crate) struct TtsArgs {
    #[command(subcommand)]
    pub command: TtsCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum TtsCommands {
    /// Convert text to speech audio.
    Convert {
        /// Voice ID to use for synthesis.
        #[arg(long)]
        voice_id: String,

        /// Text to convert to speech.
        #[arg(long)]
        text: String,

        /// Model ID to use.
        #[arg(long)]
        model_id: Option<String>,

        /// Output file path for the audio.
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Convert text to speech and stream the audio.
    ConvertStream {
        /// Voice ID to use for synthesis.
        #[arg(long)]
        voice_id: String,

        /// Text to convert to speech.
        #[arg(long)]
        text: String,

        /// Model ID to use.
        #[arg(long)]
        model_id: Option<String>,

        /// Output file path for the audio.
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Convert text to speech with timestamps.
    ConvertWithTimestamps {
        /// Voice ID to use for synthesis.
        #[arg(long)]
        voice_id: String,

        /// Text to convert to speech.
        #[arg(long)]
        text: String,

        /// Model ID to use.
        #[arg(long)]
        model_id: Option<String>,

        /// Output file path for the audio.
        #[arg(short, long)]
        output: Option<String>,
    },
}

/// Write audio bytes to file or stdout.
async fn write_audio(data: &[u8], output: &Option<String>) -> eyre::Result<()> {
    if let Some(path) = output {
        tokio::fs::write(path, data).await?;
        eprintln!("Audio written to {path}");
    } else {
        use tokio::io::AsyncWriteExt;
        let mut stdout = tokio::io::stdout();
        stdout.write_all(data).await?;
    }
    Ok(())
}

/// Execute a text-to-speech subcommand.
pub(crate) async fn execute(args: &TtsArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        TtsCommands::Convert { voice_id, text, model_id, output } => {
            let mut request = elevenlabs_sdk::types::TextToSpeechRequest::new(text);
            request.model_id = model_id.clone();
            let audio = client.text_to_speech().convert(voice_id, &request, None, None).await?;
            write_audio(&audio, output).await?;
        }
        TtsCommands::ConvertStream { voice_id, text, model_id, output } => {
            use futures_util::StreamExt;
            let mut request = elevenlabs_sdk::types::TextToSpeechRequest::new(text);
            request.model_id = model_id.clone();
            let tts = client.text_to_speech();
            let mut stream = tts.convert_stream(voice_id, &request, None, None).await?;
            let mut buf = Vec::new();
            while let Some(chunk) = stream.next().await {
                buf.extend_from_slice(&chunk?);
            }
            write_audio(&buf, output).await?;
        }
        TtsCommands::ConvertWithTimestamps { voice_id, text, model_id, output: _ } => {
            let mut request = elevenlabs_sdk::types::TextToSpeechRequest::new(text);
            request.model_id = model_id.clone();
            let response = client
                .text_to_speech()
                .convert_with_timestamps(voice_id, &request, None, None)
                .await?;
            crate::output::print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
