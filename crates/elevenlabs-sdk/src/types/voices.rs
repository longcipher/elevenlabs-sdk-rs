//! Types for the ElevenLabs Voice endpoints.
//!
//! Covers voice management operations including:
//! - `GET /v1/voices` — list all voices
//! - `GET /v1/voices/{voice_id}` — get a single voice
//! - `POST /v1/voices/add` — add a new voice (multipart)
//! - `POST /v1/voices/{voice_id}/edit` — edit a voice (multipart)
//! - `DELETE /v1/voices/{voice_id}` — delete a voice
//! - `GET /v1/voices/{voice_id}/settings` — get voice settings
//! - `POST /v1/voices/{voice_id}/settings/edit` — edit voice settings
//! - `GET /v1/voices/{voice_id}/samples/{sample_id}/audio` — get sample audio
//! - `DELETE /v1/voices/{voice_id}/samples/{sample_id}` — delete a sample
//!
//! Related sub-resources (samples, fine-tuning, sharing, verification) are
//! included as embedded response types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::{SafetyControl, VerifiedVoiceLanguage, VoiceCategory, VoiceSettings};

// ---------------------------------------------------------------------------
// Fine-Tuning
// ---------------------------------------------------------------------------

/// State of a fine-tuning process for a specific model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FineTuningState {
    /// Fine-tuning has not started.
    NotStarted,
    /// Fine-tuning is queued.
    Queued,
    /// Fine-tuning is in progress.
    FineTuning,
    /// Fine-tuning has completed.
    FineTuned,
    /// Fine-tuning has failed.
    Failed,
    /// Fine-tuning is delayed.
    Delayed,
}

/// Fine-tuning information for a voice.
///
/// Contains the state of fine-tuning across models, verification status,
/// and optional progress/message data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FineTuning {
    /// Whether the user is allowed to fine-tune this voice.
    pub is_allowed_to_fine_tune: bool,
    /// Per-model fine-tuning state (e.g. `{"eleven_multilingual_v2": "fine_tuned"}`).
    pub state: HashMap<String, FineTuningState>,
    /// List of verification failure descriptions.
    pub verification_failures: Vec<String>,
    /// Number of verification attempts made.
    pub verification_attempts_count: i64,
    /// Whether manual verification was requested.
    pub manual_verification_requested: bool,
    /// Language of the fine-tuning process.
    pub language: Option<String>,
    /// Per-model fine-tuning progress (0.0 to 1.0).
    pub progress: Option<HashMap<String, f64>>,
    /// Per-model status messages.
    pub message: Option<HashMap<String, String>>,
    /// Duration of the training dataset in seconds.
    pub dataset_duration_seconds: Option<f64>,
    /// Verification attempts with details.
    pub verification_attempts: Option<Vec<VerificationAttempt>>,
    /// Slice IDs used in fine-tuning.
    pub slice_ids: Option<Vec<String>>,
    /// Manual verification details (complex nested object).
    pub manual_verification: Option<serde_json::Value>,
    /// Maximum number of verification attempts allowed.
    pub max_verification_attempts: Option<i64>,
    /// Unix timestamp (ms) of next verification attempts reset.
    pub next_max_verification_attempts_reset_unix_ms: Option<i64>,
}

// ---------------------------------------------------------------------------
// Voice Samples
// ---------------------------------------------------------------------------

/// A voice sample (audio file) associated with a voice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSample {
    /// Unique sample identifier.
    pub sample_id: String,
    /// Original filename of the uploaded sample.
    pub file_name: String,
    /// MIME type (e.g. `"audio/mpeg"`).
    pub mime_type: String,
    /// File size in bytes.
    pub size_bytes: i64,
    /// Content hash.
    pub hash: String,
    /// Duration in seconds.
    pub duration_secs: Option<f64>,
    /// Whether background noise removal was applied.
    pub remove_background_noise: Option<bool>,
    /// Whether an isolated audio track is available.
    pub has_isolated_audio: Option<bool>,
    /// Whether an isolated audio preview is available.
    pub has_isolated_audio_preview: Option<bool>,
    /// Speaker separation details (complex nested object).
    pub speaker_separation: Option<serde_json::Value>,
    /// Trim start position (milliseconds).
    pub trim_start: Option<i64>,
    /// Trim end position (milliseconds).
    pub trim_end: Option<i64>,
}

// ---------------------------------------------------------------------------
// Voice Verification
// ---------------------------------------------------------------------------

/// A recording associated with a verification attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recording {
    /// Unique recording identifier.
    pub recording_id: String,
    /// MIME type (e.g. `"audio/mpeg"`).
    pub mime_type: String,
    /// Recording size in bytes.
    pub size_bytes: i64,
    /// Unix timestamp of the upload date.
    pub upload_date_unix: i64,
    /// Transcription of the recording content.
    pub transcription: String,
}

/// A single verification attempt for a voice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationAttempt {
    /// The text that was read for verification.
    pub text: String,
    /// Unix timestamp of the attempt.
    pub date_unix: i64,
    /// Whether the attempt was accepted.
    pub accepted: bool,
    /// Similarity score of the attempt.
    pub similarity: f64,
    /// Levenshtein distance of the transcription.
    pub levenshtein_distance: f64,
    /// Recording submitted for this attempt.
    pub recording: Option<Recording>,
}

/// Voice verification status and history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceVerification {
    /// Whether the voice requires verification before use.
    pub requires_verification: bool,
    /// Whether the voice has been verified.
    pub is_verified: bool,
    /// List of verification failure descriptions.
    pub verification_failures: Vec<String>,
    /// Total number of verification attempts.
    pub verification_attempts_count: i64,
    /// Language used for verification.
    pub language: Option<String>,
    /// Detailed verification attempts.
    pub verification_attempts: Option<Vec<VerificationAttempt>>,
}

// ---------------------------------------------------------------------------
// Voice Sharing
// ---------------------------------------------------------------------------

/// Status of voice sharing in the library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceSharingStatus {
    /// Sharing is enabled.
    Enabled,
    /// Sharing is disabled.
    Disabled,
    /// Voice was copied.
    Copied,
    /// Voice was copied but is now disabled.
    CopiedDisabled,
}

/// Review status of a shared voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    /// Review has not been requested.
    NotRequested,
    /// Review is pending.
    Pending,
    /// Review was declined.
    Declined,
    /// Review was allowed.
    Allowed,
    /// Review was allowed with modifications.
    AllowedWithChanges,
}

/// Type of reader resource restriction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaderResourceType {
    /// Read restriction.
    Read,
    /// Collection restriction.
    Collection,
}

/// A reader resource that a voice is restricted on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderResource {
    /// The type of resource.
    pub resource_type: ReaderResourceType,
    /// Unique resource identifier.
    pub resource_id: String,
}

/// Moderation check details for a shared voice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModerationCheck {
    /// Unix timestamp when the check was performed.
    pub date_checked_unix: Option<i64>,
    /// Name value that was checked.
    pub name_value: Option<String>,
    /// Whether the name check passed.
    pub name_check: Option<bool>,
    /// Description value that was checked.
    pub description_value: Option<String>,
    /// Whether the description check passed.
    pub description_check: Option<bool>,
    /// IDs of samples that were checked.
    pub sample_ids: Option<Vec<String>>,
    /// Sample check scores.
    pub sample_checks: Option<Vec<f64>>,
    /// IDs of CAPTCHAs that were checked.
    pub captcha_ids: Option<Vec<String>>,
    /// CAPTCHA check scores.
    pub captcha_checks: Option<Vec<f64>>,
}

/// Voice sharing information from the ElevenLabs Voice Library.
///
/// Contains details about how a voice is shared, its review status,
/// associated social media accounts, and moderation details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSharing {
    /// Current sharing status.
    pub status: VoiceSharingStatus,
    /// History item sample ID used for sharing preview.
    pub history_item_sample_id: Option<String>,
    /// Unix timestamp when sharing was enabled.
    pub date_unix: i64,
    /// Emails allowed to use this shared voice.
    pub whitelisted_emails: Vec<String>,
    /// Public owner identifier.
    pub public_owner_id: String,
    /// ID of the original voice this was shared from.
    pub original_voice_id: String,
    /// Whether financial rewards are enabled for this voice.
    pub financial_rewards_enabled: bool,
    /// Whether free-tier users can access this voice.
    pub free_users_allowed: bool,
    /// Whether live moderation is enabled.
    pub live_moderation_enabled: bool,
    /// Revenue rate for the voice.
    pub rate: Option<f64>,
    /// Fiat rate (USD per 1000 credits).
    pub fiat_rate: Option<f64>,
    /// Notice period in days before disabling.
    pub notice_period: i64,
    /// Unix timestamp when the voice will be disabled.
    pub disable_at_unix: Option<i64>,
    /// Whether voice mixing is allowed.
    pub voice_mixing_allowed: bool,
    /// Whether the voice is featured in the library.
    pub featured: bool,
    /// Voice category in the library (e.g. `"professional"`).
    pub category: String,
    /// Whether the reader app is enabled.
    pub reader_app_enabled: Option<bool>,
    /// URL of the voice image.
    pub image_url: Option<String>,
    /// Reason the voice was banned (if applicable).
    pub ban_reason: Option<String>,
    /// Number of users who liked this voice.
    pub liked_by_count: i64,
    /// Number of users who cloned this voice.
    pub cloned_by_count: i64,
    /// Display name of the shared voice.
    pub name: String,
    /// Description of the shared voice.
    pub description: Option<String>,
    /// Labels associated with the voice.
    pub labels: HashMap<String, String>,
    /// Current review status.
    pub review_status: ReviewStatus,
    /// Message from the reviewer.
    pub review_message: Option<String>,
    /// Whether the voice is enabled in the library.
    pub enabled_in_library: bool,
    /// Instagram username of the voice owner.
    pub instagram_username: Option<String>,
    /// Twitter/X username of the voice owner.
    pub twitter_username: Option<String>,
    /// YouTube username of the voice owner.
    pub youtube_username: Option<String>,
    /// TikTok username of the voice owner.
    pub tiktok_username: Option<String>,
    /// Moderation check details.
    pub moderation_check: Option<ModerationCheck>,
    /// Reader resources this voice is restricted on.
    pub reader_restricted_on: Option<Vec<ReaderResource>>,
}

// ---------------------------------------------------------------------------
// Voice (main response type)
// ---------------------------------------------------------------------------

/// A voice available in the ElevenLabs platform.
///
/// Returned by `GET /v1/voices` (in a list) and `GET /v1/voices/{voice_id}`
/// (individually). Contains all metadata including optional fine-tuning,
/// sharing, and verification details.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::Voice;
///
/// let json = r#"{
///     "voice_id": "21m00Tcm4TlvDq8ikWAM",
///     "name": "Rachel",
///     "category": "premade",
///     "labels": {"accent": "American", "gender": "female"},
///     "available_for_tiers": ["creator"],
///     "high_quality_base_model_ids": ["eleven_multilingual_v2"]
/// }"#;
/// let voice: Voice = serde_json::from_str(json).unwrap();
/// assert_eq!(voice.name, "Rachel");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Voice {
    /// Unique voice identifier.
    pub voice_id: String,
    /// Display name of the voice.
    pub name: String,
    /// Voice category.
    pub category: VoiceCategory,
    /// Key-value labels (e.g. `{"accent": "American", "gender": "female"}`).
    pub labels: HashMap<String, String>,
    /// Subscription tiers this voice is available for.
    pub available_for_tiers: Vec<String>,
    /// Base model IDs that support high-quality output with this voice.
    pub high_quality_base_model_ids: Vec<String>,
    /// Audio samples associated with this voice.
    pub samples: Option<Vec<VoiceSample>>,
    /// Fine-tuning information.
    pub fine_tuning: Option<FineTuning>,
    /// Human-readable description.
    pub description: Option<String>,
    /// URL of a preview audio clip.
    pub preview_url: Option<String>,
    /// Current voice settings (stability, similarity, etc.).
    pub settings: Option<VoiceSettings>,
    /// Sharing/library information.
    pub sharing: Option<VoiceSharing>,
    /// Languages verified for this voice.
    pub verified_languages: Option<Vec<VerifiedVoiceLanguage>>,
    /// IDs of collections this voice belongs to.
    pub collection_ids: Option<Vec<String>>,
    /// Safety control level.
    pub safety_control: Option<SafetyControl>,
    /// Voice verification status.
    pub voice_verification: Option<VoiceVerification>,
    /// Permission level on this resource.
    pub permission_on_resource: Option<String>,
    /// Whether the current user owns this voice.
    pub is_owner: Option<bool>,
    /// Whether this is a legacy voice.
    #[serde(default)]
    pub is_legacy: bool,
    /// Whether this voice was created by mixing other voices.
    #[serde(default)]
    pub is_mixed: bool,
    /// Unix timestamp when the voice was favourited.
    pub favorited_at_unix: Option<i64>,
    /// Unix timestamp when the voice was created.
    pub created_at_unix: Option<i64>,
}

// ---------------------------------------------------------------------------
// List / CRUD Responses
// ---------------------------------------------------------------------------

/// Response from `GET /v1/voices`.
///
/// Contains a list of all voices available to the authenticated user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVoicesResponse {
    /// List of available voices.
    pub voices: Vec<Voice>,
}

/// Response from `POST /v1/voices/add`.
///
/// Returns the ID of the newly created voice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddVoiceResponse {
    /// ID of the newly created voice.
    pub voice_id: String,
}

/// Response from `POST /v1/voices/{voice_id}/edit`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditVoiceResponse {
    /// Status message (typically `"ok"`).
    pub status: String,
}

/// Response from `DELETE /v1/voices/{voice_id}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteVoiceResponse {
    /// Status message (typically `"ok"`).
    pub status: String,
}

/// Response from `POST /v1/voices/{voice_id}/settings/edit`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditVoiceSettingsResponse {
    /// Status message (typically `"ok"`).
    pub status: String,
}

/// Response from `DELETE /v1/voices/{voice_id}/samples/{sample_id}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteVoiceSampleResponse {
    /// Status message (typically `"ok"`).
    pub status: String,
}

// ---------------------------------------------------------------------------
// Request Types
// ---------------------------------------------------------------------------

/// Request body fields for `POST /v1/voices/add`.
///
/// Note: the actual add-voice endpoint uses `multipart/form-data` with audio
/// files. This struct captures the JSON-serialisable metadata fields. Audio
/// files are attached separately as multipart parts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddVoiceRequest {
    /// Display name for the new voice.
    pub name: String,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional key-value labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}

/// Request body fields for `POST /v1/voices/{voice_id}/edit`.
///
/// Similar to [`AddVoiceRequest`], the actual endpoint uses
/// `multipart/form-data`. This struct captures metadata fields only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EditVoiceRequest {
    /// Updated display name.
    pub name: String,
    /// Updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated key-value labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}

// ---------------------------------------------------------------------------
// Library Voices (Shared)
// ---------------------------------------------------------------------------

/// A voice from the ElevenLabs shared voice library.
///
/// Returned by `GET /v1/shared-voices`. Contains public metadata about
/// library voices that can be added to the user's collection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryVoice {
    /// Public owner identifier.
    pub public_owner_id: String,
    /// Unique voice identifier.
    pub voice_id: String,
    /// Unix timestamp of when the voice was added to the library.
    pub date_unix: i64,
    /// Display name of the voice.
    pub name: String,
    /// Accent descriptor (e.g. `"American"`, `"British"`).
    pub accent: String,
    /// Gender descriptor (e.g. `"male"`, `"female"`).
    pub gender: String,
    /// Age descriptor (e.g. `"young"`, `"middle_aged"`).
    pub age: String,
    /// Descriptive adjective (e.g. `"warm"`, `"raspy"`).
    pub descriptive: String,
    /// Intended use case (e.g. `"narration"`, `"conversational"`).
    pub use_case: String,
    /// Voice category in the library.
    pub category: String,
    /// Language of the voice.
    #[serde(default)]
    pub language: Option<String>,
    /// Locale of the voice (e.g. `"en-US"`).
    #[serde(default)]
    pub locale: Option<String>,
    /// Human-readable description of the voice.
    #[serde(default)]
    pub description: Option<String>,
    /// URL of a preview audio clip.
    #[serde(default)]
    pub preview_url: Option<String>,
    /// Character usage count over the last year.
    pub usage_character_count_1y: i64,
    /// Character usage count over the last 7 days.
    pub usage_character_count_7d: i64,
    /// Play API character usage count over the last year.
    pub play_api_usage_character_count_1y: i64,
    /// Number of users who cloned this voice.
    pub cloned_by_count: i64,
    /// Revenue rate for the voice.
    #[serde(default)]
    pub rate: Option<f64>,
    /// Fiat rate (USD per 1000 credits).
    #[serde(default)]
    pub fiat_rate: Option<f64>,
    /// Whether free-tier users can access this voice.
    pub free_users_allowed: bool,
    /// Whether live moderation is enabled.
    pub live_moderation_enabled: bool,
    /// Whether the voice is featured in the library.
    pub featured: bool,
    /// Languages verified for this voice.
    #[serde(default)]
    pub verified_languages: Option<Vec<VerifiedVoiceLanguage>>,
    /// Notice period in days before disabling.
    #[serde(default)]
    pub notice_period: Option<i64>,
    /// Instagram username of the voice owner.
    #[serde(default)]
    pub instagram_username: Option<String>,
    /// Twitter/X username of the voice owner.
    #[serde(default)]
    pub twitter_username: Option<String>,
    /// YouTube username of the voice owner.
    #[serde(default)]
    pub youtube_username: Option<String>,
    /// TikTok username of the voice owner.
    #[serde(default)]
    pub tiktok_username: Option<String>,
    /// URL of the voice image.
    #[serde(default)]
    pub image_url: Option<String>,
    /// Whether the voice has been added by the current user.
    #[serde(default)]
    pub is_added_by_user: Option<bool>,
}

/// Response from `GET /v1/shared-voices`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLibraryVoicesResponse {
    /// Library voices on this page.
    pub voices: Vec<LibraryVoice>,
    /// Whether more items are available.
    pub has_more: bool,
    /// Sort ID of the last item (for pagination).
    #[serde(default)]
    pub last_sort_id: Option<String>,
}

/// Response from `POST /v1/similar-voices`.
///
/// Returns library voices similar to a provided audio sample.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSimilarVoicesResponse {
    /// Similar library voices.
    pub voices: Vec<LibraryVoice>,
    /// Whether more items are available.
    pub has_more: bool,
    /// Sort ID of the last item (for pagination).
    #[serde(default)]
    pub last_sort_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Voices v2
// ---------------------------------------------------------------------------

/// Response from `GET /v2/voices`.
///
/// Returns a paginated list of voices using token-based pagination.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVoicesV2Response {
    /// Voices on this page.
    pub voices: Vec<Voice>,
    /// Whether more items are available.
    pub has_more: bool,
    /// Total number of voices matching the query.
    pub total_count: i64,
    /// Opaque token to fetch the next page.
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn voice_deserialize_minimal() {
        let json = r#"{
            "voice_id": "21m00Tcm4TlvDq8ikWAM",
            "name": "Rachel",
            "category": "premade",
            "labels": {"accent": "American"},
            "available_for_tiers": ["creator"],
            "high_quality_base_model_ids": ["eleven_multilingual_v2"]
        }"#;
        let voice: Voice = serde_json::from_str(json).unwrap();
        assert_eq!(voice.voice_id, "21m00Tcm4TlvDq8ikWAM");
        assert_eq!(voice.name, "Rachel");
        assert_eq!(voice.category, VoiceCategory::Premade);
        assert_eq!(voice.labels.get("accent").unwrap(), "American");
        assert_eq!(voice.available_for_tiers, vec!["creator"]);
        assert!(!voice.is_legacy);
        assert!(!voice.is_mixed);
        assert!(voice.samples.is_none());
        assert!(voice.fine_tuning.is_none());
    }

    #[test]
    fn voice_deserialize_full_api_example() {
        let json = r#"{
            "voice_id": "21m00Tcm4TlvDq8ikWAM",
            "name": "Rachel",
            "category": "professional",
            "labels": {
                "accent": "American",
                "age": "middle-aged",
                "description": "expressive",
                "gender": "female",
                "use_case": "social media"
            },
            "description": "A warm, expressive voice with a touch of humor.",
            "preview_url": "https://storage.googleapis.com/eleven-public-prod/premade/voices/9BWtsMINqrJLrRacOk9x/preview.mp3",
            "available_for_tiers": ["creator", "enterprise"],
            "high_quality_base_model_ids": [
                "eleven_v2_flash",
                "eleven_multilingual_v2"
            ],
            "settings": {
                "stability": 1.0,
                "similarity_boost": 1.0,
                "style": 0.0,
                "use_speaker_boost": true,
                "speed": 1.0
            },
            "fine_tuning": {
                "is_allowed_to_fine_tune": true,
                "state": {"eleven_multilingual_v2": "fine_tuned"},
                "verification_failures": [],
                "verification_attempts_count": 2,
                "manual_verification_requested": false
            },
            "sharing": {
                "status": "enabled",
                "date_unix": 1714204800,
                "whitelisted_emails": ["example@example.com"],
                "public_owner_id": "DCwhRBWXzGAHq8TQ4Fs18",
                "original_voice_id": "DCwhRBWXzGAHq8TQ4Fs18",
                "financial_rewards_enabled": true,
                "free_users_allowed": true,
                "live_moderation_enabled": true,
                "rate": 0.05,
                "notice_period": 30,
                "voice_mixing_allowed": false,
                "featured": true,
                "category": "professional",
                "liked_by_count": 100,
                "cloned_by_count": 50,
                "name": "Rachel",
                "labels": {"accent": "American", "gender": "female"},
                "review_status": "allowed",
                "enabled_in_library": true
            },
            "voice_verification": {
                "requires_verification": false,
                "is_verified": true,
                "verification_failures": [],
                "verification_attempts_count": 0,
                "language": "en",
                "verification_attempts": [
                    {
                        "text": "Hello, how are you?",
                        "date_unix": 1714204800,
                        "accepted": true,
                        "similarity": 0.95,
                        "levenshtein_distance": 2,
                        "recording": {
                            "recording_id": "CwhRBWXzGAHq8TQ4Fs17",
                            "mime_type": "audio/mpeg",
                            "size_bytes": 1000000,
                            "upload_date_unix": 1714204800,
                            "transcription": "Hello, how are you?"
                        }
                    }
                ]
            },
            "verified_languages": [
                {
                    "language": "en",
                    "model_id": "eleven_multilingual_v2",
                    "accent": "american",
                    "locale": "en-US",
                    "preview_url": "https://storage.googleapis.com/preview.mp3"
                }
            ],
            "is_owner": false,
            "is_legacy": false,
            "is_mixed": false,
            "safety_control": "NONE",
            "created_at_unix": 1714204800
        }"#;

        let voice: Voice = serde_json::from_str(json).unwrap();
        assert_eq!(voice.voice_id, "21m00Tcm4TlvDq8ikWAM");
        assert_eq!(voice.category, VoiceCategory::Professional);
        assert_eq!(voice.labels.len(), 5);
        assert_eq!(
            voice.description.as_deref(),
            Some("A warm, expressive voice with a touch of humor.")
        );

        // Settings
        let settings = voice.settings.unwrap();
        assert_eq!(settings.stability, Some(1.0));
        assert_eq!(settings.speed, Some(1.0));

        // Fine tuning
        let ft = voice.fine_tuning.unwrap();
        assert!(ft.is_allowed_to_fine_tune);
        assert_eq!(ft.state.get("eleven_multilingual_v2"), Some(&FineTuningState::FineTuned));
        assert_eq!(ft.verification_attempts_count, 2);

        // Sharing
        let sharing = voice.sharing.unwrap();
        assert_eq!(sharing.status, VoiceSharingStatus::Enabled);
        assert_eq!(sharing.liked_by_count, 100);
        assert_eq!(sharing.review_status, ReviewStatus::Allowed);
        assert!(sharing.financial_rewards_enabled);

        // Verification
        let vv = voice.voice_verification.unwrap();
        assert!(vv.is_verified);
        assert!(!vv.requires_verification);
        let attempts = vv.verification_attempts.unwrap();
        assert_eq!(attempts.len(), 1);
        assert!(attempts[0].accepted);
        assert_eq!(attempts[0].similarity, 0.95);
        let recording = attempts[0].recording.as_ref().unwrap();
        assert_eq!(recording.recording_id, "CwhRBWXzGAHq8TQ4Fs17");
        assert_eq!(recording.transcription, "Hello, how are you?");

        // Verified languages
        let vl = voice.verified_languages.unwrap();
        assert_eq!(vl.len(), 1);
        assert_eq!(vl[0].language, "en");

        // Safety control
        assert_eq!(voice.safety_control, Some(SafetyControl::None));
        assert!(!voice.is_legacy);
        assert_eq!(voice.created_at_unix, Some(1714204800));
    }

    #[test]
    fn get_voices_response_deserialize() {
        let json = r#"{
            "voices": [
                {
                    "voice_id": "id1",
                    "name": "Voice One",
                    "category": "generated",
                    "labels": {},
                    "available_for_tiers": [],
                    "high_quality_base_model_ids": []
                },
                {
                    "voice_id": "id2",
                    "name": "Voice Two",
                    "category": "cloned",
                    "labels": {"gender": "male"},
                    "available_for_tiers": ["pro"],
                    "high_quality_base_model_ids": ["eleven_turbo_v2"]
                }
            ]
        }"#;
        let resp: GetVoicesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.voices.len(), 2);
        assert_eq!(resp.voices[0].name, "Voice One");
        assert_eq!(resp.voices[0].category, VoiceCategory::Generated);
        assert_eq!(resp.voices[1].category, VoiceCategory::Cloned);
    }

    #[test]
    fn add_voice_response_deserialize() {
        let json = r#"{"voice_id": "b38kUX8pkfYO2kHyqfFy"}"#;
        let resp: AddVoiceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.voice_id, "b38kUX8pkfYO2kHyqfFy");
    }

    #[test]
    fn edit_voice_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: EditVoiceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn delete_voice_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: DeleteVoiceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn edit_voice_settings_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: EditVoiceSettingsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn delete_voice_sample_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: DeleteVoiceSampleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn add_voice_request_serialize() {
        let req = AddVoiceRequest {
            name: "My Voice".into(),
            description: Some("A custom voice".into()),
            labels: Some(HashMap::from([("accent".into(), "British".into())])),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["name"], "My Voice");
        assert_eq!(json["description"], "A custom voice");
        assert_eq!(json["labels"]["accent"], "British");
    }

    #[test]
    fn add_voice_request_omits_none_fields() {
        let req = AddVoiceRequest { name: "Minimal".into(), description: None, labels: None };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("name"));
        assert!(!json.contains("description"));
        assert!(!json.contains("labels"));
    }

    #[test]
    fn edit_voice_request_serialize() {
        let req = EditVoiceRequest {
            name: "Updated Name".into(),
            description: None,
            labels: Some(HashMap::new()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["name"], "Updated Name");
        assert!(json.get("description").is_none());
        assert_eq!(json["labels"], serde_json::json!({}));
    }

    #[test]
    fn fine_tuning_state_round_trip() {
        let variants = [
            (FineTuningState::NotStarted, "\"not_started\""),
            (FineTuningState::Queued, "\"queued\""),
            (FineTuningState::FineTuning, "\"fine_tuning\""),
            (FineTuningState::FineTuned, "\"fine_tuned\""),
            (FineTuningState::Failed, "\"failed\""),
            (FineTuningState::Delayed, "\"delayed\""),
        ];
        for (variant, expected) in &variants {
            let json = serde_json::to_string(variant).unwrap();
            assert_eq!(&json, expected);
            let back: FineTuningState = serde_json::from_str(&json).unwrap();
            assert_eq!(*variant, back);
        }
    }

    #[test]
    fn voice_sharing_status_round_trip() {
        let variants = [
            (VoiceSharingStatus::Enabled, "\"enabled\""),
            (VoiceSharingStatus::Disabled, "\"disabled\""),
            (VoiceSharingStatus::Copied, "\"copied\""),
            (VoiceSharingStatus::CopiedDisabled, "\"copied_disabled\""),
        ];
        for (variant, expected) in &variants {
            let json = serde_json::to_string(variant).unwrap();
            assert_eq!(&json, expected);
            let back: VoiceSharingStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(*variant, back);
        }
    }

    #[test]
    fn review_status_round_trip() {
        let variants = [
            (ReviewStatus::NotRequested, "\"not_requested\""),
            (ReviewStatus::Pending, "\"pending\""),
            (ReviewStatus::Declined, "\"declined\""),
            (ReviewStatus::Allowed, "\"allowed\""),
            (ReviewStatus::AllowedWithChanges, "\"allowed_with_changes\""),
        ];
        for (variant, expected) in &variants {
            let json = serde_json::to_string(variant).unwrap();
            assert_eq!(&json, expected);
            let back: ReviewStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(*variant, back);
        }
    }

    #[test]
    fn voice_sample_deserialize() {
        let json = r#"{
            "sample_id": "DCwhRBWXzGAHq8TQ4Fs18",
            "file_name": "sample.mp3",
            "mime_type": "audio/mpeg",
            "size_bytes": 1000000,
            "hash": "1234567890"
        }"#;
        let sample: VoiceSample = serde_json::from_str(json).unwrap();
        assert_eq!(sample.sample_id, "DCwhRBWXzGAHq8TQ4Fs18");
        assert_eq!(sample.file_name, "sample.mp3");
        assert_eq!(sample.size_bytes, 1000000);
        assert!(sample.duration_secs.is_none());
    }

    #[test]
    fn voice_with_samples_deserialize() {
        let json = r#"{
            "voice_id": "abc123",
            "name": "Test",
            "category": "cloned",
            "labels": {},
            "available_for_tiers": [],
            "high_quality_base_model_ids": [],
            "samples": [
                {
                    "sample_id": "s1",
                    "file_name": "hello.mp3",
                    "mime_type": "audio/mpeg",
                    "size_bytes": 50000,
                    "hash": "abcdef",
                    "duration_secs": 3.5
                }
            ]
        }"#;
        let voice: Voice = serde_json::from_str(json).unwrap();
        let samples = voice.samples.unwrap();
        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0].duration_secs, Some(3.5));
    }

    #[test]
    fn moderation_check_deserialize() {
        let json = r#"{
            "date_checked_unix": 1714204800,
            "name_value": "Rachel",
            "name_check": true,
            "description_value": "A female voice.",
            "description_check": true,
            "sample_ids": ["s1", "s2"],
            "sample_checks": [0.95, 0.98],
            "captcha_ids": ["c1"],
            "captcha_checks": [0.99]
        }"#;
        let check: ModerationCheck = serde_json::from_str(json).unwrap();
        assert_eq!(check.date_checked_unix, Some(1714204800));
        assert_eq!(check.name_check, Some(true));
        assert_eq!(check.sample_checks.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn voice_sharing_deserialize() {
        let json = r#"{
            "status": "enabled",
            "date_unix": 1714204800,
            "whitelisted_emails": [],
            "public_owner_id": "owner1",
            "original_voice_id": "voice1",
            "financial_rewards_enabled": false,
            "free_users_allowed": true,
            "live_moderation_enabled": false,
            "notice_period": 30,
            "voice_mixing_allowed": false,
            "featured": false,
            "category": "premade",
            "liked_by_count": 0,
            "cloned_by_count": 0,
            "name": "Test Voice",
            "labels": {},
            "review_status": "not_requested",
            "enabled_in_library": false
        }"#;
        let sharing: VoiceSharing = serde_json::from_str(json).unwrap();
        assert_eq!(sharing.status, VoiceSharingStatus::Enabled);
        assert_eq!(sharing.review_status, ReviewStatus::NotRequested);
        assert!(!sharing.enabled_in_library);
    }

    #[test]
    fn recording_deserialize() {
        let json = r#"{
            "recording_id": "rec1",
            "mime_type": "audio/mpeg",
            "size_bytes": 500000,
            "upload_date_unix": 1714204800,
            "transcription": "Hello world"
        }"#;
        let rec: Recording = serde_json::from_str(json).unwrap();
        assert_eq!(rec.recording_id, "rec1");
        assert_eq!(rec.transcription, "Hello world");
    }
}
