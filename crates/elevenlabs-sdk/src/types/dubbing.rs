//! Types for the ElevenLabs Dubbing endpoints.
//!
//! Covers dubbing project management and dubbing studio operations:
//! - `POST /v1/dubbing` — dub a video or audio file
//! - `GET /v1/dubbing` — list dubbing projects
//! - `GET /v1/dubbing/{dubbing_id}` — get dubbing metadata
//! - `DELETE /v1/dubbing/{dubbing_id}` — delete a dubbing project
//! - `GET /v1/dubbing/{dubbing_id}/audio/{language_code}` — get dubbed audio
//! - `GET /v1/dubbing/{dubbing_id}/transcript/{language_code}` — get transcript
//! - `GET /v1/dubbing/{dubbing_id}/transcripts/{language_code}/format/{format_type}`
//! - `GET /v1/dubbing/resource/{dubbing_id}` — get dubbing resource
//! - `PATCH /v1/dubbing/resource/{dubbing_id}/segment/{segment_id}/{language}`
//! - `DELETE /v1/dubbing/resource/{dubbing_id}/segment/{segment_id}`
//! - `POST /v1/dubbing/resource/{dubbing_id}/speaker` — create speaker
//! - `PATCH /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}` — update speaker
//! - `POST /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}/segment` — add segment
//! - `GET /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}/similar-voices`
//! - `POST /v1/dubbing/resource/{dubbing_id}/dub` — dub segments
//! - `POST /v1/dubbing/resource/{dubbing_id}/render/{language}` — render audio/video
//! - `POST /v1/dubbing/resource/{dubbing_id}/transcribe` — transcribe segments
//! - `POST /v1/dubbing/resource/{dubbing_id}/translate` — translate segments
//! - `POST /v1/dubbing/resource/{dubbing_id}/language` — add a language
//! - `POST /v1/dubbing/resource/{dubbing_id}/migrate-segments` — move segments

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::VoiceCategory;

// ===========================================================================
// Enums
// ===========================================================================

/// The dubbing model to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DubbingModel {
    /// Dubbing v2 model.
    #[serde(rename = "dubbing_v2")]
    DubbingV2,
    /// Dubbing v3 model.
    #[serde(rename = "dubbing_v3")]
    DubbingV3,
    /// End-to-end dubbing v1 model.
    #[serde(rename = "dubbing_e2e_v1")]
    DubbingE2eV1,
}

/// Status of a dubbing project.
///
/// The `examples` in the OpenAPI spec list these as common values, but
/// the field is typed as a free-form string, so we keep this enum
/// non-exhaustive and accept unknown variants.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DubbingStatus {
    /// The dubbing project is being prepared.
    Preparing,
    /// The dubbing job is in the queue.
    Queued,
    /// The dubbing is in progress.
    Dubbing,
    /// The dubbing is complete.
    Dubbed,
    /// The dubbing has failed.
    Failed,
    /// Voices are being cloned.
    Cloning,
}

/// Transcript output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptFormat {
    /// SubRip Text format.
    Srt,
    /// WebVTT format.
    Webvtt,
    /// Structured JSON format.
    Json,
}

/// Render status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderStatus {
    /// Render completed successfully.
    Complete,
    /// Render is currently processing.
    Processing,
    /// Render has failed.
    Failed,
}

// ===========================================================================
// Core dubbing types (response)
// ===========================================================================

/// Metadata about dubbed media (duration and content type).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingMediaMetadata {
    /// The MIME content type of the media.
    pub content_type: String,
    /// Duration of the media in seconds.
    pub duration: f64,
}

/// A reference to a media file stored in the dubbing system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingMediaReference {
    /// Source path.
    pub src: String,
    /// MIME content type.
    pub content_type: String,
    /// Storage bucket name.
    pub bucket_name: String,
    /// Random path slug.
    pub random_path_slug: String,
    /// Duration in seconds.
    pub duration_secs: f64,
    /// Whether this is an audio-only file.
    pub is_audio: bool,
    /// Accessible URL for the media.
    pub url: String,
}

/// Metadata for a single dubbing project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingMetadataResponse {
    /// The ID of the dubbing project.
    pub dubbing_id: String,
    /// The name of the dubbing project.
    pub name: String,
    /// The state this dub is in.
    pub status: String,
    /// ISO-639-1 code of the original media's source language, if detected.
    pub source_language: Option<String>,
    /// ISO-639-1 codes of the languages this media has been dubbed into.
    pub target_languages: Vec<String>,
    /// Whether this dubbing project is editable in Dubbing Studio.
    #[serde(default)]
    pub editable: bool,
    /// Timestamp when this dub was created (ISO-8601).
    pub created_at: String,
    /// Metadata about the dubbed content (length, type).
    pub media_metadata: Option<DubbingMediaMetadata>,
    /// Error message if dubbing failed.
    pub error: Option<String>,
}

/// Paginated list of dubbing projects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingMetadataPageResponse {
    /// The dubbing projects on this page.
    pub dubs: Vec<DubbingMetadataResponse>,
    /// Cursor for the next page, if any.
    pub next_cursor: Option<String>,
    /// Whether there are more pages.
    pub has_more: bool,
}

/// Response after creating a new dubbing project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoDubbingResponse {
    /// The ID of the created dubbing project.
    pub dubbing_id: String,
    /// Expected duration of the dubbing in seconds.
    pub expected_duration_sec: f64,
}

/// Response after deleting a dubbing project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteDubbingResponse {
    /// Status of the deletion. "ok" on success.
    pub status: String,
}

// ===========================================================================
// Dubbing resource (studio) types (response)
// ===========================================================================

/// A render of dubbed content for a specific language.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Render {
    /// Render ID.
    pub id: String,
    /// Resource version when rendered.
    pub version: i64,
    /// Target language of the render.
    pub language: Option<String>,
    /// Type of render (e.g., audio, video). Complex type — stored as Value.
    #[serde(rename = "type")]
    pub render_type: Option<serde_json::Value>,
    /// Media reference for the rendered file.
    pub media_ref: Option<DubbingMediaReference>,
    /// Status of the render.
    pub status: RenderStatus,
}

/// A subtitle frame within a dubbed segment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentSubtitleFrame {
    /// Start time of the subtitle frame in seconds.
    pub start_time: f64,
    /// End time of the subtitle frame in seconds.
    pub end_time: f64,
    /// Lines of subtitle text.
    pub lines: Vec<String>,
}

/// A dubbed segment for a specific language.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbedSegment {
    /// Start time of the segment in seconds.
    pub start_time: f64,
    /// End time of the segment in seconds.
    pub end_time: f64,
    /// The dubbed text for this segment.
    pub text: Option<String>,
    /// Subtitle frames within this segment.
    pub subtitles: Vec<SegmentSubtitleFrame>,
    /// Whether the audio for this segment is stale and needs re-dubbing.
    pub audio_stale: bool,
    /// Media reference for the dubbed audio.
    pub media_ref: Option<DubbingMediaReference>,
}

/// A speaker segment containing source text and per-language dubs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeakerSegment {
    /// Segment ID.
    pub id: String,
    /// Start time in seconds.
    pub start_time: f64,
    /// End time in seconds.
    pub end_time: f64,
    /// Source text for this segment.
    pub text: String,
    /// Subtitle frames for this segment.
    pub subtitles: Vec<SegmentSubtitleFrame>,
    /// Per-language dubbed segments. Keys are language codes.
    pub dubs: HashMap<String, DubbedSegment>,
}

/// A speaker track within a dubbing resource.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeakerTrack {
    /// Speaker track ID.
    pub id: String,
    /// Media reference for the speaker's audio track.
    pub media_ref: DubbingMediaReference,
    /// Display name for the speaker.
    pub speaker_name: String,
    /// Per-language voice ID assignments. Keys are language codes.
    pub voices: HashMap<String, String>,
    /// Segment IDs belonging to this speaker.
    pub segments: Vec<String>,
}

/// Full dubbing resource with speakers, segments, and renders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingResource {
    /// Resource ID (same as dubbing_id).
    pub id: String,
    /// Version of the resource.
    pub version: i64,
    /// Source language code.
    pub source_language: String,
    /// Target language codes.
    pub target_languages: Vec<String>,
    /// Input media reference.
    pub input: DubbingMediaReference,
    /// Background audio track, if separated.
    pub background: Option<DubbingMediaReference>,
    /// Foreground audio track, if separated.
    pub foreground: Option<DubbingMediaReference>,
    /// Speaker tracks keyed by speaker ID.
    pub speaker_tracks: HashMap<String, SpeakerTrack>,
    /// Speaker segments keyed by segment ID.
    pub speaker_segments: HashMap<String, SpeakerSegment>,
    /// Renders keyed by render ID.
    pub renders: HashMap<String, Render>,
}

// ===========================================================================
// Transcript types (response)
// ===========================================================================

/// A single character within a transcript word.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingTranscriptCharacter {
    /// The character text.
    #[serde(default)]
    pub text: String,
    /// Start time of this character in seconds.
    #[serde(default)]
    pub start_s: f64,
    /// End time of this character in seconds.
    #[serde(default)]
    pub end_s: f64,
}

/// A single word within a transcript utterance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingTranscriptWord {
    /// The word text.
    #[serde(default)]
    pub text: String,
    /// Word type (e.g. "word", "punctuation", "unknown").
    #[serde(default)]
    pub word_type: String,
    /// Start time of this word in seconds.
    #[serde(default)]
    pub start_s: f64,
    /// End time of this word in seconds.
    #[serde(default)]
    pub end_s: f64,
    /// Individual character timings.
    #[serde(default)]
    pub characters: Vec<DubbingTranscriptCharacter>,
}

/// A single utterance (sentence/phrase) in a transcript.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingTranscriptUtterance {
    /// The full text of the utterance.
    #[serde(default)]
    pub text: String,
    /// Speaker ID who spoke this utterance.
    #[serde(default)]
    pub speaker_id: String,
    /// Start time in seconds.
    #[serde(default)]
    pub start_s: f64,
    /// End time in seconds.
    #[serde(default)]
    pub end_s: f64,
    /// Individual words in this utterance.
    #[serde(default)]
    pub words: Vec<DubbingTranscriptWord>,
}

/// A transcript for a specific language.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingTranscript {
    /// Language code of this transcript.
    pub language: String,
    /// Utterances in this transcript.
    pub utterances: Vec<DubbingTranscriptUtterance>,
}

/// Response from the transcript endpoint (same shape as `DubbingTranscript`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingTranscriptResponse {
    /// Language code of this transcript.
    pub language: String,
    /// Utterances in this transcript.
    pub utterances: Vec<DubbingTranscriptUtterance>,
}

/// Response from the formatted transcript endpoint (SRT/WebVTT/JSON).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DubbingTranscriptsResponse {
    /// The format of the transcript.
    pub transcript_format: TranscriptFormat,
    /// SRT-formatted transcript string (if format is SRT).
    pub srt: Option<String>,
    /// WebVTT-formatted transcript string (if format is WebVTT).
    pub webvtt: Option<String>,
    /// JSON-structured transcript (if format is JSON).
    pub json: Option<DubbingTranscript>,
}

// ===========================================================================
// Speaker types (response)
// ===========================================================================

/// Time range for an utterance response.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UtteranceResponse {
    /// Start time in seconds.
    pub start: f64,
    /// End time in seconds.
    pub end: f64,
}

/// Speaker metadata from the dubbing system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeakerResponse {
    /// The speaker ID.
    pub speaker_id: String,
    /// Duration of the speaker's audio in seconds.
    pub duration_secs: f64,
    /// Utterances spoken by this speaker.
    pub utterances: Option<Vec<UtteranceResponse>>,
}

/// Base64-encoded audio response for a speaker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeakerAudioResponse {
    /// Base64-encoded audio data.
    pub audio_base_64: String,
    /// MIME type of the audio (e.g. "audio/mpeg").
    pub media_type: String,
    /// Duration of the audio in seconds.
    pub duration_secs: f64,
}

/// Response after creating a new speaker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerCreatedResponse {
    /// Updated resource version.
    pub version: i64,
    /// ID of the created speaker.
    pub speaker_id: String,
}

/// Response after updating speaker metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerUpdatedResponse {
    /// Updated resource version.
    pub version: i64,
}

/// A voice similar to a speaker's voice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimilarVoice {
    /// Voice ID.
    pub voice_id: String,
    /// Display name of the voice.
    pub name: String,
    /// Category of the voice.
    pub category: VoiceCategory,
    /// Description of the voice.
    pub description: Option<String>,
    /// URL for previewing the voice.
    pub preview_url: Option<String>,
}

/// Response containing similar voices for a speaker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimilarVoicesForSpeakerResponse {
    /// List of similar voices.
    pub voices: Vec<SimilarVoice>,
}

// ===========================================================================
// Segment CRUD types
// ===========================================================================

/// Payload to create a new segment for a speaker.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SegmentCreatePayload {
    /// Start time of the segment in seconds.
    pub start_time: f64,
    /// End time of the segment in seconds.
    pub end_time: f64,
    /// Source text for the segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Per-language translations. Keys are language codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<HashMap<String, String>>,
}

/// Response after creating a new segment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegmentCreateResponse {
    /// Updated resource version.
    pub version: i64,
    /// ID of the newly created segment.
    pub new_segment: String,
}

/// Payload to update an existing segment.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SegmentUpdatePayload {
    /// New start time in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// New end time in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// New text for the segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Response after updating a segment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegmentUpdateResponse {
    /// Updated resource version.
    pub version: i64,
}

/// Response after deleting a segment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegmentDeleteResponse {
    /// Updated resource version.
    pub version: i64,
}

/// Response after dubbing segments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegmentDubResponse {
    /// Updated resource version.
    pub version: i64,
}

/// Response after migrating segments between speakers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegmentMigrationResponse {
    /// Updated resource version.
    pub version: i64,
}

/// Response after transcribing segments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegmentTranscriptionResponse {
    /// Updated resource version.
    pub version: i64,
}

/// Response after translating segments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegmentTranslationResponse {
    /// Updated resource version.
    pub version: i64,
}

// ===========================================================================
// Render response
// ===========================================================================

/// Response after starting a render of dubbed content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DubbingRenderResponse {
    /// Resource version when the render started.
    pub version: i64,
    /// ID of the render job.
    pub render_id: String,
}

// ===========================================================================
// Render type enum
// ===========================================================================

/// Output format for a dubbing render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderType {
    /// MP4 video.
    #[serde(rename = "mp4")]
    Mp4,
    /// AAC audio.
    #[serde(rename = "aac")]
    Aac,
    /// MP3 audio.
    #[serde(rename = "mp3")]
    Mp3,
    /// WAV audio.
    #[serde(rename = "wav")]
    Wav,
    /// AAF project file.
    #[serde(rename = "aaf")]
    Aaf,
    /// ZIP of individual tracks.
    #[serde(rename = "tracks_zip")]
    TracksZip,
    /// ZIP of individual clips.
    #[serde(rename = "clips_zip")]
    ClipsZip,
}

// ===========================================================================
// Additional response types
// ===========================================================================

/// Response after adding a language to a dubbing resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageAddedResponse {
    /// Updated resource version.
    pub version: i64,
}

// ===========================================================================
// Request types (Serialize only)
// ===========================================================================

/// Request body for adding a language to a dubbing resource.
///
/// Sent as JSON to `POST /v1/dubbing/resource/{dubbing_id}/language`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddLanguageRequest {
    /// ISO-639-1 language code to add.
    pub language: String,
}

/// Request body for migrating segments between speakers.
///
/// Sent as JSON to `POST /v1/dubbing/resource/{dubbing_id}/migrate-segments`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MigrateSegmentsRequest {
    /// IDs of the segments to migrate.
    pub segment_ids: Vec<String>,
    /// Target speaker ID.
    pub speaker_id: String,
}

/// Request body for transcribing segments.
///
/// Sent as JSON to `POST /v1/dubbing/resource/{dubbing_id}/transcribe`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TranscribeSegmentsRequest {
    /// IDs of the segments to transcribe.
    pub segments: Vec<String>,
}

/// Request body for translating segments.
///
/// Sent as JSON to `POST /v1/dubbing/resource/{dubbing_id}/translate`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TranslateSegmentsRequest {
    /// IDs of the segments to translate.
    pub segments: Vec<String>,
    /// Target language codes. If empty, all target languages are used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
}

/// Request body for dubbing segments.
///
/// Sent as JSON to `POST /v1/dubbing/resource/{dubbing_id}/dub`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DubSegmentsRequest {
    /// IDs of the segments to dub.
    pub segments: Vec<String>,
    /// Target language codes. If empty, all target languages are used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
}

/// Request body for rendering dubbed content.
///
/// Sent as JSON to `POST /v1/dubbing/resource/{dubbing_id}/render/{language}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderDubbingRequest {
    /// Output format for the render.
    pub render_type: RenderType,
    /// Whether to normalize volume across speakers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalize_volume: Option<bool>,
}

/// Request body for creating a new speaker in a dubbing resource.
///
/// Sent as JSON to `POST /v1/dubbing/resource/{dubbing_id}/speaker`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateSpeakerRequest {
    /// Display name for the speaker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_name: Option<String>,
    /// Voice ID to assign to the speaker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// Voice stability setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_stability: Option<f64>,
    /// Voice similarity setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_similarity: Option<f64>,
    /// Voice style setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_style: Option<f64>,
}

/// Request body for creating a new dubbing project.
///
/// This is a multipart request. File fields (`file`, `csv_file`, etc.) are
/// binary uploads handled at the client layer. This struct covers the
/// non-file fields typically sent as form parameters.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateDubbingRequest {
    /// Name of the dubbing project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL of the source video/audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// Source language code (ISO-639-1 or ISO-639-3). Defaults to "auto".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lang: Option<String>,
    /// Target language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_lang: Option<String>,
    /// Target accent for the dubbed audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_accent: Option<String>,
    /// Number of speakers in the source media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_speakers: Option<i64>,
    /// Whether to add a watermark to the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<bool>,
    /// Start time in seconds to begin dubbing from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End time in seconds to stop dubbing at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Whether to use the highest resolution available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_resolution: Option<bool>,
    /// Whether to drop the original background audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_background_audio: Option<bool>,
    /// Whether to filter profanity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_profanity_filter: Option<bool>,
    /// Whether to use dubbing studio for editing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dubbing_studio: Option<bool>,
    /// Whether to disable voice cloning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_voice_cloning: Option<bool>,
    /// Dubbing mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Frames per second for CSV-based dubbing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_fps: Option<f64>,
}

/// Payload to update speaker metadata.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSpeakerRequest {
    /// New display name for the speaker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_name: Option<String>,
    /// Voice ID to assign to the speaker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// Voice stability setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_stability: Option<f64>,
    /// Voice similarity setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_similarity: Option<f64>,
    /// Voice style setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_style: Option<f64>,
    /// Language codes for the speaker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    // -- DubbingModel -------------------------------------------------------

    #[test]
    fn dubbing_model_serde_round_trip() {
        let model = DubbingModel::DubbingV3;
        let json = serde_json::to_string(&model).unwrap();
        assert_eq!(json, "\"dubbing_v3\"");
        let back: DubbingModel = serde_json::from_str(&json).unwrap();
        assert_eq!(model, back);
    }

    // -- DubbingMediaMetadata -----------------------------------------------

    #[test]
    fn dubbing_media_metadata_deserialize() {
        let json = r#"{"content_type":"video/mp4","duration":127.5}"#;
        let meta: DubbingMediaMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(meta.content_type, "video/mp4");
        assert!((meta.duration - 127.5).abs() < f64::EPSILON);
    }

    // -- DubbingMetadataResponse --------------------------------------------

    #[test]
    fn dubbing_metadata_response_deserialize() {
        let json = r#"{
            "dubbing_id": "dub_123",
            "name": "My Dub",
            "status": "dubbed",
            "source_language": "en",
            "target_languages": ["es", "fr"],
            "editable": true,
            "created_at": "2025-01-15T10:00:00Z",
            "media_metadata": {"content_type": "video/mp4", "duration": 60.0},
            "error": null
        }"#;
        let resp: DubbingMetadataResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.dubbing_id, "dub_123");
        assert_eq!(resp.name, "My Dub");
        assert_eq!(resp.status, "dubbed");
        assert_eq!(resp.source_language.as_deref(), Some("en"));
        assert_eq!(resp.target_languages, vec!["es", "fr"]);
        assert!(resp.editable);
        assert!(resp.media_metadata.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn dubbing_metadata_response_minimal() {
        let json = r#"{
            "dubbing_id": "dub_456",
            "name": "Test",
            "status": "preparing",
            "source_language": null,
            "target_languages": [],
            "created_at": "2025-02-01T00:00:00Z"
        }"#;
        let resp: DubbingMetadataResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.dubbing_id, "dub_456");
        assert!(!resp.editable);
        assert!(resp.source_language.is_none());
    }

    // -- DubbingMetadataPageResponse ----------------------------------------

    #[test]
    fn dubbing_metadata_page_response_deserialize() {
        let json = r#"{
            "dubs": [{
                "dubbing_id": "d1",
                "name": "Dub One",
                "status": "dubbed",
                "source_language": "en",
                "target_languages": ["es"],
                "created_at": "2025-01-01T00:00:00Z"
            }],
            "next_cursor": "abc123",
            "has_more": true
        }"#;
        let page: DubbingMetadataPageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(page.dubs.len(), 1);
        assert_eq!(page.next_cursor.as_deref(), Some("abc123"));
        assert!(page.has_more);
    }

    // -- DoDubbingResponse --------------------------------------------------

    #[test]
    fn do_dubbing_response_deserialize() {
        let json = r#"{"dubbing_id":"21m00Tcm4TlvDq8ikWAM","expected_duration_sec":127.5}"#;
        let resp: DoDubbingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.dubbing_id, "21m00Tcm4TlvDq8ikWAM");
        assert!((resp.expected_duration_sec - 127.5).abs() < f64::EPSILON);
    }

    // -- DeleteDubbingResponse ----------------------------------------------

    #[test]
    fn delete_dubbing_response_deserialize() {
        let json = r#"{"status":"ok"}"#;
        let resp: DeleteDubbingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    // -- DubbingRenderResponse ----------------------------------------------

    #[test]
    fn dubbing_render_response_deserialize() {
        let json = r#"{"version":3,"render_id":"render_abc"}"#;
        let resp: DubbingRenderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.version, 3);
        assert_eq!(resp.render_id, "render_abc");
    }

    // -- SegmentSubtitleFrame -----------------------------------------------

    #[test]
    fn segment_subtitle_frame_deserialize() {
        let json = r#"{"start_time":1.0,"end_time":2.5,"lines":["Hello","World"]}"#;
        let frame: SegmentSubtitleFrame = serde_json::from_str(json).unwrap();
        assert!((frame.start_time - 1.0).abs() < f64::EPSILON);
        assert_eq!(frame.lines, vec!["Hello", "World"]);
    }

    // -- DubbingTranscript types --------------------------------------------

    #[test]
    fn dubbing_transcript_character_deserialize() {
        let json = r#"{"text":"H","start_s":0.1,"end_s":0.2}"#;
        let ch: DubbingTranscriptCharacter = serde_json::from_str(json).unwrap();
        assert_eq!(ch.text, "H");
    }

    #[test]
    fn dubbing_transcript_word_deserialize() {
        let json =
            r#"{"text":"Hello","word_type":"word","start_s":0.0,"end_s":0.5,"characters":[]}"#;
        let word: DubbingTranscriptWord = serde_json::from_str(json).unwrap();
        assert_eq!(word.text, "Hello");
        assert_eq!(word.word_type, "word");
    }

    #[test]
    fn dubbing_transcript_utterance_defaults() {
        // All fields have defaults, so an empty object should deserialize.
        let json = r#"{}"#;
        let utt: DubbingTranscriptUtterance = serde_json::from_str(json).unwrap();
        assert_eq!(utt.text, "");
        assert_eq!(utt.speaker_id, "");
    }

    #[test]
    fn dubbing_transcript_deserialize() {
        let json = r#"{
            "language": "en",
            "utterances": [{
                "text": "Hello world",
                "speaker_id": "spk1",
                "start_s": 0.0,
                "end_s": 1.5,
                "words": []
            }]
        }"#;
        let t: DubbingTranscript = serde_json::from_str(json).unwrap();
        assert_eq!(t.language, "en");
        assert_eq!(t.utterances.len(), 1);
        assert_eq!(t.utterances[0].text, "Hello world");
    }

    // -- DubbingTranscriptsResponse -----------------------------------------

    #[test]
    fn dubbing_transcripts_response_srt() {
        let json = r#"{
            "transcript_format": "srt",
            "srt": "1\n00:00:00,000 --> 00:00:01,500\nHello",
            "webvtt": null,
            "json": null
        }"#;
        let resp: DubbingTranscriptsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.transcript_format, TranscriptFormat::Srt);
        assert!(resp.srt.is_some());
        assert!(resp.json.is_none());
    }

    // -- SpeakerResponse ----------------------------------------------------

    #[test]
    fn speaker_response_deserialize() {
        let json = r#"{"speaker_id":"DCwhRBWXzGAHq8TQ4Fs18","duration_secs":5.0}"#;
        let spk: SpeakerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(spk.speaker_id, "DCwhRBWXzGAHq8TQ4Fs18");
        assert!(spk.utterances.is_none());
    }

    #[test]
    fn speaker_response_with_utterances() {
        let json = r#"{
            "speaker_id": "spk1",
            "duration_secs": 3.0,
            "utterances": [{"start": 0.0, "end": 1.0}]
        }"#;
        let spk: SpeakerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(spk.utterances.as_ref().unwrap().len(), 1);
    }

    // -- SpeakerAudioResponse -----------------------------------------------

    #[test]
    fn speaker_audio_response_deserialize() {
        let json = r#"{
            "audio_base_64": "AAAA",
            "media_type": "audio/mpeg",
            "duration_secs": 5.0
        }"#;
        let audio: SpeakerAudioResponse = serde_json::from_str(json).unwrap();
        assert_eq!(audio.media_type, "audio/mpeg");
    }

    // -- SpeakerCreatedResponse / SpeakerUpdatedResponse --------------------

    #[test]
    fn speaker_created_response_deserialize() {
        let json = r#"{"version":2,"speaker_id":"new_spk"}"#;
        let resp: SpeakerCreatedResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.version, 2);
        assert_eq!(resp.speaker_id, "new_spk");
    }

    #[test]
    fn speaker_updated_response_deserialize() {
        let json = r#"{"version":3}"#;
        let resp: SpeakerUpdatedResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.version, 3);
    }

    // -- SimilarVoicesForSpeakerResponse ------------------------------------

    #[test]
    fn similar_voices_response_deserialize() {
        let json = r#"{
            "voices": [{
                "voice_id": "v1",
                "name": "Voice One",
                "category": "premade",
                "description": "A calm voice",
                "preview_url": "https://example.com/v1.mp3"
            }]
        }"#;
        let resp: SimilarVoicesForSpeakerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.voices.len(), 1);
        assert_eq!(resp.voices[0].voice_id, "v1");
        assert_eq!(resp.voices[0].category, VoiceCategory::Premade);
    }

    // -- SegmentCreatePayload -----------------------------------------------

    #[test]
    fn segment_create_payload_serialize() {
        let payload = SegmentCreatePayload {
            start_time: 1.0,
            end_time: 5.0,
            text: Some("Hello".into()),
            translations: None,
        };
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"start_time\":1.0"));
        assert!(json.contains("\"text\":\"Hello\""));
        assert!(!json.contains("translations"));
    }

    #[test]
    fn segment_create_payload_with_translations() {
        let mut translations = HashMap::new();
        translations.insert("es".to_string(), "Hola".to_string());
        let payload = SegmentCreatePayload {
            start_time: 0.0,
            end_time: 2.0,
            text: None,
            translations: Some(translations),
        };
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"translations\""));
        assert!(!json.contains("\"text\""));
    }

    // -- SegmentCreateResponse ----------------------------------------------

    #[test]
    fn segment_create_response_deserialize() {
        let json = r#"{"version":4,"new_segment":"seg_new"}"#;
        let resp: SegmentCreateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.version, 4);
        assert_eq!(resp.new_segment, "seg_new");
    }

    // -- SegmentUpdatePayload -----------------------------------------------

    #[test]
    fn segment_update_payload_serialize_partial() {
        let payload = SegmentUpdatePayload {
            start_time: None,
            end_time: Some(10.0),
            text: Some("Updated text".into()),
        };
        let json = serde_json::to_string(&payload).unwrap();
        assert!(!json.contains("start_time"));
        assert!(json.contains("\"end_time\":10.0"));
        assert!(json.contains("\"text\":\"Updated text\""));
    }

    // -- Version-only response types ----------------------------------------

    #[test]
    fn segment_version_responses_deserialize() {
        let json = r#"{"version":5}"#;
        let _update: SegmentUpdateResponse = serde_json::from_str(json).unwrap();
        let _delete: SegmentDeleteResponse = serde_json::from_str(json).unwrap();
        let _dub: SegmentDubResponse = serde_json::from_str(json).unwrap();
        let _migrate: SegmentMigrationResponse = serde_json::from_str(json).unwrap();
        let _transcribe: SegmentTranscriptionResponse = serde_json::from_str(json).unwrap();
        let _translate: SegmentTranslationResponse = serde_json::from_str(json).unwrap();
    }

    // -- CreateDubbingRequest -----------------------------------------------

    #[test]
    fn create_dubbing_request_serialize_minimal() {
        let req = CreateDubbingRequest {
            name: Some("Test dub".into()),
            source_url: Some("https://example.com/video.mp4".into()),
            source_lang: None,
            target_lang: Some("es".into()),
            target_accent: None,
            num_speakers: None,
            watermark: None,
            start_time: None,
            end_time: None,
            highest_resolution: None,
            drop_background_audio: None,
            use_profanity_filter: None,
            dubbing_studio: None,
            disable_voice_cloning: None,
            mode: None,
            csv_fps: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"name\":\"Test dub\""));
        assert!(json.contains("\"source_url\""));
        assert!(json.contains("\"target_lang\":\"es\""));
        assert!(!json.contains("watermark"));
        assert!(!json.contains("num_speakers"));
    }

    // -- UpdateSpeakerRequest -----------------------------------------------

    #[test]
    fn update_speaker_request_serialize() {
        let req = UpdateSpeakerRequest {
            speaker_name: Some("Alice".into()),
            voice_id: Some("v123".into()),
            voice_stability: Some(0.5),
            voice_similarity: None,
            voice_style: None,
            languages: Some(vec!["en".into(), "es".into()]),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"speaker_name\":\"Alice\""));
        assert!(json.contains("\"voice_id\":\"v123\""));
        assert!(!json.contains("voice_similarity"));
        assert!(json.contains("\"languages\""));
    }

    // -- DubbingResource full deserialize -----------------------------------

    #[test]
    fn dubbing_resource_deserialize() {
        let json = r#"{
            "id": "dub_full",
            "version": 7,
            "source_language": "en",
            "target_languages": ["es"],
            "input": {
                "src": "/path/input.mp4",
                "content_type": "video/mp4",
                "bucket_name": "bucket",
                "random_path_slug": "slug123",
                "duration_secs": 120.0,
                "is_audio": false,
                "url": "https://cdn.example.com/input.mp4"
            },
            "background": null,
            "foreground": null,
            "speaker_tracks": {
                "spk1": {
                    "id": "spk1",
                    "media_ref": {
                        "src": "/path/spk1.wav",
                        "content_type": "audio/wav",
                        "bucket_name": "bucket",
                        "random_path_slug": "s1",
                        "duration_secs": 30.0,
                        "is_audio": true,
                        "url": "https://cdn.example.com/spk1.wav"
                    },
                    "speaker_name": "Speaker 1",
                    "voices": {"es": "voice_es_1"},
                    "segments": ["seg1"]
                }
            },
            "speaker_segments": {
                "seg1": {
                    "id": "seg1",
                    "start_time": 0.0,
                    "end_time": 5.0,
                    "text": "Hello world",
                    "subtitles": [{"start_time": 0.0, "end_time": 2.5, "lines": ["Hello"]},
                                  {"start_time": 2.5, "end_time": 5.0, "lines": ["world"]}],
                    "dubs": {
                        "es": {
                            "start_time": 0.0,
                            "end_time": 5.0,
                            "text": "Hola mundo",
                            "subtitles": [],
                            "audio_stale": false,
                            "media_ref": null
                        }
                    }
                }
            },
            "renders": {}
        }"#;
        let resource: DubbingResource = serde_json::from_str(json).unwrap();
        assert_eq!(resource.id, "dub_full");
        assert_eq!(resource.version, 7);
        assert_eq!(resource.source_language, "en");
        assert_eq!(resource.speaker_tracks.len(), 1);
        assert_eq!(resource.speaker_segments.len(), 1);

        let seg = &resource.speaker_segments["seg1"];
        assert_eq!(seg.text, "Hello world");
        assert_eq!(seg.subtitles.len(), 2);
        assert_eq!(seg.dubs.len(), 1);
        assert_eq!(seg.dubs["es"].text.as_deref(), Some("Hola mundo"));
    }

    // -- RenderType ---------------------------------------------------------

    #[test]
    fn render_type_serde_round_trip() {
        let rt = RenderType::Mp4;
        let json = serde_json::to_string(&rt).unwrap();
        assert_eq!(json, "\"mp4\"");
        let back: RenderType = serde_json::from_str(&json).unwrap();
        assert_eq!(rt, back);
    }

    #[test]
    fn render_type_tracks_zip() {
        let rt = RenderType::TracksZip;
        let json = serde_json::to_string(&rt).unwrap();
        assert_eq!(json, "\"tracks_zip\"");
    }

    // -- LanguageAddedResponse ----------------------------------------------

    #[test]
    fn language_added_response_deserialize() {
        let json = r#"{"version":4}"#;
        let resp: LanguageAddedResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.version, 4);
    }

    // -- AddLanguageRequest -------------------------------------------------

    #[test]
    fn add_language_request_serialize() {
        let req = AddLanguageRequest { language: "fr".into() };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"language\":\"fr\""));
    }

    // -- MigrateSegmentsRequest ---------------------------------------------

    #[test]
    fn migrate_segments_request_serialize() {
        let req = MigrateSegmentsRequest {
            segment_ids: vec!["s1".into(), "s2".into()],
            speaker_id: "spk1".into(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"segment_ids\""));
        assert!(json.contains("\"speaker_id\":\"spk1\""));
    }

    // -- TranscribeSegmentsRequest ------------------------------------------

    #[test]
    fn transcribe_segments_request_serialize() {
        let req = TranscribeSegmentsRequest { segments: vec!["seg1".into()] };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"segments\":[\"seg1\"]"));
    }

    // -- DubSegmentsRequest -------------------------------------------------

    #[test]
    fn dub_segments_request_serialize() {
        let req = DubSegmentsRequest {
            segments: vec!["seg1".into()],
            languages: Some(vec!["es".into()]),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"segments\""));
        assert!(json.contains("\"languages\""));
    }

    // -- RenderDubbingRequest -----------------------------------------------

    #[test]
    fn render_dubbing_request_serialize() {
        let req = RenderDubbingRequest { render_type: RenderType::Mp3, normalize_volume: None };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"render_type\":\"mp3\""));
        assert!(!json.contains("normalize_volume"));
    }

    // -- CreateSpeakerRequest -----------------------------------------------

    #[test]
    fn create_speaker_request_serialize() {
        let req = CreateSpeakerRequest {
            speaker_name: Some("Speaker A".into()),
            voice_id: None,
            voice_stability: None,
            voice_similarity: None,
            voice_style: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"speaker_name\":\"Speaker A\""));
        assert!(!json.contains("voice_id"));
    }
}
