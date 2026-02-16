//! Models service providing access to the ElevenLabs models endpoint.
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`list`](ModelsService::list) | `GET /v1/models` | List available models |
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
//! let models = client.models().list().await?;
//! println!("Found {} models", models.0.len());
//! # Ok(())
//! # }
//! ```

use crate::{client::ElevenLabsClient, error::Result, types::GetModelsResponse};

/// Models service providing typed access to model listing endpoints.
///
/// Obtained via [`ElevenLabsClient::models`].
#[derive(Debug)]
pub struct ModelsService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> ModelsService<'a> {
    /// Creates a new `ModelsService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Lists all available models.
    ///
    /// Calls `GET /v1/models`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn list(&self) -> Result<GetModelsResponse> {
        self.client.get("/v1/models").await
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
    async fn list_returns_models() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/models"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "model_id": "eleven_multilingual_v2",
                    "name": "Multilingual v2",
                    "can_be_finetuned": true,
                    "can_do_text_to_speech": true,
                    "can_do_voice_conversion": true,
                    "can_use_style": true,
                    "can_use_speaker_boost": true,
                    "serves_pro_voices": false,
                    "token_cost_factor": 1.0,
                    "description": "State of the art.",
                    "requires_alpha_access": false,
                    "max_characters_request_free_user": 2500,
                    "max_characters_request_subscribed_user": 5000,
                    "maximum_text_length_per_request": 1000000,
                    "languages": [{"language_id": "en", "name": "English"}],
                    "model_rates": {"character_cost_multiplier": 1.0},
                    "concurrency_group": "standard"
                }
            ])))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.models().list().await.unwrap();
        assert_eq!(result.0.len(), 1);
        assert_eq!(result.0[0].model_id, "eleven_multilingual_v2");
    }

    #[tokio::test]
    async fn list_returns_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/models"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.models().list().await.unwrap();
        assert!(result.0.is_empty());
    }
}
