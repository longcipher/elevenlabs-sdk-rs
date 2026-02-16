//! History CLI subcommands.

use clap::{Args, Subcommand};

/// History operations.
#[derive(Debug, Args)]
pub(crate) struct HistoryArgs {
    #[command(subcommand)]
    pub command: HistoryCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum HistoryCommands {
    /// List history items.
    List,

    /// Get a specific history item.
    Get {
        /// History item ID.
        #[arg(long)]
        history_item_id: String,
    },

    /// Get audio for a history item.
    GetAudio {
        /// History item ID.
        #[arg(long)]
        history_item_id: String,

        /// Output file path for the audio.
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Delete a history item.
    Delete {
        /// History item ID.
        #[arg(long)]
        history_item_id: String,
    },
}

/// Execute a history subcommand.
pub(crate) async fn execute(args: &HistoryArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        HistoryCommands::List => {
            let response = client.history().list(None, None, None).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        HistoryCommands::Get { history_item_id } => {
            let response = client.history().get(history_item_id).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        HistoryCommands::GetAudio { history_item_id, output } => {
            let audio = client.history().get_audio(history_item_id).await?;
            if let Some(path) = output {
                tokio::fs::write(path, &audio).await?;
                eprintln!("Audio written to {path}");
            } else {
                use tokio::io::AsyncWriteExt;
                let mut stdout = tokio::io::stdout();
                stdout.write_all(&audio).await?;
            }
        }
        HistoryCommands::Delete { history_item_id } => {
            let response = client.history().delete(history_item_id).await?;
            crate::output::print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
