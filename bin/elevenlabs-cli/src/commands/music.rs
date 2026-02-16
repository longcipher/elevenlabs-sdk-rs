//! Music CLI subcommands.

use clap::{Args, Subcommand};

/// Music generation operations.
#[derive(Debug, Args)]
pub(crate) struct MusicArgs {
    #[command(subcommand)]
    pub command: MusicCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum MusicCommands {
    /// Plan a music generation.
    Plan {
        /// Prompt describing the desired music.
        #[arg(long)]
        prompt: String,
    },

    /// Compose music from a plan.
    Compose {
        /// Prompt describing the desired music.
        #[arg(long)]
        prompt: String,

        /// Output file path for the audio.
        #[arg(short, long)]
        output: Option<String>,
    },
}

/// Execute a music subcommand.
pub(crate) async fn execute(args: &MusicArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        MusicCommands::Plan { prompt } => {
            let request = elevenlabs_sdk::types::MusicPlanRequest {
                prompt: prompt.clone(),
                ..Default::default()
            };
            let response = client.music().plan(&request).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        MusicCommands::Compose { prompt, output } => {
            let request = elevenlabs_sdk::types::MusicComposeRequest {
                prompt: Some(prompt.clone()),
                ..Default::default()
            };
            let audio = client.music().compose(&request).await?;
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
