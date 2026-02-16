//! Sound generation service providing access to the sound-effect endpoint.
//!
//! This module wraps the single sound-generation endpoint exposed by the
//! ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`generate`](SoundGenerationService::generate) | `POST /v1/sound-generation` | Generate a sound effect from text |
//!
//! The response is raw audio bytes (`audio/mpeg`).
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, types::SoundGenerationRequest};
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! let request = SoundGenerationRequest {
//!     text: "A large, ancient wooden door slowly opening.".into(),
//!     ..Default::default()
//! };
//! let audio = client.sound_generation().generate(&request).await?;
//!
//! println!("Received {} bytes of audio", audio.len());
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;

use crate::{client::ElevenLabsClient, error::Result, types::SoundGenerationRequest};

/// Sound generation service providing typed access to the sound-effect
/// endpoint.
///
/// Obtained via [`ElevenLabsClient::sound_generation`].
#[derive(Debug)]
pub struct SoundGenerationService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> SoundGenerationService<'a> {
    /// Creates a new `SoundGenerationService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Generates a sound effect from a text description, returning the full
    /// audio as raw bytes.
    ///
    /// Calls `POST /v1/sound-generation` with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `request` — The sound generation request with text prompt, duration, model, etc.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// read.
    pub async fn generate(&self, request: &SoundGenerationRequest) -> Result<Bytes> {
        self.client.post_bytes("/v1/sound-generation", request).await
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

    use crate::{ElevenLabsClient, config::ClientConfig, types::SoundGenerationRequest};

    #[tokio::test]
    async fn generate_returns_audio_bytes() {
        let mock_server = MockServer::start().await;
        let audio_bytes: &[u8] = b"\xff\xfb\x90\x00fake-sound-effect";

        Mock::given(method("POST"))
            .and(path("/v1/sound-generation"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(audio_bytes, "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request =
            SoundGenerationRequest { text: "Thunder rolling".into(), ..Default::default() };
        let result = client.sound_generation().generate(&request).await.unwrap();

        assert_eq!(result.as_ref(), audio_bytes);
    }

    #[tokio::test]
    async fn generate_with_custom_params() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/sound-generation"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(b"custom-sfx", "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = SoundGenerationRequest {
            text: "Rain on a tin roof".into(),
            r#loop: true,
            duration_seconds: Some(10.0),
            prompt_influence: 0.8,
            ..Default::default()
        };
        let result = client.sound_generation().generate(&request).await.unwrap();

        assert_eq!(result.as_ref(), b"custom-sfx");
    }

    #[tokio::test]
    async fn generate_handles_api_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/sound-generation"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "detail": "Invalid request: text is required"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = SoundGenerationRequest::default();
        let result = client.sound_generation().generate(&request).await;

        assert!(result.is_err());
    }
}
