//! Audio isolation service providing access to vocal/speech isolation endpoints.
//!
//! This module wraps the two audio-isolation endpoints exposed by the
//! ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`isolate`](AudioIsolationService::isolate) | `POST /v1/audio-isolation` | Isolate vocals/speech (full audio) |
//! | [`isolate_stream`](AudioIsolationService::isolate_stream) | `POST /v1/audio-isolation/stream` | Isolate vocals/speech (streaming) |
//!
//! Both endpoints accept `multipart/form-data` with an audio file and
//! optional configuration fields. The response is raw audio bytes.
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, types::AudioIsolationRequest};
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! let request = AudioIsolationRequest::default();
//! let audio = client
//!     .audio_isolation()
//!     .isolate(&request, b"audio-data", "input.mp3", "audio/mpeg")
//!     .await?;
//!
//! println!("Received {} bytes of isolated audio", audio.len());
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;
use futures_core::Stream;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{AudioIsolationRequest, AudioIsolationStreamRequest},
};

/// Audio isolation service providing typed access to vocal/speech isolation
/// endpoints.
///
/// Obtained via [`ElevenLabsClient::audio_isolation`].
#[derive(Debug)]
pub struct AudioIsolationService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> AudioIsolationService<'a> {
    /// Creates a new `AudioIsolationService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Isolates vocals/speech from audio, returning the full isolated audio
    /// as raw bytes.
    ///
    /// Calls `POST /v1/audio-isolation` with `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `request` — Configuration fields (file format, preview, etc.).
    /// * `audio_data` — Raw bytes of the input audio file.
    /// * `filename` — Filename for the audio part (e.g. `"input.mp3"`).
    /// * `content_type` — MIME type of the audio file (e.g. `"audio/mpeg"`).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// read.
    pub async fn isolate(
        &self,
        request: &AudioIsolationRequest,
        audio_data: &[u8],
        filename: &str,
        content_type: &str,
    ) -> Result<Bytes> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body =
            build_audio_isolation_multipart(&boundary, request, audio_data, filename, content_type);
        let ct = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart_bytes("/v1/audio-isolation", body, &ct).await
    }

    /// Isolates vocals/speech from audio, returning a stream of audio byte
    /// chunks.
    ///
    /// Calls `POST /v1/audio-isolation/stream` with `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `request` — Configuration fields (file format, etc.).
    /// * `audio_data` — Raw bytes of the input audio file.
    /// * `filename` — Filename for the audio part (e.g. `"input.mp3"`).
    /// * `content_type` — MIME type of the audio file (e.g. `"audio/mpeg"`).
    ///
    /// # Errors
    ///
    /// Returns an error if the initial API request fails. Individual stream
    /// items may also carry transport errors.
    pub async fn isolate_stream(
        &self,
        request: &AudioIsolationStreamRequest,
        audio_data: &[u8],
        filename: &str,
        content_type: &str,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>>> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_audio_isolation_stream_multipart(
            &boundary,
            request,
            audio_data,
            filename,
            content_type,
        );
        let ct = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart_stream("/v1/audio-isolation/stream", body, &ct).await
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

/// Builds the multipart body for `POST /v1/audio-isolation`.
fn build_audio_isolation_multipart(
    boundary: &str,
    request: &AudioIsolationRequest,
    audio_data: &[u8],
    filename: &str,
    content_type: &str,
) -> Vec<u8> {
    let mut buf = Vec::new();

    // Audio file (required field: "audio")
    append_file_part(&mut buf, boundary, "audio", filename, content_type, audio_data);

    // file_format (optional)
    if let Some(ref ff) = request.file_format &&
        let Ok(json) = serde_json::to_string(ff)
    {
        let value = json.trim_matches('"');
        append_text_field(&mut buf, boundary, "file_format", value);
    }

    // preview_b64 (optional)
    if let Some(ref preview) = request.preview_b64 {
        append_text_field(&mut buf, boundary, "preview_b64", preview);
    }

    buf.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    buf
}

/// Builds the multipart body for `POST /v1/audio-isolation/stream`.
fn build_audio_isolation_stream_multipart(
    boundary: &str,
    request: &AudioIsolationStreamRequest,
    audio_data: &[u8],
    filename: &str,
    content_type: &str,
) -> Vec<u8> {
    let mut buf = Vec::new();

    // Audio file (required field: "audio")
    append_file_part(&mut buf, boundary, "audio", filename, content_type, audio_data);

    // file_format (optional)
    if let Some(ref ff) = request.file_format &&
        let Ok(json) = serde_json::to_string(ff)
    {
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
        matchers::{header, method, path},
    };

    use crate::{
        ElevenLabsClient,
        config::ClientConfig,
        types::{AudioIsolationRequest, AudioIsolationStreamRequest},
    };

    // -- isolate ------------------------------------------------------------

    #[tokio::test]
    async fn isolate_returns_audio_bytes() {
        let mock_server = MockServer::start().await;
        let audio_bytes: &[u8] = b"\xff\xfb\x90\x00fake-isolated-audio";

        Mock::given(method("POST"))
            .and(path("/v1/audio-isolation"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(audio_bytes, "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = AudioIsolationRequest::default();
        let result = client
            .audio_isolation()
            .isolate(&request, b"input-audio", "input.mp3", "audio/mpeg")
            .await
            .unwrap();

        assert_eq!(result.as_ref(), audio_bytes);
    }

    #[tokio::test]
    async fn isolate_sends_multipart_with_audio() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/audio-isolation"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(b"output-audio", "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = AudioIsolationRequest {
            file_format: Some(crate::types::AudioIsolationFileFormat::PcmS16le16),
            preview_b64: Some("aGVsbG8=".into()),
        };
        let result = client
            .audio_isolation()
            .isolate(&request, b"raw-audio-data", "recording.wav", "audio/wav")
            .await
            .unwrap();

        assert_eq!(result.as_ref(), b"output-audio");
    }

    // -- isolate_stream -----------------------------------------------------

    #[tokio::test]
    async fn isolate_stream_returns_stream() {
        use futures_core::Stream;

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/audio-isolation/stream"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(b"streaming-isolated-audio", "audio/mpeg"),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = AudioIsolationStreamRequest::default();
        let svc = client.audio_isolation();
        let stream =
            svc.isolate_stream(&request, b"input-audio", "input.mp3", "audio/mpeg").await.unwrap();

        fn assert_stream<S: Stream>(_s: &S) {}
        assert_stream(&stream);
    }

    // -- multipart helpers --------------------------------------------------

    #[test]
    fn build_audio_isolation_multipart_contains_audio_and_fields() {
        let request = AudioIsolationRequest {
            file_format: Some(crate::types::AudioIsolationFileFormat::PcmS16le16),
            preview_b64: Some("aGVsbG8=".into()),
        };
        let boundary = "test-boundary";
        let body = super::build_audio_isolation_multipart(
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
        assert!(body_str.contains("pcm_s16le_16"));
        assert!(body_str.contains("aGVsbG8="));
        assert!(body_str.contains("--test-boundary--"));
    }

    #[test]
    fn build_audio_isolation_stream_multipart_contains_audio() {
        let request = AudioIsolationStreamRequest::default();
        let boundary = "test-boundary";
        let body = super::build_audio_isolation_stream_multipart(
            boundary,
            &request,
            b"fake-audio",
            "input.mp3",
            "audio/mpeg",
        );
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("input.mp3"));
        assert!(body_str.contains("fake-audio"));
        assert!(body_str.contains("--test-boundary--"));
    }

    #[test]
    fn uuid_v4_simple_returns_32_char_hex() {
        let id = super::uuid_v4_simple();
        assert_eq!(id.len(), 32);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
