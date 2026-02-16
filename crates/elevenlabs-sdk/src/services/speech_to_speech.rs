//! Speech-to-speech service providing access to S2S endpoints.
//!
//! This module wraps the two S2S endpoints exposed by the ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`convert`](SpeechToSpeechService::convert) | `POST /v1/speech-to-speech/{voice_id}` | Convert speech (full audio) |
//! | [`convert_stream`](SpeechToSpeechService::convert_stream) | `POST /v1/speech-to-speech/{voice_id}/stream` | Convert speech (streaming) |
//!
//! Both endpoints accept `multipart/form-data` with an audio file and
//! optional configuration fields. The response is raw audio bytes.
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, types::SpeechToSpeechRequest};
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! let request = SpeechToSpeechRequest::default();
//! let audio = client
//!     .speech_to_speech()
//!     .convert("voice_id", &request, b"fake-audio", "audio.mp3", "audio/mpeg", None)
//!     .await?;
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
    types::{OutputFormat, SpeechToSpeechRequest},
};

/// Speech-to-speech service providing typed access to S2S endpoints.
///
/// Obtained via [`ElevenLabsClient::speech_to_speech`].
#[derive(Debug)]
pub struct SpeechToSpeechService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> SpeechToSpeechService<'a> {
    /// Creates a new `SpeechToSpeechService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Builds the endpoint path with an optional `output_format` query
    /// parameter.
    fn build_path(voice_id: &str, suffix: &str, output_format: Option<OutputFormat>) -> String {
        let mut path = format!("/v1/speech-to-speech/{voice_id}{suffix}");
        if let Some(fmt) = output_format {
            path.push_str("?output_format=");
            path.push_str(&fmt.to_string());
        }
        path
    }

    /// Converts speech using the given voice, returning the full audio as raw
    /// bytes.
    ///
    /// Calls `POST /v1/speech-to-speech/{voice_id}` with
    /// `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The target voice ID for conversion.
    /// * `request` — Configuration fields (model, voice settings, etc.).
    /// * `audio_data` — Raw bytes of the input audio file.
    /// * `filename` — Filename for the audio part (e.g. `"input.mp3"`).
    /// * `content_type` — MIME type of the audio file (e.g. `"audio/mpeg"`).
    /// * `output_format` — Optional output audio format.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// read.
    pub async fn convert(
        &self,
        voice_id: &str,
        request: &SpeechToSpeechRequest,
        audio_data: &[u8],
        filename: &str,
        content_type: &str,
        output_format: Option<OutputFormat>,
    ) -> Result<Bytes> {
        let path = Self::build_path(voice_id, "", output_format);
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_s2s_multipart(&boundary, request, audio_data, filename, content_type);
        let ct = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart_bytes(&path, body, &ct).await
    }

    /// Converts speech using the given voice, returning a stream of audio
    /// byte chunks.
    ///
    /// Calls `POST /v1/speech-to-speech/{voice_id}/stream` with
    /// `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The target voice ID for conversion.
    /// * `request` — Configuration fields (model, voice settings, etc.).
    /// * `audio_data` — Raw bytes of the input audio file.
    /// * `filename` — Filename for the audio part (e.g. `"input.mp3"`).
    /// * `content_type` — MIME type of the audio file (e.g. `"audio/mpeg"`).
    /// * `output_format` — Optional output audio format.
    ///
    /// # Errors
    ///
    /// Returns an error if the initial API request fails. Individual stream
    /// items may also carry transport errors.
    pub async fn convert_stream(
        &self,
        voice_id: &str,
        request: &SpeechToSpeechRequest,
        audio_data: &[u8],
        filename: &str,
        content_type: &str,
        output_format: Option<OutputFormat>,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>>> {
        let path = Self::build_path(voice_id, "/stream", output_format);
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_s2s_multipart(&boundary, request, audio_data, filename, content_type);
        let ct = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart_stream(&path, body, &ct).await
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

/// Builds the multipart body for `POST /v1/speech-to-speech/{voice_id}`
/// and `POST /v1/speech-to-speech/{voice_id}/stream`.
fn build_s2s_multipart(
    boundary: &str,
    request: &SpeechToSpeechRequest,
    audio_data: &[u8],
    filename: &str,
    content_type: &str,
) -> Vec<u8> {
    let mut buf = Vec::new();

    // Audio file (required field: "audio")
    append_file_part(&mut buf, boundary, "audio", filename, content_type, audio_data);

    // model_id (always sent)
    append_text_field(&mut buf, boundary, "model_id", &request.model_id);

    // voice_settings (JSON-encoded string, optional)
    if let Some(ref vs) = request.voice_settings &&
        let Ok(json) = serde_json::to_string(vs)
    {
        append_text_field(&mut buf, boundary, "voice_settings", &json);
    }

    // seed (optional)
    if let Some(seed) = request.seed {
        append_text_field(&mut buf, boundary, "seed", &seed.to_string());
    }

    // remove_background_noise (bool → string)
    append_text_field(
        &mut buf,
        boundary,
        "remove_background_noise",
        if request.remove_background_noise { "true" } else { "false" },
    );

    // file_format (optional)
    if let Some(ref ff) = request.file_format &&
        let Ok(json) = serde_json::to_string(ff)
    {
        // Serialized as JSON string with quotes; strip them for the form field.
        let value = json.trim_matches('"');
        append_text_field(&mut buf, boundary, "file_format", value);
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
        matchers::{header, method, path, query_param},
    };

    use crate::{
        ElevenLabsClient,
        config::ClientConfig,
        types::{OutputFormat, SpeechToSpeechRequest},
    };

    // -- convert -----------------------------------------------------------

    #[tokio::test]
    async fn convert_returns_audio_bytes() {
        let mock_server = MockServer::start().await;
        let audio_bytes: &[u8] = b"\xff\xfb\x90\x00fake-s2s-output";

        Mock::given(method("POST"))
            .and(path("/v1/speech-to-speech/voice123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(audio_bytes, "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = SpeechToSpeechRequest::default();
        let result = client
            .speech_to_speech()
            .convert("voice123", &request, b"input-audio-data", "input.mp3", "audio/mpeg", None)
            .await
            .unwrap();

        assert_eq!(result.as_ref(), audio_bytes);
    }

    #[tokio::test]
    async fn convert_with_output_format() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/speech-to-speech/voice123"))
            .and(query_param("output_format", "pcm_16000"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(b"pcm-data", "audio/pcm"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = SpeechToSpeechRequest::default();
        let result = client
            .speech_to_speech()
            .convert(
                "voice123",
                &request,
                b"input-audio",
                "input.mp3",
                "audio/mpeg",
                Some(OutputFormat::Pcm_16000),
            )
            .await
            .unwrap();

        assert_eq!(result.as_ref(), b"pcm-data");
    }

    #[tokio::test]
    async fn convert_sends_multipart_with_audio() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/speech-to-speech/voice456"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(b"output-audio", "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = SpeechToSpeechRequest {
            model_id: "eleven_english_sts_v2".into(),
            voice_settings: None,
            seed: Some(42),
            remove_background_noise: true,
            file_format: None,
        };
        let result = client
            .speech_to_speech()
            .convert(
                "voice456",
                &request,
                b"my-audio-file-bytes",
                "recording.wav",
                "audio/wav",
                None,
            )
            .await
            .unwrap();

        assert_eq!(result.as_ref(), b"output-audio");
    }

    // -- convert_stream ----------------------------------------------------

    #[tokio::test]
    async fn convert_stream_returns_stream() {
        use futures_core::Stream;

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/speech-to-speech/voice789/stream"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(b"streaming-s2s-audio", "audio/mpeg"),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = SpeechToSpeechRequest::default();
        let s2s = client.speech_to_speech();
        let stream = s2s
            .convert_stream("voice789", &request, b"input-audio", "input.mp3", "audio/mpeg", None)
            .await
            .unwrap();

        fn assert_stream<S: Stream>(_s: &S) {}
        assert_stream(&stream);
    }

    // -- build_path --------------------------------------------------------

    #[test]
    fn build_path_no_params() {
        let path = super::SpeechToSpeechService::build_path("v123", "", None);
        assert_eq!(path, "/v1/speech-to-speech/v123");
    }

    #[test]
    fn build_path_with_stream_suffix() {
        let path = super::SpeechToSpeechService::build_path("v123", "/stream", None);
        assert_eq!(path, "/v1/speech-to-speech/v123/stream");
    }

    #[test]
    fn build_path_with_output_format() {
        let path =
            super::SpeechToSpeechService::build_path("v123", "", Some(OutputFormat::Pcm_16000));
        assert_eq!(path, "/v1/speech-to-speech/v123?output_format=pcm_16000");
    }

    // -- multipart helpers -------------------------------------------------

    #[test]
    fn build_s2s_multipart_contains_audio_and_fields() {
        let request = SpeechToSpeechRequest {
            model_id: "eleven_english_sts_v2".into(),
            voice_settings: None,
            seed: Some(42),
            remove_background_noise: false,
            file_format: None,
        };
        let boundary = "test-boundary";
        let body = super::build_s2s_multipart(
            boundary,
            &request,
            b"fake-audio",
            "input.mp3",
            "audio/mpeg",
        );
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("input.mp3"));
        assert!(body_str.contains("audio/mpeg"));
        assert!(body_str.contains("fake-audio"));
        assert!(body_str.contains("eleven_english_sts_v2"));
        assert!(body_str.contains("42"));
        assert!(body_str.contains("remove_background_noise"));
        assert!(body_str.contains("false"));
        assert!(body_str.contains("--test-boundary--"));
    }

    #[test]
    fn uuid_v4_simple_returns_32_char_hex() {
        let id = super::uuid_v4_simple();
        assert_eq!(id.len(), 32);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
