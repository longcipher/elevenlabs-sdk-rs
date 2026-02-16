//! Audio Native service providing access to Audio Native project endpoints.
//!
//! This module wraps the three Audio Native endpoints exposed by the
//! ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`create_project`](AudioNativeService::create_project) | `POST /v1/audio-native` | Create an Audio Native project |
//! | [`get_settings`](AudioNativeService::get_settings) | `GET /v1/audio-native/{project_id}/settings` | Get project settings |
//! | [`update_content`](AudioNativeService::update_content) | `POST /v1/audio-native/{project_id}/content` | Update project content |
//!
//! The create and content-update endpoints accept `multipart/form-data`.
//! The settings endpoint returns JSON.
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, types::AudioNativeCreateProjectRequest};
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! let request =
//!     AudioNativeCreateProjectRequest { name: "My Article".into(), ..Default::default() };
//! let response = client
//!     .audio_native()
//!     .create_project(&request, Some((b"<html>content</html>", "article.html", "text/html")))
//!     .await?;
//!
//! println!("Project ID: {}", response.project_id);
//! # Ok(())
//! # }
//! ```

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{
        AudioNativeCreateProjectRequest, AudioNativeCreateProjectResponse,
        AudioNativeEditContentResponse, AudioNativeUpdateContentRequest,
        GetAudioNativeProjectSettingsResponse,
    },
};

/// Audio Native service providing typed access to Audio Native project
/// endpoints.
///
/// Obtained via [`ElevenLabsClient::audio_native`].
#[derive(Debug)]
pub struct AudioNativeService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> AudioNativeService<'a> {
    /// Creates a new `AudioNativeService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Creates an Audio Native–enabled project.
    ///
    /// Calls `POST /v1/audio-native` with `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `request` — Configuration fields (name, voice, auto-convert, etc.).
    /// * `file` — Optional file as `(data, filename, content_type)`. Used to provide the article
    ///   content (HTML or plain text).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn create_project(
        &self,
        request: &AudioNativeCreateProjectRequest,
        file: Option<(&[u8], &str, &str)>,
    ) -> Result<AudioNativeCreateProjectResponse> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_create_project_multipart(&boundary, request, file);
        let content_type = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart("/v1/audio-native", body, &content_type).await
    }

    /// Retrieves settings for an Audio Native project.
    ///
    /// Calls `GET /v1/audio-native/{project_id}/settings`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID to retrieve settings for.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_settings(
        &self,
        project_id: &str,
    ) -> Result<GetAudioNativeProjectSettingsResponse> {
        let path = format!("/v1/audio-native/{project_id}/settings");
        self.client.get(&path).await
    }

    /// Updates the content of an Audio Native project.
    ///
    /// Calls `POST /v1/audio-native/{project_id}/content` with
    /// `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID to update.
    /// * `request` — Configuration fields (auto-convert, auto-publish).
    /// * `file` — Optional file as `(data, filename, content_type)`. Used to provide updated
    ///   article content (HTML or plain text).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn update_content(
        &self,
        project_id: &str,
        request: &AudioNativeUpdateContentRequest,
        file: Option<(&[u8], &str, &str)>,
    ) -> Result<AudioNativeEditContentResponse> {
        let path = format!("/v1/audio-native/{project_id}/content");
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_update_content_multipart(&boundary, request, file);
        let content_type = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart(&path, body, &content_type).await
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

/// Builds the multipart body for `POST /v1/audio-native`.
fn build_create_project_multipart(
    boundary: &str,
    request: &AudioNativeCreateProjectRequest,
    file: Option<(&[u8], &str, &str)>,
) -> Vec<u8> {
    let mut buf = Vec::new();

    // File (optional)
    if let Some((data, filename, ct)) = file {
        append_file_part(&mut buf, boundary, "file", filename, ct, data);
    }

    // name (required)
    append_text_field(&mut buf, boundary, "name", &request.name);

    // image (optional)
    if let Some(ref image) = request.image {
        append_text_field(&mut buf, boundary, "image", image);
    }

    // author (optional)
    if let Some(ref author) = request.author {
        append_text_field(&mut buf, boundary, "author", author);
    }

    // title (optional)
    if let Some(ref title) = request.title {
        append_text_field(&mut buf, boundary, "title", title);
    }

    // small (bool → string)
    append_text_field(&mut buf, boundary, "small", if request.small { "true" } else { "false" });

    // text_color (optional)
    if let Some(ref tc) = request.text_color {
        append_text_field(&mut buf, boundary, "text_color", tc);
    }

    // background_color (optional)
    if let Some(ref bg) = request.background_color {
        append_text_field(&mut buf, boundary, "background_color", bg);
    }

    // sessionization
    append_text_field(&mut buf, boundary, "sessionization", &request.sessionization.to_string());

    // voice_id (optional)
    if let Some(ref vid) = request.voice_id {
        append_text_field(&mut buf, boundary, "voice_id", vid);
    }

    // model_id (optional)
    if let Some(ref mid) = request.model_id {
        append_text_field(&mut buf, boundary, "model_id", mid);
    }

    // auto_convert (bool → string)
    append_text_field(
        &mut buf,
        boundary,
        "auto_convert",
        if request.auto_convert { "true" } else { "false" },
    );

    // apply_text_normalization (optional)
    if let Some(ref norm) = request.apply_text_normalization &&
        let Ok(json) = serde_json::to_string(norm)
    {
        let value = json.trim_matches('"');
        append_text_field(&mut buf, boundary, "apply_text_normalization", value);
    }

    // pronunciation_dictionary_locators (repeated field)
    for locator in &request.pronunciation_dictionary_locators {
        append_text_field(&mut buf, boundary, "pronunciation_dictionary_locators", locator);
    }

    buf.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    buf
}

/// Builds the multipart body for
/// `POST /v1/audio-native/{project_id}/content`.
fn build_update_content_multipart(
    boundary: &str,
    request: &AudioNativeUpdateContentRequest,
    file: Option<(&[u8], &str, &str)>,
) -> Vec<u8> {
    let mut buf = Vec::new();

    // File (optional)
    if let Some((data, filename, ct)) = file {
        append_file_part(&mut buf, boundary, "file", filename, ct, data);
    }

    // auto_convert (bool → string)
    append_text_field(
        &mut buf,
        boundary,
        "auto_convert",
        if request.auto_convert { "true" } else { "false" },
    );

    // auto_publish (bool → string)
    append_text_field(
        &mut buf,
        boundary,
        "auto_publish",
        if request.auto_publish { "true" } else { "false" },
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
        types::{AudioNativeCreateProjectRequest, AudioNativeUpdateContentRequest},
    };

    // -- create_project -----------------------------------------------------

    #[tokio::test]
    async fn create_project_returns_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/audio-native"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "project_id": "proj_abc",
                "converting": false,
                "html_snippet": "<div id='player'></div>"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request =
            AudioNativeCreateProjectRequest { name: "My Article".into(), ..Default::default() };
        let result = client
            .audio_native()
            .create_project(&request, Some((b"<html>content</html>", "article.html", "text/html")))
            .await
            .unwrap();

        assert_eq!(result.project_id, "proj_abc");
        assert!(!result.converting);
        assert_eq!(result.html_snippet, "<div id='player'></div>");
    }

    #[tokio::test]
    async fn create_project_without_file() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/audio-native"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "project_id": "proj_xyz",
                "converting": true,
                "html_snippet": "<div></div>"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = AudioNativeCreateProjectRequest {
            name: "No File".into(),
            auto_convert: true,
            ..Default::default()
        };
        let result = client.audio_native().create_project(&request, None).await.unwrap();

        assert_eq!(result.project_id, "proj_xyz");
        assert!(result.converting);
    }

    // -- get_settings -------------------------------------------------------

    #[tokio::test]
    async fn get_settings_returns_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/audio-native/proj_abc/settings"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "enabled": true,
                "snapshot_id": "snap_123",
                "settings": {
                    "title": "My Article",
                    "image": "",
                    "author": "Jane",
                    "small": false,
                    "text_color": "#000",
                    "background_color": "#FFF",
                    "sessionization": 1,
                    "audio_path": null,
                    "audio_url": null,
                    "status": "ready"
                }
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.audio_native().get_settings("proj_abc").await.unwrap();

        assert!(result.enabled);
        assert_eq!(result.snapshot_id.as_deref(), Some("snap_123"));
        let settings = result.settings.unwrap();
        assert_eq!(settings.title, "My Article");
    }

    // -- update_content -----------------------------------------------------

    #[tokio::test]
    async fn update_content_returns_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/audio-native/proj_abc/content"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "project_id": "proj_abc",
                "converting": true,
                "publishing": false,
                "html_snippet": "<div id='player'></div>"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let request = AudioNativeUpdateContentRequest { auto_convert: true, auto_publish: false };
        let result = client
            .audio_native()
            .update_content(
                "proj_abc",
                &request,
                Some((b"<html>new content</html>", "article.html", "text/html")),
            )
            .await
            .unwrap();

        assert_eq!(result.project_id, "proj_abc");
        assert!(result.converting);
        assert!(!result.publishing);
    }

    // -- multipart helpers --------------------------------------------------

    #[test]
    fn build_create_project_multipart_contains_fields() {
        let request = AudioNativeCreateProjectRequest {
            name: "Test Project".into(),
            author: Some("Author".into()),
            auto_convert: true,
            ..Default::default()
        };
        let boundary = "test-boundary";
        let body = super::build_create_project_multipart(
            boundary,
            &request,
            Some((b"<p>content</p>", "content.html", "text/html")),
        );
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("content.html"));
        assert!(body_str.contains("text/html"));
        assert!(body_str.contains("Test Project"));
        assert!(body_str.contains("Author"));
        assert!(body_str.contains("auto_convert"));
        assert!(body_str.contains("true"));
        assert!(body_str.contains("--test-boundary--"));
    }

    #[test]
    fn build_update_content_multipart_contains_fields() {
        let request = AudioNativeUpdateContentRequest { auto_convert: true, auto_publish: true };
        let boundary = "test-boundary";
        let body = super::build_update_content_multipart(boundary, &request, None);
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("auto_convert"));
        assert!(body_str.contains("auto_publish"));
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
