//! Forced alignment CLI subcommands.

use clap::{Args, Subcommand};

/// Forced alignment operations.
#[derive(Debug, Args)]
pub(crate) struct ForcedAlignmentArgs {
    #[command(subcommand)]
    pub command: ForcedAlignmentCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum ForcedAlignmentCommands {
    /// Create a forced alignment.
    Create {
        /// Path to the audio file.
        #[arg(long)]
        audio: String,

        /// Transcript text to align.
        #[arg(long)]
        text: String,
    },
}

/// Execute a forced-alignment subcommand.
pub(crate) async fn execute(args: &ForcedAlignmentArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        ForcedAlignmentCommands::Create { audio, text } => {
            let audio_data = tokio::fs::read(audio).await?;
            let filename = std::path::Path::new(audio)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("audio.mp3");
            let response = client.forced_alignment().create(&audio_data, filename, text).await?;
            crate::output::print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
