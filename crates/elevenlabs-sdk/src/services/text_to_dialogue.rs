//! Text-to-dialogue service providing access to multi-voice dialogue
//! generation endpoints.
//!
//! This module wraps the four text-to-dialogue endpoints exposed by the
//! ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`convert`](TextToDialogueService::convert) | `POST /v1/text-to-dialogue` | Full audio bytes |
//! | [`convert_stream`](TextToDialogueService::convert_stream) | `POST /v1/text-to-dialogue/stream` | Streaming audio bytes |
//! | [`convert_with_timestamps`](TextToDialogueService::convert_with_timestamps) | `POST /v1/text-to-dialogue/with-timestamps` | JSON with audio + alignment + voice segments |
//! | [`convert_stream_with_timestamps`](TextToDialogueService::convert_stream_with_timestamps) | `POST /v1/text-to-dialogue/stream/with-timestamps` | Streaming JSON chunks with timestamps |
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{
//!     ClientConfig, ElevenLabsClient,
//!     types::{DialogueInput, TextToDialogueRequest},
//! };
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! let request = TextToDialogueRequest {
//!     inputs: vec![
//!         DialogueInput { text: "Hello!".into(), voice_id: "voice1".into() },
//!         DialogueInput { text: "Hi there!".into(), voice_id: "voice2".into() },
//!     ],
//!     ..Default::default()
//! };
//! let audio = client.text_to_dialogue().convert(&request).await?;
//!
//! println!("Received {} bytes of dialogue audio", audio.len());
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;
use futures_core::Stream;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{AudioWithTimestampsAndVoiceSegmentsResponse, TextToDialogueRequest},
};

/// Text-to-dialogue service providing typed access to multi-voice dialogue
/// generation endpoints.
///
/// Obtained via [`ElevenLabsClient::text_to_dialogue`].
#[derive(Debug)]
pub struct TextToDialogueService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> TextToDialogueService<'a> {
    /// Creates a new `TextToDialogueService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Converts multi-voice dialogue to speech, returning the full audio as
    /// raw bytes.
    ///
    /// Calls `POST /v1/text-to-dialogue` with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `request` — The dialogue request body with input lines, model, etc.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// read.
    pub async fn convert(&self, request: &TextToDialogueRequest) -> Result<Bytes> {
        self.client.post_bytes("/v1/text-to-dialogue", request).await
    }

    /// Converts multi-voice dialogue to speech, returning a stream of audio
    /// byte chunks.
    ///
    /// Calls `POST /v1/text-to-dialogue/stream` with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `request` — The dialogue request body.
    ///
    /// # Errors
    ///
    /// Returns an error if the initial API request fails. Individual stream
    /// items may also carry transport errors.
    pub async fn convert_stream(
        &self,
        request: &TextToDialogueRequest,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>>> {
        self.client.post_stream("/v1/text-to-dialogue/stream", request).await
    }

    /// Converts multi-voice dialogue to speech with character-level timestamp
    /// alignment and voice segment information.
    ///
    /// Calls `POST /v1/text-to-dialogue/with-timestamps` with a JSON body.
    ///
    /// Returns an [`AudioWithTimestampsAndVoiceSegmentsResponse`] containing
    /// base64-encoded audio, optional alignment data, and voice segments.
    ///
    /// # Arguments
    ///
    /// * `request` — The dialogue request body.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn convert_with_timestamps(
        &self,
        request: &TextToDialogueRequest,
    ) -> Result<AudioWithTimestampsAndVoiceSegmentsResponse> {
        self.client.post("/v1/text-to-dialogue/with-timestamps", request).await
    }

    /// Converts multi-voice dialogue to speech with streaming and timestamp
    /// alignment.
    ///
    /// Calls `POST /v1/text-to-dialogue/stream/with-timestamps` with a JSON
    /// body.
    ///
    /// Returns a stream of raw byte chunks. Each chunk is a JSON-encoded
    /// [`StreamingAudioChunkWithTimestampsAndVoiceSegments`](crate::types::StreamingAudioChunkWithTimestampsAndVoiceSegments)
    /// that callers should deserialize.
    ///
    /// # Arguments
    ///
    /// * `request` — The dialogue request body.
    ///
    /// # Errors
    ///
    /// Returns an error if the initial API request fails.
    pub async fn convert_stream_with_timestamps(
        &self,
        request: &TextToDialogueRequest,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>>> {
        self.client.post_stream("/v1/text-to-dialogue/stream/with-timestamps", request).await
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

    use crate::{
        ElevenLabsClient,
        config::ClientConfig,
        types::{DialogueInput, TextToDialogueRequest},
    };

    fn sample_request() -> TextToDialogueRequest {
        TextToDialogueRequest {
            inputs: vec![
                DialogueInput { text: "Hello!".into(), voice_id: "voice1".into() },
                DialogueInput { text: "Hi there!".into(), voice_id: "voice2".into() },
            ],
            ..Default::default()
        }
    }

    // -- convert -----------------------------------------------------------

    #[tokio::test]
    async fn convert_returns_audio_bytes() {
        let mock_server = MockServer::start().await;
        let audio_bytes: &[u8] = b"\xff\xfb\x90\x00fake-dialogue-audio";

        Mock::given(method("POST"))
            .and(path("/v1/text-to-dialogue"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(audio_bytes, "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.text_to_dialogue().convert(&sample_request()).await.unwrap();

        assert_eq!(result.as_ref(), audio_bytes);
    }

    // -- convert_stream ----------------------------------------------------

    #[tokio::test]
    async fn convert_stream_returns_stream() {
        use futures_core::Stream;

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-dialogue/stream"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(b"\xff\xfb\x90\x00streamed-dialogue", "audio/mpeg"),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = sample_request();
        let svc = client.text_to_dialogue();
        let stream = svc.convert_stream(&request).await.unwrap();

        // Verify we got a stream (type-level check).
        fn assert_stream<S: Stream>(_s: &S) {}
        assert_stream(&stream);
    }

    // -- convert_with_timestamps -------------------------------------------

    #[tokio::test]
    async fn convert_with_timestamps_returns_response() {
        let mock_server = MockServer::start().await;

        let response_json = serde_json::json!({
            "audio_base64": "SGVsbG8=",
            "alignment": {
                "characters": ["H", "e", "l", "l", "o"],
                "character_start_times_seconds": [0.0, 0.1, 0.2, 0.3, 0.4],
                "character_end_times_seconds": [0.1, 0.2, 0.3, 0.4, 0.5]
            },
            "normalized_alignment": null,
            "voice_segments": [
                {
                    "voice_id": "voice1",
                    "start_time_seconds": 0.0,
                    "end_time_seconds": 0.5,
                    "character_start_index": 0,
                    "character_end_index": 5,
                    "dialogue_input_index": 0
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/v1/text-to-dialogue/with-timestamps"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result =
            client.text_to_dialogue().convert_with_timestamps(&sample_request()).await.unwrap();

        assert_eq!(result.audio_base64, "SGVsbG8=");
        assert!(result.alignment.is_some());
        assert_eq!(result.voice_segments.len(), 1);
        assert_eq!(result.voice_segments[0].voice_id, "voice1");
        assert_eq!(result.voice_segments[0].dialogue_input_index, 0);
    }

    // -- error handling ----------------------------------------------------

    #[tokio::test]
    async fn convert_handles_api_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-dialogue"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "detail": "Invalid request: inputs is required"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.text_to_dialogue().convert(&TextToDialogueRequest::default()).await;

        assert!(result.is_err());
    }
}
