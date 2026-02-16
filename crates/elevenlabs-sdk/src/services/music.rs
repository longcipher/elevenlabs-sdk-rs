//! Music service providing access to music generation and stem-separation
//! endpoints.
//!
//! This module wraps the five music endpoints exposed by the ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`plan`](MusicService::plan) | `POST /v1/music/plan` | Generate a composition plan |
//! | [`compose`](MusicService::compose) | `POST /v1/music` | Compose music (full audio) |
//! | [`compose_detailed`](MusicService::compose_detailed) | `POST /v1/music/detailed` | Compose music with detailed metadata |
//! | [`compose_stream`](MusicService::compose_stream) | `POST /v1/music/stream` | Compose music (streaming) |
//! | [`separate_stems`](MusicService::separate_stems) | `POST /v1/music/stem-separation` | Separate audio into stems |
//!
//! The plan, compose, compose-detailed, and stream endpoints accept JSON.
//! The stem-separation endpoint accepts `multipart/form-data` with an audio
//! file.
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{
//!     ClientConfig, ElevenLabsClient,
//!     types::{MusicComposeRequest, MusicPlanRequest},
//! };
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! // Generate a composition plan
//! let plan_req =
//!     MusicPlanRequest { prompt: "An upbeat pop song about summer".into(), ..Default::default() };
//! let plan = client.music().plan(&plan_req).await?;
//!
//! // Compose from the plan
//! let compose_req = MusicComposeRequest { composition_plan: Some(plan), ..Default::default() };
//! let audio = client.music().compose(&compose_req).await?;
//!
//! println!("Received {} bytes of music", audio.len());
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;
use futures_core::Stream;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{
        DetailedMusicResponse, MusicComposeRequest, MusicPlanRequest, MusicPrompt,
        MusicStemSeparationRequest,
    },
};

/// Music service providing typed access to music generation and
/// stem-separation endpoints.
///
/// Obtained via [`ElevenLabsClient::music`].
#[derive(Debug)]
pub struct MusicService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> MusicService<'a> {
    /// Creates a new `MusicService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Generates a composition plan from a text prompt.
    ///
    /// Calls `POST /v1/music/plan` with a JSON body.
    ///
    /// Returns a [`MusicPrompt`] that can be passed to [`compose`](Self::compose),
    /// [`compose_detailed`](Self::compose_detailed), or
    /// [`compose_stream`](Self::compose_stream).
    ///
    /// # Arguments
    ///
    /// * `request` — The plan request with prompt, desired length, etc.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn plan(&self, request: &MusicPlanRequest) -> Result<MusicPrompt> {
        self.client.post("/v1/music/plan", request).await
    }

    /// Composes music, returning the full audio as raw bytes.
    ///
    /// Calls `POST /v1/music` with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `request` — The compose request. Exactly one of `prompt` or `composition_plan` must be
    ///   set.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// read.
    pub async fn compose(&self, request: &MusicComposeRequest) -> Result<Bytes> {
        self.client.post_bytes("/v1/music", request).await
    }

    /// Composes music and returns detailed metadata alongside the audio.
    ///
    /// Calls `POST /v1/music/detailed` with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `request` — The compose request. Exactly one of `prompt` or `composition_plan` must be
    ///   set.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn compose_detailed(
        &self,
        request: &MusicComposeRequest,
    ) -> Result<DetailedMusicResponse> {
        self.client.post("/v1/music/detailed", request).await
    }

    /// Composes music, returning a stream of audio byte chunks.
    ///
    /// Calls `POST /v1/music/stream` with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `request` — The compose request. Exactly one of `prompt` or `composition_plan` must be
    ///   set.
    ///
    /// # Errors
    ///
    /// Returns an error if the initial API request fails. Individual stream
    /// items may also carry transport errors.
    pub async fn compose_stream(
        &self,
        request: &MusicComposeRequest,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>>> {
        self.client.post_stream("/v1/music/stream", request).await
    }

    /// Separates an audio file into individual stems (e.g. vocals, drums,
    /// bass).
    ///
    /// Calls `POST /v1/music/stem-separation` with `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `request` — Configuration fields (stem variation, C2PA signing).
    /// * `audio_data` — Raw bytes of the input audio file.
    /// * `filename` — Filename for the audio part (e.g. `"song.mp3"`).
    /// * `content_type` — MIME type of the audio file (e.g. `"audio/mpeg"`).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// read.
    pub async fn separate_stems(
        &self,
        request: &MusicStemSeparationRequest,
        audio_data: &[u8],
        filename: &str,
        content_type: &str,
    ) -> Result<Bytes> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body =
            build_stem_separation_multipart(&boundary, request, audio_data, filename, content_type);
        let ct = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart_bytes("/v1/music/stem-separation", body, &ct).await
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

/// Builds the multipart body for `POST /v1/music/stem-separation`.
fn build_stem_separation_multipart(
    boundary: &str,
    request: &MusicStemSeparationRequest,
    audio_data: &[u8],
    filename: &str,
    content_type: &str,
) -> Vec<u8> {
    let mut buf = Vec::new();

    // Audio file (required field: "audio")
    append_file_part(&mut buf, boundary, "audio", filename, content_type, audio_data);

    // stem_variation_id
    if let Ok(json) = serde_json::to_string(&request.stem_variation_id) {
        let value = json.trim_matches('"');
        append_text_field(&mut buf, boundary, "stem_variation_id", value);
    }

    // sign_with_c2pa (bool → string)
    append_text_field(
        &mut buf,
        boundary,
        "sign_with_c2pa",
        if request.sign_with_c2pa { "true" } else { "false" },
    );

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
        types::{MusicComposeRequest, MusicPlanRequest, MusicStemSeparationRequest, StemVariation},
    };

    // -- plan ---------------------------------------------------------------

    #[tokio::test]
    async fn plan_returns_music_prompt() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/music/plan"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "positive_global_styles": ["pop"],
                "negative_global_styles": [],
                "sections": [{
                    "section_name": "Verse 1",
                    "positive_local_styles": ["acoustic"],
                    "negative_local_styles": [],
                    "duration_ms": 15000,
                    "lines": ["Hello world"]
                }]
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request =
            MusicPlanRequest { prompt: "An upbeat pop song".into(), ..Default::default() };
        let result = client.music().plan(&request).await.unwrap();

        assert_eq!(result.positive_global_styles, vec!["pop"]);
        assert_eq!(result.sections.len(), 1);
        assert_eq!(result.sections[0].section_name, "Verse 1");
    }

    // -- compose ------------------------------------------------------------

    #[tokio::test]
    async fn compose_returns_audio_bytes() {
        let mock_server = MockServer::start().await;
        let audio_bytes: &[u8] = b"\xff\xfb\x90\x00fake-music-audio";

        Mock::given(method("POST"))
            .and(path("/v1/music"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(audio_bytes, "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = MusicComposeRequest {
            prompt: Some("A mellow jazz piece".into()),
            ..Default::default()
        };
        let result = client.music().compose(&request).await.unwrap();

        assert_eq!(result.as_ref(), audio_bytes);
    }

    // -- compose_detailed ---------------------------------------------------

    #[tokio::test]
    async fn compose_detailed_returns_metadata() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/music/detailed"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "composition_plan": {
                    "positive_global_styles": ["jazz"],
                    "negative_global_styles": [],
                    "sections": [{
                        "section_name": "Intro",
                        "positive_local_styles": [],
                        "negative_local_styles": [],
                        "duration_ms": 5000,
                        "lines": []
                    }]
                },
                "song_metadata": {
                    "title": "Jazz Piece",
                    "description": "A mellow jazz piece",
                    "genres": ["jazz"],
                    "languages": ["en"],
                    "is_explicit": false
                },
                "words_timestamps": null
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = MusicComposeRequest {
            prompt: Some("A mellow jazz piece".into()),
            ..Default::default()
        };
        let result = client.music().compose_detailed(&request).await.unwrap();

        assert_eq!(result.song_metadata.title.as_deref(), Some("Jazz Piece"));
        assert_eq!(result.composition_plan.sections.len(), 1);
        assert!(result.words_timestamps.is_none());
    }

    // -- compose_stream -----------------------------------------------------

    #[tokio::test]
    async fn compose_stream_returns_stream() {
        use futures_core::Stream;

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/music/stream"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(b"streaming-music-audio", "audio/mpeg"),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request =
            MusicComposeRequest { prompt: Some("Epic orchestral".into()), ..Default::default() };
        let svc = client.music();
        let stream = svc.compose_stream(&request).await.unwrap();

        fn assert_stream<S: Stream>(_s: &S) {}
        assert_stream(&stream);
    }

    // -- separate_stems -----------------------------------------------------

    #[tokio::test]
    async fn separate_stems_returns_audio_bytes() {
        let mock_server = MockServer::start().await;
        let audio_bytes: &[u8] = b"\xff\xfb\x90\x00fake-stem-output";

        Mock::given(method("POST"))
            .and(path("/v1/music/stem-separation"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(audio_bytes, "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = MusicStemSeparationRequest::default();
        let result = client
            .music()
            .separate_stems(&request, b"input-song", "song.mp3", "audio/mpeg")
            .await
            .unwrap();

        assert_eq!(result.as_ref(), audio_bytes);
    }

    #[tokio::test]
    async fn separate_stems_with_two_stems() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/music/stem-separation"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(b"two-stems", "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = MusicStemSeparationRequest {
            stem_variation_id: StemVariation::TwoStemsV1,
            sign_with_c2pa: true,
        };
        let result = client
            .music()
            .separate_stems(&request, b"input-song", "song.mp3", "audio/mpeg")
            .await
            .unwrap();

        assert_eq!(result.as_ref(), b"two-stems");
    }

    // -- multipart helpers --------------------------------------------------

    #[test]
    fn build_stem_separation_multipart_contains_fields() {
        let request = MusicStemSeparationRequest {
            stem_variation_id: StemVariation::TwoStemsV1,
            sign_with_c2pa: true,
        };
        let boundary = "test-boundary";
        let body = super::build_stem_separation_multipart(
            boundary,
            &request,
            b"fake-song",
            "song.mp3",
            "audio/mpeg",
        );
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("song.mp3"));
        assert!(body_str.contains("audio/mpeg"));
        assert!(body_str.contains("fake-song"));
        assert!(body_str.contains("two_stems_v1"));
        assert!(body_str.contains("sign_with_c2pa"));
        assert!(body_str.contains("true"));
        assert!(body_str.contains("--test-boundary--"));
    }

    #[test]
    fn uuid_v4_simple_returns_32_char_hex() {
        let id = super::uuid_v4_simple();
        assert_eq!(id.len(), 32);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
