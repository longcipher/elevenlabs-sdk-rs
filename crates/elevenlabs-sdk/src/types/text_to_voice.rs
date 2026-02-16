//! Types for the ElevenLabs Text-to-Voice (voice design) endpoints.
//!
//! Covers:
//! - `POST /v1/text-to-voice/create-previews` — generate voice previews
//! - `POST /v1/text-to-voice` — create a voice from a preview
//! - `POST /v1/text-to-voice/design` — design a voice
//! - `POST /v1/text-to-voice/{voice_id}/remix` — remix a voice
//! - `GET  /v1/text-to-voice/{generated_voice_id}/stream` — stream preview audio

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Voice Design Model
// ---------------------------------------------------------------------------

/// Model identifiers available for voice design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoiceDesignModel {
    /// Multilingual text-to-voice v2.
    #[serde(rename = "eleven_multilingual_ttv_v2")]
    MultilingualTtvV2,
    /// Text-to-voice v3.
    #[serde(rename = "eleven_ttv_v3")]
    TtvV3,
}

// ---------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------

/// Request body for `POST /v1/text-to-voice/create-previews`.
///
/// Generates voice previews from a description. Only `voice_description`
/// is required.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VoicePreviewsRequest {
    /// A text description of the desired voice characteristics.
    pub voice_description: String,

    /// Optional text to speak in the preview. If omitted, text is
    /// auto-generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Whether to auto-generate preview text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_generate_text: Option<bool>,

    /// Loudness adjustment for the generated voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loudness: Option<f64>,

    /// Quality parameter (higher = better quality, slower generation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<f64>,

    /// Seed for deterministic generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Guidance scale for voice design.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,

    /// Whether to enhance the generated audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_enhance: Option<bool>,
}

/// Request body for `POST /v1/text-to-voice/design`.
///
/// Designs a voice from a description with more options than
/// [`VoicePreviewsRequest`].
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VoiceDesignRequest {
    /// A text description of the desired voice characteristics.
    pub voice_description: String,

    /// Model to use for voice design.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<VoiceDesignModel>,

    /// Optional text to speak in the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Whether to auto-generate preview text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_generate_text: Option<bool>,

    /// Loudness adjustment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loudness: Option<f64>,

    /// Seed for deterministic generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Guidance scale for voice design.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,

    /// Whether to stream the preview audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_previews: Option<bool>,

    /// Whether to enhance the generated audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_enhance: Option<bool>,

    /// Quality parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<f64>,

    /// Base64-encoded reference audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_audio_base64: Option<String>,

    /// How strongly to match the reference audio prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_strength: Option<f64>,
}

/// Request body for `POST /v1/text-to-voice` (create voice from preview).
///
/// Creates a permanent voice from a previously generated voice preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateVoiceFromPreviewRequest {
    /// Name for the new voice.
    pub voice_name: String,

    /// Description for the new voice (20-1000 characters).
    pub voice_description: String,

    /// The generated voice ID obtained from the preview response.
    pub generated_voice_id: String,

    /// Optional metadata labels (e.g. `{"language": "en"}`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,

    /// IDs of previews that were played but not selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub played_not_selected_voice_ids: Option<Vec<String>>,
}

/// Request body for `POST /v1/text-to-voice/{voice_id}/remix`.
///
/// Remixes an existing voice with a new description and optional
/// configuration.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VoiceRemixRequest {
    /// Description of the changes to make to the voice.
    pub voice_description: String,

    /// Optional text to speak in the preview (100-1000 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Whether to automatically generate a text suitable for the
    /// voice description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_generate_text: Option<bool>,

    /// Controls the volume level. -1 is quietest, 1 is loudest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loudness: Option<f64>,

    /// Seed for deterministic generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Guidance scale for voice design.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,

    /// Whether to stream preview audio rather than embed in response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_previews: Option<bool>,

    /// The remixing session ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remixing_session_id: Option<String>,

    /// The ID of the remixing session iteration to attach to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remixing_session_iteration_id: Option<String>,

    /// Controls the balance of prompt versus reference audio (0.0-1.0).
    /// Only supported with the `eleven_ttv_v3` model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_strength: Option<f64>,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

/// A single voice preview returned by the API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoicePreviewResponse {
    /// Base64-encoded audio data of the preview.
    pub audio_base_64: String,
    /// Generated voice ID that can be used to create a permanent voice.
    pub generated_voice_id: String,
    /// Media type of the audio (e.g. `"audio/mpeg"`).
    pub media_type: String,
    /// Duration of the preview in seconds.
    pub duration_secs: f64,
    /// Language of the generated voice.
    pub language: serde_json::Value,
}

/// Response from `POST /v1/text-to-voice/create-previews`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoicePreviewsResponse {
    /// The generated voice previews.
    pub previews: Vec<VoicePreviewResponse>,
    /// The text used to preview the voices.
    pub text: String,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn voice_design_model_round_trip() {
        let v = VoiceDesignModel::MultilingualTtvV2;
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, r#""eleven_multilingual_ttv_v2""#);
        let back: VoiceDesignModel = serde_json::from_str(&json).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn voice_previews_request_serialize() {
        let req = VoicePreviewsRequest {
            voice_description: "A warm female voice".into(),
            text: Some("Hello world".into()),
            auto_generate_text: None,
            loudness: None,
            quality: None,
            seed: None,
            guidance_scale: None,
            should_enhance: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"voice_description\":\"A warm female voice\""));
        assert!(json.contains("\"text\":\"Hello world\""));
        assert!(!json.contains("auto_generate_text"));
    }

    #[test]
    fn create_voice_from_preview_serialize() {
        let mut labels = HashMap::new();
        labels.insert("language".into(), "en".into());
        let req = CreateVoiceFromPreviewRequest {
            voice_name: "My Voice".into(),
            voice_description: "A warm and friendly voice".into(),
            generated_voice_id: "gen123".into(),
            labels: Some(labels),
            played_not_selected_voice_ids: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"voice_name\":\"My Voice\""));
        assert!(json.contains("\"generated_voice_id\":\"gen123\""));
        assert!(!json.contains("played_not_selected_voice_ids"));
    }

    #[test]
    fn voice_previews_response_deserialize() {
        let json = r#"{
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
        }"#;
        let resp: VoicePreviewsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.previews.len(), 1);
        assert_eq!(resp.previews[0].generated_voice_id, "gen1");
        assert_eq!(resp.text, "Hello world");
    }

    #[test]
    fn voice_design_request_serialize() {
        let req = VoiceDesignRequest {
            voice_description: "Warm male narrator".into(),
            model_id: Some(VoiceDesignModel::TtvV3),
            text: None,
            auto_generate_text: Some(true),
            loudness: None,
            seed: None,
            guidance_scale: Some(3.0),
            stream_previews: None,
            should_enhance: None,
            quality: None,
            reference_audio_base64: None,
            prompt_strength: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"model_id\":\"eleven_ttv_v3\""));
        assert!(json.contains("\"auto_generate_text\":true"));
        assert!(!json.contains("\"text\":"));
    }
}
