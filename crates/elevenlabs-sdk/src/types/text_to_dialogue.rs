//! Types for the ElevenLabs Text-to-Dialogue endpoints.
//!
//! Covers:
//! - `POST /v1/text-to-dialogue` (convert)
//! - `POST /v1/text-to-dialogue/stream` (stream)
//! - `POST /v1/text-to-dialogue/with-timestamps` (convert with timestamps)
//! - `POST /v1/text-to-dialogue/stream/with-timestamps` (stream with timestamps)
//!
//! All four endpoints share the same request body shape.

use serde::{Deserialize, Serialize};

use super::{
    common::VoiceSettings,
    text_to_speech::{
        CharacterAlignment, PronunciationDictionaryVersionLocator, TextNormalization,
    },
};

// ---------------------------------------------------------------------------
// Dialogue Input
// ---------------------------------------------------------------------------

/// A single text-and-voice pair for multi-voice dialogue generation.
///
/// Used as an element of [`TextToDialogueRequest::inputs`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct DialogueInput {
    /// The text to be converted into speech.
    pub text: String,
    /// The ID of the voice to be used for this line.
    pub voice_id: String,
}

/// A dialogue input line as returned by the API in history items.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DialogueInputResponse {
    /// The text of the dialogue input line.
    pub text: String,
    /// The ID of the voice used for this line.
    pub voice_id: String,
    /// The name of the voice used for this line.
    pub voice_name: String,
}

// ---------------------------------------------------------------------------
// Request
// ---------------------------------------------------------------------------

/// Request body for all four text-to-dialogue endpoints.
///
/// Only `inputs` is required; all other fields are optional.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::{DialogueInput, TextToDialogueRequest};
///
/// let req = TextToDialogueRequest {
///     inputs: vec![
///         DialogueInput { text: "Hello!".into(), voice_id: "voice1".into() },
///         DialogueInput { text: "Hi there!".into(), voice_id: "voice2".into() },
///     ],
///     ..Default::default()
/// };
/// assert_eq!(req.inputs.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct TextToDialogueRequest {
    /// A list of dialogue inputs, each containing text and a voice ID.
    pub inputs: Vec<DialogueInput>,

    /// Identifier of the model to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,

    /// Language code (ISO 639-1) used to enforce a language for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// Settings controlling the dialogue generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<VoiceSettings>,

    /// Pronunciation dictionary locators applied in order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>,

    /// Seed for deterministic generation. Must be between 0 and 4294967295.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Controls text normalization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<TextNormalization>,
}

// ---------------------------------------------------------------------------
// Voice Segment
// ---------------------------------------------------------------------------

/// A voice segment describing which voice produced a portion of
/// dialogue audio.
///
/// Returned by the text-to-dialogue with-timestamps endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSegment {
    /// The voice ID used for this segment.
    pub voice_id: String,
    /// Start time of this voice segment in seconds.
    pub start_time_seconds: f64,
    /// End time of this voice segment in seconds.
    pub end_time_seconds: f64,
    /// Start index in the characters array.
    pub character_start_index: i64,
    /// End index in the characters array (exclusive).
    pub character_end_index: i64,
    /// Line of the dialogue (script) that this segment is part of.
    pub dialogue_input_index: i64,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

/// Response from `POST /v1/text-to-dialogue/with-timestamps`.
///
/// Contains full audio as a base64-encoded string along with optional
/// character-level alignment data and voice segments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioWithTimestampsAndVoiceSegmentsResponse {
    /// Base64-encoded audio data.
    pub audio_base64: String,
    /// Character-level alignment for the original text.
    pub alignment: Option<CharacterAlignment>,
    /// Character-level alignment for the normalized text.
    pub normalized_alignment: Option<CharacterAlignment>,
    /// Voice segments describing which voice produced each portion.
    pub voice_segments: Vec<VoiceSegment>,
}

/// A single chunk from `POST /v1/text-to-dialogue/stream/with-timestamps`.
///
/// The streaming-with-timestamps endpoint delivers multiple chunks, each
/// containing a portion of the audio and its corresponding alignment and
/// voice segment data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamingAudioChunkWithTimestampsAndVoiceSegments {
    /// Base64-encoded audio data for this chunk.
    pub audio_base64: String,
    /// Character-level alignment for the original text in this chunk.
    pub alignment: Option<CharacterAlignment>,
    /// Character-level alignment for the normalized text in this chunk.
    pub normalized_alignment: Option<CharacterAlignment>,
    /// Voice segments for this chunk.
    pub voice_segments: Vec<VoiceSegment>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn dialogue_input_serialize() {
        let input = DialogueInput { text: "Hello!".into(), voice_id: "v1".into() };
        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("\"text\":\"Hello!\""));
        assert!(json.contains("\"voice_id\":\"v1\""));
    }

    #[test]
    fn dialogue_input_response_deserialize() {
        let json = r#"{
            "text": "Hello!",
            "voice_id": "v1",
            "voice_name": "Rachel"
        }"#;
        let r: DialogueInputResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.text, "Hello!");
        assert_eq!(r.voice_name, "Rachel");
    }

    #[test]
    fn text_to_dialogue_request_serialize() {
        let req = TextToDialogueRequest {
            inputs: vec![
                DialogueInput { text: "Hello".into(), voice_id: "v1".into() },
                DialogueInput { text: "Hi".into(), voice_id: "v2".into() },
            ],
            model_id: Some("eleven_multilingual_v2".into()),
            ..Default::default()
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"inputs\""));
        assert!(json.contains("\"model_id\""));
        assert!(!json.contains("language_code"));
    }

    #[test]
    fn text_to_dialogue_request_omits_none_fields() {
        let req = TextToDialogueRequest { inputs: vec![], ..Default::default() };
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("model_id"));
        assert!(!json.contains("seed"));
        assert!(!json.contains("settings"));
    }
}
