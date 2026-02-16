//! Types for the ElevenLabs Audio Native endpoints.
//!
//! Covers the three Audio Native endpoints:
//! - `POST /v1/audio-native` — create an Audio Native–enabled project
//! - `GET /v1/audio-native/{project_id}/settings` — get project settings
//! - `POST /v1/audio-native/{project_id}/content` — update project content
//!
//! The create and content-update endpoints accept `multipart/form-data`.
//! The types below capture the **non-file** fields the caller provides.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Text Normalization (Audio Native–specific)
// ---------------------------------------------------------------------------

/// Controls how text normalization is applied for Audio Native projects.
///
/// This extends the standard three-mode normalization (`auto`, `on`, `off`)
/// with an additional `apply_english` mode that forces English-language
/// normalization rules.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioNativeTextNormalization {
    /// System decides automatically.
    #[default]
    Auto,
    /// Always apply text normalization.
    On,
    /// Never apply text normalization.
    Off,
    /// Apply English-language text normalization rules.
    ApplyEnglish,
}

// ---------------------------------------------------------------------------
// Project status
// ---------------------------------------------------------------------------

/// Current state of an Audio Native project.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioNativeProjectStatus {
    /// The project is still being processed.
    Processing,
    /// The project is ready.
    #[default]
    Ready,
}

// ---------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------

/// Request fields for `POST /v1/audio-native`.
///
/// Creates an Audio Native–enabled project. Uses `multipart/form-data`;
/// the file itself is provided separately in the service layer.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::AudioNativeCreateProjectRequest;
///
/// let req = AudioNativeCreateProjectRequest { name: "My Article".into(), ..Default::default() };
/// assert_eq!(req.name, "My Article");
/// assert!(!req.auto_convert);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct AudioNativeCreateProjectRequest {
    /// Project name (required).
    pub name: String,

    /// Image URL used in the player (deprecated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Author shown in the player and inserted at the start of the article.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Title shown in the player and inserted at the top of the article.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Whether to use the small player (deprecated).
    pub small: bool,

    /// Text color used in the player (CSS hex).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,

    /// Background color used in the player (CSS hex).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,

    /// How many minutes to persist the session across page reloads (deprecated).
    pub sessionization: i64,

    /// Voice ID used to voice the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,

    /// TTS model ID used in the player.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,

    /// Whether to auto-convert the project to audio.
    pub auto_convert: bool,

    /// Text normalization mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<AudioNativeTextNormalization>,

    /// Pronunciation dictionary locators encoded as JSON strings.
    ///
    /// Each element is a JSON-encoded string containing
    /// `pronunciation_dictionary_id` and `version_id`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pronunciation_dictionary_locators: Vec<String>,
}

/// Request fields for `POST /v1/audio-native/{project_id}/content`.
///
/// Updates an Audio Native project's content. Uses `multipart/form-data`;
/// the file itself is provided separately in the service layer.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::AudioNativeUpdateContentRequest;
///
/// let req = AudioNativeUpdateContentRequest::default();
/// assert!(!req.auto_convert);
/// assert!(!req.auto_publish);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct AudioNativeUpdateContentRequest {
    /// Whether to auto-convert the project to audio.
    pub auto_convert: bool,

    /// Whether to auto-publish the new project snapshot after conversion.
    pub auto_publish: bool,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

/// Response from `POST /v1/audio-native`.
///
/// Returned after successfully creating an Audio Native project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioNativeCreateProjectResponse {
    /// The ID of the created Audio Native project.
    pub project_id: String,
    /// Whether the project is currently being converted.
    pub converting: bool,
    /// The HTML snippet to embed the Audio Native player.
    pub html_snippet: String,
}

/// Response from `POST /v1/audio-native/{project_id}/content`.
///
/// Returned after successfully updating the content of an Audio Native project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioNativeEditContentResponse {
    /// The ID of the project.
    pub project_id: String,
    /// Whether the project is currently being converted.
    pub converting: bool,
    /// Whether the project is currently being published.
    pub publishing: bool,
    /// The HTML snippet to embed the Audio Native player.
    pub html_snippet: String,
}

/// Settings for an Audio Native project.
///
/// Contains player configuration such as title, author, colors, and the
/// current audio path/URL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioNativeProjectSettings {
    /// The title of the project.
    pub title: String,
    /// The image URL of the project.
    pub image: String,
    /// The author of the project.
    pub author: String,
    /// Whether the project uses the small player.
    pub small: bool,
    /// The text color (CSS hex).
    pub text_color: String,
    /// The background color (CSS hex).
    pub background_color: String,
    /// How many minutes to persist the session across page reloads.
    pub sessionization: i64,
    /// The path of the audio file, if available.
    pub audio_path: Option<String>,
    /// The URL of the audio file, if available.
    pub audio_url: Option<String>,
    /// Current state of the project.
    #[serde(default)]
    pub status: AudioNativeProjectStatus,
}

/// Response from `GET /v1/audio-native/{project_id}/settings`.
///
/// Returns whether the project is enabled and its player settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetAudioNativeProjectSettingsResponse {
    /// Whether the project is enabled.
    pub enabled: bool,
    /// The ID of the latest snapshot of the project.
    pub snapshot_id: Option<String>,
    /// The player settings of the project.
    pub settings: Option<AudioNativeProjectSettings>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    // -- AudioNativeTextNormalization ----------------------------------------

    #[test]
    fn text_normalization_default_is_auto() {
        assert_eq!(AudioNativeTextNormalization::default(), AudioNativeTextNormalization::Auto);
    }

    #[test]
    fn text_normalization_serde_round_trip() {
        for variant in [
            AudioNativeTextNormalization::Auto,
            AudioNativeTextNormalization::On,
            AudioNativeTextNormalization::Off,
            AudioNativeTextNormalization::ApplyEnglish,
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: AudioNativeTextNormalization = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn text_normalization_serde_names() {
        assert_eq!(
            serde_json::to_string(&AudioNativeTextNormalization::Auto).unwrap(),
            r#""auto""#
        );
        assert_eq!(serde_json::to_string(&AudioNativeTextNormalization::On).unwrap(), r#""on""#);
        assert_eq!(serde_json::to_string(&AudioNativeTextNormalization::Off).unwrap(), r#""off""#);
        assert_eq!(
            serde_json::to_string(&AudioNativeTextNormalization::ApplyEnglish).unwrap(),
            r#""apply_english""#
        );
    }

    // -- AudioNativeCreateProjectRequest ------------------------------------

    #[test]
    fn create_request_default_values() {
        let req = AudioNativeCreateProjectRequest::default();
        assert!(req.name.is_empty());
        assert!(req.image.is_none());
        assert!(req.author.is_none());
        assert!(req.title.is_none());
        assert!(!req.small);
        assert!(req.text_color.is_none());
        assert!(req.background_color.is_none());
        assert_eq!(req.sessionization, 0);
        assert!(req.voice_id.is_none());
        assert!(req.model_id.is_none());
        assert!(!req.auto_convert);
        assert!(req.apply_text_normalization.is_none());
        assert!(req.pronunciation_dictionary_locators.is_empty());
    }

    #[test]
    fn create_request_minimal_serialization() {
        let req =
            AudioNativeCreateProjectRequest { name: "My Article".into(), ..Default::default() };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj["name"], "My Article");
        assert_eq!(obj["small"], false);
        assert_eq!(obj["sessionization"], 0);
        assert_eq!(obj["auto_convert"], false);
        // Optional fields should be absent
        assert!(!obj.contains_key("image"));
        assert!(!obj.contains_key("author"));
        assert!(!obj.contains_key("title"));
        assert!(!obj.contains_key("text_color"));
        assert!(!obj.contains_key("background_color"));
        assert!(!obj.contains_key("voice_id"));
        assert!(!obj.contains_key("model_id"));
        assert!(!obj.contains_key("apply_text_normalization"));
        assert!(!obj.contains_key("pronunciation_dictionary_locators"));
    }

    #[test]
    fn create_request_full_serialization() {
        let req = AudioNativeCreateProjectRequest {
            name: "My Article".into(),
            image: Some("https://example.com/img.jpg".into()),
            author: Some("Jane".into()),
            title: Some("The Title".into()),
            small: true,
            text_color: Some("#000000".into()),
            background_color: Some("#FFFFFF".into()),
            sessionization: 5,
            voice_id: Some("voice123".into()),
            model_id: Some("model456".into()),
            auto_convert: true,
            apply_text_normalization: Some(AudioNativeTextNormalization::On),
            pronunciation_dictionary_locators: vec![
                r#"{"pronunciation_dictionary_id":"abc","version_id":"def"}"#.into(),
            ],
        };
        let json = serde_json::to_string_pretty(&req).unwrap();
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"image\""));
        assert!(json.contains("\"author\""));
        assert!(json.contains("\"small\""));
        assert!(json.contains("\"voice_id\""));
        assert!(json.contains("\"auto_convert\""));
        assert!(json.contains("\"apply_text_normalization\""));
        assert!(json.contains("\"pronunciation_dictionary_locators\""));
    }

    // -- AudioNativeUpdateContentRequest ------------------------------------

    #[test]
    fn update_content_request_default_values() {
        let req = AudioNativeUpdateContentRequest::default();
        assert!(!req.auto_convert);
        assert!(!req.auto_publish);
    }

    #[test]
    fn update_content_request_serialization() {
        let req = AudioNativeUpdateContentRequest { auto_convert: true, auto_publish: true };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["auto_convert"], true);
        assert_eq!(v["auto_publish"], true);
    }

    // -- AudioNativeCreateProjectResponse -----------------------------------

    #[test]
    fn create_response_deserialization() {
        let json = r#"{
            "project_id": "JBFqnCBsd6RMkjVDRZzb",
            "converting": false,
            "html_snippet": "<div id='audio-native-player'></div>"
        }"#;
        let resp: AudioNativeCreateProjectResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.project_id, "JBFqnCBsd6RMkjVDRZzb");
        assert!(!resp.converting);
        assert_eq!(resp.html_snippet, "<div id='audio-native-player'></div>");
    }

    // -- AudioNativeEditContentResponse -------------------------------------

    #[test]
    fn edit_content_response_deserialization() {
        let json = r#"{
            "project_id": "JBFqnCBsd6RMkjVDRZzb",
            "converting": false,
            "publishing": true,
            "html_snippet": "<div id='audio-native-player'></div>"
        }"#;
        let resp: AudioNativeEditContentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.project_id, "JBFqnCBsd6RMkjVDRZzb");
        assert!(!resp.converting);
        assert!(resp.publishing);
    }

    // -- AudioNativeProjectSettings -----------------------------------------

    #[test]
    fn project_settings_deserialization() {
        let json = r##"{
            "title": "My Project",
            "image": "https://example.com/image.jpg",
            "author": "John Doe",
            "small": false,
            "text_color": "#000000",
            "background_color": "#FFFFFF",
            "sessionization": 1,
            "audio_path": "audio/my_project.mp3",
            "audio_url": "https://example.com/audio/my_project.mp3",
            "status": "ready"
        }"##;
        let settings: AudioNativeProjectSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.title, "My Project");
        assert_eq!(settings.author, "John Doe");
        assert!(!settings.small);
        assert_eq!(settings.sessionization, 1);
        assert_eq!(settings.audio_path.as_deref(), Some("audio/my_project.mp3"));
        assert_eq!(settings.status, AudioNativeProjectStatus::Ready);
    }

    #[test]
    fn project_settings_nullable_fields() {
        let json = r##"{
            "title": "Test",
            "image": "",
            "author": "A",
            "small": false,
            "text_color": "#000",
            "background_color": "#FFF",
            "sessionization": 0
        }"##;
        let settings: AudioNativeProjectSettings = serde_json::from_str(json).unwrap();
        assert!(settings.audio_path.is_none());
        assert!(settings.audio_url.is_none());
        assert_eq!(settings.status, AudioNativeProjectStatus::Ready);
    }

    // -- GetAudioNativeProjectSettingsResponse --------------------------------

    #[test]
    fn get_settings_response_full_deserialization() {
        let json = r##"{
            "enabled": true,
            "snapshot_id": "JBFqnCBsd6RMkjVDRZzb",
            "settings": {
                "title": "My Project",
                "image": "https://example.com/image.jpg",
                "author": "John Doe",
                "small": false,
                "text_color": "#000000",
                "background_color": "#FFFFFF",
                "sessionization": 1,
                "audio_path": null,
                "audio_url": null,
                "status": "processing"
            }
        }"##;
        let resp: GetAudioNativeProjectSettingsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.enabled);
        assert_eq!(resp.snapshot_id.as_deref(), Some("JBFqnCBsd6RMkjVDRZzb"));
        let settings = resp.settings.unwrap();
        assert_eq!(settings.status, AudioNativeProjectStatus::Processing);
    }

    #[test]
    fn get_settings_response_minimal_deserialization() {
        let json = r#"{"enabled": false}"#;
        let resp: GetAudioNativeProjectSettingsResponse = serde_json::from_str(json).unwrap();
        assert!(!resp.enabled);
        assert!(resp.snapshot_id.is_none());
        assert!(resp.settings.is_none());
    }

    // -- AudioNativeProjectStatus -------------------------------------------

    #[test]
    fn project_status_default_is_ready() {
        assert_eq!(AudioNativeProjectStatus::default(), AudioNativeProjectStatus::Ready);
    }
}
