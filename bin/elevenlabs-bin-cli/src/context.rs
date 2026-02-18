//! Shared context helpers for CLI command execution.

use elevenlabs_sdk::{ClientConfig, ElevenLabsClient};

use crate::cli::Cli;

/// Build an [`ElevenLabsClient`] from CLI global options.
///
/// Uses `--api-key` / `ELEVENLABS_API_KEY` and optionally `--base-url` /
/// `ELEVENLABS_BASE_URL` to construct the SDK client.
///
/// # Errors
///
/// Returns an error if the API key is not provided or client construction fails.
pub(crate) fn build_client(cli: &Cli) -> eyre::Result<ElevenLabsClient> {
    let api_key = cli
        .api_key
        .as_deref()
        .ok_or_else(|| eyre::eyre!("API key required — set --api-key or ELEVENLABS_API_KEY"))?;

    let mut builder = ClientConfig::builder(api_key);

    if let Some(ref base_url) = cli.base_url {
        builder = builder.base_url(base_url);
    }

    let config = builder.build();
    let client = ElevenLabsClient::new(config)?;
    Ok(client)
}
