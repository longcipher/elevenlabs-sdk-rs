//! Types for the ElevenLabs Studio (Projects) endpoints.
//!
//! Covers project and chapter management, snapshots, podcasts, and content:
//! - `GET /v1/studio/projects` — list projects
//! - `GET /v1/studio/projects/{project_id}` — get a project
//! - `POST /v1/studio/projects` — create a project
//! - `POST /v1/studio/projects/{project_id}` — update a project
//! - `DELETE /v1/studio/projects/{project_id}` — delete a project
//! - `POST /v1/studio/projects/{project_id}/convert` — convert a project
//! - `POST /v1/studio/projects/{project_id}/content` — update project content
//! - `POST /v1/studio/projects/{project_id}/pronunciation-dictionaries`
//! - `GET /v1/studio/projects/{project_id}/snapshots` — list snapshots
//! - `GET /v1/studio/projects/{project_id}/snapshots/{id}` — get snapshot
//! - `POST /v1/studio/projects/{project_id}/snapshots/{id}/stream` — stream audio
//! - `POST /v1/studio/projects/{project_id}/snapshots/{id}/archive` — archive
//! - `GET /v1/studio/projects/{project_id}/muted-tracks` — get muted tracks
//! - `GET /v1/studio/projects/{project_id}/chapters` — list chapters
//! - `GET /v1/studio/projects/{project_id}/chapters/{id}` — get a chapter
//! - `POST /v1/studio/projects/{project_id}/chapters` — create a chapter
//! - `POST /v1/studio/projects/{project_id}/chapters/{id}` — update chapter
//! - `DELETE /v1/studio/projects/{project_id}/chapters/{id}` — delete chapter
//! - `POST /v1/studio/projects/{project_id}/chapters/{id}/convert` — convert chapter
//! - `GET /v1/studio/projects/{project_id}/chapters/{id}/snapshots`
//! - `GET /v1/studio/projects/{project_id}/chapters/{id}/snapshots/{snap_id}`
//! - `POST /v1/studio/projects/{project_id}/chapters/{id}/snapshots/{snap_id}/stream`
//! - `POST /v1/studio/podcasts` — create a podcast

use serde::{Deserialize, Serialize};

// ===========================================================================
// Enums
// ===========================================================================

/// Status of a project creation task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectCreationStatus {
    /// Task is pending.
    Pending,
    /// Task is in progress.
    Creating,
    /// Task completed successfully.
    Finished,
    /// Task failed.
    Failed,
}

/// Type of project creation action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectCreationType {
    /// Blank project.
    Blank,
    /// Podcast generation.
    GeneratePodcast,
    /// Auto-assign voices.
    AutoAssignVoices,
    /// Dub video.
    DubVideo,
    /// Import speech.
    ImportSpeech,
}

/// Target audience for a project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetAudience {
    /// Content for children.
    Children,
    /// Content for young adults.
    #[serde(rename = "young adult")]
    YoungAdult,
    /// Content for adults.
    Adult,
    /// Content for all ages.
    #[serde(rename = "all ages")]
    AllAges,
}

/// Fiction classification for a project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FictionType {
    /// Fiction content.
    Fiction,
    /// Non-fiction content.
    NonFiction,
}

/// Source type for a project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectSourceType {
    /// Blank project.
    Blank,
    /// Book import.
    Book,
    /// Article import.
    Article,
    /// GenFM (podcast) generation.
    Genfm,
    /// Video import.
    Video,
    /// Screenplay import.
    Screenplay,
}

/// Aspect ratio for a project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AspectRatio {
    /// 16:9 landscape.
    #[serde(rename = "16:9")]
    Landscape,
    /// 9:16 portrait.
    #[serde(rename = "9:16")]
    Portrait,
    /// 4:5 social media.
    #[serde(rename = "4:5")]
    Social,
    /// 1:1 square.
    #[serde(rename = "1:1")]
    Square,
}

/// State of a project or chapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectState {
    /// Default state (not converting).
    Default,
    /// Currently converting.
    Converting,
}

/// Block sub-type for chapter content input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockSubType {
    /// Paragraph.
    P,
    /// Heading level 1.
    H1,
    /// Heading level 2.
    H2,
    /// Heading level 3.
    H3,
}

// ===========================================================================
// Project creation meta
// ===========================================================================

/// Metadata about the creation progress of a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectCreationMeta {
    /// Progress of project creation (0.0 to 1.0).
    pub creation_progress: f64,
    /// Status of the creation task.
    pub status: ProjectCreationStatus,
    /// Type of creation action.
    #[serde(rename = "type")]
    pub creation_type: ProjectCreationType,
}

// ===========================================================================
// Project types (response)
// ===========================================================================

/// A Studio project (summary view).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectResponse {
    /// Project ID.
    pub project_id: String,
    /// Project name.
    pub name: String,
    /// Unix timestamp of creation.
    pub create_date_unix: i64,
    /// User ID who created the project.
    pub created_by_user_id: Option<String>,
    /// Default voice ID for titles.
    pub default_title_voice_id: String,
    /// Default voice ID for paragraphs.
    pub default_paragraph_voice_id: String,
    /// Default model ID.
    pub default_model_id: String,
    /// Unix timestamp of last conversion.
    pub last_conversion_date_unix: Option<i64>,
    /// Whether the project can be downloaded.
    pub can_be_downloaded: bool,
    /// Title of the project.
    pub title: Option<String>,
    /// Author of the project.
    pub author: Option<String>,
    /// Description of the project.
    pub description: Option<String>,
    /// Genres of the project.
    pub genres: Option<Vec<String>>,
    /// Cover image URL.
    pub cover_image_url: Option<String>,
    /// Target audience.
    pub target_audience: Option<TargetAudience>,
    /// Language code.
    pub language: Option<String>,
    /// Content type (e.g. "Novel").
    pub content_type: Option<String>,
    /// Original publication date.
    pub original_publication_date: Option<String>,
    /// Whether the project contains mature content.
    pub mature_content: Option<bool>,
    /// ISBN number.
    pub isbn_number: Option<String>,
    /// Whether volume normalization is enabled.
    pub volume_normalization: bool,
    /// Current state of the project.
    pub state: ProjectState,
    /// Access level for the current user.
    pub access_level: String,
    /// Fiction/non-fiction classification.
    pub fiction: Option<FictionType>,
    /// Whether quality check is enabled.
    pub quality_check_on: bool,
    /// Whether quality check is on during bulk conversion.
    pub quality_check_on_when_bulk_convert: bool,
    /// Creation metadata (progress, status).
    pub creation_meta: Option<ProjectCreationMeta>,
    /// Source type of the project.
    pub source_type: Option<ProjectSourceType>,
    /// Whether chapters are enabled.
    pub chapters_enabled: Option<bool>,
    /// Whether captions are enabled.
    pub captions_enabled: Option<bool>,
    /// Caption style configuration (complex — stored as Value).
    pub caption_style: Option<serde_json::Value>,
    /// Caption style overrides per template (complex — stored as Value).
    pub caption_style_template_overrides: Option<serde_json::Value>,
    /// Public share ID, if shared.
    pub public_share_id: Option<String>,
    /// Aspect ratio of the project.
    pub aspect_ratio: Option<AspectRatio>,
}

/// A Studio project (extended/detailed view).
///
/// Contains all fields from [`ProjectResponse`] plus chapters, pronunciation
/// dictionaries, voices, and assets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectExtendedResponse {
    /// Project ID.
    pub project_id: String,
    /// Project name.
    pub name: String,
    /// Unix timestamp of creation.
    pub create_date_unix: i64,
    /// User ID who created the project.
    pub created_by_user_id: Option<String>,
    /// Default voice ID for titles.
    pub default_title_voice_id: String,
    /// Default voice ID for paragraphs.
    pub default_paragraph_voice_id: String,
    /// Default model ID.
    pub default_model_id: String,
    /// Unix timestamp of last conversion.
    pub last_conversion_date_unix: Option<i64>,
    /// Whether the project can be downloaded.
    pub can_be_downloaded: bool,
    /// Title of the project.
    pub title: Option<String>,
    /// Author of the project.
    pub author: Option<String>,
    /// Description of the project.
    pub description: Option<String>,
    /// Genres of the project.
    pub genres: Option<Vec<String>>,
    /// Cover image URL.
    pub cover_image_url: Option<String>,
    /// Target audience.
    pub target_audience: Option<TargetAudience>,
    /// Language code.
    pub language: Option<String>,
    /// Content type (e.g. "Novel").
    pub content_type: Option<String>,
    /// Original publication date.
    pub original_publication_date: Option<String>,
    /// Whether the project contains mature content.
    pub mature_content: Option<bool>,
    /// ISBN number.
    pub isbn_number: Option<String>,
    /// Whether volume normalization is enabled.
    pub volume_normalization: bool,
    /// Current state of the project.
    pub state: ProjectState,
    /// Access level for the current user.
    pub access_level: String,
    /// Fiction/non-fiction classification.
    pub fiction: Option<FictionType>,
    /// Whether quality check is enabled.
    pub quality_check_on: bool,
    /// Whether quality check is on during bulk conversion.
    pub quality_check_on_when_bulk_convert: bool,
    /// Creation metadata (progress, status).
    pub creation_meta: Option<ProjectCreationMeta>,
    /// Source type of the project.
    pub source_type: Option<ProjectSourceType>,
    /// Whether chapters are enabled.
    pub chapters_enabled: Option<bool>,
    /// Whether captions are enabled.
    pub captions_enabled: Option<bool>,
    /// Caption style (complex — stored as Value).
    pub caption_style: Option<serde_json::Value>,
    /// Caption style overrides (complex — stored as Value).
    pub caption_style_template_overrides: Option<serde_json::Value>,
    /// Public share ID, if shared.
    pub public_share_id: Option<String>,
    /// Aspect ratio of the project.
    pub aspect_ratio: Option<AspectRatio>,
    /// Quality preset identifier.
    pub quality_preset: String,
    /// Chapters in this project.
    pub chapters: Vec<ChapterResponse>,
    /// Pronunciation dictionary versions.
    pub pronunciation_dictionary_versions: Vec<serde_json::Value>,
    /// Pronunciation dictionary locators.
    pub pronunciation_dictionary_locators: Vec<serde_json::Value>,
    /// Text normalization setting.
    pub apply_text_normalization: String,
    /// Additional experimental settings.
    #[serde(default)]
    pub experimental: serde_json::Value,
    /// Project assets (images, audio, video — complex nested array).
    pub assets: Vec<serde_json::Value>,
    /// Voices used in this project.
    pub voices: Vec<ProjectVoiceResponse>,
    /// Base voices for the project.
    pub base_voices: Option<Vec<serde_json::Value>>,
    /// Publishing read metadata.
    pub publishing_read: Option<serde_json::Value>,
}

/// Response containing a list of projects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProjectsResponse {
    /// List of projects.
    pub projects: Vec<ProjectResponse>,
}

/// Response after adding a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProjectResponse {
    /// The created project.
    pub project: ProjectResponse,
}

/// Response after editing a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditProjectResponse {
    /// The updated project.
    pub project: ProjectResponse,
}

/// Response after deleting a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteProjectResponse {
    /// Status. "ok" on success.
    pub status: String,
}

/// Response after converting a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvertProjectResponse {
    /// Status. "ok" on success.
    pub status: String,
}

// ===========================================================================
// Snapshot types (response)
// ===========================================================================

/// A project snapshot (summary view).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectSnapshotResponse {
    /// Snapshot ID.
    pub project_snapshot_id: String,
    /// Project ID.
    pub project_id: String,
    /// Unix timestamp of creation.
    pub created_at_unix: i64,
    /// Snapshot name.
    pub name: String,
    /// Deprecated audio upload metadata.
    pub audio_upload: Option<serde_json::Value>,
    /// Deprecated zip upload metadata.
    pub zip_upload: Option<serde_json::Value>,
}

/// A project snapshot (extended view with alignment data).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSnapshotExtendedResponse {
    /// Snapshot ID.
    pub project_snapshot_id: String,
    /// Project ID.
    pub project_id: String,
    /// Unix timestamp of creation.
    pub created_at_unix: i64,
    /// Snapshot name.
    pub name: String,
    /// Deprecated audio upload metadata.
    pub audio_upload: Option<serde_json::Value>,
    /// Deprecated zip upload metadata.
    pub zip_upload: Option<serde_json::Value>,
    /// Character alignment data.
    pub character_alignments: Vec<serde_json::Value>,
    /// Total audio duration in seconds.
    pub audio_duration_secs: f64,
}

/// Response containing a list of project snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectSnapshotsResponse {
    /// List of snapshots.
    pub snapshots: Vec<ProjectSnapshotResponse>,
}

// ===========================================================================
// Muted tracks
// ===========================================================================

/// Response listing muted chapter tracks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectMutedTracksResponse {
    /// Chapter IDs with muted tracks.
    pub chapter_ids: Vec<String>,
}

// ===========================================================================
// Project voice
// ===========================================================================

/// A voice configured within a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectVoiceResponse {
    /// Voice ID.
    pub voice_id: String,
    /// Display alias.
    pub alias: String,
    /// Stability setting.
    pub stability: f64,
    /// Similarity boost setting.
    pub similarity_boost: f64,
    /// Style exaggeration setting.
    pub style: f64,
    /// Whether this voice is pinned in the project.
    pub is_pinned: bool,
    /// Whether speaker boost is enabled.
    pub use_speaker_boost: bool,
    /// Volume gain.
    pub volume_gain: f64,
    /// Speed multiplier.
    pub speed: f64,
}

// ===========================================================================
// Chapter types (response)
// ===========================================================================

/// Chapter statistics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChapterStatistics {
    /// Number of unconverted characters.
    pub characters_unconverted: i64,
    /// Number of converted characters.
    pub characters_converted: i64,
    /// Number of converted paragraphs.
    pub paragraphs_converted: i64,
    /// Number of unconverted paragraphs.
    pub paragraphs_unconverted: i64,
}

/// A chapter within a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterResponse {
    /// Chapter ID.
    pub chapter_id: String,
    /// Chapter name.
    pub name: String,
    /// Unix timestamp of last conversion.
    pub last_conversion_date_unix: Option<i64>,
    /// Conversion progress (0.0 to 1.0).
    pub conversion_progress: Option<f64>,
    /// Whether the chapter can be downloaded.
    pub can_be_downloaded: bool,
    /// Current state.
    pub state: ProjectState,
    /// Whether the chapter has a video.
    pub has_video: Option<bool>,
    /// Voice IDs used by this chapter.
    pub voice_ids: Option<Vec<String>>,
    /// Chapter statistics.
    pub statistics: Option<ChapterStatistics>,
    /// Last conversion error, if any.
    pub last_conversion_error: Option<String>,
}

/// A chapter with its full content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterWithContentResponse {
    /// Chapter ID.
    pub chapter_id: String,
    /// Chapter name.
    pub name: String,
    /// Unix timestamp of last conversion.
    pub last_conversion_date_unix: Option<i64>,
    /// Conversion progress (0.0 to 1.0).
    pub conversion_progress: Option<f64>,
    /// Whether the chapter can be downloaded.
    pub can_be_downloaded: bool,
    /// Current state.
    pub state: ProjectState,
    /// Whether the chapter has a video.
    pub has_video: Option<bool>,
    /// Voice IDs used by this chapter.
    pub voice_ids: Option<Vec<String>>,
    /// Chapter statistics.
    pub statistics: Option<ChapterStatistics>,
    /// Last conversion error, if any.
    pub last_conversion_error: Option<String>,
    /// The chapter content.
    pub content: ChapterContentResponse,
}

/// Response containing a list of chapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChaptersResponse {
    /// List of chapters.
    pub chapters: Vec<ChapterResponse>,
}

/// Response after adding a chapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddChapterResponse {
    /// The created chapter (with content).
    pub chapter: ChapterWithContentResponse,
}

/// Response after editing a chapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditChapterResponse {
    /// The updated chapter (with content).
    pub chapter: ChapterWithContentResponse,
}

/// Response after deleting a chapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteChapterResponse {
    /// Status. "ok" on success.
    pub status: String,
}

/// Response after converting a chapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvertChapterResponse {
    /// Status. "ok" on success.
    pub status: String,
}

// ===========================================================================
// Chapter snapshot types (response)
// ===========================================================================

/// A chapter snapshot (summary view).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChapterSnapshotResponse {
    /// Snapshot ID.
    pub chapter_snapshot_id: String,
    /// Project ID.
    pub project_id: String,
    /// Chapter ID.
    pub chapter_id: String,
    /// Unix timestamp of creation.
    pub created_at_unix: i64,
    /// Snapshot name.
    pub name: String,
}

/// A chapter snapshot with character alignment data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChapterSnapshotExtendedResponse {
    /// Snapshot ID.
    pub chapter_snapshot_id: String,
    /// Project ID.
    pub project_id: String,
    /// Chapter ID.
    pub chapter_id: String,
    /// Unix timestamp of creation.
    pub created_at_unix: i64,
    /// Snapshot name.
    pub name: String,
    /// Character alignment data.
    pub character_alignments: Vec<serde_json::Value>,
}

/// Response containing a list of chapter snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChapterSnapshotsResponse {
    /// List of chapter snapshots.
    pub snapshots: Vec<ChapterSnapshotResponse>,
}

// ===========================================================================
// Chapter content types (response)
// ===========================================================================

/// TTS node within a chapter content block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChapterContentTtsNode {
    /// Node type (always "tts_node").
    #[serde(rename = "type")]
    pub node_type: String,
    /// Voice ID for this node.
    pub voice_id: String,
    /// Text content.
    pub text: String,
}

/// An extensible placeholder node for future content types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChapterContentExtendableNode {
    /// Node type (always "_other").
    #[serde(rename = "type")]
    pub node_type: String,
}

/// A content block within a chapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChapterContentBlockResponse {
    /// Block ID.
    pub block_id: String,
    /// Nodes in this block (TTS nodes or extensible nodes).
    /// Uses `serde_json::Value` because nodes are a polymorphic union.
    pub nodes: Vec<serde_json::Value>,
}

/// Chapter content containing blocks of TTS nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChapterContentResponse {
    /// Content blocks.
    pub blocks: Vec<ChapterContentBlockResponse>,
}

// ===========================================================================
// Chapter content input types (Serialize only)
// ===========================================================================

/// TTS node for chapter content input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChapterContentTtsNodeInput {
    /// Node type. Must be "tts_node".
    #[serde(rename = "type")]
    pub node_type: String,
    /// Text content.
    pub text: String,
    /// Voice ID for this node.
    pub voice_id: String,
}

/// A content block for chapter input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChapterContentBlockInput {
    /// Block sub-type (p, h1, h2, h3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<BlockSubType>,
    /// TTS nodes within this block.
    pub nodes: Vec<ChapterContentTtsNodeInput>,
    /// Existing block ID (for updates).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

/// Chapter content input for creating or updating chapter content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChapterContentInput {
    /// Content blocks.
    pub blocks: Vec<ChapterContentBlockInput>,
}

// ===========================================================================
// Podcast types
// ===========================================================================

/// Response after creating a podcast.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PodcastProjectResponse {
    /// The project created for this podcast.
    pub project: ProjectResponse,
}

/// Conversation mode voice data for a podcast.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PodcastConversationModeData {
    /// Voice ID for the host.
    pub host_voice_id: String,
    /// Voice ID for the guest.
    pub guest_voice_id: String,
}

/// Conversation podcast mode configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PodcastConversationMode {
    /// Must be "conversation".
    #[serde(rename = "type")]
    pub mode_type: String,
    /// Voice configuration for the conversation.
    pub conversation: PodcastConversationModeData,
}

/// Bulletin mode voice data for a podcast.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PodcastBulletinModeData {
    /// Voice ID for the host.
    pub host_voice_id: String,
}

/// Bulletin podcast mode configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PodcastBulletinMode {
    /// Must be "bulletin".
    #[serde(rename = "type")]
    pub mode_type: String,
    /// Voice configuration for the bulletin.
    pub bulletin: PodcastBulletinModeData,
}

/// Text source for a podcast.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PodcastTextSource {
    /// Must be "text".
    #[serde(rename = "type")]
    pub source_type: String,
    /// The text content to generate a podcast from.
    pub text: String,
}

/// URL source for a podcast.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PodcastUrlSource {
    /// Must be "url".
    #[serde(rename = "type")]
    pub source_type: String,
    /// The URL to generate a podcast from.
    pub url: String,
}

// ===========================================================================
// Project asset types (response)
// ===========================================================================

/// A voice response within a project context.
///
/// Contains voice settings specific to this project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectVideoThumbnailSheet {
    /// Index of the first thumbnail on this sheet.
    pub start_thumbnail_index: i64,
    /// Number of thumbnails on this sheet.
    pub thumbnail_count: i64,
    /// Signed URL for the thumbnail sheet image.
    pub signed_cloud_url: String,
}

/// Chapter metadata used during reading/import.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadMetadataChapter {
    /// Chapter name.
    pub chapter_name: String,
    /// Word count.
    pub word_count: i64,
    /// Character count.
    pub char_count: i64,
    /// Starting character offset in the full text.
    pub starting_char_offset: i64,
    /// Whether HTML has been parsed.
    #[serde(default)]
    pub has_parsed_html: bool,
    /// Whether a summary has been generated.
    #[serde(default)]
    pub has_summary: bool,
    /// Duration in seconds (if audio was imported).
    pub duration_seconds: Option<f64>,
    /// File number.
    pub file_number: Option<String>,
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    // -- ProjectCreationMeta ------------------------------------------------

    #[test]
    fn project_creation_meta_deserialize() {
        let json = r#"{"creation_progress":0.5,"status":"pending","type":"blank"}"#;
        let meta: ProjectCreationMeta = serde_json::from_str(json).unwrap();
        assert!((meta.creation_progress - 0.5).abs() < f64::EPSILON);
        assert_eq!(meta.status, ProjectCreationStatus::Pending);
        assert_eq!(meta.creation_type, ProjectCreationType::Blank);
    }

    // -- ProjectResponse ----------------------------------------------------

    #[test]
    fn project_response_deserialize() {
        let json = r#"{
            "project_id": "proj_001",
            "name": "My Project",
            "create_date_unix": 1714204800,
            "created_by_user_id": "user_abc",
            "default_title_voice_id": "voice_t",
            "default_paragraph_voice_id": "voice_p",
            "default_model_id": "eleven_multilingual_v2",
            "last_conversion_date_unix": 1714204800,
            "can_be_downloaded": true,
            "title": "My Project",
            "author": "John Doe",
            "description": "A description",
            "genres": ["Novel"],
            "cover_image_url": null,
            "target_audience": "young adult",
            "language": "en",
            "content_type": "Novel",
            "original_publication_date": null,
            "mature_content": false,
            "isbn_number": null,
            "volume_normalization": true,
            "state": "default",
            "access_level": "owner",
            "fiction": "fiction",
            "quality_check_on": false,
            "quality_check_on_when_bulk_convert": false,
            "creation_meta": null,
            "source_type": null,
            "chapters_enabled": true,
            "captions_enabled": false,
            "caption_style": null,
            "caption_style_template_overrides": null,
            "public_share_id": null,
            "aspect_ratio": null
        }"#;
        let proj: ProjectResponse = serde_json::from_str(json).unwrap();
        assert_eq!(proj.project_id, "proj_001");
        assert_eq!(proj.name, "My Project");
        assert!(proj.can_be_downloaded);
        assert_eq!(proj.state, ProjectState::Default);
        assert_eq!(proj.target_audience, Some(TargetAudience::YoungAdult));
        assert_eq!(proj.fiction, Some(FictionType::Fiction));
    }

    #[test]
    fn project_response_minimal() {
        let json = r#"{
            "project_id": "proj_002",
            "name": "Minimal",
            "create_date_unix": 0,
            "created_by_user_id": null,
            "default_title_voice_id": "v1",
            "default_paragraph_voice_id": "v2",
            "default_model_id": "m1",
            "can_be_downloaded": false,
            "volume_normalization": false,
            "state": "converting",
            "access_level": "viewer",
            "quality_check_on": true,
            "quality_check_on_when_bulk_convert": true
        }"#;
        let proj: ProjectResponse = serde_json::from_str(json).unwrap();
        assert_eq!(proj.project_id, "proj_002");
        assert_eq!(proj.state, ProjectState::Converting);
        assert!(proj.title.is_none());
        assert!(proj.genres.is_none());
    }

    // -- GetProjectsResponse ------------------------------------------------

    #[test]
    fn get_projects_response_deserialize() {
        let json = r#"{
            "projects": [{
                "project_id": "p1",
                "name": "P1",
                "create_date_unix": 0,
                "created_by_user_id": null,
                "default_title_voice_id": "v1",
                "default_paragraph_voice_id": "v2",
                "default_model_id": "m1",
                "can_be_downloaded": true,
                "volume_normalization": true,
                "state": "default",
                "access_level": "owner",
                "quality_check_on": false,
                "quality_check_on_when_bulk_convert": false
            }]
        }"#;
        let resp: GetProjectsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.projects.len(), 1);
        assert_eq!(resp.projects[0].project_id, "p1");
    }

    // -- DeleteProjectResponse / ConvertProjectResponse ---------------------

    #[test]
    fn delete_project_response_deserialize() {
        let json = r#"{"status":"ok"}"#;
        let resp: DeleteProjectResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn convert_project_response_deserialize() {
        let json = r#"{"status":"ok"}"#;
        let resp: ConvertProjectResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    // -- ProjectSnapshotResponse --------------------------------------------

    #[test]
    fn project_snapshot_response_deserialize() {
        let json = r#"{
            "project_snapshot_id": "snap_1",
            "project_id": "proj_1",
            "created_at_unix": 1714204800,
            "name": "Snapshot 1"
        }"#;
        let snap: ProjectSnapshotResponse = serde_json::from_str(json).unwrap();
        assert_eq!(snap.project_snapshot_id, "snap_1");
        assert_eq!(snap.created_at_unix, 1714204800);
    }

    // -- ProjectSnapshotsResponse -------------------------------------------

    #[test]
    fn project_snapshots_response_deserialize() {
        let json = r#"{
            "snapshots": [{
                "project_snapshot_id": "snap_1",
                "project_id": "proj_1",
                "created_at_unix": 0,
                "name": "S1"
            }]
        }"#;
        let resp: ProjectSnapshotsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.snapshots.len(), 1);
    }

    // -- ProjectMutedTracksResponse -----------------------------------------

    #[test]
    fn project_muted_tracks_response_deserialize() {
        let json = r#"{"chapter_ids":["ch1","ch2"]}"#;
        let resp: ProjectMutedTracksResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.chapter_ids, vec!["ch1", "ch2"]);
    }

    // -- ProjectVoiceResponse -----------------------------------------------

    #[test]
    fn project_voice_response_deserialize() {
        let json = r#"{
            "voice_id": "v1",
            "alias": "Narrator",
            "stability": 0.75,
            "similarity_boost": 0.8,
            "style": 0.0,
            "is_pinned": true,
            "use_speaker_boost": false,
            "volume_gain": 1.0,
            "speed": 1.0
        }"#;
        let voice: ProjectVoiceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(voice.voice_id, "v1");
        assert_eq!(voice.alias, "Narrator");
        assert!(voice.is_pinned);
    }

    // -- ChapterStatistics --------------------------------------------------

    #[test]
    fn chapter_statistics_deserialize() {
        let json = r#"{
            "characters_unconverted": 1000,
            "characters_converted": 500,
            "paragraphs_converted": 20,
            "paragraphs_unconverted": 10
        }"#;
        let stats: ChapterStatistics = serde_json::from_str(json).unwrap();
        assert_eq!(stats.characters_unconverted, 1000);
        assert_eq!(stats.paragraphs_converted, 20);
    }

    // -- ChapterResponse ----------------------------------------------------

    #[test]
    fn chapter_response_deserialize() {
        let json = r#"{
            "chapter_id": "ch_1",
            "name": "Chapter 1",
            "last_conversion_date_unix": 1714204800,
            "conversion_progress": 0.5,
            "can_be_downloaded": true,
            "state": "default",
            "has_video": false,
            "voice_ids": ["v1", "v2"],
            "statistics": {
                "characters_unconverted": 100,
                "characters_converted": 200,
                "paragraphs_converted": 5,
                "paragraphs_unconverted": 3
            },
            "last_conversion_error": null
        }"#;
        let ch: ChapterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(ch.chapter_id, "ch_1");
        assert!(ch.can_be_downloaded);
        assert_eq!(ch.voice_ids.as_ref().unwrap().len(), 2);
        assert!(ch.statistics.is_some());
    }

    #[test]
    fn chapter_response_minimal() {
        let json = r#"{
            "chapter_id": "ch_2",
            "name": "Ch2",
            "can_be_downloaded": false,
            "state": "converting"
        }"#;
        let ch: ChapterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(ch.state, ProjectState::Converting);
        assert!(ch.voice_ids.is_none());
    }

    // -- ChapterWithContentResponse -----------------------------------------

    #[test]
    fn chapter_with_content_deserialize() {
        let json = r#"{
            "chapter_id": "ch_1",
            "name": "Ch1",
            "can_be_downloaded": true,
            "state": "default",
            "content": {
                "blocks": [{
                    "block_id": "b1",
                    "nodes": [{"type": "tts_node", "voice_id": "v1", "text": "Hello world"}]
                }]
            }
        }"#;
        let ch: ChapterWithContentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(ch.chapter_id, "ch_1");
        assert_eq!(ch.content.blocks.len(), 1);
        assert_eq!(ch.content.blocks[0].block_id, "b1");
    }

    // -- GetChaptersResponse ------------------------------------------------

    #[test]
    fn get_chapters_response_deserialize() {
        let json = r#"{
            "chapters": [{
                "chapter_id": "ch_1",
                "name": "Ch1",
                "can_be_downloaded": true,
                "state": "default"
            }]
        }"#;
        let resp: GetChaptersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.chapters.len(), 1);
    }

    // -- DeleteChapterResponse / ConvertChapterResponse ---------------------

    #[test]
    fn delete_chapter_response_deserialize() {
        let json = r#"{"status":"ok"}"#;
        let resp: DeleteChapterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn convert_chapter_response_deserialize() {
        let json = r#"{"status":"ok"}"#;
        let resp: ConvertChapterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    // -- ChapterSnapshotResponse --------------------------------------------

    #[test]
    fn chapter_snapshot_response_deserialize() {
        let json = r#"{
            "chapter_snapshot_id": "csnap_1",
            "project_id": "proj_1",
            "chapter_id": "ch_1",
            "created_at_unix": 1714204800,
            "name": "Chapter Snapshot"
        }"#;
        let snap: ChapterSnapshotResponse = serde_json::from_str(json).unwrap();
        assert_eq!(snap.chapter_snapshot_id, "csnap_1");
        assert_eq!(snap.chapter_id, "ch_1");
    }

    // -- ChapterSnapshotsResponse -------------------------------------------

    #[test]
    fn chapter_snapshots_response_deserialize() {
        let json = r#"{
            "snapshots": [{
                "chapter_snapshot_id": "cs1",
                "project_id": "p1",
                "chapter_id": "c1",
                "created_at_unix": 0,
                "name": "S"
            }]
        }"#;
        let resp: ChapterSnapshotsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.snapshots.len(), 1);
    }

    // -- ChapterContentResponse ---------------------------------------------

    #[test]
    fn chapter_content_response_deserialize() {
        let json = r#"{
            "blocks": [{
                "block_id": "b1",
                "nodes": [
                    {"type": "tts_node", "voice_id": "v1", "text": "Hello"},
                    {"type": "_other"}
                ]
            }]
        }"#;
        let content: ChapterContentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(content.blocks.len(), 1);
        assert_eq!(content.blocks[0].nodes.len(), 2);
    }

    // -- ChapterContentInput (request) --------------------------------------

    #[test]
    fn chapter_content_input_serialize() {
        let input = ChapterContentInput {
            blocks: vec![ChapterContentBlockInput {
                sub_type: Some(BlockSubType::P),
                nodes: vec![ChapterContentTtsNodeInput {
                    node_type: "tts_node".into(),
                    text: "Hello world".into(),
                    voice_id: "v1".into(),
                }],
                block_id: None,
            }],
        };
        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("\"sub_type\":\"p\""));
        assert!(json.contains("\"type\":\"tts_node\""));
        assert!(json.contains("\"text\":\"Hello world\""));
        assert!(!json.contains("block_id"));
    }

    // -- Podcast types ------------------------------------------------------

    #[test]
    fn podcast_conversation_mode_serialize() {
        let mode = PodcastConversationMode {
            mode_type: "conversation".into(),
            conversation: PodcastConversationModeData {
                host_voice_id: "host_v".into(),
                guest_voice_id: "guest_v".into(),
            },
        };
        let json = serde_json::to_string(&mode).unwrap();
        assert!(json.contains("\"type\":\"conversation\""));
        assert!(json.contains("\"host_voice_id\":\"host_v\""));
        assert!(json.contains("\"guest_voice_id\":\"guest_v\""));
    }

    #[test]
    fn podcast_bulletin_mode_serialize() {
        let mode = PodcastBulletinMode {
            mode_type: "bulletin".into(),
            bulletin: PodcastBulletinModeData { host_voice_id: "host_v".into() },
        };
        let json = serde_json::to_string(&mode).unwrap();
        assert!(json.contains("\"type\":\"bulletin\""));
    }

    #[test]
    fn podcast_text_source_serialize() {
        let src = PodcastTextSource { source_type: "text".into(), text: "Hello podcast".into() };
        let json = serde_json::to_string(&src).unwrap();
        assert!(json.contains("\"type\":\"text\""));
        assert!(json.contains("\"text\":\"Hello podcast\""));
    }

    #[test]
    fn podcast_url_source_serialize() {
        let src = PodcastUrlSource { source_type: "url".into(), url: "https://example.com".into() };
        let json = serde_json::to_string(&src).unwrap();
        assert!(json.contains("\"type\":\"url\""));
        assert!(json.contains("\"url\":\"https://example.com\""));
    }

    // -- PodcastProjectResponse ---------------------------------------------

    #[test]
    fn podcast_project_response_deserialize() {
        let json = r#"{
            "project": {
                "project_id": "pod_1",
                "name": "My Podcast",
                "create_date_unix": 0,
                "created_by_user_id": null,
                "default_title_voice_id": "v1",
                "default_paragraph_voice_id": "v2",
                "default_model_id": "m1",
                "can_be_downloaded": true,
                "volume_normalization": true,
                "state": "default",
                "access_level": "owner",
                "quality_check_on": false,
                "quality_check_on_when_bulk_convert": false
            }
        }"#;
        let resp: PodcastProjectResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.project.project_id, "pod_1");
    }

    // -- ReadMetadataChapter ------------------------------------------------

    #[test]
    fn read_metadata_chapter_deserialize() {
        let json = r#"{
            "chapter_name": "Chapter 1",
            "word_count": 5000,
            "char_count": 25000,
            "starting_char_offset": 0,
            "has_parsed_html": true,
            "has_summary": false,
            "duration_seconds": 120.5,
            "file_number": "01"
        }"#;
        let ch: ReadMetadataChapter = serde_json::from_str(json).unwrap();
        assert_eq!(ch.chapter_name, "Chapter 1");
        assert_eq!(ch.word_count, 5000);
        assert!(ch.has_parsed_html);
        assert!(!ch.has_summary);
        assert_eq!(ch.duration_seconds, Some(120.5));
    }

    #[test]
    fn read_metadata_chapter_minimal() {
        let json = r#"{
            "chapter_name": "Ch",
            "word_count": 0,
            "char_count": 0,
            "starting_char_offset": 0
        }"#;
        let ch: ReadMetadataChapter = serde_json::from_str(json).unwrap();
        assert!(!ch.has_parsed_html);
        assert!(ch.duration_seconds.is_none());
    }

    // -- ProjectSnapshotExtendedResponse ------------------------------------

    #[test]
    fn project_snapshot_extended_deserialize() {
        let json = r#"{
            "project_snapshot_id": "snap_ext",
            "project_id": "proj_1",
            "created_at_unix": 1714204800,
            "name": "Extended Snapshot",
            "character_alignments": [],
            "audio_duration_secs": 123.45
        }"#;
        let snap: ProjectSnapshotExtendedResponse = serde_json::from_str(json).unwrap();
        assert_eq!(snap.project_snapshot_id, "snap_ext");
        assert!((snap.audio_duration_secs - 123.45).abs() < f64::EPSILON);
    }

    // -- AspectRatio --------------------------------------------------------

    #[test]
    fn aspect_ratio_serde_round_trip() {
        let ratios = vec![
            (AspectRatio::Landscape, "\"16:9\""),
            (AspectRatio::Portrait, "\"9:16\""),
            (AspectRatio::Social, "\"4:5\""),
            (AspectRatio::Square, "\"1:1\""),
        ];
        for (ratio, expected) in ratios {
            let json = serde_json::to_string(&ratio).unwrap();
            assert_eq!(json, expected);
            let back: AspectRatio = serde_json::from_str(&json).unwrap();
            assert_eq!(ratio, back);
        }
    }
}
