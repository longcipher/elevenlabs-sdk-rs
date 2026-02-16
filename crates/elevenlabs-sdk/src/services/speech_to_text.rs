//! Speech-to-text service providing access to STT endpoints.
//!
//! This module wraps the three STT endpoints exposed by the ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`transcribe`](SpeechToTextService::transcribe) | `POST /v1/speech-to-text` | Transcribe audio |
//! | [`get_transcript`](SpeechToTextService::get_transcript) | `GET /v1/speech-to-text/transcripts/{transcription_id}` | Retrieve a transcript |
//! | [`delete_transcript`](SpeechToTextService::delete_transcript) | `DELETE /v1/speech-to-text/transcripts/{transcription_id}` | Delete a transcript |
//!
//! The transcription endpoint accepts `multipart/form-data` with an audio
//! file (or a `cloud_storage_url`) and configuration fields.
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, types::SpeechToTextRequest};
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! let request = SpeechToTextRequest::default();
//! let transcript = client
//!     .speech_to_text()
//!     .transcribe(&request, Some((b"audio-data", "recording.mp3", "audio/mpeg")))
//!     .await?;
//!
//! println!("Transcript: {}", transcript.text);
//! # Ok(())
//! # }
//! ```

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{SpeechToTextChunkResponse, SpeechToTextRequest},
};

/// Speech-to-text service providing typed access to STT endpoints.
///
/// Obtained via [`ElevenLabsClient::speech_to_text`].
#[derive(Debug)]
pub struct SpeechToTextService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> SpeechToTextService<'a> {
    /// Creates a new `SpeechToTextService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Transcribes audio to text.
    ///
    /// Calls `POST /v1/speech-to-text` with `multipart/form-data`.
    ///
    /// Either a file (via `audio_file`) or a `cloud_storage_url` in the
    /// request must be provided, but not both.
    ///
    /// # Arguments
    ///
    /// * `request` — Configuration fields (model, language, diarization, etc.).
    /// * `audio_file` — Optional audio file as `(data, filename, content_type)`. Required when
    ///   `cloud_storage_url` is `None`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn transcribe(
        &self,
        request: &SpeechToTextRequest,
        audio_file: Option<(&[u8], &str, &str)>,
    ) -> Result<SpeechToTextChunkResponse> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_stt_multipart(&boundary, request, audio_file);
        let content_type = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart("/v1/speech-to-text", body, &content_type).await
    }

    /// Retrieves a previously created transcript.
    ///
    /// Calls `GET /v1/speech-to-text/transcripts/{transcription_id}`.
    ///
    /// # Arguments
    ///
    /// * `transcription_id` — The transcription ID to retrieve.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_transcript(
        &self,
        transcription_id: &str,
    ) -> Result<SpeechToTextChunkResponse> {
        let path = format!("/v1/speech-to-text/transcripts/{transcription_id}");
        self.client.get(&path).await
    }

    /// Deletes a transcript.
    ///
    /// Calls `DELETE /v1/speech-to-text/transcripts/{transcription_id}`.
    ///
    /// # Arguments
    ///
    /// * `transcription_id` — The transcription ID to delete.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete_transcript(&self, transcription_id: &str) -> Result<()> {
        let path = format!("/v1/speech-to-text/transcripts/{transcription_id}");
        self.client.delete(&path).await
    }
}

// ---------------------------------------------------------------------------
// Multipart helpers
// ---------------------------------------------------------------------------

/// Generates a simple pseudo-random hex string for multipart boundaries.
fn uuid_v4_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    format!("{nanos:032x}")
}

/// Appends a text field to a multipart body buffer.
fn append_text_field(buf: &mut Vec<u8>, boundary: &str, name: &str, value: &str) {
    buf.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    buf.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").as_bytes(),
    );
    buf.extend_from_slice(value.as_bytes());
    buf.extend_from_slice(b"\r\n");
}

/// Appends a file part to a multipart body buffer.
fn append_file_part(
    buf: &mut Vec<u8>,
    boundary: &str,
    field_name: &str,
    filename: &str,
    content_type: &str,
    data: &[u8],
) {
    buf.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    buf.extend_from_slice(
        format!(
            "Content-Disposition: form-data; name=\"{field_name}\"; filename=\"{filename}\"\r\n"
        )
        .as_bytes(),
    );
    buf.extend_from_slice(format!("Content-Type: {content_type}\r\n\r\n").as_bytes());
    buf.extend_from_slice(data);
    buf.extend_from_slice(b"\r\n");
}

/// Serializes a serde enum to its plain string value (strips surrounding
/// JSON quotes).
fn enum_to_str<T: serde::Serialize>(val: &T) -> Option<String> {
    serde_json::to_string(val).ok().map(|s| s.trim_matches('"').to_owned())
}

/// Builds the multipart body for `POST /v1/speech-to-text`.
fn build_stt_multipart(
    boundary: &str,
    request: &SpeechToTextRequest,
    audio_file: Option<(&[u8], &str, &str)>,
) -> Vec<u8> {
    let mut buf = Vec::new();

    // Audio file (optional — may use cloud_storage_url instead)
    if let Some((data, filename, content_type)) = audio_file {
        append_file_part(&mut buf, boundary, "file", filename, content_type, data);
    }

    // model_id
    if let Some(model_str) = enum_to_str(&request.model_id) {
        append_text_field(&mut buf, boundary, "model_id", &model_str);
    }

    // language_code
    if let Some(ref lang) = request.language_code {
        append_text_field(&mut buf, boundary, "language_code", lang);
    }

    // tag_audio_events
    append_text_field(
        &mut buf,
        boundary,
        "tag_audio_events",
        if request.tag_audio_events { "true" } else { "false" },
    );

    // num_speakers
    if let Some(n) = request.num_speakers {
        append_text_field(&mut buf, boundary, "num_speakers", &n.to_string());
    }

    // timestamps_granularity
    if let Some(ts_str) = enum_to_str(&request.timestamps_granularity) {
        append_text_field(&mut buf, boundary, "timestamps_granularity", &ts_str);
    }

    // diarize
    append_text_field(
        &mut buf,
        boundary,
        "diarize",
        if request.diarize { "true" } else { "false" },
    );

    // diarization_threshold
    if let Some(threshold) = request.diarization_threshold {
        append_text_field(&mut buf, boundary, "diarization_threshold", &threshold.to_string());
    }

    // additional_formats (JSON array)
    if let Some(ref fmts) = request.additional_formats &&
        let Ok(json) = serde_json::to_string(fmts)
    {
        append_text_field(&mut buf, boundary, "additional_formats", &json);
    }

    // file_format
    if let Some(ref ff) = request.file_format &&
        let Some(ff_str) = enum_to_str(ff)
    {
        append_text_field(&mut buf, boundary, "file_format", &ff_str);
    }

    // cloud_storage_url
    if let Some(ref url) = request.cloud_storage_url {
        append_text_field(&mut buf, boundary, "cloud_storage_url", url);
    }

    // webhook
    append_text_field(
        &mut buf,
        boundary,
        "webhook",
        if request.webhook { "true" } else { "false" },
    );

    // webhook_id
    if let Some(ref wh_id) = request.webhook_id {
        append_text_field(&mut buf, boundary, "webhook_id", wh_id);
    }

    // temperature
    if let Some(temp) = request.temperature {
        append_text_field(&mut buf, boundary, "temperature", &temp.to_string());
    }

    // seed
    if let Some(seed) = request.seed {
        append_text_field(&mut buf, boundary, "seed", &seed.to_string());
    }

    // use_multi_channel
    append_text_field(
        &mut buf,
        boundary,
        "use_multi_channel",
        if request.use_multi_channel { "true" } else { "false" },
    );

    // webhook_metadata
    if let Some(ref meta) = request.webhook_metadata {
        append_text_field(&mut buf, boundary, "webhook_metadata", meta);
    }

    // entity_detection (JSON array)
    if let Some(ref entities) = request.entity_detection &&
        let Ok(json) = serde_json::to_string(entities)
    {
        append_text_field(&mut buf, boundary, "entity_detection", &json);
    }

    // keyterms (JSON array)
    if let Some(ref terms) = request.keyterms &&
        let Ok(json) = serde_json::to_string(terms)
    {
        append_text_field(&mut buf, boundary, "keyterms", &json);
    }

    buf.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    buf
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

    use crate::{ElevenLabsClient, config::ClientConfig, types::SpeechToTextRequest};

    // -- transcribe --------------------------------------------------------

    #[tokio::test]
    async fn transcribe_returns_chunk_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/speech-to-text"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "language_code": "eng",
                "language_probability": 0.98,
                "text": "Hello world!",
                "words": [
                    {"text": "Hello", "start": 0.0, "end": 0.5, "type": "word", "logprob": -0.124},
                    {"text": " ", "start": 0.5, "end": 0.5, "type": "spacing", "logprob": 0.0},
                    {"text": "world!", "start": 0.5, "end": 1.2, "type": "word", "logprob": -0.089}
                ]
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = SpeechToTextRequest::default();
        let result = client
            .speech_to_text()
            .transcribe(&request, Some((b"fake-audio", "audio.mp3", "audio/mpeg")))
            .await
            .unwrap();

        assert_eq!(result.text, "Hello world!");
        assert_eq!(result.language_code, "eng");
        assert_eq!(result.words.len(), 3);
    }

    #[tokio::test]
    async fn transcribe_with_cloud_storage_url() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/speech-to-text"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "language_code": "eng",
                "language_probability": 0.99,
                "text": "From the cloud.",
                "words": [
                    {"text": "From", "start": 0.0, "end": 0.3, "type": "word", "logprob": -0.05},
                    {"text": " ", "start": 0.3, "end": 0.3, "type": "spacing", "logprob": 0.0},
                    {"text": "the", "start": 0.3, "end": 0.5, "type": "word", "logprob": -0.03},
                    {"text": " ", "start": 0.5, "end": 0.5, "type": "spacing", "logprob": 0.0},
                    {"text": "cloud.", "start": 0.5, "end": 1.0, "type": "word", "logprob": -0.08}
                ]
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = SpeechToTextRequest {
            cloud_storage_url: Some("https://example.com/audio.mp3".into()),
            ..SpeechToTextRequest::default()
        };
        let result = client.speech_to_text().transcribe(&request, None).await.unwrap();

        assert_eq!(result.text, "From the cloud.");
    }

    #[tokio::test]
    async fn transcribe_with_diarization() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/speech-to-text"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "language_code": "eng",
                "language_probability": 0.95,
                "text": "Hi there.",
                "words": [
                    {
                        "text": "Hi",
                        "start": 0.0,
                        "end": 0.3,
                        "type": "word",
                        "logprob": -0.1,
                        "speaker_id": "speaker_0"
                    },
                    {"text": " ", "start": 0.3, "end": 0.3, "type": "spacing", "logprob": 0.0},
                    {
                        "text": "there.",
                        "start": 0.3,
                        "end": 0.8,
                        "type": "word",
                        "logprob": -0.07,
                        "speaker_id": "speaker_1"
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = SpeechToTextRequest {
            diarize: true,
            num_speakers: Some(2),
            ..SpeechToTextRequest::default()
        };
        let result = client
            .speech_to_text()
            .transcribe(&request, Some((b"audio-data", "test.wav", "audio/wav")))
            .await
            .unwrap();

        assert_eq!(result.text, "Hi there.");
        assert_eq!(result.words[0].speaker_id.as_deref(), Some("speaker_0"));
        assert_eq!(result.words[2].speaker_id.as_deref(), Some("speaker_1"));
    }

    // -- get_transcript ----------------------------------------------------

    #[tokio::test]
    async fn get_transcript_returns_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/speech-to-text/transcripts/tx_abc123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "language_code": "eng",
                    "language_probability": 0.97,
                    "text": "Stored transcript.",
                    "words": [
                        {"text": "Stored", "start": 0.0, "end": 0.4, "type": "word", "logprob": -0.06},
                        {"text": " ", "start": 0.4, "end": 0.4, "type": "spacing", "logprob": 0.0},
                        {"text": "transcript.", "start": 0.4, "end": 1.0, "type": "word", "logprob": -0.1}
                    ],
                    "transcription_id": "tx_abc123"
                })),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.speech_to_text().get_transcript("tx_abc123").await.unwrap();

        assert_eq!(result.text, "Stored transcript.");
        assert_eq!(result.transcription_id.as_deref(), Some("tx_abc123"));
    }

    // -- delete_transcript -------------------------------------------------

    #[tokio::test]
    async fn delete_transcript_succeeds() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/speech-to-text/transcripts/tx_del456"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.speech_to_text().delete_transcript("tx_del456").await;

        assert!(result.is_ok());
    }

    // -- multipart helpers -------------------------------------------------

    #[test]
    fn build_stt_multipart_contains_file_and_fields() {
        let request = SpeechToTextRequest {
            language_code: Some("en".into()),
            diarize: true,
            num_speakers: Some(2),
            ..SpeechToTextRequest::default()
        };
        let boundary = "test-boundary";
        let body = super::build_stt_multipart(
            boundary,
            &request,
            Some((b"audio-bytes", "test.mp3", "audio/mpeg")),
        );
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("test.mp3"));
        assert!(body_str.contains("audio/mpeg"));
        assert!(body_str.contains("audio-bytes"));
        assert!(body_str.contains("scribe_v2"));
        assert!(body_str.contains("language_code"));
        assert!(body_str.contains("en"));
        assert!(body_str.contains("diarize"));
        assert!(body_str.contains("true"));
        assert!(body_str.contains("num_speakers"));
        assert!(body_str.contains("2"));
        assert!(body_str.contains("--test-boundary--"));
    }

    #[test]
    fn build_stt_multipart_without_file() {
        let request = SpeechToTextRequest {
            cloud_storage_url: Some("https://example.com/audio.mp3".into()),
            ..SpeechToTextRequest::default()
        };
        let boundary = "test-boundary";
        let body = super::build_stt_multipart(boundary, &request, None);
        let body_str = String::from_utf8_lossy(&body);
        // Should not contain file part
        assert!(!body_str.contains("filename="));
        // Should contain cloud_storage_url field
        assert!(body_str.contains("cloud_storage_url"));
        assert!(body_str.contains("https://example.com/audio.mp3"));
        assert!(body_str.contains("--test-boundary--"));
    }

    #[test]
    fn uuid_v4_simple_returns_32_char_hex() {
        let id = super::uuid_v4_simple();
        assert_eq!(id.len(), 32);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
