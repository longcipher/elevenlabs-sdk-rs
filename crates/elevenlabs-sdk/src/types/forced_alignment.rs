//! Types for the ElevenLabs Forced Alignment endpoint.
//!
//! Covers `POST /v1/forced-alignment` — align audio with text to get
//! character-level and word-level timing information.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Response
// ---------------------------------------------------------------------------

/// A single character with timing information from the aligner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForcedAlignmentCharacter {
    /// The character that was transcribed.
    pub text: String,
    /// Start time of the character in seconds.
    pub start: f64,
    /// End time of the character in seconds.
    pub end: f64,
}

/// A single word with timing information from the aligner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForcedAlignmentWord {
    /// The word that was transcribed.
    pub text: String,
    /// Start time of the word in seconds.
    pub start: f64,
    /// End time of the word in seconds.
    pub end: f64,
    /// Average alignment loss/confidence score for this word.
    pub loss: f64,
}

/// Response from `POST /v1/forced-alignment`.
///
/// Contains character-level and word-level timing information, plus an
/// overall loss/confidence score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForcedAlignmentResponse {
    /// Character-level timing information.
    pub characters: Vec<ForcedAlignmentCharacter>,
    /// Word-level timing information.
    pub words: Vec<ForcedAlignmentWord>,
    /// Average alignment loss across all characters (lower = better fit).
    pub loss: f64,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn forced_alignment_character_deserialize() {
        let json = r#"{"text": "H", "start": 0.0, "end": 0.02}"#;
        let ch: ForcedAlignmentCharacter = serde_json::from_str(json).unwrap();
        assert_eq!(ch.text, "H");
        assert!((ch.end - 0.02).abs() < f64::EPSILON);
    }

    #[test]
    fn forced_alignment_word_deserialize() {
        let json = r#"{
            "text": "Hello",
            "start": 0.0,
            "end": 1.02,
            "loss": 0.12
        }"#;
        let word: ForcedAlignmentWord = serde_json::from_str(json).unwrap();
        assert_eq!(word.text, "Hello");
        assert!((word.loss - 0.12).abs() < f64::EPSILON);
    }

    #[test]
    fn forced_alignment_response_deserialize() {
        let json = r#"{
            "characters": [
                {"text": "H", "start": 0.0, "end": 0.05},
                {"text": "i", "start": 0.05, "end": 0.10}
            ],
            "words": [
                {"text": "Hi", "start": 0.0, "end": 0.10, "loss": 0.08}
            ],
            "loss": 0.08
        }"#;
        let resp: ForcedAlignmentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.characters.len(), 2);
        assert_eq!(resp.words.len(), 1);
        assert!((resp.loss - 0.08).abs() < f64::EPSILON);
    }
}
