//! Audio isolation CLI subcommands.

use clap::{Args, Subcommand};

/// Audio isolation operations.
#[derive(Debug, Args)]
pub(crate) struct AudioIsolationArgs {
    #[command(subcommand)]
    pub command: AudioIsolationCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum AudioIsolationCommands {
    /// Isolate audio from background noise.
    Isolate {
        /// Path to the input audio file.
        #[arg(long)]
        input: String,

        /// Output file path for the isolated audio.
        #[arg(short, long)]
        output: Option<String>,
    },
}

/// Execute an audio-isolation subcommand.
pub(crate) async fn execute(args: &AudioIsolationArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        AudioIsolationCommands::Isolate { input, output } => {
            let audio_data = tokio::fs::read(input).await?;
            let filename = std::path::Path::new(input)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("audio.mp3");
            let request = elevenlabs_sdk::types::AudioIsolationRequest::default();
            let audio = client
                .audio_isolation()
                .isolate(&request, &audio_data, filename, "audio/mpeg")
                .await?;
            if let Some(path) = output {
                tokio::fs::write(path, &audio).await?;
                eprintln!("Isolated audio written to {path}");
            } else {
                use tokio::io::AsyncWriteExt;
                let mut stdout = tokio::io::stdout();
                stdout.write_all(&audio).await?;
            }
        }
    }
    Ok(())
}
