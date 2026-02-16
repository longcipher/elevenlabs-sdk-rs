//! Studio service providing access to project, chapter, snapshot, podcast,
//! and pronunciation dictionary management endpoints.
//!
//! This module wraps all studio and pronunciation-dictionary endpoints
//! exposed by the ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`get_projects`](StudioService::get_projects) | `GET /v1/studio/projects` | List projects |
//! | [`get_project`](StudioService::get_project) | `GET /v1/studio/projects/{id}` | Get project by ID |
//! | [`add_project`](StudioService::add_project) | `POST /v1/studio/projects` | Create a project (multipart) |
//! | [`edit_project`](StudioService::edit_project) | `POST /v1/studio/projects/{id}` | Update a project |
//! | [`delete_project`](StudioService::delete_project) | `DELETE /v1/studio/projects/{id}` | Delete a project |
//! | [`convert_project`](StudioService::convert_project) | `POST /v1/studio/projects/{id}/convert` | Convert a project |
//! | [`edit_project_content`](StudioService::edit_project_content) | `POST /v1/studio/projects/{id}/content` | Update project content (multipart) |
//! | [`update_pronunciation_dictionaries`](StudioService::update_pronunciation_dictionaries) | `POST /v1/studio/projects/{id}/pronunciation-dictionaries` | Attach dictionaries |
//! | [`get_project_snapshots`](StudioService::get_project_snapshots) | `GET /v1/studio/projects/{id}/snapshots` | List project snapshots |
//! | [`get_project_snapshot`](StudioService::get_project_snapshot) | `GET /v1/studio/projects/{id}/snapshots/{snap_id}` | Get project snapshot |
//! | [`stream_project_snapshot_audio`](StudioService::stream_project_snapshot_audio) | `POST /v1/studio/projects/{id}/snapshots/{snap_id}/stream` | Stream snapshot audio |
//! | [`stream_project_snapshot_archive`](StudioService::stream_project_snapshot_archive) | `POST /v1/studio/projects/{id}/snapshots/{snap_id}/archive` | Stream snapshot archive |
//! | [`get_project_muted_tracks`](StudioService::get_project_muted_tracks) | `GET /v1/studio/projects/{id}/muted-tracks` | Get muted tracks |
//! | [`get_chapters`](StudioService::get_chapters) | `GET /v1/studio/projects/{id}/chapters` | List chapters |
//! | [`get_chapter`](StudioService::get_chapter) | `GET /v1/studio/projects/{id}/chapters/{ch_id}` | Get chapter |
//! | [`add_chapter`](StudioService::add_chapter) | `POST /v1/studio/projects/{id}/chapters` | Create a chapter |
//! | [`edit_chapter`](StudioService::edit_chapter) | `POST /v1/studio/projects/{id}/chapters/{ch_id}` | Update a chapter |
//! | [`delete_chapter`](StudioService::delete_chapter) | `DELETE /v1/studio/projects/{id}/chapters/{ch_id}` | Delete a chapter |
//! | [`convert_chapter`](StudioService::convert_chapter) | `POST /v1/studio/projects/{id}/chapters/{ch_id}/convert` | Convert a chapter |
//! | [`get_chapter_snapshots`](StudioService::get_chapter_snapshots) | `GET /v1/studio/projects/{id}/chapters/{ch_id}/snapshots` | List chapter snapshots |
//! | [`get_chapter_snapshot`](StudioService::get_chapter_snapshot) | `GET /v1/studio/projects/{id}/chapters/{ch_id}/snapshots/{snap_id}` | Get chapter snapshot |
//! | [`stream_chapter_snapshot_audio`](StudioService::stream_chapter_snapshot_audio) | `POST /v1/studio/projects/{id}/chapters/{ch_id}/snapshots/{snap_id}/stream` | Stream chapter snapshot audio |
//! | [`create_podcast`](StudioService::create_podcast) | `POST /v1/studio/podcasts` | Create a podcast |
//! | [`get_pronunciation_dictionaries`](StudioService::get_pronunciation_dictionaries) | `GET /v1/pronunciation-dictionaries` | List dictionaries |
//! | [`get_pronunciation_dictionary`](StudioService::get_pronunciation_dictionary) | `GET /v1/pronunciation-dictionaries/{id}` | Get dictionary |
//! | [`download_pronunciation_dictionary_version`](StudioService::download_pronunciation_dictionary_version) | `GET /v1/pronunciation-dictionaries/{id}/{ver}/download` | Download version PLS |
//! | [`create_pronunciation_dictionary_from_file`](StudioService::create_pronunciation_dictionary_from_file) | `POST /v1/pronunciation-dictionaries/add-from-file` | Create from file (multipart) |
//! | [`create_pronunciation_dictionary_from_rules`](StudioService::create_pronunciation_dictionary_from_rules) | `POST /v1/pronunciation-dictionaries/add-from-rules` | Create from rules |
//! | [`add_pronunciation_rules`](StudioService::add_pronunciation_rules) | `POST /v1/pronunciation-dictionaries/{id}/add-rules` | Add rules |
//! | [`remove_pronunciation_rules`](StudioService::remove_pronunciation_rules) | `POST /v1/pronunciation-dictionaries/{id}/remove-rules` | Remove rules |
//! | [`update_pronunciation_dictionary`](StudioService::update_pronunciation_dictionary) | `PATCH /v1/pronunciation-dictionaries/{id}` | Update dictionary |
//!
//! # Example
//!
//! ```no_run
//! use elevenlabs_sdk::{ClientConfig, ElevenLabsClient};
//!
//! # async fn example() -> elevenlabs_sdk::Result<()> {
//! let config = ClientConfig::builder("your-api-key").build();
//! let client = ElevenLabsClient::new(config)?;
//!
//! let projects = client.studio().get_projects().await?;
//! println!("Found {} projects", projects.projects.len());
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;
use futures_core::Stream;
use serde::Serialize;

use crate::types::{
    AddChapterResponse,
    AddProjectResponse,
    // Pronunciation
    AddPronunciationDictionaryResponse,
    AddPronunciationRulesRequest,
    ChapterSnapshotExtendedResponse,
    ChapterSnapshotsResponse,
    ChapterWithContentResponse,
    ConvertChapterResponse,
    ConvertProjectResponse,
    DeleteChapterResponse,
    DeleteProjectResponse,
    EditChapterResponse,
    EditProjectResponse,
    GetChaptersResponse,
    GetProjectsResponse,
    GetPronunciationDictionariesResponse,
    PodcastProjectResponse,
    ProjectExtendedResponse,
    ProjectMutedTracksResponse,
    ProjectSnapshotExtendedResponse,
    ProjectSnapshotsResponse,
    PronunciationDictionaryLocatorRequest,
    PronunciationDictionaryMetadata,
    PronunciationDictionaryRulesResponse,
    RemovePronunciationRulesRequest,
    UpdatePronunciationDictionaryRequest,
};
use crate::{client::ElevenLabsClient, error::Result};

/// Studio service providing typed access to project, chapter, snapshot,
/// podcast, and pronunciation dictionary endpoints.
///
/// Obtained via [`ElevenLabsClient::studio`].
#[derive(Debug)]
pub struct StudioService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> StudioService<'a> {
    /// Creates a new `StudioService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    // =======================================================================
    // Project CRUD
    // =======================================================================

    /// Lists all projects.
    ///
    /// Calls `GET /v1/studio/projects`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_projects(&self) -> Result<GetProjectsResponse> {
        self.client.get("/v1/studio/projects").await
    }

    /// Gets a project by ID (extended view with chapters, voices, etc.).
    ///
    /// Calls `GET /v1/studio/projects/{project_id}`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_project(&self, project_id: &str) -> Result<ProjectExtendedResponse> {
        let path = format!("/v1/studio/projects/{project_id}");
        self.client.get(&path).await
    }

    /// Creates a new project.
    ///
    /// Calls `POST /v1/studio/projects` with `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `request` — Project configuration fields.
    /// * `from_document` — Optional document file as `(filename, content_type, data)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn add_project(
        &self,
        request: &AddProjectRequest,
        from_document: Option<(&str, &str, &[u8])>,
    ) -> Result<AddProjectResponse> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_add_project_multipart(&boundary, request, from_document);
        let content_type = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart("/v1/studio/projects", body, &content_type).await
    }

    /// Updates a project.
    ///
    /// Calls `POST /v1/studio/projects/{project_id}` with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `request` — Updated project fields.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn edit_project(
        &self,
        project_id: &str,
        request: &EditProjectRequest,
    ) -> Result<EditProjectResponse> {
        let path = format!("/v1/studio/projects/{project_id}");
        self.client.post(&path, request).await
    }

    /// Deletes a project.
    ///
    /// Calls `DELETE /v1/studio/projects/{project_id}`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete_project(&self, project_id: &str) -> Result<DeleteProjectResponse> {
        let path = format!("/v1/studio/projects/{project_id}");
        self.client.delete_json(&path).await
    }

    /// Converts a project (starts TTS rendering).
    ///
    /// Calls `POST /v1/studio/projects/{project_id}/convert`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn convert_project(&self, project_id: &str) -> Result<ConvertProjectResponse> {
        let path = format!("/v1/studio/projects/{project_id}/convert");
        self.client.post(&path, &serde_json::Value::Null).await
    }

    /// Updates project content from a URL, document, or JSON.
    ///
    /// Calls `POST /v1/studio/projects/{project_id}/content` with
    /// `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `request` — Content update fields.
    /// * `from_document` — Optional document file as `(filename, content_type, data)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn edit_project_content(
        &self,
        project_id: &str,
        request: &EditProjectContentRequest,
        from_document: Option<(&str, &str, &[u8])>,
    ) -> Result<serde_json::Value> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_edit_content_multipart(&boundary, request, from_document);
        let content_type = format!("multipart/form-data; boundary={boundary}");
        let path = format!("/v1/studio/projects/{project_id}/content");
        self.client.post_multipart(&path, body, &content_type).await
    }

    /// Attaches pronunciation dictionaries to a project.
    ///
    /// Calls `POST /v1/studio/projects/{project_id}/pronunciation-dictionaries`
    /// with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `request` — Dictionary locators and settings.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn update_pronunciation_dictionaries(
        &self,
        project_id: &str,
        request: &UpdateProjectPronunciationDictionariesRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/studio/projects/{project_id}/pronunciation-dictionaries");
        self.client.post(&path, request).await
    }

    // =======================================================================
    // Project snapshots
    // =======================================================================

    /// Lists snapshots for a project.
    ///
    /// Calls `GET /v1/studio/projects/{project_id}/snapshots`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_project_snapshots(
        &self,
        project_id: &str,
    ) -> Result<ProjectSnapshotsResponse> {
        let path = format!("/v1/studio/projects/{project_id}/snapshots");
        self.client.get(&path).await
    }

    /// Gets a specific project snapshot with alignment data.
    ///
    /// Calls `GET /v1/studio/projects/{project_id}/snapshots/{project_snapshot_id}`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `snapshot_id` — The snapshot ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_project_snapshot(
        &self,
        project_id: &str,
        snapshot_id: &str,
    ) -> Result<ProjectSnapshotExtendedResponse> {
        let path = format!("/v1/studio/projects/{project_id}/snapshots/{snapshot_id}");
        self.client.get(&path).await
    }

    /// Streams audio for a project snapshot.
    ///
    /// Calls `POST /v1/studio/projects/{project_id}/snapshots/{project_snapshot_id}/stream`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `snapshot_id` — The snapshot ID.
    /// * `convert_to_mpeg` — Whether to convert the audio to MPEG format.
    ///
    /// # Errors
    ///
    /// Returns an error if the initial API request fails. Individual stream
    /// items may also carry transport errors.
    pub async fn stream_project_snapshot_audio(
        &self,
        project_id: &str,
        snapshot_id: &str,
        convert_to_mpeg: Option<bool>,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>>> {
        let path = format!("/v1/studio/projects/{project_id}/snapshots/{snapshot_id}/stream");
        let body = SnapshotStreamRequest { convert_to_mpeg };
        self.client.post_stream(&path, &body).await
    }

    /// Streams an archive (zip) for a project snapshot.
    ///
    /// Calls `POST /v1/studio/projects/{project_id}/snapshots/{project_snapshot_id}/archive`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `snapshot_id` — The snapshot ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the initial API request fails. Individual stream
    /// items may also carry transport errors.
    pub async fn stream_project_snapshot_archive(
        &self,
        project_id: &str,
        snapshot_id: &str,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>>> {
        let path = format!("/v1/studio/projects/{project_id}/snapshots/{snapshot_id}/archive");
        self.client.post_stream(&path, &serde_json::Value::Null).await
    }

    // =======================================================================
    // Muted tracks
    // =======================================================================

    /// Gets the muted chapter tracks for a project.
    ///
    /// Calls `GET /v1/studio/projects/{project_id}/muted-tracks`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_project_muted_tracks(
        &self,
        project_id: &str,
    ) -> Result<ProjectMutedTracksResponse> {
        let path = format!("/v1/studio/projects/{project_id}/muted-tracks");
        self.client.get(&path).await
    }

    // =======================================================================
    // Chapter CRUD
    // =======================================================================

    /// Lists chapters in a project.
    ///
    /// Calls `GET /v1/studio/projects/{project_id}/chapters`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_chapters(&self, project_id: &str) -> Result<GetChaptersResponse> {
        let path = format!("/v1/studio/projects/{project_id}/chapters");
        self.client.get(&path).await
    }

    /// Gets a chapter by ID (with full content).
    ///
    /// Calls `GET /v1/studio/projects/{project_id}/chapters/{chapter_id}`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `chapter_id` — The chapter ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_chapter(
        &self,
        project_id: &str,
        chapter_id: &str,
    ) -> Result<ChapterWithContentResponse> {
        let path = format!("/v1/studio/projects/{project_id}/chapters/{chapter_id}");
        self.client.get(&path).await
    }

    /// Creates a new chapter in a project.
    ///
    /// Calls `POST /v1/studio/projects/{project_id}/chapters` with a JSON
    /// body.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `request` — Chapter creation fields.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn add_chapter(
        &self,
        project_id: &str,
        request: &AddChapterRequest,
    ) -> Result<AddChapterResponse> {
        let path = format!("/v1/studio/projects/{project_id}/chapters");
        self.client.post(&path, request).await
    }

    /// Updates a chapter.
    ///
    /// Calls `POST /v1/studio/projects/{project_id}/chapters/{chapter_id}`
    /// with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `chapter_id` — The chapter ID.
    /// * `request` — Updated chapter fields.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn edit_chapter(
        &self,
        project_id: &str,
        chapter_id: &str,
        request: &EditChapterRequest,
    ) -> Result<EditChapterResponse> {
        let path = format!("/v1/studio/projects/{project_id}/chapters/{chapter_id}");
        self.client.post(&path, request).await
    }

    /// Deletes a chapter.
    ///
    /// Calls `DELETE /v1/studio/projects/{project_id}/chapters/{chapter_id}`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `chapter_id` — The chapter ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete_chapter(
        &self,
        project_id: &str,
        chapter_id: &str,
    ) -> Result<DeleteChapterResponse> {
        let path = format!("/v1/studio/projects/{project_id}/chapters/{chapter_id}");
        self.client.delete_json(&path).await
    }

    /// Converts a chapter (starts TTS rendering).
    ///
    /// Calls `POST /v1/studio/projects/{project_id}/chapters/{chapter_id}/convert`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `chapter_id` — The chapter ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn convert_chapter(
        &self,
        project_id: &str,
        chapter_id: &str,
    ) -> Result<ConvertChapterResponse> {
        let path = format!("/v1/studio/projects/{project_id}/chapters/{chapter_id}/convert");
        self.client.post(&path, &serde_json::Value::Null).await
    }

    // =======================================================================
    // Chapter snapshots
    // =======================================================================

    /// Lists snapshots for a chapter.
    ///
    /// Calls `GET /v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `chapter_id` — The chapter ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_chapter_snapshots(
        &self,
        project_id: &str,
        chapter_id: &str,
    ) -> Result<ChapterSnapshotsResponse> {
        let path = format!("/v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots");
        self.client.get(&path).await
    }

    /// Gets a specific chapter snapshot with alignment data.
    ///
    /// Calls `GET
    /// /v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots/{chapter_snapshot_id}`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `chapter_id` — The chapter ID.
    /// * `snapshot_id` — The chapter snapshot ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_chapter_snapshot(
        &self,
        project_id: &str,
        chapter_id: &str,
        snapshot_id: &str,
    ) -> Result<ChapterSnapshotExtendedResponse> {
        let path = format!(
            "/v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots/{snapshot_id}"
        );
        self.client.get(&path).await
    }

    /// Streams audio for a chapter snapshot.
    ///
    /// Calls `POST
    /// /v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots/{chapter_snapshot_id}/
    /// stream`.
    ///
    /// # Arguments
    ///
    /// * `project_id` — The project ID.
    /// * `chapter_id` — The chapter ID.
    /// * `snapshot_id` — The chapter snapshot ID.
    /// * `convert_to_mpeg` — Whether to convert the audio to MPEG format.
    ///
    /// # Errors
    ///
    /// Returns an error if the initial API request fails. Individual stream
    /// items may also carry transport errors.
    pub async fn stream_chapter_snapshot_audio(
        &self,
        project_id: &str,
        chapter_id: &str,
        snapshot_id: &str,
        convert_to_mpeg: Option<bool>,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>>> {
        let path = format!(
            "/v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots/{snapshot_id}/stream"
        );
        let body = SnapshotStreamRequest { convert_to_mpeg };
        self.client.post_stream(&path, &body).await
    }

    // =======================================================================
    // Podcasts
    // =======================================================================

    /// Creates a podcast project.
    ///
    /// Calls `POST /v1/studio/podcasts` with a JSON body.
    ///
    /// The `mode` and `source` fields are polymorphic — use
    /// [`PodcastConversationMode`](crate::types::PodcastConversationMode) /
    /// [`PodcastBulletinMode`](crate::types::PodcastBulletinMode) and
    /// [`PodcastTextSource`](crate::types::PodcastTextSource) /
    /// [`PodcastUrlSource`](crate::types::PodcastUrlSource).
    ///
    /// # Arguments
    ///
    /// * `request` — Podcast creation fields.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn create_podcast(
        &self,
        request: &CreatePodcastRequest,
    ) -> Result<PodcastProjectResponse> {
        self.client.post("/v1/studio/podcasts", request).await
    }

    // =======================================================================
    // Pronunciation dictionaries
    // =======================================================================

    /// Lists pronunciation dictionaries with optional pagination.
    ///
    /// Calls `GET /v1/pronunciation-dictionaries`.
    ///
    /// # Arguments
    ///
    /// * `cursor` — Pagination cursor from a previous response.
    /// * `page_size` — Maximum number of results per page.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_pronunciation_dictionaries(
        &self,
        cursor: Option<&str>,
        page_size: Option<u32>,
    ) -> Result<GetPronunciationDictionariesResponse> {
        let mut path = "/v1/pronunciation-dictionaries".to_owned();
        let mut params = Vec::new();
        if let Some(c) = cursor {
            params.push(format!("cursor={c}"));
        }
        if let Some(ps) = page_size {
            params.push(format!("page_size={ps}"));
        }
        if !params.is_empty() {
            path.push('?');
            path.push_str(&params.join("&"));
        }
        self.client.get(&path).await
    }

    /// Gets metadata for a single pronunciation dictionary.
    ///
    /// Calls `GET /v1/pronunciation-dictionaries/{pronunciation_dictionary_id}`.
    ///
    /// # Arguments
    ///
    /// * `dictionary_id` — The pronunciation dictionary ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_pronunciation_dictionary(
        &self,
        dictionary_id: &str,
    ) -> Result<PronunciationDictionaryMetadata> {
        let path = format!("/v1/pronunciation-dictionaries/{dictionary_id}");
        self.client.get(&path).await
    }

    /// Downloads a pronunciation dictionary version as PLS XML.
    ///
    /// Calls `GET /v1/pronunciation-dictionaries/{dictionary_id}/{version_id}/download`.
    ///
    /// Returns the raw PLS file bytes.
    ///
    /// # Arguments
    ///
    /// * `dictionary_id` — The pronunciation dictionary ID.
    /// * `version_id` — The version ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn download_pronunciation_dictionary_version(
        &self,
        dictionary_id: &str,
        version_id: &str,
    ) -> Result<Bytes> {
        let path = format!("/v1/pronunciation-dictionaries/{dictionary_id}/{version_id}/download");
        self.client.get_bytes(&path).await
    }

    /// Creates a pronunciation dictionary from an uploaded PLS/CSV file.
    ///
    /// Calls `POST /v1/pronunciation-dictionaries/add-from-file` with
    /// `multipart/form-data`.
    ///
    /// # Arguments
    ///
    /// * `name` — Dictionary name.
    /// * `description` — Optional description.
    /// * `file` — The file as `(filename, content_type, data)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn create_pronunciation_dictionary_from_file(
        &self,
        name: &str,
        description: Option<&str>,
        file: (&str, &str, &[u8]),
    ) -> Result<AddPronunciationDictionaryResponse> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_add_from_file_multipart(&boundary, name, description, file);
        let content_type = format!("multipart/form-data; boundary={boundary}");
        self.client
            .post_multipart("/v1/pronunciation-dictionaries/add-from-file", body, &content_type)
            .await
    }

    /// Creates a pronunciation dictionary from inline rules.
    ///
    /// Calls `POST /v1/pronunciation-dictionaries/add-from-rules` with a
    /// JSON body.
    ///
    /// # Arguments
    ///
    /// * `request` — Dictionary name, rules, etc.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn create_pronunciation_dictionary_from_rules(
        &self,
        request: &CreatePronunciationDictionaryFromRulesRequest,
    ) -> Result<AddPronunciationDictionaryResponse> {
        self.client.post("/v1/pronunciation-dictionaries/add-from-rules", request).await
    }

    /// Adds rules to a pronunciation dictionary.
    ///
    /// Calls `POST /v1/pronunciation-dictionaries/{pronunciation_dictionary_id}/add-rules`
    /// with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `dictionary_id` — The pronunciation dictionary ID.
    /// * `request` — Rules to add.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn add_pronunciation_rules(
        &self,
        dictionary_id: &str,
        request: &AddPronunciationRulesRequest,
    ) -> Result<PronunciationDictionaryRulesResponse> {
        let path = format!("/v1/pronunciation-dictionaries/{dictionary_id}/add-rules");
        self.client.post(&path, request).await
    }

    /// Removes rules from a pronunciation dictionary.
    ///
    /// Calls `POST /v1/pronunciation-dictionaries/{pronunciation_dictionary_id}/remove-rules`
    /// with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `dictionary_id` — The pronunciation dictionary ID.
    /// * `request` — Rule strings to remove.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn remove_pronunciation_rules(
        &self,
        dictionary_id: &str,
        request: &RemovePronunciationRulesRequest,
    ) -> Result<PronunciationDictionaryRulesResponse> {
        let path = format!("/v1/pronunciation-dictionaries/{dictionary_id}/remove-rules");
        self.client.post(&path, request).await
    }

    /// Updates a pronunciation dictionary (name, archived status).
    ///
    /// Calls `PATCH /v1/pronunciation-dictionaries/{pronunciation_dictionary_id}`
    /// with a JSON body.
    ///
    /// # Arguments
    ///
    /// * `dictionary_id` — The pronunciation dictionary ID.
    /// * `request` — Fields to update.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn update_pronunciation_dictionary(
        &self,
        dictionary_id: &str,
        request: &UpdatePronunciationDictionaryRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/pronunciation-dictionaries/{dictionary_id}");
        self.client.patch(&path, request).await
    }
}

// ===========================================================================
// Request types (Serialize only, local to this service)
// ===========================================================================

/// Request body for creating a project.
///
/// Used with [`StudioService::add_project`]. Sent as multipart form-data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddProjectRequest {
    /// Project name (required).
    pub name: String,
    /// Default voice ID for titles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_title_voice_id: Option<String>,
    /// Default voice ID for paragraphs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_paragraph_voice_id: Option<String>,
    /// Default model ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model_id: Option<String>,
    /// URL to import content from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_url: Option<String>,
    /// Quality preset (e.g. "standard", "high").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_preset: Option<String>,
    /// Project title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Author name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Project description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether to enable volume normalization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_normalization: Option<bool>,
    /// Language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Content type classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Fiction/non-fiction type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiction: Option<String>,
    /// Whether to auto-convert after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert: Option<bool>,
}

/// Request body for editing a project.
///
/// Used with [`StudioService::edit_project`]. Sent as JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EditProjectRequest {
    /// Project name (required).
    pub name: String,
    /// Default voice ID for titles (required).
    pub default_title_voice_id: String,
    /// Default voice ID for paragraphs (required).
    pub default_paragraph_voice_id: String,
    /// Project title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Author name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// ISBN number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn_number: Option<String>,
    /// Whether to enable volume normalization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_normalization: Option<bool>,
}

/// Request body for updating project content.
///
/// Used with [`StudioService::edit_project_content`]. Sent as multipart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EditProjectContentRequest {
    /// URL to import content from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_url: Option<String>,
    /// JSON content string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_content_json: Option<String>,
    /// Whether to auto-convert after update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert: Option<bool>,
}

/// Request body for adding a chapter.
///
/// Used with [`StudioService::add_chapter`]. Sent as JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddChapterRequest {
    /// Chapter name (required).
    pub name: String,
    /// URL to import chapter content from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_url: Option<String>,
}

/// Request body for editing a chapter.
///
/// Used with [`StudioService::edit_chapter`]. Sent as JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EditChapterRequest {
    /// Chapter name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Chapter content. See [`ChapterContentInput`](crate::types::ChapterContentInput).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}

/// Request body for streaming a snapshot's audio.
///
/// Used internally for snapshot stream endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SnapshotStreamRequest {
    /// Whether to convert the audio to MPEG format.
    #[serde(skip_serializing_if = "Option::is_none")]
    convert_to_mpeg: Option<bool>,
}

/// Request body for creating a podcast.
///
/// Used with [`StudioService::create_podcast`]. Sent as JSON.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[expect(clippy::derive_partial_eq_without_eq, reason = "serde_json::Value does not implement Eq")]
pub struct CreatePodcastRequest {
    /// Model ID (required).
    pub model_id: String,
    /// Podcast mode configuration (required). Use
    /// [`PodcastConversationMode`](crate::types::PodcastConversationMode) or
    /// [`PodcastBulletinMode`](crate::types::PodcastBulletinMode).
    pub mode: serde_json::Value,
    /// Podcast source (required). Use
    /// [`PodcastTextSource`](crate::types::PodcastTextSource) or
    /// [`PodcastUrlSource`](crate::types::PodcastUrlSource).
    pub source: serde_json::Value,
    /// Quality preset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_preset: Option<String>,
    /// Duration scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_scale: Option<String>,
    /// Language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Request body for attaching pronunciation dictionaries to a project.
///
/// Used with [`StudioService::update_pronunciation_dictionaries`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdateProjectPronunciationDictionariesRequest {
    /// List of dictionary locators.
    pub pronunciation_dictionary_locators: Vec<PronunciationDictionaryLocatorRequest>,
    /// Whether to invalidate affected text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidate_affected_text: Option<bool>,
}

/// Request body for creating a pronunciation dictionary from rules.
///
/// Used with [`StudioService::create_pronunciation_dictionary_from_rules`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreatePronunciationDictionaryFromRulesRequest {
    /// Dictionary name (required).
    pub name: String,
    /// Rules to add (can be alias or phoneme rules, serialized as JSON).
    pub rules: Vec<serde_json::Value>,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ===========================================================================
// Multipart helpers
// ===========================================================================

/// Generates a simple pseudo-UUID v4 hex string (32 chars, no dashes).
fn uuid_v4_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();

    let pid = u128::from(std::process::id());
    let val = t ^ (pid << 64);
    format!("{val:032x}")
}

/// Appends a text form field to a multipart body.
fn append_text_part(buf: &mut Vec<u8>, boundary: &str, name: &str, value: &str) {
    buf.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n"
        )
        .as_bytes(),
    );
}

/// Appends a file part to a multipart body.
fn append_file_part(
    buf: &mut Vec<u8>,
    boundary: &str,
    name: &str,
    filename: &str,
    content_type: &str,
    data: &[u8],
) {
    buf.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"; \
             filename=\"{filename}\"\r\nContent-Type: {content_type}\r\n\r\n"
        )
        .as_bytes(),
    );
    buf.extend_from_slice(data);
    buf.extend_from_slice(b"\r\n");
}

/// Builds a multipart body for `POST /v1/studio/projects`.
fn build_add_project_multipart(
    boundary: &str,
    request: &AddProjectRequest,
    from_document: Option<(&str, &str, &[u8])>,
) -> Vec<u8> {
    let mut buf = Vec::new();

    append_text_part(&mut buf, boundary, "name", &request.name);

    if let Some(ref v) = request.default_title_voice_id {
        append_text_part(&mut buf, boundary, "default_title_voice_id", v);
    }
    if let Some(ref v) = request.default_paragraph_voice_id {
        append_text_part(&mut buf, boundary, "default_paragraph_voice_id", v);
    }
    if let Some(ref v) = request.default_model_id {
        append_text_part(&mut buf, boundary, "default_model_id", v);
    }
    if let Some(ref v) = request.from_url {
        append_text_part(&mut buf, boundary, "from_url", v);
    }
    if let Some(ref v) = request.quality_preset {
        append_text_part(&mut buf, boundary, "quality_preset", v);
    }
    if let Some(ref v) = request.title {
        append_text_part(&mut buf, boundary, "title", v);
    }
    if let Some(ref v) = request.author {
        append_text_part(&mut buf, boundary, "author", v);
    }
    if let Some(ref v) = request.description {
        append_text_part(&mut buf, boundary, "description", v);
    }
    if let Some(v) = request.volume_normalization {
        append_text_part(&mut buf, boundary, "volume_normalization", &v.to_string());
    }
    if let Some(ref v) = request.language {
        append_text_part(&mut buf, boundary, "language", v);
    }
    if let Some(ref v) = request.content_type {
        append_text_part(&mut buf, boundary, "content_type", v);
    }
    if let Some(ref v) = request.fiction {
        append_text_part(&mut buf, boundary, "fiction", v);
    }
    if let Some(v) = request.auto_convert {
        append_text_part(&mut buf, boundary, "auto_convert", &v.to_string());
    }
    if let Some((filename, ct, data)) = from_document {
        append_file_part(&mut buf, boundary, "from_document", filename, ct, data);
    }

    buf.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    buf
}

/// Builds a multipart body for `POST /v1/studio/projects/{id}/content`.
fn build_edit_content_multipart(
    boundary: &str,
    request: &EditProjectContentRequest,
    from_document: Option<(&str, &str, &[u8])>,
) -> Vec<u8> {
    let mut buf = Vec::new();

    if let Some(ref v) = request.from_url {
        append_text_part(&mut buf, boundary, "from_url", v);
    }
    if let Some(ref v) = request.from_content_json {
        append_text_part(&mut buf, boundary, "from_content_json", v);
    }
    if let Some(v) = request.auto_convert {
        append_text_part(&mut buf, boundary, "auto_convert", &v.to_string());
    }
    if let Some((filename, ct, data)) = from_document {
        append_file_part(&mut buf, boundary, "from_document", filename, ct, data);
    }

    buf.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    buf
}

/// Builds a multipart body for
/// `POST /v1/pronunciation-dictionaries/add-from-file`.
fn build_add_from_file_multipart(
    boundary: &str,
    name: &str,
    description: Option<&str>,
    file: (&str, &str, &[u8]),
) -> Vec<u8> {
    let mut buf = Vec::new();

    append_text_part(&mut buf, boundary, "name", name);
    if let Some(desc) = description {
        append_text_part(&mut buf, boundary, "description", desc);
    }

    let (filename, ct, data) = file;
    append_file_part(&mut buf, boundary, "file", filename, ct, data);

    buf.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    buf
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, header, method, path},
    };

    use super::*;
    use crate::{
        ElevenLabsClient,
        config::ClientConfig,
        types::{
            AddPronunciationRulesRequest, PronunciationDictionaryLocatorRequest,
            RemovePronunciationRulesRequest, UpdatePronunciationDictionaryRequest,
        },
    };

    /// Helper to create a test client pointed at a mock server.
    fn test_client(uri: &str) -> ElevenLabsClient {
        let config = ClientConfig::builder("test-key").base_url(uri).build();
        ElevenLabsClient::new(config).unwrap()
    }

    // -- get_projects ------------------------------------------------------

    #[tokio::test]
    async fn get_projects_returns_list() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/studio/projects"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "projects": [{
                    "project_id": "proj_1",
                    "name": "My Project",
                    "create_date_unix": 1714204800,
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
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.studio().get_projects().await.unwrap();
        assert_eq!(result.projects.len(), 1);
        assert_eq!(result.projects[0].project_id, "proj_1");
    }

    // -- delete_project ----------------------------------------------------

    #[tokio::test]
    async fn delete_project_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/studio/projects/proj_1"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.studio().delete_project("proj_1").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    // -- get_chapters ------------------------------------------------------

    #[tokio::test]
    async fn get_chapters_returns_list() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/studio/projects/proj_1/chapters"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "chapters": [{
                    "chapter_id": "ch_1",
                    "name": "Chapter 1",
                    "can_be_downloaded": true,
                    "state": "default"
                }]
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.studio().get_chapters("proj_1").await.unwrap();
        assert_eq!(result.chapters.len(), 1);
        assert_eq!(result.chapters[0].chapter_id, "ch_1");
    }

    // -- add_chapter -------------------------------------------------------

    #[tokio::test]
    async fn add_chapter_returns_chapter() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/studio/projects/proj_1/chapters"))
            .and(header("xi-api-key", "test-key"))
            .and(body_json(serde_json::json!({
                "name": "New Chapter"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "chapter": {
                    "chapter_id": "ch_new",
                    "name": "New Chapter",
                    "can_be_downloaded": false,
                    "state": "default",
                    "content": {
                        "blocks": []
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = AddChapterRequest { name: "New Chapter".into(), from_url: None };
        let result = client.studio().add_chapter("proj_1", &req).await.unwrap();
        assert_eq!(result.chapter.chapter_id, "ch_new");
    }

    // -- delete_chapter ----------------------------------------------------

    #[tokio::test]
    async fn delete_chapter_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/studio/projects/proj_1/chapters/ch_1"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.studio().delete_chapter("proj_1", "ch_1").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    // -- get_project_snapshots ---------------------------------------------

    #[tokio::test]
    async fn get_project_snapshots_returns_list() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/studio/projects/proj_1/snapshots"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "snapshots": [{
                    "project_snapshot_id": "snap_1",
                    "project_id": "proj_1",
                    "created_at_unix": 1714204800,
                    "name": "Snapshot 1"
                }]
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.studio().get_project_snapshots("proj_1").await.unwrap();
        assert_eq!(result.snapshots.len(), 1);
        assert_eq!(result.snapshots[0].project_snapshot_id, "snap_1");
    }

    // -- get_project_muted_tracks ------------------------------------------

    #[tokio::test]
    async fn get_project_muted_tracks_returns_ids() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/studio/projects/proj_1/muted-tracks"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "chapter_ids": ["ch_1", "ch_2"]
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.studio().get_project_muted_tracks("proj_1").await.unwrap();
        assert_eq!(result.chapter_ids, vec!["ch_1", "ch_2"]);
    }

    // -- create_podcast ----------------------------------------------------

    #[tokio::test]
    async fn create_podcast_returns_project() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/studio/podcasts"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
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
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = CreatePodcastRequest {
            model_id: "eleven_multilingual_v2".into(),
            mode: serde_json::json!({
                "type": "conversation",
                "conversation": {
                    "host_voice_id": "host_v",
                    "guest_voice_id": "guest_v"
                }
            }),
            source: serde_json::json!({
                "type": "text",
                "text": "Hello podcast world"
            }),
            quality_preset: None,
            duration_scale: None,
            language: None,
        };
        let result = client.studio().create_podcast(&req).await.unwrap();
        assert_eq!(result.project.project_id, "pod_1");
    }

    // -- get_pronunciation_dictionaries ------------------------------------

    #[tokio::test]
    async fn get_pronunciation_dictionaries_returns_list() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/pronunciation-dictionaries"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "pronunciation_dictionaries": [{
                    "id": "dict1",
                    "latest_version_id": "v1",
                    "latest_version_rules_num": 5,
                    "name": "Dict One",
                    "permission_on_resource": "admin",
                    "created_by": "user1",
                    "creation_time_unix": 1700000000
                }],
                "has_more": false,
                "next_cursor": null
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.studio().get_pronunciation_dictionaries(None, None).await.unwrap();
        assert_eq!(result.pronunciation_dictionaries.len(), 1);
        assert!(!result.has_more);
    }

    // -- add_pronunciation_rules -------------------------------------------

    #[tokio::test]
    async fn add_pronunciation_rules_returns_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/pronunciation-dictionaries/dict1/add-rules"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "dict1",
                "version_id": "v2",
                "version_rules_num": 7
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = AddPronunciationRulesRequest {
            rules: vec![serde_json::json!({
                "type": "alias",
                "string_to_replace": "ElevenLabs",
                "alias": "Eleven Labs"
            })],
        };
        let result = client.studio().add_pronunciation_rules("dict1", &req).await.unwrap();
        assert_eq!(result.version_rules_num, 7);
        assert_eq!(result.version_id, "v2");
    }

    // -- convert_project ---------------------------------------------------

    #[tokio::test]
    async fn convert_project_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/studio/projects/proj_1/convert"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.studio().convert_project("proj_1").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    // -- edit_project ------------------------------------------------------

    #[tokio::test]
    async fn edit_project_returns_updated() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/studio/projects/proj_1"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "project": {
                    "project_id": "proj_1",
                    "name": "Updated Name",
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
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = EditProjectRequest {
            name: "Updated Name".into(),
            default_title_voice_id: "v1".into(),
            default_paragraph_voice_id: "v2".into(),
            title: None,
            author: None,
            isbn_number: None,
            volume_normalization: None,
        };
        let result = client.studio().edit_project("proj_1", &req).await.unwrap();
        assert_eq!(result.project.name, "Updated Name");
    }

    // -- multipart helpers -------------------------------------------------

    #[test]
    fn uuid_v4_simple_returns_hex() {
        let id = uuid_v4_simple();
        assert!(!id.is_empty());
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn build_add_project_multipart_contains_fields() {
        let req = AddProjectRequest {
            name: "Test".into(),
            default_title_voice_id: Some("vt".into()),
            default_paragraph_voice_id: None,
            default_model_id: None,
            from_url: Some("https://example.com".into()),
            quality_preset: None,
            title: None,
            author: None,
            description: None,
            volume_normalization: Some(true),
            language: None,
            content_type: None,
            fiction: None,
            auto_convert: None,
        };
        let body = build_add_project_multipart("test-boundary", &req, None);
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("Test"));
        assert!(body_str.contains("from_url"));
        assert!(body_str.contains("volume_normalization"));
        assert!(body_str.contains("--test-boundary--"));
    }

    #[test]
    fn build_add_from_file_multipart_contains_fields() {
        let body = build_add_from_file_multipart(
            "test-boundary",
            "My Dict",
            Some("A test dict"),
            ("dict.pls", "application/pls+xml", b"<pls>fake</pls>"),
        );
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("My Dict"));
        assert!(body_str.contains("A test dict"));
        assert!(body_str.contains("dict.pls"));
        assert!(body_str.contains("<pls>fake</pls>"));
        assert!(body_str.contains("--test-boundary--"));
    }
}
