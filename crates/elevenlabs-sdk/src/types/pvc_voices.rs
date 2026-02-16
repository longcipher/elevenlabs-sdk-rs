//! Types for the ElevenLabs PVC (Professional Voice Cloning) endpoints.
//!
//! Covers:
//! - `POST   /v1/voices/pvc` — create a PVC voice
//! - `POST   /v1/voices/pvc/{voice_id}` — edit a PVC voice
//! - `POST   /v1/voices/pvc/{voice_id}/samples` — add samples
//! - `POST   /v1/voices/pvc/{voice_id}/samples/{sample_id}` — update a sample
//! - `POST   /v1/voices/pvc/{voice_id}/train` — start training
//! - `POST   /v1/voices/pvc/{voice_id}/verification` — request manual verification
//! - `GET    /v1/voices/pvc/{voice_id}/captcha` — get captcha
//! - `POST   /v1/voices/pvc/{voice_id}/captcha` — verify captcha
//! - `GET    /v1/voices/pvc/{voice_id}/samples/{sample_id}/audio` — get sample audio
//! - `GET    /v1/voices/pvc/{voice_id}/samples/{sample_id}/waveform` — get waveform
//! - `GET    /v1/voices/pvc/{voice_id}/samples/{sample_id}/speakers` — get speakers
//! - `GET    /v1/voices/pvc/{voice_id}/samples/{sample_id}/speakers/{speaker_id}/audio`
//! - `POST   /v1/voices/pvc/{voice_id}/samples/{sample_id}/separate-speakers`
//! - `DELETE /v1/voices/pvc/{voice_id}/samples/{sample_id}` — delete sample

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// Request body for creating a PVC voice.
#[derive(Debug, Clone, Serialize)]
pub struct CreatePvcVoiceRequest {
    /// Display name for the voice.
    pub name: String,
    /// Optional description of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional labels for the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}

/// Request body for editing a PVC voice.
#[derive(Debug, Clone, Serialize)]
pub struct EditPvcVoiceRequest {
    /// Updated display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}

/// Request body for updating a PVC voice sample.
#[derive(Debug, Clone, Serialize)]
pub struct EditPvcVoiceSampleRequest {
    /// Whether to apply noise removal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
    /// Selected speaker ID after speaker separation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_speaker_id: Option<String>,
    /// Trim start position in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_start: Option<i64>,
    /// Trim end position in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_end: Option<i64>,
    /// New file name for the sample.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Status of a speaker separation process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeakerSeparationStatus {
    /// Not yet started.
    NotStarted,
    /// In progress.
    Pending,
    /// Completed successfully.
    Completed,
    /// Failed.
    Failed,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

/// Response containing a base64-encoded audio preview of a voice sample.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSamplePreviewResponse {
    /// Base64-encoded audio data.
    pub audio_base_64: String,
    /// ID of the voice.
    pub voice_id: String,
    /// ID of the sample.
    pub sample_id: String,
    /// Media type (e.g. `"audio/mpeg"`).
    pub media_type: String,
    /// Duration in seconds.
    #[serde(default)]
    pub duration_secs: Option<f64>,
}

/// Visual waveform data for a voice sample.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSampleWaveformResponse {
    /// ID of the sample.
    pub sample_id: String,
    /// Waveform values (amplitude data points).
    pub visual_waveform: Vec<f64>,
}

/// Response from captcha verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifyPvcCaptchaResponse {
    /// Status string, typically `"ok"`.
    pub status: String,
}

/// Response from requesting manual verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestPvcManualVerificationResponse {
    /// Status string, typically `"ok"`.
    pub status: String,
}

/// Response from starting PVC voice training.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartPvcTrainingResponse {
    /// Status string, typically `"ok"`.
    pub status: String,
}

/// Response from deleting a PVC voice sample.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletePvcSampleResponse {
    /// Status string, typically `"ok"`.
    pub status: String,
}

/// Speaker separation status response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerSeparationResponse {
    /// Voice ID.
    pub voice_id: String,
    /// Sample ID.
    pub sample_id: String,
    /// Separation status.
    pub status: SpeakerSeparationStatus,
    /// Separated speakers, keyed by speaker ID.
    #[serde(default)]
    pub speakers: Option<HashMap<String, serde_json::Value>>,
    /// Selected speaker IDs.
    #[serde(default)]
    pub selected_speaker_ids: Option<Vec<String>>,
}

/// Response from starting speaker separation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartSpeakerSeparationResponse {
    /// Status string.
    pub status: String,
}

/// Response from retrieving the PVC voice captcha.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetPvcCaptchaResponse {
    /// Captcha image data or parameters.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn speaker_separation_status_deserialize() {
        let s: SpeakerSeparationStatus = serde_json::from_str(r#""not_started""#).unwrap();
        assert_eq!(s, SpeakerSeparationStatus::NotStarted);

        let s: SpeakerSeparationStatus = serde_json::from_str(r#""completed""#).unwrap();
        assert_eq!(s, SpeakerSeparationStatus::Completed);
    }

    #[test]
    fn voice_sample_preview_deserialize() {
        let json = r#"{
            "audio_base_64": "base64data",
            "voice_id": "v1",
            "sample_id": "s1",
            "media_type": "audio/mpeg",
            "duration_secs": 3.5
        }"#;
        let resp: VoiceSamplePreviewResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.voice_id, "v1");
        assert_eq!(resp.duration_secs, Some(3.5));
    }

    #[test]
    fn voice_sample_preview_without_duration() {
        let json = r#"{
            "audio_base_64": "data",
            "voice_id": "v1",
            "sample_id": "s1",
            "media_type": "audio/mpeg"
        }"#;
        let resp: VoiceSamplePreviewResponse = serde_json::from_str(json).unwrap();
        assert!(resp.duration_secs.is_none());
    }

    #[test]
    fn voice_sample_waveform_deserialize() {
        let json = r#"{
            "sample_id": "s1",
            "visual_waveform": [0.1, 0.5, 0.3, 0.8, 0.2]
        }"#;
        let resp: VoiceSampleWaveformResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.sample_id, "s1");
        assert_eq!(resp.visual_waveform.len(), 5);
    }

    #[test]
    fn verify_captcha_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: VerifyPvcCaptchaResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn manual_verification_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: RequestPvcManualVerificationResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn start_training_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: StartPvcTrainingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn delete_pvc_sample_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: DeletePvcSampleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }
}
