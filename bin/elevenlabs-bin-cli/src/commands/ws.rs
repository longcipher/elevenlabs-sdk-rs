//! WebSocket CLI subcommands.

use clap::{Args, Subcommand};

/// WebSocket operations (TTS streaming, Conversational AI).
#[derive(Debug, Args)]
pub(crate) struct WsArgs {
    #[command(subcommand)]
    pub command: WsCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum WsCommands {
    /// Stream text-to-speech over WebSocket.
    Tts {
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

    /// Start a conversational AI WebSocket session.
    Conversation {
        /// Agent ID to connect to.
        #[arg(long)]
        agent_id: String,
    },
}

/// Execute a WebSocket subcommand.
pub(crate) async fn execute(args: &WsArgs, cli: &crate::cli::Cli) -> eyre::Result<()> {
    let client_config = {
        let api_key = cli
            .api_key
            .as_deref()
            .ok_or_else(|| eyre::eyre!("API key required — set --api-key or ELEVENLABS_API_KEY"))?;
        let mut builder = elevenlabs_sdk::ClientConfig::builder(api_key);
        if let Some(ref base_url) = cli.base_url {
            builder = builder.base_url(base_url);
        }
        builder.build()
    };

    match &args.command {
        WsCommands::Tts { voice_id, text, model_id, output } => {
            let ws_config = elevenlabs_sdk::TtsWsConfig {
                voice_id: voice_id.clone(),
                model_id: model_id.clone().unwrap_or_else(|| "eleven_turbo_v2".into()),
                voice_settings: None,
                generation_config: None,
                output_format: None,
            };
            let mut ws = elevenlabs_sdk::TtsWebSocket::connect(&client_config, &ws_config).await?;
            ws.send_text(text).await?;
            ws.flush().await?;

            let mut audio_buf = Vec::new();
            while let Some(resp) = ws.recv().await? {
                if let Some(ref b64) = resp.audio {
                    use base64::Engine;
                    if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(b64) {
                        audio_buf.extend_from_slice(&decoded);
                    }
                }
                if resp.is_final == Some(true) {
                    break;
                }
            }
            ws.close().await?;

            if let Some(path) = output {
                tokio::fs::write(path, &audio_buf).await?;
                eprintln!("Audio written to {path}");
            } else {
                use tokio::io::AsyncWriteExt;
                let mut stdout = tokio::io::stdout();
                stdout.write_all(&audio_buf).await?;
            }
        }
        WsCommands::Conversation { agent_id } => {
            eprintln!("Starting conversation with agent {agent_id}...");
            eprintln!(
                "Conversational AI WebSocket requires audio I/O — use the SDK directly for full interactive sessions."
            );
        }
    }
    Ok(())
}
