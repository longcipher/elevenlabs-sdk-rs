//! Types for the ElevenLabs Samples endpoints.
//!
//! Covers:
//! - `GET    /v1/voices/{voice_id}/samples/{sample_id}/audio` — download sample audio
//! - `DELETE /v1/voices/{voice_id}/samples/{sample_id}` — delete a sample

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Response
// ---------------------------------------------------------------------------

/// Metadata for a voice sample.
///
/// Returned as part of voice details. Contains file information and
/// processing status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SampleResponse {
    /// Unique identifier for this sample.
    pub sample_id: String,
    /// Original file name of the sample.
    pub file_name: String,
    /// MIME type of the sample file.
    pub mime_type: String,
    /// Size of the sample file in bytes.
    pub size_bytes: i64,
    /// Hash of the sample file content.
    pub hash: String,
    /// Duration of the sample in seconds.
    #[serde(default)]
    pub duration_secs: Option<f64>,
    /// Whether background noise removal was requested.
    #[serde(default)]
    pub remove_background_noise: Option<bool>,
    /// Whether an isolated audio track is available.
    #[serde(default)]
    pub has_isolated_audio: Option<bool>,
    /// Whether an isolated audio preview is available.
    #[serde(default)]
    pub has_isolated_audio_preview: Option<bool>,
    /// Speaker separation results, if available.
    #[serde(default)]
    pub speaker_separation: Option<serde_json::Value>,
    /// Trim start position.
    #[serde(default)]
    pub trim_start: Option<i64>,
    /// Trim end position.
    #[serde(default)]
    pub trim_end: Option<i64>,
}

/// Response from `DELETE /v1/voices/{voice_id}/samples/{sample_id}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteSampleResponse {
    /// Status string, typically `"ok"`.
    pub status: String,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn sample_response_deserialize() {
        let json = r#"{
            "sample_id": "s1",
            "file_name": "sample.mp3",
            "mime_type": "audio/mpeg",
            "size_bytes": 102400,
            "hash": "abc123hash",
            "duration_secs": 5.2
        }"#;
        let sample: SampleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(sample.sample_id, "s1");
        assert_eq!(sample.file_name, "sample.mp3");
        assert_eq!(sample.size_bytes, 102400);
        assert_eq!(sample.duration_secs, Some(5.2));
    }

    #[test]
    fn sample_response_minimal() {
        let json = r#"{
            "sample_id": "s2",
            "file_name": "voice.wav",
            "mime_type": "audio/wav",
            "size_bytes": 50000,
            "hash": "xyz789"
        }"#;
        let sample: SampleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(sample.sample_id, "s2");
        assert!(sample.duration_secs.is_none());
        assert!(sample.remove_background_noise.is_none());
    }

    #[test]
    fn delete_sample_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: DeleteSampleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }
}
