//! Agents CLI subcommands.

use clap::{Args, Subcommand};

use crate::{cli::Cli, context::build_client, output::print_json};

/// Conversational AI agent operations.
#[derive(Debug, Args)]
pub(crate) struct AgentsArgs {
    #[command(subcommand)]
    pub command: AgentsCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum AgentsCommands {
    /// List all agents.
    List,

    /// Get details about a specific agent.
    Get {
        /// Agent ID to retrieve.
        #[arg(long)]
        agent_id: String,
    },

    /// Create a new agent.
    Create {
        /// Name of the agent.
        #[arg(long)]
        name: String,
    },

    /// Delete an agent.
    Delete {
        /// Agent ID to delete.
        #[arg(long)]
        agent_id: String,
    },

    /// List conversations for an agent.
    ListConversations {
        /// Agent ID to list conversations for.
        #[arg(long)]
        agent_id: String,
    },

    /// Get a specific conversation.
    GetConversation {
        /// Conversation ID to retrieve.
        #[arg(long)]
        conversation_id: String,
    },
}

/// Execute an agents subcommand.
pub(crate) async fn execute(args: &AgentsArgs, cli: &Cli) -> eyre::Result<()> {
    let client = build_client(cli)?;

    match &args.command {
        AgentsCommands::List => {
            let response = client.agents().list_agents(None).await?;
            print_json(&response, cli.format)?;
        }
        AgentsCommands::Get { agent_id } => {
            let response = client.agents().get_agent(agent_id).await?;
            print_json(&response, cli.format)?;
        }
        AgentsCommands::Create { name } => {
            let request = elevenlabs_sdk::types::CreateAgentRequest {
                name: Some(name.clone()),
                ..Default::default()
            };
            let response = client.agents().create_agent(&request).await?;
            print_json(&response, cli.format)?;
        }
        AgentsCommands::Delete { agent_id } => {
            client.agents().delete_agent(agent_id).await?;
            eprintln!("Agent {agent_id} deleted");
        }
        AgentsCommands::ListConversations { agent_id } => {
            let response = client.agents().list_conversations(Some(agent_id), None).await?;
            print_json(&response, cli.format)?;
        }
        AgentsCommands::GetConversation { conversation_id } => {
            let response = client.agents().get_conversation(conversation_id).await?;
            print_json(&response, cli.format)?;
        }
    }
    Ok(())
}
