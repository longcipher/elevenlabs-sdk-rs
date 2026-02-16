//! Voices service providing access to voice management endpoints.
//!
//! This module wraps the voice management endpoints exposed by the ElevenLabs
//! API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`list`](VoicesService::list) | `GET /v1/voices` | List all voices |
//! | [`get`](VoicesService::get) | `GET /v1/voices/{voice_id}` | Get a single voice |
//! | [`get_default_settings`](VoicesService::get_default_settings) | `GET /v1/voices/settings/default` | Get default voice settings |
//! | [`get_settings`](VoicesService::get_settings) | `GET /v1/voices/{voice_id}/settings` | Get voice settings |
//! | [`edit_settings`](VoicesService::edit_settings) | `POST /v1/voices/{voice_id}/settings/edit` | Edit voice settings |
//! | [`add`](VoicesService::add) | `POST /v1/voices/add` | Add a new voice (multipart) |
//! | [`edit`](VoicesService::edit) | `POST /v1/voices/{voice_id}/edit` | Edit a voice (multipart) |
//! | [`delete`](VoicesService::delete) | `DELETE /v1/voices/{voice_id}` | Delete a voice |
//! | [`add_sharing`](VoicesService::add_sharing) | `POST /v1/voices/add/{public_user_id}/{voice_id}` | Add a shared voice |
//! | [`get_sample_audio`](VoicesService::get_sample_audio) | `GET /v1/voices/{voice_id}/samples/{sample_id}/audio` | Get sample audio |
//! | [`delete_sample`](VoicesService::delete_sample) | `DELETE /v1/voices/{voice_id}/samples/{sample_id}` | Delete a sample |
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
//! let voices = client.voices().list(None).await?;
//! println!("Found {} voices", voices.voices.len());
//!
//! let voice = client.voices().get("voice_id", None).await?;
//! println!("Voice name: {}", voice.name);
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{
        AddVoiceRequest, AddVoiceResponse, DeleteVoiceResponse, DeleteVoiceSampleResponse,
        EditVoiceRequest, EditVoiceResponse, EditVoiceSettingsResponse, GetLibraryVoicesResponse,
        GetSimilarVoicesResponse, GetVoicesResponse, GetVoicesV2Response, Voice, VoiceSettings,
    },
};

/// Voices service providing typed access to voice management endpoints.
///
/// Obtained via [`ElevenLabsClient::voices`].
#[derive(Debug)]
pub struct VoicesService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> VoicesService<'a> {
    /// Creates a new `VoicesService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Lists all voices available to the authenticated user.
    ///
    /// Calls `GET /v1/voices`.
    ///
    /// # Arguments
    ///
    /// * `show_legacy` — When `true`, includes legacy voices in the response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn list(&self, show_legacy: Option<bool>) -> Result<GetVoicesResponse> {
        let mut path = "/v1/voices".to_owned();
        if show_legacy == Some(true) {
            path.push_str("?show_legacy=true");
        }
        self.client.get(&path).await
    }

    /// Gets a single voice by ID.
    ///
    /// Calls `GET /v1/voices/{voice_id}`.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID to retrieve.
    /// * `with_settings` — When `true`, includes voice settings in the response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get(&self, voice_id: &str, with_settings: Option<bool>) -> Result<Voice> {
        let mut path = format!("/v1/voices/{voice_id}");
        if with_settings == Some(true) {
            path.push_str("?with_settings=true");
        }
        self.client.get(&path).await
    }

    /// Gets the default voice settings.
    ///
    /// Calls `GET /v1/voices/settings/default`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_default_settings(&self) -> Result<VoiceSettings> {
        self.client.get("/v1/voices/settings/default").await
    }

    /// Gets the settings for a specific voice.
    ///
    /// Calls `GET /v1/voices/{voice_id}/settings`.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID whose settings to retrieve.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_settings(&self, voice_id: &str) -> Result<VoiceSettings> {
        let path = format!("/v1/voices/{voice_id}/settings");
        self.client.get(&path).await
    }

    /// Edits the settings for a specific voice.
    ///
    /// Calls `POST /v1/voices/{voice_id}/settings/edit`.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID whose settings to update.
    /// * `settings` — The new voice settings.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn edit_settings(
        &self,
        voice_id: &str,
        settings: &VoiceSettings,
    ) -> Result<EditVoiceSettingsResponse> {
        let path = format!("/v1/voices/{voice_id}/settings/edit");
        self.client.post(&path, settings).await
    }

    /// Adds a new voice.
    ///
    /// Calls `POST /v1/voices/add` with `multipart/form-data`.
    ///
    /// Audio files (samples) should be provided via the `files` parameter.
    /// Each tuple contains `(filename, content_type, data)`.
    ///
    /// # Arguments
    ///
    /// * `request` — Voice metadata (name, description, labels).
    /// * `files` — Audio sample files as `(filename, content_type, bytes)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn add(
        &self,
        request: &AddVoiceRequest,
        files: &[(&str, &str, &[u8])],
    ) -> Result<AddVoiceResponse> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_add_voice_multipart(&boundary, request, files);
        let content_type = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart("/v1/voices/add", body, &content_type).await
    }

    /// Edits an existing voice.
    ///
    /// Calls `POST /v1/voices/{voice_id}/edit` with `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID to edit.
    /// * `request` — Updated voice metadata (name, description, labels).
    /// * `files` — Optional new audio sample files as `(filename, content_type, bytes)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn edit(
        &self,
        voice_id: &str,
        request: &EditVoiceRequest,
        files: &[(&str, &str, &[u8])],
    ) -> Result<EditVoiceResponse> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_edit_voice_multipart(&boundary, request, files);
        let content_type = format!("multipart/form-data; boundary={boundary}");
        let path = format!("/v1/voices/{voice_id}/edit");
        self.client.post_multipart(&path, body, &content_type).await
    }

    /// Deletes a voice.
    ///
    /// Calls `DELETE /v1/voices/{voice_id}`.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID to delete.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete(&self, voice_id: &str) -> Result<DeleteVoiceResponse> {
        let path = format!("/v1/voices/{voice_id}");
        self.client.delete_json(&path).await
    }

    /// Adds a shared voice from the voice library.
    ///
    /// Calls `POST /v1/voices/add/{public_user_id}/{voice_id}`.
    ///
    /// # Arguments
    ///
    /// * `public_user_id` — The public user ID of the voice owner.
    /// * `voice_id` — The voice ID to add from the library.
    /// * `new_name` — Display name for the added voice.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn add_sharing(
        &self,
        public_user_id: &str,
        voice_id: &str,
        new_name: &str,
    ) -> Result<AddVoiceResponse> {
        let path = format!("/v1/voices/add/{public_user_id}/{voice_id}");
        #[derive(serde::Serialize)]
        struct Body<'b> {
            new_name: &'b str,
        }
        self.client.post(&path, &Body { new_name }).await
    }

    /// Gets the audio data for a specific voice sample.
    ///
    /// Calls `GET /v1/voices/{voice_id}/samples/{sample_id}/audio`.
    ///
    /// Returns the raw audio bytes.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID.
    /// * `sample_id` — The sample ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get_sample_audio(&self, voice_id: &str, sample_id: &str) -> Result<Bytes> {
        let path = format!("/v1/voices/{voice_id}/samples/{sample_id}/audio");
        self.client.get_bytes(&path).await
    }

    /// Deletes a voice sample.
    ///
    /// Calls `DELETE /v1/voices/{voice_id}/samples/{sample_id}`.
    ///
    /// # Arguments
    ///
    /// * `voice_id` — The voice ID.
    /// * `sample_id` — The sample ID to delete.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete_sample(
        &self,
        voice_id: &str,
        sample_id: &str,
    ) -> Result<DeleteVoiceSampleResponse> {
        let path = format!("/v1/voices/{voice_id}/samples/{sample_id}");
        self.client.delete_json(&path).await
    }

    // ── Library / Shared Voices ──────────────────────────────────────

    /// Lists shared voices from the voice library.
    ///
    /// Calls `GET /v1/shared-voices`.
    ///
    /// # Arguments
    ///
    /// * `page_size` — Number of voices per page.
    /// * `category` — Filter by category (e.g. `"professional"`).
    /// * `gender` — Filter by gender.
    /// * `age` — Filter by age group.
    /// * `accent` — Filter by accent.
    /// * `language` — Filter by language.
    /// * `search` — Free-text search query.
    /// * `page` — Page number (0-indexed).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[expect(clippy::too_many_arguments, reason = "mirrors API query params")]
    pub async fn get_shared_voices(
        &self,
        page_size: Option<u32>,
        category: Option<&str>,
        gender: Option<&str>,
        age: Option<&str>,
        accent: Option<&str>,
        language: Option<&str>,
        search: Option<&str>,
        page: Option<u32>,
    ) -> Result<GetLibraryVoicesResponse> {
        let mut path = "/v1/shared-voices".to_owned();
        let mut sep = '?';
        if let Some(v) = page_size {
            path.push_str(&format!("{sep}page_size={v}"));
            sep = '&';
        }
        if let Some(v) = category {
            path.push_str(&format!("{sep}category={v}"));
            sep = '&';
        }
        if let Some(v) = gender {
            path.push_str(&format!("{sep}gender={v}"));
            sep = '&';
        }
        if let Some(v) = age {
            path.push_str(&format!("{sep}age={v}"));
            sep = '&';
        }
        if let Some(v) = accent {
            path.push_str(&format!("{sep}accent={v}"));
            sep = '&';
        }
        if let Some(v) = language {
            path.push_str(&format!("{sep}language={v}"));
            sep = '&';
        }
        if let Some(v) = search {
            path.push_str(&format!("{sep}search={v}"));
            sep = '&';
        }
        if let Some(v) = page {
            path.push_str(&format!("{sep}page={v}"));
        }
        self.client.get(&path).await
    }

    /// Finds voices similar to an uploaded audio sample.
    ///
    /// Calls `POST /v1/similar-voices` with `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `audio_data` — Raw bytes of the reference audio.
    /// * `file_name` — File name (e.g. `"sample.mp3"`).
    /// * `similarity_threshold` — Minimum similarity score (0.0–2.0).
    /// * `top_k` — Maximum number of results.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get_similar_voices(
        &self,
        audio_data: &[u8],
        file_name: &str,
        similarity_threshold: Option<f64>,
        top_k: Option<u32>,
    ) -> Result<GetSimilarVoicesResponse> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let mut body = Vec::new();

        append_file_part(
            &mut body,
            &boundary,
            "audio_file",
            file_name,
            "application/octet-stream",
            audio_data,
        );

        if let Some(v) = similarity_threshold {
            append_text_field(&mut body, &boundary, "similarity_threshold", &v.to_string());
        }
        if let Some(v) = top_k {
            append_text_field(&mut body, &boundary, "top_k", &v.to_string());
        }

        body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
        let content_type = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart("/v1/similar-voices", body, &content_type).await
    }

    /// Lists voices using the v2 API with pagination.
    ///
    /// Calls `GET /v2/voices`.
    ///
    /// # Arguments
    ///
    /// * `next_page_token` — Pagination cursor from a previous response.
    /// * `page_size` — Number of voices per page.
    /// * `search` — Free-text search query.
    /// * `sort` — Sort field.
    /// * `voice_type` — Filter by voice type.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get_voices_v2(
        &self,
        next_page_token: Option<&str>,
        page_size: Option<u32>,
        search: Option<&str>,
        sort: Option<&str>,
        voice_type: Option<&str>,
    ) -> Result<GetVoicesV2Response> {
        let mut path = "/v2/voices".to_owned();
        let mut sep = '?';
        if let Some(v) = next_page_token {
            path.push_str(&format!("{sep}next_page_token={v}"));
            sep = '&';
        }
        if let Some(v) = page_size {
            path.push_str(&format!("{sep}page_size={v}"));
            sep = '&';
        }
        if let Some(v) = search {
            path.push_str(&format!("{sep}search={v}"));
            sep = '&';
        }
        if let Some(v) = sort {
            path.push_str(&format!("{sep}sort={v}"));
            sep = '&';
        }
        if let Some(v) = voice_type {
            path.push_str(&format!("{sep}voice_type={v}"));
        }
        self.client.get(&path).await
    }
}

// ---------------------------------------------------------------------------
// Multipart helpers
// ---------------------------------------------------------------------------

/// Generates a simple pseudo-random hex string for multipart boundaries.
///
/// Not cryptographically secure — only needs to be unique enough that it
/// does not collide with body content.
pub(crate) fn uuid_v4_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    format!("{nanos:032x}")
}

/// Appends a text field to a multipart body buffer.
pub(crate) fn append_text_field(buf: &mut Vec<u8>, boundary: &str, name: &str, value: &str) {
    buf.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    buf.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").as_bytes(),
    );
    buf.extend_from_slice(value.as_bytes());
    buf.extend_from_slice(b"\r\n");
}

/// Appends a file part to a multipart body buffer.
pub(crate) fn append_file_part(
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

/// Builds the multipart body for `POST /v1/voices/add`.
fn build_add_voice_multipart(
    boundary: &str,
    request: &AddVoiceRequest,
    files: &[(&str, &str, &[u8])],
) -> Vec<u8> {
    let mut buf = Vec::new();

    append_text_field(&mut buf, boundary, "name", &request.name);

    if let Some(ref desc) = request.description {
        append_text_field(&mut buf, boundary, "description", desc);
    }

    if let Some(ref labels) = request.labels &&
        let Ok(json) = serde_json::to_string(labels)
    {
        append_text_field(&mut buf, boundary, "labels", &json);
    }

    for (filename, content_type, data) in files {
        append_file_part(&mut buf, boundary, "files", filename, content_type, data);
    }

    buf.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    buf
}

/// Builds the multipart body for `POST /v1/voices/{voice_id}/edit`.
fn build_edit_voice_multipart(
    boundary: &str,
    request: &EditVoiceRequest,
    files: &[(&str, &str, &[u8])],
) -> Vec<u8> {
    let mut buf = Vec::new();

    append_text_field(&mut buf, boundary, "name", &request.name);

    if let Some(ref desc) = request.description {
        append_text_field(&mut buf, boundary, "description", desc);
    }

    if let Some(ref labels) = request.labels &&
        let Ok(json) = serde_json::to_string(labels)
    {
        append_text_field(&mut buf, boundary, "labels", &json);
    }

    for (filename, content_type, data) in files {
        append_file_part(&mut buf, boundary, "files", filename, content_type, data);
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
        types::{AddVoiceRequest, EditVoiceRequest, VoiceSettings},
    };

    // -- list --------------------------------------------------------------

    #[tokio::test]
    async fn list_returns_voices() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/voices"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voices": [
                    {
                        "voice_id": "v1",
                        "name": "Rachel",
                        "category": "premade",
                        "labels": {},
                        "available_for_tiers": [],
                        "high_quality_base_model_ids": []
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.voices().list(None).await.unwrap();
        assert_eq!(result.voices.len(), 1);
        assert_eq!(result.voices[0].name, "Rachel");
    }

    #[tokio::test]
    async fn list_with_show_legacy() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/voices"))
            .and(query_param("show_legacy", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voices": []
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.voices().list(Some(true)).await.unwrap();
        assert!(result.voices.is_empty());
    }

    // -- get ---------------------------------------------------------------

    #[tokio::test]
    async fn get_returns_voice() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/voices/voice123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voice_id": "voice123",
                "name": "Rachel",
                "category": "premade",
                "labels": {"accent": "American"},
                "available_for_tiers": ["creator"],
                "high_quality_base_model_ids": ["eleven_multilingual_v2"]
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let voice = client.voices().get("voice123", None).await.unwrap();
        assert_eq!(voice.voice_id, "voice123");
        assert_eq!(voice.name, "Rachel");
    }

    #[tokio::test]
    async fn get_with_settings() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/voices/voice123"))
            .and(query_param("with_settings", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voice_id": "voice123",
                "name": "Rachel",
                "category": "premade",
                "labels": {},
                "available_for_tiers": [],
                "high_quality_base_model_ids": [],
                "settings": {
                    "stability": 0.5,
                    "similarity_boost": 0.75
                }
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let voice = client.voices().get("voice123", Some(true)).await.unwrap();
        let settings = voice.settings.unwrap();
        assert_eq!(settings.stability, Some(0.5));
    }

    // -- get_default_settings ----------------------------------------------

    #[tokio::test]
    async fn get_default_settings_returns_settings() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/voices/settings/default"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "stability": 0.5,
                "similarity_boost": 0.75,
                "style": 0.0,
                "use_speaker_boost": true,
                "speed": 1.0
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let settings = client.voices().get_default_settings().await.unwrap();
        assert_eq!(settings.stability, Some(0.5));
        assert_eq!(settings.speed, Some(1.0));
    }

    // -- get_settings ------------------------------------------------------

    #[tokio::test]
    async fn get_settings_returns_settings() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/voices/voice123/settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "stability": 0.8,
                "similarity_boost": 0.9
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let settings = client.voices().get_settings("voice123").await.unwrap();
        assert_eq!(settings.stability, Some(0.8));
        assert_eq!(settings.similarity_boost, Some(0.9));
    }

    // -- edit_settings -----------------------------------------------------

    #[tokio::test]
    async fn edit_settings_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/voices/voice123/settings/edit"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let settings = VoiceSettings {
            stability: Some(0.7),
            similarity_boost: Some(0.8),
            ..VoiceSettings::default()
        };
        let result = client.voices().edit_settings("voice123", &settings).await.unwrap();
        assert_eq!(result.status, "ok");
    }

    // -- delete ------------------------------------------------------------

    #[tokio::test]
    async fn delete_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/voices/voice123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.voices().delete("voice123").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    // -- add ---------------------------------------------------------------

    #[tokio::test]
    async fn add_voice_without_files() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/voices/add"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voice_id": "new_voice_123"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let req = AddVoiceRequest {
            name: "My Voice".into(),
            description: Some("A test voice".into()),
            labels: None,
        };
        let result = client.voices().add(&req, &[]).await.unwrap();
        assert_eq!(result.voice_id, "new_voice_123");
    }

    #[tokio::test]
    async fn add_voice_with_files() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/voices/add"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voice_id": "new_voice_456"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let fake_audio = b"fake-audio-data";
        let req = AddVoiceRequest { name: "Cloned Voice".into(), description: None, labels: None };
        let result = client
            .voices()
            .add(&req, &[("sample.mp3", "audio/mpeg", fake_audio.as_slice())])
            .await
            .unwrap();
        assert_eq!(result.voice_id, "new_voice_456");
    }

    // -- edit --------------------------------------------------------------

    #[tokio::test]
    async fn edit_voice_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/voices/voice123/edit"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let req = EditVoiceRequest {
            name: "Updated Name".into(),
            description: Some("Updated desc".into()),
            labels: None,
        };
        let result = client.voices().edit("voice123", &req, &[]).await.unwrap();
        assert_eq!(result.status, "ok");
    }

    // -- add_sharing -------------------------------------------------------

    #[tokio::test]
    async fn add_sharing_returns_voice_id() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/voices/add/user123/voice456"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voice_id": "shared_voice_789"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result =
            client.voices().add_sharing("user123", "voice456", "My Shared Voice").await.unwrap();
        assert_eq!(result.voice_id, "shared_voice_789");
    }

    // -- get_sample_audio --------------------------------------------------

    #[tokio::test]
    async fn get_sample_audio_returns_bytes() {
        let mock_server = MockServer::start().await;
        let audio_data = b"fake-sample-audio";

        Mock::given(method("GET"))
            .and(path("/v1/voices/voice123/samples/sample456/audio"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(audio_data.as_slice(), "audio/mpeg"),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.voices().get_sample_audio("voice123", "sample456").await.unwrap();
        assert_eq!(result.as_ref(), audio_data);
    }

    // -- delete_sample -----------------------------------------------------

    #[tokio::test]
    async fn delete_sample_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/voices/voice123/samples/sample456"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.voices().delete_sample("voice123", "sample456").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    // -- multipart helpers -------------------------------------------------

    #[test]
    fn uuid_v4_simple_returns_32_char_hex() {
        let id = super::uuid_v4_simple();
        assert_eq!(id.len(), 32);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn build_add_voice_multipart_contains_name() {
        let req = AddVoiceRequest {
            name: "TestVoice".into(),
            description: Some("desc".into()),
            labels: None,
        };
        let boundary = "test-boundary";
        let body = super::build_add_voice_multipart(boundary, &req, &[]);
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("TestVoice"));
        assert!(body_str.contains("desc"));
        assert!(body_str.contains("--test-boundary--"));
    }

    #[test]
    fn build_add_voice_multipart_contains_file() {
        let req = AddVoiceRequest { name: "V".into(), description: None, labels: None };
        let boundary = "test-boundary";
        let body = super::build_add_voice_multipart(
            boundary,
            &req,
            &[("sample.mp3", "audio/mpeg", b"audio-data")],
        );
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("sample.mp3"));
        assert!(body_str.contains("audio/mpeg"));
        assert!(body_str.contains("audio-data"));
    }
}
