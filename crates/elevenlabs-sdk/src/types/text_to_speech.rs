//! Types for the ElevenLabs Text-to-Speech endpoints.
//!
//! Covers the four TTS endpoints:
//! - `POST /v1/text-to-speech/{voice_id}` (convert)
//! - `POST /v1/text-to-speech/{voice_id}/stream` (stream)
//! - `POST /v1/text-to-speech/{voice_id}/with-timestamps` (convert with timestamps)
//! - `POST /v1/text-to-speech/{voice_id}/stream/with-timestamps` (stream with timestamps)
//!
//! All four endpoints share the same request body shape; only the response
//! differs (audio bytes vs. JSON with alignment data).

use serde::{Deserialize, Serialize};

use super::common::VoiceSettings;

// ---------------------------------------------------------------------------
// Text Normalization
// ---------------------------------------------------------------------------

/// Controls how text normalization is applied before synthesis.
///
/// When set to `Auto` (the default), the system automatically decides whether
/// to apply text normalization (e.g., spelling out numbers). `On` forces
/// normalization on every request; `Off` skips it entirely.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextNormalization {
    /// System decides automatically.
    #[default]
    Auto,
    /// Always apply text normalization.
    On,
    /// Never apply text normalization.
    Off,
}

// ---------------------------------------------------------------------------
// Pronunciation Dictionary Locator
// ---------------------------------------------------------------------------

/// Locator for a specific version of a pronunciation dictionary.
///
/// Used inside [`TextToSpeechRequest::pronunciation_dictionary_locators`] to
/// apply pronunciation overrides. Up to 3 locators may be specified per
/// request and they are applied in order.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PronunciationDictionaryVersionLocator {
    /// The ID of the pronunciation dictionary.
    pub pronunciation_dictionary_id: String,
    /// The ID of the version. If not provided, the latest version is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Request
// ---------------------------------------------------------------------------

/// Request body for all four TTS endpoints.
///
/// The request body is identical across convert, stream, convert-with-timestamps,
/// and stream-with-timestamps. Only `text` is required; all other fields are
/// optional and will use server defaults when omitted.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::{TextToSpeechRequest, VoiceSettings};
///
/// let req = TextToSpeechRequest::new("Hello, world!");
/// assert_eq!(req.text, "Hello, world!");
/// assert!(req.model_id.is_none());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TextToSpeechRequest {
    /// The text to convert to speech.
    pub text: String,

    /// Identifier of the model to use (e.g. `"eleven_multilingual_v2"`).
    /// Query available models via `GET /v1/models`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,

    /// ISO 639-1 language code used to enforce a language for the model
    /// and text normalization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// Voice settings overriding the stored defaults for the given voice.
    /// Applied only to this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,

    /// Pronunciation dictionary locators applied to the text in order.
    /// Up to 3 locators per request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>,

    /// Seed for deterministic generation. Must be between 0 and 4294967295.
    /// Determinism is not guaranteed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// Text that came before this request's text. Improves speech continuity
    /// when concatenating multiple generations. Ignored if
    /// `previous_request_ids` is also set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_text: Option<String>,

    /// Text that comes after this request's text. Improves speech continuity
    /// when concatenating multiple generations. Ignored if
    /// `next_request_ids` is also set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_text: Option<String>,

    /// Request IDs of samples generated before this one. Improves continuity
    /// when splitting a large task. Maximum of 3 IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_request_ids: Option<Vec<String>>,

    /// Request IDs of samples that come after this one. Useful for
    /// regenerating a clip while maintaining natural flow. Maximum of 3 IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_request_ids: Option<Vec<String>>,

    /// Controls text normalization mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<TextNormalization>,

    /// Enables language-specific text normalization. Can heavily increase
    /// latency. Currently only supported for Japanese.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_language_text_normalization: Option<bool>,
}

impl TextToSpeechRequest {
    /// Creates a new TTS request with only the required `text` field.
    ///
    /// All optional fields default to `None`.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            model_id: None,
            language_code: None,
            voice_settings: None,
            pronunciation_dictionary_locators: None,
            seed: None,
            previous_text: None,
            next_text: None,
            previous_request_ids: None,
            next_request_ids: None,
            apply_text_normalization: None,
            apply_language_text_normalization: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

/// Character-level alignment timestamps.
///
/// Each element in `characters` has a corresponding start and end time in
/// seconds at the same index position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterAlignment {
    /// Individual characters (may include spaces, punctuation, etc.).
    pub characters: Vec<String>,
    /// Start time in seconds for each character.
    pub character_start_times_seconds: Vec<f64>,
    /// End time in seconds for each character.
    pub character_end_times_seconds: Vec<f64>,
}

/// Response from `POST /v1/text-to-speech/{voice_id}/with-timestamps`.
///
/// Contains the full audio as a base64-encoded string along with optional
/// character-level alignment data.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::AudioWithTimestampsResponse;
///
/// let json = r#"{
///     "audio_base64": "SGVsbG8=",
///     "alignment": {
///         "characters": ["H","e","l","l","o"],
///         "character_start_times_seconds": [0.0,0.1,0.2,0.3,0.4],
///         "character_end_times_seconds": [0.1,0.2,0.3,0.4,0.5]
///     },
///     "normalized_alignment": null
/// }"#;
/// let resp: AudioWithTimestampsResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(resp.audio_base64, "SGVsbG8=");
/// assert!(resp.alignment.is_some());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioWithTimestampsResponse {
    /// Base64-encoded audio data.
    pub audio_base64: String,
    /// Character-level alignment for the original text.
    pub alignment: Option<CharacterAlignment>,
    /// Character-level alignment for the normalized text.
    pub normalized_alignment: Option<CharacterAlignment>,
}

/// A single chunk from `POST /v1/text-to-speech/{voice_id}/stream/with-timestamps`.
///
/// The streaming-with-timestamps endpoint delivers multiple chunks, each
/// containing a portion of the audio and its corresponding alignment data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamingAudioChunkWithTimestamps {
    /// Base64-encoded audio data for this chunk.
    pub audio_base64: String,
    /// Character-level alignment for the original text in this chunk.
    pub alignment: Option<CharacterAlignment>,
    /// Character-level alignment for the normalized text in this chunk.
    pub normalized_alignment: Option<CharacterAlignment>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    // -- TextNormalization ---------------------------------------------------

    #[test]
    fn text_normalization_default_is_auto() {
        assert_eq!(TextNormalization::default(), TextNormalization::Auto);
    }

    #[test]
    fn text_normalization_serde_round_trip() {
        for variant in [TextNormalization::Auto, TextNormalization::On, TextNormalization::Off] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: TextNormalization = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn text_normalization_serde_names() {
        assert_eq!(serde_json::to_string(&TextNormalization::Auto).unwrap(), r#""auto""#);
        assert_eq!(serde_json::to_string(&TextNormalization::On).unwrap(), r#""on""#);
        assert_eq!(serde_json::to_string(&TextNormalization::Off).unwrap(), r#""off""#);
    }

    // -- PronunciationDictionaryVersionLocator --------------------------------

    #[test]
    fn pronunciation_dict_locator_round_trip() {
        let locator = PronunciationDictionaryVersionLocator {
            pronunciation_dictionary_id: "dict_abc".into(),
            version_id: Some("v1".into()),
        };
        let json = serde_json::to_string(&locator).unwrap();
        let back: PronunciationDictionaryVersionLocator = serde_json::from_str(&json).unwrap();
        assert_eq!(locator, back);
    }

    #[test]
    fn pronunciation_dict_locator_omits_none_version() {
        let locator = PronunciationDictionaryVersionLocator {
            pronunciation_dictionary_id: "dict_abc".into(),
            version_id: None,
        };
        let json = serde_json::to_string(&locator).unwrap();
        assert!(!json.contains("version_id"));
    }

    #[test]
    fn pronunciation_dict_locator_deserialize_from_api_example() {
        let json = r#"{"pronunciation_dictionary_id":"test","version_id":"id2"}"#;
        let loc: PronunciationDictionaryVersionLocator = serde_json::from_str(json).unwrap();
        assert_eq!(loc.pronunciation_dictionary_id, "test");
        assert_eq!(loc.version_id.as_deref(), Some("id2"));
    }

    // -- TextToSpeechRequest -------------------------------------------------

    #[test]
    fn tts_request_new_minimal() {
        let req = TextToSpeechRequest::new("Hello");
        assert_eq!(req.text, "Hello");
        assert!(req.model_id.is_none());
        assert!(req.voice_settings.is_none());
    }

    #[test]
    fn tts_request_minimal_serializes_only_text() {
        let req = TextToSpeechRequest::new("Hello");
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert_eq!(obj["text"], "Hello");
    }

    #[test]
    fn tts_request_full_serialization() {
        let req = TextToSpeechRequest {
            text: "This is a test.".into(),
            model_id: Some("eleven_multilingual_v2".into()),
            language_code: Some("en".into()),
            voice_settings: Some(VoiceSettings {
                stability: Some(0.5),
                similarity_boost: Some(0.75),
                style: None,
                use_speaker_boost: None,
                speed: None,
            }),
            pronunciation_dictionary_locators: Some(vec![PronunciationDictionaryVersionLocator {
                pronunciation_dictionary_id: "test".into(),
                version_id: Some("id2".into()),
            }]),
            seed: Some(12345),
            previous_text: Some("Before.".into()),
            next_text: Some("After.".into()),
            previous_request_ids: Some(vec!["req1".into(), "req2".into()]),
            next_request_ids: Some(vec!["req3".into()]),
            apply_text_normalization: Some(TextNormalization::Auto),
            apply_language_text_normalization: Some(false),
        };
        let json = serde_json::to_string_pretty(&req).unwrap();
        // Verify key fields are present.
        assert!(json.contains("\"text\""));
        assert!(json.contains("\"model_id\""));
        assert!(json.contains("\"voice_settings\""));
        assert!(json.contains("\"pronunciation_dictionary_locators\""));
        assert!(json.contains("\"seed\""));
        assert!(json.contains("\"previous_text\""));
        assert!(json.contains("\"next_text\""));
        assert!(json.contains("\"previous_request_ids\""));
        assert!(json.contains("\"next_request_ids\""));
        assert!(json.contains("\"apply_text_normalization\""));
        assert!(json.contains("\"apply_language_text_normalization\""));

        // Verify the JSON deserializes as a valid object.
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["text"], "This is a test.");
        assert_eq!(v["model_id"], "eleven_multilingual_v2");
        assert_eq!(v["seed"], 12345);
        assert_eq!(v["apply_text_normalization"], "auto");
    }

    // -- CharacterAlignment --------------------------------------------------

    #[test]
    fn character_alignment_deserialize() {
        let json = r#"{
            "characters": ["H","e","l","l","o"],
            "character_start_times_seconds": [0.0, 0.1, 0.2, 0.3, 0.4],
            "character_end_times_seconds": [0.1, 0.2, 0.3, 0.4, 0.5]
        }"#;
        let alignment: CharacterAlignment = serde_json::from_str(json).unwrap();
        assert_eq!(alignment.characters.len(), 5);
        assert_eq!(alignment.characters[0], "H");
        assert!((alignment.character_start_times_seconds[0] - 0.0).abs() < f64::EPSILON);
        assert!((alignment.character_end_times_seconds[4] - 0.5).abs() < f64::EPSILON);
    }

    // -- AudioWithTimestampsResponse -----------------------------------------

    #[test]
    fn audio_with_timestamps_deserialize_from_api_example() {
        // Example from the OpenAPI spec.
        let json = r#"{
            "audio_base64": "base64_encoded_audio_string",
            "alignment": {
                "characters": ["H","e","l","l","o"],
                "character_start_times_seconds": [0.0, 0.1, 0.2, 0.3, 0.4],
                "character_end_times_seconds": [0.1, 0.2, 0.3, 0.4, 0.5]
            },
            "normalized_alignment": {
                "characters": ["H","e","l","l","o"],
                "character_start_times_seconds": [0.0, 0.1, 0.2, 0.3, 0.4],
                "character_end_times_seconds": [0.1, 0.2, 0.3, 0.4, 0.5]
            }
        }"#;
        let resp: AudioWithTimestampsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.audio_base64, "base64_encoded_audio_string");
        let alignment = resp.alignment.unwrap();
        assert_eq!(alignment.characters, vec!["H", "e", "l", "l", "o"]);
        assert!(resp.normalized_alignment.is_some());
    }

    #[test]
    fn audio_with_timestamps_null_alignment() {
        let json = r#"{
            "audio_base64": "SGVsbG8=",
            "alignment": null,
            "normalized_alignment": null
        }"#;
        let resp: AudioWithTimestampsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.audio_base64, "SGVsbG8=");
        assert!(resp.alignment.is_none());
        assert!(resp.normalized_alignment.is_none());
    }

    #[test]
    fn audio_with_timestamps_missing_optional_alignment() {
        // alignment and normalized_alignment not present at all.
        let json = r#"{"audio_base64": "SGVsbG8="}"#;
        let resp: AudioWithTimestampsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.alignment.is_none());
        assert!(resp.normalized_alignment.is_none());
    }

    // -- StreamingAudioChunkWithTimestamps ------------------------------------

    #[test]
    fn streaming_chunk_deserialize_from_api_example() {
        let json = r#"{
            "audio_base64": "base64_encoded_audio_chunk",
            "alignment": {
                "characters": ["H","e"],
                "character_start_times_seconds": [0.0, 0.1],
                "character_end_times_seconds": [0.1, 0.2]
            },
            "normalized_alignment": {
                "characters": ["H","e"],
                "character_start_times_seconds": [0.0, 0.1],
                "character_end_times_seconds": [0.1, 0.2]
            }
        }"#;
        let chunk: StreamingAudioChunkWithTimestamps = serde_json::from_str(json).unwrap();
        assert_eq!(chunk.audio_base64, "base64_encoded_audio_chunk");
        let alignment = chunk.alignment.unwrap();
        assert_eq!(alignment.characters.len(), 2);
    }

    #[test]
    fn streaming_chunk_null_alignments() {
        let json = r#"{
            "audio_base64": "chunk_data",
            "alignment": null,
            "normalized_alignment": null
        }"#;
        let chunk: StreamingAudioChunkWithTimestamps = serde_json::from_str(json).unwrap();
        assert_eq!(chunk.audio_base64, "chunk_data");
        assert!(chunk.alignment.is_none());
        assert!(chunk.normalized_alignment.is_none());
    }
}
