//! Voice generation service providing access to legacy voice generation
//! endpoints.
//!
//! This module wraps the three voice-generation endpoints exposed by the
//! ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`get_parameters`](VoiceGenerationService::get_parameters) | `GET /v1/voice-generation/generate-voice/parameters` | List generation parameters |
//! | [`generate_random`](VoiceGenerationService::generate_random) | `POST /v1/voice-generation/generate-voice` | Generate a random voice (audio bytes) |
//! | [`create_voice`](VoiceGenerationService::create_voice) | `POST /v1/voice-generation/create-voice` | Create a voice from a generated preview |
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{
//!     ClientConfig, ElevenLabsClient,
//!     types::{GenerateRandomVoiceRequest, GenerateVoiceAge, GenerateVoiceGender},
//! };
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! // List available parameters
//! let params = client.voice_generation().get_parameters().await?;
//! println!("Available genders: {:?}", params.genders);
//!
//! // Generate a random voice
//! let request = GenerateRandomVoiceRequest {
//!     gender: GenerateVoiceGender::Female,
//!     accent: "british".into(),
//!     age: GenerateVoiceAge::Young,
//!     accent_strength: 1.0,
//!     text: "Every act of kindness carries value.".repeat(3),
//! };
//! let audio = client.voice_generation().generate_random(&request).await?;
//!
//! println!("Received {} bytes of audio", audio.len());
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{
        CreateGeneratedVoiceRequest, GenerateRandomVoiceRequest, Voice, VoiceGenerationParameters,
    },
};

/// Voice generation service providing typed access to legacy voice
/// generation endpoints.
///
/// Obtained via [`ElevenLabsClient::voice_generation`].
#[derive(Debug)]
pub struct VoiceGenerationService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> VoiceGenerationService<'a> {
    /// Creates a new `VoiceGenerationService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Lists available voice generation parameters (genders, accents,
    /// ages, and their value ranges).
    ///
    /// Calls `GET /v1/voice-generation/generate-voice/parameters`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_parameters(&self) -> Result<VoiceGenerationParameters> {
        self.client.get("/v1/voice-generation/generate-voice/parameters").await
    }

    /// Generates a random voice with the specified characteristics,
    /// returning the preview audio as raw bytes.
    ///
    /// Calls `POST /v1/voice-generation/generate-voice` with a JSON body.
    ///
    /// The generated voice ID is returned in the response headers
    /// (`generated_voice_id`), which can be used with
    /// [`create_voice`](Self::create_voice) to persist the voice.
    ///
    /// # Arguments
    ///
    /// * `request` — The generation request with gender, accent, age, accent strength, and preview
    ///   text.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// read.
    pub async fn generate_random(&self, request: &GenerateRandomVoiceRequest) -> Result<Bytes> {
        self.client.post_bytes("/v1/voice-generation/generate-voice", request).await
    }

    /// Creates a persistent voice from a previously generated voice
    /// preview.
    ///
    /// Calls `POST /v1/voice-generation/create-voice` with a JSON body.
    ///
    /// Returns the created [`Voice`].
    ///
    /// # Arguments
    ///
    /// * `request` — The create request with voice name, description, and the generated voice ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn create_voice(&self, request: &CreateGeneratedVoiceRequest) -> Result<Voice> {
        self.client.post("/v1/voice-generation/create-voice", request).await
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
            CreateGeneratedVoiceRequest, GenerateRandomVoiceRequest, GenerateVoiceAge,
            GenerateVoiceGender,
        },
    };

    // -- get_parameters ----------------------------------------------------

    #[tokio::test]
    async fn get_parameters_returns_parameters() {
        let mock_server = MockServer::start().await;

        let response_json = serde_json::json!({
            "genders": [
                {"name": "Female", "code": "female"},
                {"name": "Male", "code": "male"}
            ],
            "accents": [
                {"name": "British", "code": "british"},
                {"name": "American", "code": "american"}
            ],
            "ages": [
                {"name": "Young", "code": "young"},
                {"name": "Middle Aged", "code": "middle_aged"},
                {"name": "Old", "code": "old"}
            ],
            "minimum_characters": 100,
            "maximum_characters": 1000,
            "minimum_accent_strength": 0.3,
            "maximum_accent_strength": 2.0
        });

        Mock::given(method("GET"))
            .and(path("/v1/voice-generation/generate-voice/parameters"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.voice_generation().get_parameters().await.unwrap();

        assert_eq!(result.genders.len(), 2);
        assert_eq!(result.genders[0].code, "female");
        assert_eq!(result.accents.len(), 2);
        assert_eq!(result.ages.len(), 3);
        assert_eq!(result.minimum_characters, 100);
    }

    // -- generate_random ---------------------------------------------------

    #[tokio::test]
    async fn generate_random_returns_audio_bytes() {
        let mock_server = MockServer::start().await;
        let audio_bytes: &[u8] = b"\xff\xfb\x90\x00random-voice-audio";

        Mock::given(method("POST"))
            .and(path("/v1/voice-generation/generate-voice"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(audio_bytes, "audio/mpeg"))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = GenerateRandomVoiceRequest {
            gender: GenerateVoiceGender::Female,
            accent: "british".into(),
            age: GenerateVoiceAge::Young,
            accent_strength: 1.5,
            text: "ab".repeat(60),
        };
        let result = client.voice_generation().generate_random(&request).await.unwrap();

        assert_eq!(result.as_ref(), audio_bytes);
    }

    // -- create_voice ------------------------------------------------------

    #[tokio::test]
    async fn create_voice_returns_voice() {
        let mock_server = MockServer::start().await;

        let voice_json = serde_json::json!({
            "voice_id": "v-gen-123",
            "name": "Generated Voice",
            "category": "generated",
            "labels": {},
            "available_for_tiers": [],
            "high_quality_base_model_ids": [],
            "is_legacy": false,
            "is_mixed": false
        });

        Mock::given(method("POST"))
            .and(path("/v1/voice-generation/create-voice"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&voice_json))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = CreateGeneratedVoiceRequest {
            voice_name: "Generated Voice".into(),
            voice_description: "A dynamically generated voice".into(),
            generated_voice_id: "gen-id-456".into(),
            played_not_selected_voice_ids: None,
            labels: None,
        };
        let result = client.voice_generation().create_voice(&request).await.unwrap();

        assert_eq!(result.voice_id, "v-gen-123");
        assert_eq!(result.name, "Generated Voice");
    }

    // -- error handling ----------------------------------------------------

    #[tokio::test]
    async fn generate_random_handles_api_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/voice-generation/generate-voice"))
            .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
                "detail": "Validation error: text too short"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = GenerateRandomVoiceRequest {
            gender: GenerateVoiceGender::Male,
            accent: "american".into(),
            age: GenerateVoiceAge::MiddleAged,
            accent_strength: 1.0,
            text: "short".into(),
        };
        let result = client.voice_generation().generate_random(&request).await;

        assert!(result.is_err());
    }
}
