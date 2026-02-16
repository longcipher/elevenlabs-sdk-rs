//! Sound generation CLI subcommands.

use clap::{Args, Subcommand};

/// Sound effect generation operations.
#[derive(Debug, Args)]
pub(crate) struct SoundGenerationArgs {
    #[command(subcommand)]
    pub command: SoundGenerationCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum SoundGenerationCommands {
    /// Generate a sound effect.
    Generate {
        /// Prompt describing the desired sound.
        #[arg(long)]
        text: String,

        /// Duration of the sound in seconds.
        #[arg(long)]
        duration_seconds: Option<f64>,

        /// Output file path for the audio.
        #[arg(short, long)]
        output: Option<String>,
    },
}

/// Execute a sound-generation subcommand.
pub(crate) async fn execute(args: &SoundGenerationArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        SoundGenerationCommands::Generate { text, duration_seconds, output } => {
            let request = elevenlabs_sdk::types::SoundGenerationRequest {
                text: text.clone(),
                duration_seconds: *duration_seconds,
                ..Default::default()
            };
            let audio = client.sound_generation().generate(&request).await?;
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
