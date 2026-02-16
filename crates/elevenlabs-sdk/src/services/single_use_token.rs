//! Single-use token service for generating one-time access tokens.
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{ClientConfig, ElevenLabsClient};
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! let token = client.single_use_token().create("tts").await?;
//! println!("Token: {}", token.token);
//! # Ok(())
//! # }
//! ```

use crate::{client::ElevenLabsClient, error::Result, types::SingleUseTokenResponse};

/// Single-use token service providing typed access to token generation.
///
/// Obtained via [`ElevenLabsClient::single_use_token`].
#[derive(Debug)]
pub struct SingleUseTokenService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> SingleUseTokenService<'a> {
    /// Creates a new `SingleUseTokenService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Creates a single-use token for the given token type.
    ///
    /// Calls `POST /v1/single-use-token/{token_type}`.
    ///
    /// # Arguments
    ///
    /// * `token_type` — The type of token to create (e.g. `"tts"`).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn create(&self, token_type: &str) -> Result<SingleUseTokenResponse> {
        let path = format!("/v1/single-use-token/{token_type}");
        self.client.post(&path, &serde_json::json!({})).await
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use crate::{ElevenLabsClient, config::ClientConfig};

    #[tokio::test]
    async fn create_returns_token() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/single-use-token/tts"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"token": "tok_abc123"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.single_use_token().create("tts").await.unwrap();
        assert_eq!(result.token, "tok_abc123");
    }
}
