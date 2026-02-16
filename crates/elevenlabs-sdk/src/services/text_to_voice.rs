//! Text-to-voice service providing access to voice design and preview
//! endpoints.
//!
//! This module wraps the five text-to-voice endpoints exposed by the
//! ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`create_previews`](TextToVoiceService::create_previews) | `POST /v1/text-to-voice/create-previews` | Generate voice previews |
//! | [`create_voice`](TextToVoiceService::create_voice) | `POST /v1/text-to-voice` | Create a voice from a preview |
//! | [`design`](TextToVoiceService::design) | `POST /v1/text-to-voice/design` | Design a voice |
//! | [`remix`](TextToVoiceService::remix) | `POST /v1/text-to-voice/{voice_id}/remix` | Remix an existing voice |
//! | [`stream_preview`](TextToVoiceService::stream_preview) | `GET /v1/text-to-voice/{generated_voice_id}/stream` | Stream preview audio |
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, types::VoicePreviewsRequest};
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! let request = VoicePreviewsRequest {
//!     voice_description: "A warm female narrator".into(),
//!     text: Some("Hello, world!".into()),
//!     auto_generate_text: None,
//!     loudness: None,
//!     quality: None,
//!     seed: None,
//!     guidance_scale: None,
//!     should_enhance: None,
//! };
//! let previews = client.text_to_voice().create_previews(&request).await?;
//!
//! println!("Got {} previews", previews.previews.len());
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{
        CreateVoiceFromPreviewRequest, Voice, VoiceDesignRequest, VoicePreviewsRequest,
        VoicePreviewsResponse, VoiceRemixRequest,
    },
};

/// Text-to-voice service providing typed access to voice design and
/// preview endpoints.
///
/// Obtained via [`ElevenLabsClient::text_to_voice`].
#[derive(Debug)]
pub struct TextToVoiceService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> TextToVoiceService<'a> {
    /// Creates a new `TextToVoiceService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Generates voice previews from a text description.
    ///
    /// Calls `POST /v1/text-to-voice/create-previews` with a JSON body.
    ///
    /// Returns a [`VoicePreviewsResponse`] containing preview audio and
    /// generated voice IDs that can be used with [`create_voice`](Self::create_voice)
    /// or [`stream_preview`](Self::stream_preview).
    ///
    /// # Arguments
    ///
    /// * `request` — The voice previews request with description, optional text, and generation
    ///   parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn create_previews(
        &self,
        request: &VoicePreviewsRequest,
    ) -> Result<VoicePreviewsResponse> {
        self.client.post("/v1/text-to-voice/create-previews", request).await
    }

    /// Creates a permanent voice from a previously generated voice preview.
    ///
    /// Calls `POST /v1/text-to-voice` with a JSON body.
    ///
    /// Returns the created [`Voice`].
    ///
    /// # Arguments
    ///
    /// * `request` — The create request with voice name, description, and the generated voice ID
    ///   from a preview.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn create_voice(&self, request: &CreateVoiceFromPreviewRequest) -> Result<Voice> {
        self.client.post("/v1/text-to-voice", request).await
    }

    /// Designs a voice from a text description with full control over
    /// generation parameters.
    ///
    /// Calls `POST /v1/text-to-voice/design` with a JSON body.
    ///
    /// Returns a [`VoicePreviewsResponse`] containing the designed voice
    /// previews.
    ///
    /// # Arguments
    ///
    /// * `request` — The voice design request with description, model, and generation parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn design(&self, request: &VoiceDesignRequest) -> Result<VoicePreviewsResponse> {
        self.client.post("/v1/text-to-voice/design", request).await
    }

    /// Remixes an existing voice with a new description.
    ///
    /// Calls `POST /v1/text-to-voice/{voice_id}/remix` with a JSON body.
    ///
    /// Returns a [`VoicePreviewsResponse`] containing the remixed voice
    /// previews.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The ID of the voice to remix.
    /// * `request` — The remix request with description of desired changes and generation
    ///   parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn remix(
        &self,
        voice_id: &str,
        request: &VoiceRemixRequest,
    ) -> Result<VoicePreviewsResponse> {
        let path = format!("/v1/text-to-voice/{voice_id}/remix");
        self.client.post(&path, request).await
    }

    /// Streams preview audio for a generated voice.
    ///
    /// Calls `GET /v1/text-to-voice/{generated_voice_id}/stream`.
    ///
    /// Returns the preview audio as raw bytes.
    ///
    /// # Arguments
    ///
    /// * `generated_voice_id` — The generated voice ID obtained from a preview response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// read.
    pub async fn stream_preview(&self, generated_voice_id: &str) -> Result<Bytes> {
        let path = format!("/v1/text-to-voice/{generated_voice_id}/stream");
        self.client.get_bytes(&path).await
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
        types::{
            CreateVoiceFromPreviewRequest, VoiceDesignRequest, VoicePreviewsRequest,
            VoiceRemixRequest,
        },
    };

    // -- create_previews ---------------------------------------------------

    #[tokio::test]
    async fn create_previews_returns_response() {
        let mock_server = MockServer::start().await;

        let response_json = serde_json::json!({
            "previews": [
                {
                    "audio_base_64": "base64data",
                    "generated_voice_id": "gen1",
                    "media_type": "audio/mpeg",
                    "duration_secs": 3.5,
                    "language": "en"
                }
            ],
            "text": "Hello world"
        });

        Mock::given(method("POST"))
            .and(path("/v1/text-to-voice/create-previews"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = VoicePreviewsRequest {
            voice_description: "A warm female voice".into(),
            text: Some("Hello world".into()),
            auto_generate_text: None,
            loudness: None,
            quality: None,
            seed: None,
            guidance_scale: None,
            should_enhance: None,
        };
        let result = client.text_to_voice().create_previews(&request).await.unwrap();

        assert_eq!(result.previews.len(), 1);
        assert_eq!(result.previews[0].generated_voice_id, "gen1");
        assert_eq!(result.text, "Hello world");
    }

    // -- create_voice ------------------------------------------------------

    #[tokio::test]
    async fn create_voice_returns_voice() {
        let mock_server = MockServer::start().await;

        let voice_json = serde_json::json!({
            "voice_id": "v123",
            "name": "My Voice",
            "category": "generated",
            "labels": {"language": "en"},
            "available_for_tiers": [],
            "high_quality_base_model_ids": [],
            "is_legacy": false,
            "is_mixed": false
        });

        Mock::given(method("POST"))
            .and(path("/v1/text-to-voice"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&voice_json))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = CreateVoiceFromPreviewRequest {
            voice_name: "My Voice".into(),
            voice_description: "A warm and friendly voice".into(),
            generated_voice_id: "gen123".into(),
            labels: None,
            played_not_selected_voice_ids: None,
        };
        let result = client.text_to_voice().create_voice(&request).await.unwrap();

        assert_eq!(result.voice_id, "v123");
        assert_eq!(result.name, "My Voice");
    }

    // -- design ------------------------------------------------------------

    #[tokio::test]
    async fn design_returns_previews_response() {
        let mock_server = MockServer::start().await;

        let response_json = serde_json::json!({
            "previews": [
                {
                    "audio_base_64": "designed-audio",
                    "generated_voice_id": "gen-design-1",
                    "media_type": "audio/mpeg",
                    "duration_secs": 2.0,
                    "language": "en"
                }
            ],
            "text": "Test text"
        });

        Mock::given(method("POST"))
            .and(path("/v1/text-to-voice/design"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = VoiceDesignRequest {
            voice_description: "Deep male narrator".into(),
            model_id: None,
            text: None,
            auto_generate_text: Some(true),
            loudness: None,
            seed: None,
            guidance_scale: None,
            stream_previews: None,
            should_enhance: None,
            quality: None,
            reference_audio_base64: None,
            prompt_strength: None,
        };
        let result = client.text_to_voice().design(&request).await.unwrap();

        assert_eq!(result.previews.len(), 1);
        assert_eq!(result.previews[0].generated_voice_id, "gen-design-1");
    }

    // -- remix -------------------------------------------------------------

    #[tokio::test]
    async fn remix_returns_previews_response() {
        let mock_server = MockServer::start().await;

        let response_json = serde_json::json!({
            "previews": [
                {
                    "audio_base_64": "remixed-audio",
                    "generated_voice_id": "gen-remix-1",
                    "media_type": "audio/mpeg",
                    "duration_secs": 2.5,
                    "language": "en"
                }
            ],
            "text": "Remix text"
        });

        Mock::given(method("POST"))
            .and(path("/v1/text-to-voice/voice123/remix"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = VoiceRemixRequest {
            voice_description: "Make the voice deeper".into(),
            text: None,
            auto_generate_text: Some(true),
            loudness: None,
            seed: None,
            guidance_scale: None,
            stream_previews: None,
            remixing_session_id: None,
            remixing_session_iteration_id: None,
            prompt_strength: None,
        };
        let result = client.text_to_voice().remix("voice123", &request).await.unwrap();

        assert_eq!(result.previews.len(), 1);
        assert_eq!(result.previews[0].generated_voice_id, "gen-remix-1");
    }

    // -- stream_preview ----------------------------------------------------

    #[tokio::test]
    async fn stream_preview_returns_audio_bytes() {
        let mock_server = MockServer::start().await;
        let audio_bytes: &[u8] = b"\xff\xfb\x90\x00preview-audio";

        Mock::given(method("GET"))
            .and(path("/v1/text-to-voice/gen123/stream"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(audio_bytes, "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.text_to_voice().stream_preview("gen123").await.unwrap();

        assert_eq!(result.as_ref(), audio_bytes);
    }

    // -- error handling ----------------------------------------------------

    #[tokio::test]
    async fn create_previews_handles_api_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-voice/create-previews"))
            .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
                "detail": "Validation error"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = VoicePreviewsRequest {
            voice_description: "test".into(),
            text: None,
            auto_generate_text: None,
            loudness: None,
            quality: None,
            seed: None,
            guidance_scale: None,
            should_enhance: None,
        };
        let result = client.text_to_voice().create_previews(&request).await;

        assert!(result.is_err());
    }
}
