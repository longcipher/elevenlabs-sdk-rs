//! Text-to-dialogue CLI subcommands.

use clap::{Args, Subcommand};

/// Text-to-dialogue conversion operations.
#[derive(Debug, Args)]
pub(crate) struct TextToDialogueArgs {
    #[command(subcommand)]
    pub command: TextToDialogueCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum TextToDialogueCommands {
    /// Convert text to dialogue audio.
    Convert {
        /// Text to convert to dialogue.
        #[arg(long)]
        text: String,

        /// Output file path for the audio.
        #[arg(short, long)]
        output: Option<String>,
    },
}

/// Execute a text-to-dialogue subcommand.
pub(crate) async fn execute(args: &TextToDialogueArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        TextToDialogueCommands::Convert { text, output } => {
            let request = elevenlabs_sdk::types::TextToDialogueRequest {
                inputs: vec![elevenlabs_sdk::types::DialogueInput {
                    text: text.clone(),
                    voice_id: String::new(),
                }],
                ..Default::default()
            };
            let audio = client.text_to_dialogue().convert(&request).await?;
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
