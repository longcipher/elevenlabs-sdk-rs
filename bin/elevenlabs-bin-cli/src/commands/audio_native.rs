//! Audio native CLI subcommands.

use clap::{Args, Subcommand};

/// Audio native project operations.
#[derive(Debug, Args)]
pub(crate) struct AudioNativeArgs {
    #[command(subcommand)]
    pub command: AudioNativeCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum AudioNativeCommands {
    /// Create an audio native project.
    CreateProject {
        /// Name of the project.
        #[arg(long)]
        name: String,
    },

    /// Get audio native settings.
    GetSettings {
        /// Project ID to get settings for.
        #[arg(long)]
        project_id: String,
    },
}

/// Execute an audio-native subcommand.
pub(crate) async fn execute(args: &AudioNativeArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        AudioNativeCommands::CreateProject { name } => {
            let request = elevenlabs_sdk::types::AudioNativeCreateProjectRequest {
                name: name.clone(),
                ..Default::default()
            };
            let response = client.audio_native().create_project(&request, None).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        AudioNativeCommands::GetSettings { project_id } => {
            let response = client.audio_native().get_settings(project_id).await?;
            crate::output::print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
