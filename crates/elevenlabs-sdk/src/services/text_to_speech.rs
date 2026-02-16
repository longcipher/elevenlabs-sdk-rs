//! Text-to-speech service providing access to TTS endpoints.
//!
//! This module wraps the four TTS endpoints exposed by the ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`convert`](TextToSpeechService::convert) | `POST /v1/text-to-speech/{voice_id}` | Full audio bytes |
//! | [`convert_with_timestamps`](TextToSpeechService::convert_with_timestamps) | `POST /v1/text-to-speech/{voice_id}/with-timestamps` | JSON with audio + alignment |
//! | [`convert_stream`](TextToSpeechService::convert_stream) | `POST /v1/text-to-speech/{voice_id}/stream` | Streaming audio bytes |
//! | [`convert_stream_with_timestamps`](TextToSpeechService::convert_stream_with_timestamps) | `POST /v1/text-to-speech/{voice_id}/stream/with-timestamps` | Streaming JSON chunks |
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{
//!     ClientConfig, ElevenLabsClient,
//!     types::{OutputFormat, TextToSpeechRequest},
//! };
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! let request = TextToSpeechRequest::new("Hello, world!");
//! let audio = client.text_to_speech().convert("voice_id", &request, None, None).await?;
//!
//! println!("Received {} bytes of audio", audio.len());
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;
use futures_core::Stream;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{AudioWithTimestampsResponse, OutputFormat, TextToSpeechRequest},
};

/// Text-to-speech service providing typed access to TTS endpoints.
///
/// Obtained via [`ElevenLabsClient::text_to_speech`].
#[derive(Debug)]
pub struct TextToSpeechService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> TextToSpeechService<'a> {
    /// Creates a new `TextToSpeechService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Builds the endpoint path with optional query parameters.
    ///
    /// The base path is `/v1/text-to-speech/{voice_id}` with an optional
    /// suffix (e.g. `/stream`, `/with-timestamps`).
    fn build_path(
        voice_id: &str,
        suffix: &str,
        output_format: Option<OutputFormat>,
        optimize_streaming_latency: Option<u8>,
    ) -> String {
        let mut path = format!("/v1/text-to-speech/{voice_id}{suffix}");

        let mut sep = '?';

        if let Some(fmt) = output_format {
            path.push(sep);
            path.push_str("output_format=");
            path.push_str(&fmt.to_string());
            sep = '&';
        }

        if let Some(latency) = optimize_streaming_latency {
            path.push(sep);
            path.push_str("optimize_streaming_latency=");
            path.push_str(&latency.to_string());
        }

        path
    }

    /// Converts text to speech, returning the full audio as raw bytes.
    ///
    /// Calls `POST /v1/text-to-speech/{voice_id}`.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID to use for synthesis.
    /// * `request` — The TTS request body (text, model, voice settings, etc.).
    /// * `output_format` — Optional output format (defaults to `mp3_44100_128`).
    /// * `optimize_streaming_latency` — Optional latency optimization level (0–4).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be read.
    pub async fn convert(
        &self,
        voice_id: &str,
        request: &TextToSpeechRequest,
        output_format: Option<OutputFormat>,
        optimize_streaming_latency: Option<u8>,
    ) -> Result<Bytes> {
        let path = Self::build_path(voice_id, "", output_format, optimize_streaming_latency);
        self.client.post_bytes(&path, request).await
    }

    /// Converts text to speech with character-level timestamp alignment.
    ///
    /// Calls `POST /v1/text-to-speech/{voice_id}/with-timestamps`.
    ///
    /// Returns an [`AudioWithTimestampsResponse`] containing base64-encoded
    /// audio and optional alignment data.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID to use for synthesis.
    /// * `request` — The TTS request body.
    /// * `output_format` — Optional output format.
    /// * `optimize_streaming_latency` — Optional latency optimization level (0–4).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn convert_with_timestamps(
        &self,
        voice_id: &str,
        request: &TextToSpeechRequest,
        output_format: Option<OutputFormat>,
        optimize_streaming_latency: Option<u8>,
    ) -> Result<AudioWithTimestampsResponse> {
        let path = Self::build_path(
            voice_id,
            "/with-timestamps",
            output_format,
            optimize_streaming_latency,
        );
        self.client.post(&path, request).await
    }

    /// Converts text to speech, returning a stream of audio byte chunks.
    ///
    /// Calls `POST /v1/text-to-speech/{voice_id}/stream`.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID to use for synthesis.
    /// * `request` — The TTS request body.
    /// * `output_format` — Optional output format.
    /// * `optimize_streaming_latency` — Optional latency optimization level (0–4).
    ///
    /// # Errors
    ///
    /// Returns an error if the initial API request fails. Individual stream
    /// items may also carry transport errors.
    pub async fn convert_stream(
        &self,
        voice_id: &str,
        request: &TextToSpeechRequest,
        output_format: Option<OutputFormat>,
        optimize_streaming_latency: Option<u8>,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>>> {
        let path = Self::build_path(voice_id, "/stream", output_format, optimize_streaming_latency);
        self.client.post_stream(&path, request).await
    }

    /// Converts text to speech with streaming and timestamp alignment.
    ///
    /// Calls `POST /v1/text-to-speech/{voice_id}/stream/with-timestamps`.
    ///
    /// Returns a stream of raw byte chunks. Each chunk is a JSON-encoded
    /// [`StreamingAudioChunkWithTimestamps`](crate::types::StreamingAudioChunkWithTimestamps)
    /// that callers should deserialize.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID to use for synthesis.
    /// * `request` — The TTS request body.
    /// * `output_format` — Optional output format.
    /// * `optimize_streaming_latency` — Optional latency optimization level (0–4).
    ///
    /// # Errors
    ///
    /// Returns an error if the initial API request fails.
    pub async fn convert_stream_with_timestamps(
        &self,
        voice_id: &str,
        request: &TextToSpeechRequest,
        output_format: Option<OutputFormat>,
        optimize_streaming_latency: Option<u8>,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>>> {
        let path = Self::build_path(
            voice_id,
            "/stream/with-timestamps",
            output_format,
            optimize_streaming_latency,
        );
        self.client.post_stream(&path, request).await
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
        matchers::{header, method, path, query_param},
    };

    use crate::{
        ElevenLabsClient,
        config::ClientConfig,
        types::{OutputFormat, TextToSpeechRequest},
    };

    // -- convert -----------------------------------------------------------

    #[tokio::test]
    async fn convert_returns_audio_bytes() {
        let mock_server = MockServer::start().await;
        let audio_bytes: &[u8] = b"\xff\xfb\x90\x00fake-mp3-data";

        Mock::given(method("POST"))
            .and(path("/v1/text-to-speech/voice123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(audio_bytes, "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = TextToSpeechRequest::new("Hello, world!");
        let result =
            client.text_to_speech().convert("voice123", &request, None, None).await.unwrap();

        assert_eq!(result.as_ref(), audio_bytes);
    }

    #[tokio::test]
    async fn convert_with_output_format_query_param() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-speech/voice123"))
            .and(query_param("output_format", "pcm_16000"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(b"pcm-data", "audio/pcm"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = TextToSpeechRequest::new("Hello");
        let result = client
            .text_to_speech()
            .convert("voice123", &request, Some(OutputFormat::Pcm_16000), None)
            .await
            .unwrap();

        assert_eq!(result.as_ref(), b"pcm-data");
    }

    #[tokio::test]
    async fn convert_with_optimize_streaming_latency() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-speech/voice123"))
            .and(query_param("optimize_streaming_latency", "3"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(b"audio", "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = TextToSpeechRequest::new("Hello");
        let result =
            client.text_to_speech().convert("voice123", &request, None, Some(3)).await.unwrap();

        assert_eq!(result.as_ref(), b"audio");
    }

    #[tokio::test]
    async fn convert_with_both_query_params() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-speech/voice123"))
            .and(query_param("output_format", "mp3_44100_192"))
            .and(query_param("optimize_streaming_latency", "2"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(b"audio", "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = TextToSpeechRequest::new("Hello");
        let result = client
            .text_to_speech()
            .convert("voice123", &request, Some(OutputFormat::Mp3_44100_192), Some(2))
            .await
            .unwrap();

        assert_eq!(result.as_ref(), b"audio");
    }

    // -- convert_with_timestamps -------------------------------------------

    #[tokio::test]
    async fn convert_with_timestamps_returns_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-speech/voice456/with-timestamps"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "audio_base64": "SGVsbG8=",
                "alignment": {
                    "characters": ["H","e","l","l","o"],
                    "character_start_times_seconds": [0.0,0.1,0.2,0.3,0.4],
                    "character_end_times_seconds": [0.1,0.2,0.3,0.4,0.5]
                },
                "normalized_alignment": null
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = TextToSpeechRequest::new("Hello");
        let result: crate::types::AudioWithTimestampsResponse = client
            .text_to_speech()
            .convert_with_timestamps("voice456", &request, None, None)
            .await
            .unwrap();

        assert_eq!(result.audio_base64, "SGVsbG8=");
        let alignment = result.alignment.unwrap();
        assert_eq!(alignment.characters, vec!["H", "e", "l", "l", "o"]);
        assert!(result.normalized_alignment.is_none());
    }

    #[tokio::test]
    async fn convert_with_timestamps_with_output_format() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-speech/voice456/with-timestamps"))
            .and(query_param("output_format", "pcm_24000"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "audio_base64": "cGNtZGF0YQ==",
                "alignment": null,
                "normalized_alignment": null
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = TextToSpeechRequest::new("Test");
        let result: crate::types::AudioWithTimestampsResponse = client
            .text_to_speech()
            .convert_with_timestamps("voice456", &request, Some(OutputFormat::Pcm_24000), None)
            .await
            .unwrap();

        assert_eq!(result.audio_base64, "cGNtZGF0YQ==");
    }

    // -- convert_stream ----------------------------------------------------

    #[tokio::test]
    async fn convert_stream_returns_stream() {
        use futures_core::Stream;

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-speech/voice789/stream"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(b"streaming-audio-data", "audio/mpeg"),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = TextToSpeechRequest::new("Stream me");
        let tts = client.text_to_speech();
        let stream = tts.convert_stream("voice789", &request, None, None).await.unwrap();

        // Verify we got a stream (type-level check).
        fn assert_stream<S: Stream>(_s: &S) {}
        assert_stream(&stream);
    }

    // -- convert_stream_with_timestamps ------------------------------------

    #[tokio::test]
    async fn convert_stream_with_timestamps_returns_stream() {
        use futures_core::Stream;

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-speech/voiceABC/stream/with-timestamps"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(b"streaming-json-chunks", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = TextToSpeechRequest::new("Timestamps");
        let tts = client.text_to_speech();
        let stream =
            tts.convert_stream_with_timestamps("voiceABC", &request, None, None).await.unwrap();

        fn assert_stream<S: Stream>(_s: &S) {}
        assert_stream(&stream);
    }

    // -- build_path --------------------------------------------------------

    #[test]
    fn build_path_no_params() {
        let path = super::TextToSpeechService::build_path("v123", "", None, None);
        assert_eq!(path, "/v1/text-to-speech/v123");
    }

    #[test]
    fn build_path_with_output_format() {
        let path = super::TextToSpeechService::build_path(
            "v123",
            "/stream",
            Some(OutputFormat::Pcm_16000),
            None,
        );
        assert_eq!(path, "/v1/text-to-speech/v123/stream?output_format=pcm_16000");
    }

    #[test]
    fn build_path_with_latency() {
        let path =
            super::TextToSpeechService::build_path("v123", "/with-timestamps", None, Some(4));
        assert_eq!(path, "/v1/text-to-speech/v123/with-timestamps?optimize_streaming_latency=4");
    }

    #[test]
    fn build_path_with_both_params() {
        let path = super::TextToSpeechService::build_path(
            "v123",
            "/stream/with-timestamps",
            Some(OutputFormat::Mp3_44100_128),
            Some(2),
        );
        assert_eq!(
            path,
            "/v1/text-to-speech/v123/stream/with-timestamps?output_format=mp3_44100_128&optimize_streaming_latency=2"
        );
    }
}
