//! Types for the ElevenLabs Audio Isolation endpoints.
//!
//! Covers the two audio-isolation endpoints:
//! - `POST /v1/audio-isolation` — isolate vocals/speech from audio
//! - `POST /v1/audio-isolation/stream` — stream isolated vocals/speech
//!
//! Both endpoints accept `multipart/form-data` with an audio file and optional
//! configuration fields. The response is raw audio bytes.
//!
//! The types below capture the **non-file** fields the caller provides.
//! Actual multipart encoding is handled in the service layer.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Input Audio Format
// ---------------------------------------------------------------------------

/// Format of the input audio file sent to audio-isolation endpoints.
///
/// Specifying `PcmS16le16` (16-bit PCM, 16 kHz, mono, little-endian) avoids
/// server-side decoding and reduces latency. Use `Other` (the default) for
/// any encoded format (MP3, WAV, OGG, etc.).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioIsolationFileFormat {
    /// 16-bit PCM at 16 kHz, mono, little-endian. Lowest latency.
    #[serde(rename = "pcm_s16le_16")]
    PcmS16le16,
    /// Any other encoded audio format (MP3, WAV, OGG, etc.).
    #[default]
    Other,
}

// ---------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------

/// Configuration fields for `POST /v1/audio-isolation`.
///
/// This endpoint uses `multipart/form-data`. This struct captures every
/// non-file field; the audio file itself is provided separately when
/// building the multipart request in the service layer.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::AudioIsolationRequest;
///
/// let req = AudioIsolationRequest::default();
/// assert!(req.file_format.is_none());
/// assert!(req.preview_b64.is_none());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct AudioIsolationRequest {
    /// Format of the input audio file. Using `PcmS16le16` reduces latency
    /// because the server skips decoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<AudioIsolationFileFormat>,

    /// Optional preview image encoded as base64 for tracking this generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_b64: Option<String>,
}

/// Configuration fields for `POST /v1/audio-isolation/stream`.
///
/// This endpoint uses `multipart/form-data`. This struct captures every
/// non-file field; the audio file itself is provided separately when
/// building the multipart request in the service layer.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::AudioIsolationStreamRequest;
///
/// let req = AudioIsolationStreamRequest::default();
/// assert!(req.file_format.is_none());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct AudioIsolationStreamRequest {
    /// Format of the input audio file. Using `PcmS16le16` reduces latency
    /// because the server skips decoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<AudioIsolationFileFormat>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    // -- AudioIsolationFileFormat --------------------------------------------

    #[test]
    fn file_format_default_is_other() {
        assert_eq!(AudioIsolationFileFormat::default(), AudioIsolationFileFormat::Other);
    }

    #[test]
    fn file_format_serde_round_trip() {
        for variant in [AudioIsolationFileFormat::PcmS16le16, AudioIsolationFileFormat::Other] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: AudioIsolationFileFormat = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn file_format_serde_names() {
        assert_eq!(
            serde_json::to_string(&AudioIsolationFileFormat::PcmS16le16).unwrap(),
            r#""pcm_s16le_16""#
        );
        assert_eq!(serde_json::to_string(&AudioIsolationFileFormat::Other).unwrap(), r#""other""#);
    }

    // -- AudioIsolationRequest -----------------------------------------------

    #[test]
    fn request_default_values() {
        let req = AudioIsolationRequest::default();
        assert!(req.file_format.is_none());
        assert!(req.preview_b64.is_none());
    }

    #[test]
    fn request_minimal_serialization() {
        let req = AudioIsolationRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = v.as_object().unwrap();
        assert!(!obj.contains_key("file_format"));
        assert!(!obj.contains_key("preview_b64"));
    }

    #[test]
    fn request_full_serialization() {
        let req = AudioIsolationRequest {
            file_format: Some(AudioIsolationFileFormat::PcmS16le16),
            preview_b64: Some("aGVsbG8=".into()),
        };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["file_format"], "pcm_s16le_16");
        assert_eq!(v["preview_b64"], "aGVsbG8=");
    }

    // -- AudioIsolationStreamRequest -----------------------------------------

    #[test]
    fn stream_request_default_values() {
        let req = AudioIsolationStreamRequest::default();
        assert!(req.file_format.is_none());
    }

    #[test]
    fn stream_request_minimal_serialization() {
        let req = AudioIsolationStreamRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = v.as_object().unwrap();
        assert!(!obj.contains_key("file_format"));
    }

    #[test]
    fn stream_request_full_serialization() {
        let req =
            AudioIsolationStreamRequest { file_format: Some(AudioIsolationFileFormat::Other) };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["file_format"], "other");
    }
}
