//! Forced alignment service for aligning text to audio.
//!
//! Provides a multipart endpoint that takes an audio file and text input,
//! returning character-level alignment data.
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
//! let audio = std::fs::read("sample.mp3").unwrap();
//! let alignment = client.forced_alignment().create(&audio, "sample.mp3", "Hello world").await?;
//! println!("Aligned {} characters", alignment.characters.len());
//! # Ok(())
//! # }
//! ```

use super::voices::{append_file_part, append_text_field, uuid_v4_simple};
use crate::{client::ElevenLabsClient, error::Result, types::ForcedAlignmentResponse};

/// Forced alignment service providing typed access to alignment endpoints.
///
/// Obtained via [`ElevenLabsClient::forced_alignment`].
#[derive(Debug)]
pub struct ForcedAlignmentService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> ForcedAlignmentService<'a> {
    /// Creates a new `ForcedAlignmentService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Aligns text to an audio file and returns character-level alignment data.
    ///
    /// Calls `POST /v1/forced-alignment` with a multipart request containing
    /// the audio file and the text to align.
    ///
    /// # Arguments
    ///
    /// * `audio_data` — Raw bytes of the audio file.
    /// * `file_name` — File name for the audio (e.g. `"audio.mp3"`).
    /// * `text` — The text to align against the audio.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn create(
        &self,
        audio_data: &[u8],
        file_name: &str,
        text: &str,
    ) -> Result<ForcedAlignmentResponse> {
        let boundary = uuid_v4_simple();
        let mut body = Vec::new();

        append_file_part(
            &mut body,
            &boundary,
            "file",
            file_name,
            "application/octet-stream",
            audio_data,
        );
        append_text_field(&mut body, &boundary, "text", text);

        // Close the multipart body.
        body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

        let content_type = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart("/v1/forced-alignment", body, &content_type).await
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
    async fn create_returns_alignment() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/forced-alignment"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "characters": [
                    {"text": "H", "start": 0.0, "end": 0.1},
                    {"text": "e", "start": 0.1, "end": 0.2},
                    {"text": "l", "start": 0.2, "end": 0.3}
                ],
                "words": [
                    {"text": "Hel", "start": 0.0, "end": 0.3, "loss": 0.1}
                ],
                "loss": 0.5
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let audio = b"fake-audio-data";
        let result = client.forced_alignment().create(audio, "test.mp3", "Hello").await.unwrap();

        assert_eq!(result.characters.len(), 3);
    }
}
