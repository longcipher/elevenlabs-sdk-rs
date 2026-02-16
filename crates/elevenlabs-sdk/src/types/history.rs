//! Types for the ElevenLabs Speech History endpoints.
//!
//! Covers:
//! - `GET  /v1/history` — list speech history items
//! - `GET  /v1/history/{history_item_id}` — get a single history item
//! - `GET  /v1/history/{history_item_id}/audio` — download audio
//! - `DELETE /v1/history/{history_item_id}` — delete a history item
//! - `POST /v1/history/download` — download multiple items

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// State of a speech history item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryItemState {
    /// Item has been created and audio is available.
    Created,
    /// Item has been deleted.
    Deleted,
    /// Audio is still being processed.
    Processing,
}

/// Source of a speech history item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HistoryItemSource {
    /// Text-to-speech.
    TTS,
    /// Speech-to-speech.
    STS,
    /// Studio projects.
    Projects,
    /// Pronunciation dictionary.
    PD,
    /// Audio native.
    AN,
    /// Dubbing.
    Dubbing,
    /// Play API.
    PlayAPI,
    /// Conversational AI (Agents Platform).
    ConvAI,
    /// Voice generation.
    VoiceGeneration,
}

// ---------------------------------------------------------------------------
// Feedback
// ---------------------------------------------------------------------------

/// User feedback associated with a generated audio item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackResponse {
    /// Whether the user liked the generated item.
    pub thumbs_up: bool,
    /// Free-text feedback from the user.
    pub feedback: String,
    /// Whether the user flagged emotional issues.
    pub emotions: bool,
    /// Whether the user flagged inaccurate cloning.
    pub inaccurate_clone: bool,
    /// Whether the user flagged audio glitches.
    pub glitches: bool,
    /// Whether the user commented on audio quality.
    pub audio_quality: bool,
    /// Whether the user provided other feedback.
    pub other: bool,
    /// Review status (defaults to `"not_reviewed"`).
    #[serde(default)]
    pub review_status: Option<String>,
}

// ---------------------------------------------------------------------------
// History Alignment
// ---------------------------------------------------------------------------

/// Character-level alignment data for a history item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryAlignment {
    /// The individual characters.
    pub characters: Vec<String>,
    /// Start time of each character in seconds.
    pub character_start_times_seconds: Vec<f64>,
    /// End time of each character in seconds.
    pub character_end_times_seconds: Vec<f64>,
}

/// Alignment and normalized alignment for a history item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryAlignments {
    /// Raw alignment data.
    pub alignment: HistoryAlignment,
    /// Normalized alignment data.
    pub normalized_alignment: HistoryAlignment,
}

// ---------------------------------------------------------------------------
// Speech History Item
// ---------------------------------------------------------------------------

/// A single item from the speech history.
///
/// Contains metadata, settings, and optional alignment data for a
/// previously generated audio clip.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeechHistoryItem {
    /// Unique identifier for this history item.
    pub history_item_id: String,
    /// Request ID associated with the generation.
    #[serde(default)]
    pub request_id: Option<String>,
    /// ID of the voice used.
    #[serde(default)]
    pub voice_id: Option<String>,
    /// ID of the model used.
    #[serde(default)]
    pub model_id: Option<String>,
    /// Name of the voice used.
    #[serde(default)]
    pub voice_name: Option<String>,
    /// Category of the voice (e.g. `"premade"`, `"cloned"`).
    #[serde(default)]
    pub voice_category: Option<String>,
    /// Text used to generate the audio.
    #[serde(default)]
    pub text: Option<String>,
    /// Unix timestamp of when the item was created.
    pub date_unix: i64,
    /// Character count before this generation.
    pub character_count_change_from: i64,
    /// Character count after this generation.
    pub character_count_change_to: i64,
    /// MIME type of the generated audio (e.g. `"audio/mpeg"`).
    pub content_type: String,
    /// Current state of the history item.
    pub state: HistoryItemState,
    /// Voice settings used for generation (variable structure).
    #[serde(default)]
    pub settings: Option<serde_json::Value>,
    /// Feedback associated with the item, if any.
    #[serde(default)]
    pub feedback: Option<FeedbackResponse>,
    /// Share link identifier, if shared.
    #[serde(default)]
    pub share_link_id: Option<String>,
    /// Source that produced this item.
    #[serde(default)]
    pub source: Option<HistoryItemSource>,
    /// Character-level alignment data, if requested.
    #[serde(default)]
    pub alignments: Option<HistoryAlignments>,
    /// Dialogue inputs if this was a multi-voice dialogue generation.
    #[serde(default)]
    pub dialogue: Option<Vec<super::text_to_dialogue::DialogueInputResponse>>,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

/// Response from `GET /v1/history`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpeechHistoryResponse {
    /// Speech history items on this page.
    pub history: Vec<SpeechHistoryItem>,
    /// ID of the last history item on this page (for pagination).
    #[serde(default)]
    pub last_history_item_id: Option<String>,
    /// Whether more items are available.
    pub has_more: bool,
    /// Unix timestamp of the last scanned item.
    #[serde(default)]
    pub scanned_until: Option<i64>,
}

/// Response from `DELETE /v1/history/{history_item_id}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteHistoryItemResponse {
    /// Status string, typically `"ok"`.
    pub status: String,
}

// ---------------------------------------------------------------------------
// Request
// ---------------------------------------------------------------------------

/// Request body for `POST /v1/history/download`.
///
/// Downloads one or more history items as audio files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DownloadHistoryItemsRequest {
    /// List of history item IDs to download.
    pub history_item_ids: Vec<String>,

    /// Output format to transcode the audio (e.g. `"wav"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn history_item_state_deserialize() {
        let s: HistoryItemState = serde_json::from_str(r#""created""#).unwrap();
        assert_eq!(s, HistoryItemState::Created);
        let s: HistoryItemState = serde_json::from_str(r#""processing""#).unwrap();
        assert_eq!(s, HistoryItemState::Processing);
    }

    #[test]
    fn history_item_source_deserialize() {
        let s: HistoryItemSource = serde_json::from_str(r#""TTS""#).unwrap();
        assert_eq!(s, HistoryItemSource::TTS);
        let s: HistoryItemSource = serde_json::from_str(r#""ConvAI""#).unwrap();
        assert_eq!(s, HistoryItemSource::ConvAI);
    }

    #[test]
    fn feedback_response_deserialize() {
        let json = r#"{
            "thumbs_up": true,
            "feedback": "Great voice!",
            "emotions": false,
            "inaccurate_clone": false,
            "glitches": false,
            "audio_quality": true,
            "other": false,
            "review_status": "not_reviewed"
        }"#;
        let fb: FeedbackResponse = serde_json::from_str(json).unwrap();
        assert!(fb.thumbs_up);
        assert_eq!(fb.feedback, "Great voice!");
    }

    #[test]
    fn history_alignment_deserialize() {
        let json = r#"{
            "characters": ["H", "e", "l", "l", "o"],
            "character_start_times_seconds": [0.0, 0.1, 0.2, 0.3, 0.4],
            "character_end_times_seconds": [0.1, 0.2, 0.3, 0.4, 0.5]
        }"#;
        let a: HistoryAlignment = serde_json::from_str(json).unwrap();
        assert_eq!(a.characters.len(), 5);
    }

    #[test]
    fn speech_history_item_deserialize() {
        let json = r#"{
            "history_item_id": "abc123",
            "date_unix": 1714650306,
            "character_count_change_from": 17189,
            "character_count_change_to": 17231,
            "content_type": "audio/mpeg",
            "state": "created",
            "voice_id": "21m00Tcm4TlvDq8ikWAM",
            "voice_name": "Rachel",
            "voice_category": "premade",
            "model_id": "eleven_multilingual_v2",
            "text": "Hello, world!",
            "source": "TTS",
            "settings": {
                "stability": 0.71,
                "similarity_boost": 0.5
            }
        }"#;
        let item: SpeechHistoryItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.history_item_id, "abc123");
        assert_eq!(item.state, HistoryItemState::Created);
        assert_eq!(item.voice_name, Some("Rachel".into()));
        assert_eq!(item.source, Some(HistoryItemSource::TTS));
    }

    #[test]
    fn get_speech_history_response_deserialize() {
        let json = r#"{
            "history": [
                {
                    "history_item_id": "item1",
                    "date_unix": 1714650306,
                    "character_count_change_from": 100,
                    "character_count_change_to": 150,
                    "content_type": "audio/mpeg",
                    "state": "created"
                }
            ],
            "last_history_item_id": "item1",
            "has_more": false
        }"#;
        let resp: GetSpeechHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.history.len(), 1);
        assert!(!resp.has_more);
    }

    #[test]
    fn delete_history_item_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: DeleteHistoryItemResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn download_history_items_request_serialize() {
        let req = DownloadHistoryItemsRequest {
            history_item_ids: vec!["id1".into(), "id2".into()],
            output_format: Some("wav".into()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"history_item_ids\""));
        assert!(json.contains("\"output_format\":\"wav\""));
    }

    #[test]
    fn download_history_items_request_omits_none() {
        let req = DownloadHistoryItemsRequest {
            history_item_ids: vec!["id1".into()],
            output_format: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("output_format"));
    }
}
