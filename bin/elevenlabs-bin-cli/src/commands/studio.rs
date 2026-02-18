//! Studio CLI subcommands.

use clap::{Args, Subcommand};

/// Studio project operations.
#[derive(Debug, Args)]
pub(crate) struct StudioArgs {
    #[command(subcommand)]
    pub command: StudioCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum StudioCommands {
    /// List all studio projects.
    GetProjects,

    /// Get details about a studio project.
    GetProject {
        /// Project ID to retrieve.
        #[arg(long)]
        project_id: String,
    },

    /// Add a new studio project.
    AddProject {
        /// Name of the project.
        #[arg(long)]
        name: String,
    },

    /// Delete a studio project.
    DeleteProject {
        /// Project ID to delete.
        #[arg(long)]
        project_id: String,
    },
}

/// Execute a studio subcommand.
pub(crate) async fn execute(args: &StudioArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client = crate::context::build_client(cli)?;

    match &args.command {
        StudioCommands::GetProjects => {
            let response = client.studio().get_projects().await?;
            crate::output::print_json(&response, cli.format)?;
        }
        StudioCommands::GetProject { project_id } => {
            let response = client.studio().get_project(project_id).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        StudioCommands::AddProject { name } => {
            let request = elevenlabs_sdk::services::studio::AddProjectRequest {
                name: name.clone(),
                default_title_voice_id: None,
                default_paragraph_voice_id: None,
                default_model_id: None,
                from_url: None,
                quality_preset: None,
                title: None,
                author: None,
                description: None,
                volume_normalization: None,
                language: None,
                content_type: None,
                fiction: None,
                auto_convert: None,
            };
            let response = client.studio().add_project(&request, None).await?;
            crate::output::print_json(&response, cli.format)?;
        }
        StudioCommands::DeleteProject { project_id } => {
            let response = client.studio().delete_project(project_id).await?;
            crate::output::print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
