//! Types for the ElevenLabs Sound Generation endpoint.
//!
//! Covers the single sound-generation endpoint:
//! - `POST /v1/sound-generation` — generate a sound effect from text
//!
//! The response is raw audio bytes (`audio/mpeg`), so no response type is
//! defined here.

use serde::Serialize;

// ---------------------------------------------------------------------------
// Request
// ---------------------------------------------------------------------------

/// Request body for `POST /v1/sound-generation`.
///
/// Generates a sound effect from a text description. The response is raw
/// audio bytes.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::SoundGenerationRequest;
///
/// let req = SoundGenerationRequest {
///     text: "A large, ancient wooden door slowly opening.".into(),
///     ..Default::default()
/// };
/// assert_eq!(req.model_id, "eleven_text_to_sound_v2");
/// assert_eq!(req.prompt_influence, 0.3);
/// assert!(!req.r#loop);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SoundGenerationRequest {
    /// The text description that will be converted into a sound effect.
    pub text: String,

    /// Whether to create a sound effect that loops smoothly.
    /// Only available for the `eleven_text_to_sound_v2` model.
    #[serde(rename = "loop")]
    pub r#loop: bool,

    /// Duration of the generated sound in seconds (0.5–30.0).
    /// If `None`, the server picks an optimal duration from the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<f64>,

    /// How closely the generation follows the prompt (0.0–1.0).
    /// Higher values produce more prompt-adherent but less variable results.
    pub prompt_influence: f64,

    /// The model ID to use for sound generation.
    pub model_id: String,
}

impl Default for SoundGenerationRequest {
    fn default() -> Self {
        Self {
            text: String::new(),
            r#loop: false,
            duration_seconds: None,
            prompt_influence: 0.3,
            model_id: "eleven_text_to_sound_v2".into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn request_default_values() {
        let req = SoundGenerationRequest::default();
        assert!(req.text.is_empty());
        assert!(!req.r#loop);
        assert!(req.duration_seconds.is_none());
        assert!((req.prompt_influence - 0.3).abs() < f64::EPSILON);
        assert_eq!(req.model_id, "eleven_text_to_sound_v2");
    }

    #[test]
    fn request_minimal_serialization() {
        let req = SoundGenerationRequest { text: "Thunder rolling".into(), ..Default::default() };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj["text"], "Thunder rolling");
        assert_eq!(obj["loop"], false);
        assert!(!obj.contains_key("duration_seconds"));
        assert_eq!(obj["prompt_influence"], 0.3);
        assert_eq!(obj["model_id"], "eleven_text_to_sound_v2");
    }

    #[test]
    fn request_full_serialization() {
        let req = SoundGenerationRequest {
            text: "A cat purring softly".into(),
            r#loop: true,
            duration_seconds: Some(5.0),
            prompt_influence: 0.7,
            model_id: "eleven_text_to_sound_v2".into(),
        };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["text"], "A cat purring softly");
        assert_eq!(v["loop"], true);
        assert_eq!(v["duration_seconds"], 5.0);
        assert_eq!(v["prompt_influence"], 0.7);
    }

    #[test]
    fn loop_field_serializes_as_loop() {
        let req =
            SoundGenerationRequest { text: "beep".into(), r#loop: true, ..Default::default() };
        let json = serde_json::to_string(&req).unwrap();
        // The JSON key must be "loop", not "r#loop".
        assert!(json.contains(r#""loop":true"#) || json.contains(r#""loop": true"#));
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["loop"], true);
    }
}
