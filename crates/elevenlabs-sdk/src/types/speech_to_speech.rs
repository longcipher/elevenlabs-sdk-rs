//! Types for the ElevenLabs Speech-to-Speech endpoints.
//!
//! Covers the two S2S endpoints:
//! - `POST /v1/speech-to-speech/{voice_id}` — convert speech
//! - `POST /v1/speech-to-speech/{voice_id}/stream` — stream converted speech
//!
//! Both endpoints accept multipart/form-data with an audio file and optional
//! configuration fields. The response is raw audio bytes (`audio/mpeg`).
//!
//! The types below capture the **non-file** fields the caller provides.
//! Actual multipart encoding is handled in the service layer.

use serde::{Deserialize, Serialize};

use super::common::VoiceSettings;

// ---------------------------------------------------------------------------
// Input Audio Format
// ---------------------------------------------------------------------------

/// Format of the input audio file sent to speech-to-speech endpoints.
///
/// Specifying `PcmS16le16` (16-bit PCM, 16 kHz, mono, little-endian) avoids
/// server-side decoding and reduces latency. Use `Other` (the default) for
/// any encoded format (MP3, WAV, OGG, etc.).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeechToSpeechFileFormat {
    /// 16-bit PCM at 16 kHz, mono, little-endian. Lowest latency.
    #[serde(rename = "pcm_s16le_16")]
    PcmS16le16,
    /// Any other encoded audio format (MP3, WAV, OGG, etc.).
    #[default]
    Other,
}

// ---------------------------------------------------------------------------
// Request
// ---------------------------------------------------------------------------

/// Configuration fields for `POST /v1/speech-to-speech/{voice_id}` and
/// `POST /v1/speech-to-speech/{voice_id}/stream`.
///
/// Both endpoints use `multipart/form-data`. This struct captures every
/// non-file field; the audio file itself is provided separately when
/// building the multipart request in the service layer.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::SpeechToSpeechRequest;
///
/// let req = SpeechToSpeechRequest::default();
/// assert_eq!(req.model_id, "eleven_english_sts_v2");
/// assert!(!req.remove_background_noise);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpeechToSpeechRequest {
    /// Identifier of the model to use. The model must support speech-to-speech
    /// (check `can_do_voice_conversion` on the model object).
    pub model_id: String,

    /// Voice settings overriding the stored defaults for the given voice.
    /// Sent as a JSON-encoded string in the multipart form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,

    /// Seed for deterministic generation. Must be between 0 and 4 294 967 295.
    /// Determinism is not guaranteed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// When `true`, removes background noise from the input audio using the
    /// ElevenLabs audio-isolation model. Only applies to Voice Changer.
    pub remove_background_noise: bool,

    /// Format of the input audio file. Using `PcmS16le16` reduces latency
    /// because the server skips decoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<SpeechToSpeechFileFormat>,
}

impl Default for SpeechToSpeechRequest {
    fn default() -> Self {
        Self {
            model_id: "eleven_english_sts_v2".into(),
            voice_settings: None,
            seed: None,
            remove_background_noise: false,
            file_format: None,
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

    // -- SpeechToSpeechFileFormat --------------------------------------------

    #[test]
    fn file_format_default_is_other() {
        assert_eq!(SpeechToSpeechFileFormat::default(), SpeechToSpeechFileFormat::Other);
    }

    #[test]
    fn file_format_serde_round_trip() {
        for variant in [SpeechToSpeechFileFormat::PcmS16le16, SpeechToSpeechFileFormat::Other] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: SpeechToSpeechFileFormat = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn file_format_serde_names() {
        assert_eq!(
            serde_json::to_string(&SpeechToSpeechFileFormat::PcmS16le16).unwrap(),
            r#""pcm_s16le_16""#
        );
        assert_eq!(serde_json::to_string(&SpeechToSpeechFileFormat::Other).unwrap(), r#""other""#);
    }

    // -- SpeechToSpeechRequest -----------------------------------------------

    #[test]
    fn request_default_values() {
        let req = SpeechToSpeechRequest::default();
        assert_eq!(req.model_id, "eleven_english_sts_v2");
        assert!(req.voice_settings.is_none());
        assert!(req.seed.is_none());
        assert!(!req.remove_background_noise);
        assert!(req.file_format.is_none());
    }

    #[test]
    fn request_minimal_serialization() {
        let req = SpeechToSpeechRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = v.as_object().unwrap();
        // Only model_id and remove_background_noise should always appear.
        assert_eq!(obj["model_id"], "eleven_english_sts_v2");
        assert_eq!(obj["remove_background_noise"], false);
        assert!(!obj.contains_key("voice_settings"));
        assert!(!obj.contains_key("seed"));
        assert!(!obj.contains_key("file_format"));
    }

    #[test]
    fn request_full_serialization() {
        let req = SpeechToSpeechRequest {
            model_id: "eleven_english_sts_v2".into(),
            voice_settings: Some(VoiceSettings {
                stability: Some(0.5),
                similarity_boost: Some(0.75),
                style: None,
                use_speaker_boost: None,
                speed: None,
            }),
            seed: Some(42),
            remove_background_noise: true,
            file_format: Some(SpeechToSpeechFileFormat::PcmS16le16),
        };
        let json = serde_json::to_string_pretty(&req).unwrap();
        assert!(json.contains("\"model_id\""));
        assert!(json.contains("\"voice_settings\""));
        assert!(json.contains("\"seed\""));
        assert!(json.contains("\"remove_background_noise\""));
        assert!(json.contains("\"file_format\""));

        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["seed"], 42);
        assert_eq!(v["remove_background_noise"], true);
        assert_eq!(v["file_format"], "pcm_s16le_16");
    }

    #[test]
    fn request_voice_settings_json_encoded_string() {
        // The S2S API expects voice_settings as a JSON-encoded string inside
        // the multipart form. Verify we can serialize VoiceSettings to a
        // JSON string and deserialize it back.
        let vs = VoiceSettings {
            stability: Some(0.5),
            similarity_boost: Some(0.75),
            style: None,
            use_speaker_boost: Some(true),
            speed: None,
        };
        let json_str = serde_json::to_string(&vs).unwrap();
        let back: VoiceSettings = serde_json::from_str(&json_str).unwrap();
        assert_eq!(vs, back);
    }
}
