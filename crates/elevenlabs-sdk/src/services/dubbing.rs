//! Dubbing service providing access to dubbing project management and
//! dubbing studio endpoints.
//!
//! This module wraps all dubbing endpoints exposed by the ElevenLabs API:
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`create`](DubbingService::create) | `POST /v1/dubbing` | Create a dubbing project (multipart) |
//! | [`list`](DubbingService::list) | `GET /v1/dubbing` | List dubbing projects |
//! | [`get`](DubbingService::get) | `GET /v1/dubbing/{dubbing_id}` | Get dubbing metadata |
//! | [`delete`](DubbingService::delete) | `DELETE /v1/dubbing/{dubbing_id}` | Delete a dubbing project |
//! | [`get_audio`](DubbingService::get_audio) | `GET /v1/dubbing/{dubbing_id}/audio/{language_code}` | Get dubbed audio/video |
//! | [`get_transcript`](DubbingService::get_transcript) | `GET /v1/dubbing/{dubbing_id}/transcript/{language_code}` | Get transcript |
//! | [`get_transcript_formatted`](DubbingService::get_transcript_formatted) | `GET /v1/dubbing/{id}/transcripts/{lang}/format/{fmt}` | Get formatted transcript |
//! | [`get_resource`](DubbingService::get_resource) | `GET /v1/dubbing/resource/{dubbing_id}` | Get full dubbing resource |
//! | [`add_language`](DubbingService::add_language) | `POST /v1/dubbing/resource/{dubbing_id}/language` | Add a language |
//! | [`create_speaker`](DubbingService::create_speaker) | `POST /v1/dubbing/resource/{dubbing_id}/speaker` | Create a speaker |
//! | [`update_speaker`](DubbingService::update_speaker) | `PATCH /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}` | Update speaker |
//! | [`get_similar_voices`](DubbingService::get_similar_voices) | `GET /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}/similar-voices` | Similar voices |
//! | [`create_segment`](DubbingService::create_segment) | `POST /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}/segment` | Create segment |
//! | [`update_segment`](DubbingService::update_segment) | `PATCH /v1/dubbing/resource/{dubbing_id}/segment/{segment_id}/{language}` | Update segment |
//! | [`delete_segment`](DubbingService::delete_segment) | `DELETE /v1/dubbing/resource/{dubbing_id}/segment/{segment_id}` | Delete segment |
//! | [`dub_segments`](DubbingService::dub_segments) | `POST /v1/dubbing/resource/{dubbing_id}/dub` | Dub segments |
//! | [`render`](DubbingService::render) | `POST /v1/dubbing/resource/{dubbing_id}/render/{language}` | Render audio/video |
//! | [`transcribe_segments`](DubbingService::transcribe_segments) | `POST /v1/dubbing/resource/{dubbing_id}/transcribe` | Transcribe segments |
//! | [`translate_segments`](DubbingService::translate_segments) | `POST /v1/dubbing/resource/{dubbing_id}/translate` | Translate segments |
//! | [`migrate_segments`](DubbingService::migrate_segments) | `POST /v1/dubbing/resource/{dubbing_id}/migrate-segments` | Migrate segments |
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
//! let dubs = client.dubbing().list(None, None).await?;
//! println!("Found {} dubbing projects", dubs.dubs.len());
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{
        AddLanguageRequest, CreateDubbingRequest, CreateSpeakerRequest, DeleteDubbingResponse,
        DoDubbingResponse, DubSegmentsRequest, DubbingMetadataPageResponse,
        DubbingMetadataResponse, DubbingRenderResponse, DubbingResource, DubbingTranscriptResponse,
        DubbingTranscriptsResponse, LanguageAddedResponse, MigrateSegmentsRequest,
        RenderDubbingRequest, SegmentCreatePayload, SegmentCreateResponse, SegmentDeleteResponse,
        SegmentDubResponse, SegmentMigrationResponse, SegmentTranscriptionResponse,
        SegmentTranslationResponse, SegmentUpdatePayload, SegmentUpdateResponse,
        SimilarVoicesForSpeakerResponse, SpeakerCreatedResponse, SpeakerUpdatedResponse,
        TranscribeSegmentsRequest, TranscriptFormat, TranslateSegmentsRequest,
        UpdateSpeakerRequest,
    },
};

/// Dubbing service providing typed access to dubbing project management and
/// dubbing studio endpoints.
///
/// Obtained via [`ElevenLabsClient::dubbing`].
#[derive(Debug)]
pub struct DubbingService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> DubbingService<'a> {
    /// Creates a new `DubbingService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    // =======================================================================
    // Core dubbing CRUD
    // =======================================================================

    /// Creates a new dubbing project.
    ///
    /// Calls `POST /v1/dubbing` with `multipart/form-data`.
    ///
    /// File fields (source media, CSV, foreground/background audio) are
    /// optional binary uploads. Non-file fields are taken from
    /// [`CreateDubbingRequest`].
    ///
    /// # Arguments
    ///
    /// * `request` — Project configuration (name, languages, etc.).
    /// * `file` — Optional source media file as `(filename, content_type, data)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn create(
        &self,
        request: &CreateDubbingRequest,
        file: Option<(&str, &str, &[u8])>,
    ) -> Result<DoDubbingResponse> {
        let boundary = format!("----ElevenLabsSDK{}", uuid_v4_simple());
        let body = build_create_dubbing_multipart(&boundary, request, file);
        let content_type = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart("/v1/dubbing", body, &content_type).await
    }

    /// Lists dubbing projects with optional pagination.
    ///
    /// Calls `GET /v1/dubbing`.
    ///
    /// # Arguments
    ///
    /// * `page_size` — Maximum number of results per page.
    /// * `cursor` — Pagination cursor from a previous response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn list(
        &self,
        page_size: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<DubbingMetadataPageResponse> {
        let mut path = "/v1/dubbing".to_owned();
        let mut params = Vec::new();
        if let Some(ps) = page_size {
            params.push(format!("page_size={ps}"));
        }
        if let Some(c) = cursor {
            params.push(format!("cursor={c}"));
        }
        if !params.is_empty() {
            path.push('?');
            path.push_str(&params.join("&"));
        }
        self.client.get(&path).await
    }

    /// Gets metadata for a single dubbing project.
    ///
    /// Calls `GET /v1/dubbing/{dubbing_id}`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get(&self, dubbing_id: &str) -> Result<DubbingMetadataResponse> {
        let path = format!("/v1/dubbing/{dubbing_id}");
        self.client.get(&path).await
    }

    /// Deletes a dubbing project.
    ///
    /// Calls `DELETE /v1/dubbing/{dubbing_id}`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete(&self, dubbing_id: &str) -> Result<DeleteDubbingResponse> {
        let path = format!("/v1/dubbing/{dubbing_id}");
        self.client.delete_json(&path).await
    }

    // =======================================================================
    // Audio & transcript retrieval
    // =======================================================================

    /// Gets the dubbed audio or video file for a specific language.
    ///
    /// Calls `GET /v1/dubbing/{dubbing_id}/audio/{language_code}`.
    ///
    /// Returns raw bytes of the media file.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `language_code` — ISO-639-1 language code.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get_audio(&self, dubbing_id: &str, language_code: &str) -> Result<Bytes> {
        let path = format!("/v1/dubbing/{dubbing_id}/audio/{language_code}");
        self.client.get_bytes(&path).await
    }

    /// Gets the transcript for a specific language.
    ///
    /// Calls `GET /v1/dubbing/{dubbing_id}/transcript/{language_code}`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `language_code` — ISO-639-1 language code.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_transcript(
        &self,
        dubbing_id: &str,
        language_code: &str,
    ) -> Result<DubbingTranscriptResponse> {
        let path = format!("/v1/dubbing/{dubbing_id}/transcript/{language_code}");
        self.client.get(&path).await
    }

    /// Gets a formatted transcript (SRT, WebVTT, or JSON).
    ///
    /// Calls `GET /v1/dubbing/{dubbing_id}/transcripts/{language_code}/format/{format_type}`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `language_code` — ISO-639-1 language code.
    /// * `format` — Desired transcript format.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_transcript_formatted(
        &self,
        dubbing_id: &str,
        language_code: &str,
        format: TranscriptFormat,
    ) -> Result<DubbingTranscriptsResponse> {
        let format_str = match format {
            TranscriptFormat::Srt => "srt",
            TranscriptFormat::Webvtt => "webvtt",
            TranscriptFormat::Json => "json",
        };
        let path =
            format!("/v1/dubbing/{dubbing_id}/transcripts/{language_code}/format/{format_str}");
        self.client.get(&path).await
    }

    // =======================================================================
    // Dubbing resource (studio)
    // =======================================================================

    /// Gets the full dubbing resource with speakers, segments, and renders.
    ///
    /// Calls `GET /v1/dubbing/resource/{dubbing_id}`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_resource(&self, dubbing_id: &str) -> Result<DubbingResource> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}");
        self.client.get(&path).await
    }

    /// Adds a target language to a dubbing resource.
    ///
    /// Calls `POST /v1/dubbing/resource/{dubbing_id}/language`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `request` — The language to add.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn add_language(
        &self,
        dubbing_id: &str,
        request: &AddLanguageRequest,
    ) -> Result<LanguageAddedResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/language");
        self.client.post(&path, request).await
    }

    // =======================================================================
    // Speaker management
    // =======================================================================

    /// Creates a new speaker in a dubbing resource.
    ///
    /// Calls `POST /v1/dubbing/resource/{dubbing_id}/speaker`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `request` — Speaker metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn create_speaker(
        &self,
        dubbing_id: &str,
        request: &CreateSpeakerRequest,
    ) -> Result<SpeakerCreatedResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/speaker");
        self.client.post(&path, request).await
    }

    /// Updates speaker metadata.
    ///
    /// Calls `PATCH /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `speaker_id` — The speaker ID to update.
    /// * `request` — Updated speaker metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn update_speaker(
        &self,
        dubbing_id: &str,
        speaker_id: &str,
        request: &UpdateSpeakerRequest,
    ) -> Result<SpeakerUpdatedResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}");
        self.client.patch(&path, request).await
    }

    /// Gets voices similar to a speaker's voice.
    ///
    /// Calls `GET /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}/similar-voices`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `speaker_id` — The speaker ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_similar_voices(
        &self,
        dubbing_id: &str,
        speaker_id: &str,
    ) -> Result<SimilarVoicesForSpeakerResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}/similar-voices");
        self.client.get(&path).await
    }

    // =======================================================================
    // Segment management
    // =======================================================================

    /// Creates a new segment for a speaker.
    ///
    /// Calls `POST /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}/segment`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `speaker_id` — The speaker ID to add the segment to.
    /// * `request` — Segment timing and text.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn create_segment(
        &self,
        dubbing_id: &str,
        speaker_id: &str,
        request: &SegmentCreatePayload,
    ) -> Result<SegmentCreateResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}/segment");
        self.client.post(&path, request).await
    }

    /// Updates a segment for a specific language.
    ///
    /// Calls `PATCH /v1/dubbing/resource/{dubbing_id}/segment/{segment_id}/{language}`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `segment_id` — The segment ID to update.
    /// * `language` — The language code for this segment update.
    /// * `request` — Updated segment fields.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn update_segment(
        &self,
        dubbing_id: &str,
        segment_id: &str,
        language: &str,
        request: &SegmentUpdatePayload,
    ) -> Result<SegmentUpdateResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/segment/{segment_id}/{language}");
        self.client.patch(&path, request).await
    }

    /// Deletes a segment from a dubbing resource.
    ///
    /// Calls `DELETE /v1/dubbing/resource/{dubbing_id}/segment/{segment_id}`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `segment_id` — The segment ID to delete.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn delete_segment(
        &self,
        dubbing_id: &str,
        segment_id: &str,
    ) -> Result<SegmentDeleteResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/segment/{segment_id}");
        self.client.delete_json(&path).await
    }

    // =======================================================================
    // Dubbing operations (dub, transcribe, translate, render, migrate)
    // =======================================================================

    /// Dubs specified segments (generates speech).
    ///
    /// Calls `POST /v1/dubbing/resource/{dubbing_id}/dub`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `request` — Segments and languages to dub.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn dub_segments(
        &self,
        dubbing_id: &str,
        request: &DubSegmentsRequest,
    ) -> Result<SegmentDubResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/dub");
        self.client.post(&path, request).await
    }

    /// Renders dubbed audio or video for a specific language.
    ///
    /// Calls `POST /v1/dubbing/resource/{dubbing_id}/render/{language}`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `language` — The target language code.
    /// * `request` — Render configuration (format, volume normalization).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn render(
        &self,
        dubbing_id: &str,
        language: &str,
        request: &RenderDubbingRequest,
    ) -> Result<DubbingRenderResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/render/{language}");
        self.client.post(&path, request).await
    }

    /// Transcribes specified segments from source audio.
    ///
    /// Calls `POST /v1/dubbing/resource/{dubbing_id}/transcribe`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `request` — Segments to transcribe.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn transcribe_segments(
        &self,
        dubbing_id: &str,
        request: &TranscribeSegmentsRequest,
    ) -> Result<SegmentTranscriptionResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/transcribe");
        self.client.post(&path, request).await
    }

    /// Translates segments into target languages.
    ///
    /// Calls `POST /v1/dubbing/resource/{dubbing_id}/translate`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `request` — Segments and optional target languages.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn translate_segments(
        &self,
        dubbing_id: &str,
        request: &TranslateSegmentsRequest,
    ) -> Result<SegmentTranslationResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/translate");
        self.client.post(&path, request).await
    }

    /// Migrates segments from one speaker to another.
    ///
    /// Calls `POST /v1/dubbing/resource/{dubbing_id}/migrate-segments`.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` — The dubbing project ID.
    /// * `request` — Segment IDs and target speaker.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn migrate_segments(
        &self,
        dubbing_id: &str,
        request: &MigrateSegmentsRequest,
    ) -> Result<SegmentMigrationResponse> {
        let path = format!("/v1/dubbing/resource/{dubbing_id}/migrate-segments");
        self.client.post(&path, request).await
    }
}

// ---------------------------------------------------------------------------
// Multipart helpers
// ---------------------------------------------------------------------------

/// Generates a simple pseudo-random hex string for multipart boundaries.
fn uuid_v4_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    format!("{nanos:032x}")
}

/// Appends a text field to a multipart body buffer.
fn append_text_field(buf: &mut Vec<u8>, boundary: &str, name: &str, value: &str) {
    buf.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    buf.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").as_bytes(),
    );
    buf.extend_from_slice(value.as_bytes());
    buf.extend_from_slice(b"\r\n");
}

/// Appends a file part to a multipart body buffer.
fn append_file_part(
    buf: &mut Vec<u8>,
    boundary: &str,
    field_name: &str,
    filename: &str,
    content_type: &str,
    data: &[u8],
) {
    buf.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    buf.extend_from_slice(
        format!(
            "Content-Disposition: form-data; name=\"{field_name}\"; filename=\"{filename}\"\r\n"
        )
        .as_bytes(),
    );
    buf.extend_from_slice(format!("Content-Type: {content_type}\r\n\r\n").as_bytes());
    buf.extend_from_slice(data);
    buf.extend_from_slice(b"\r\n");
}

/// Builds the multipart body for `POST /v1/dubbing`.
fn build_create_dubbing_multipart(
    boundary: &str,
    request: &CreateDubbingRequest,
    file: Option<(&str, &str, &[u8])>,
) -> Vec<u8> {
    let mut buf = Vec::new();

    if let Some(ref name) = request.name {
        append_text_field(&mut buf, boundary, "name", name);
    }
    if let Some(ref source_url) = request.source_url {
        append_text_field(&mut buf, boundary, "source_url", source_url);
    }
    if let Some(ref source_lang) = request.source_lang {
        append_text_field(&mut buf, boundary, "source_lang", source_lang);
    }
    if let Some(ref target_lang) = request.target_lang {
        append_text_field(&mut buf, boundary, "target_lang", target_lang);
    }
    if let Some(ref target_accent) = request.target_accent {
        append_text_field(&mut buf, boundary, "target_accent", target_accent);
    }
    if let Some(num_speakers) = request.num_speakers {
        append_text_field(&mut buf, boundary, "num_speakers", &num_speakers.to_string());
    }
    if let Some(watermark) = request.watermark {
        append_text_field(&mut buf, boundary, "watermark", &watermark.to_string());
    }
    if let Some(start_time) = request.start_time {
        append_text_field(&mut buf, boundary, "start_time", &start_time.to_string());
    }
    if let Some(end_time) = request.end_time {
        append_text_field(&mut buf, boundary, "end_time", &end_time.to_string());
    }
    if let Some(highest_resolution) = request.highest_resolution {
        append_text_field(
            &mut buf,
            boundary,
            "highest_resolution",
            &highest_resolution.to_string(),
        );
    }
    if let Some(drop_background_audio) = request.drop_background_audio {
        append_text_field(
            &mut buf,
            boundary,
            "drop_background_audio",
            &drop_background_audio.to_string(),
        );
    }
    if let Some(use_profanity_filter) = request.use_profanity_filter {
        append_text_field(
            &mut buf,
            boundary,
            "use_profanity_filter",
            &use_profanity_filter.to_string(),
        );
    }
    if let Some(dubbing_studio) = request.dubbing_studio {
        append_text_field(&mut buf, boundary, "dubbing_studio", &dubbing_studio.to_string());
    }
    if let Some(disable_voice_cloning) = request.disable_voice_cloning {
        append_text_field(
            &mut buf,
            boundary,
            "disable_voice_cloning",
            &disable_voice_cloning.to_string(),
        );
    }
    if let Some(ref mode) = request.mode {
        append_text_field(&mut buf, boundary, "mode", mode);
    }
    if let Some(csv_fps) = request.csv_fps {
        append_text_field(&mut buf, boundary, "csv_fps", &csv_fps.to_string());
    }

    if let Some((filename, content_type, data)) = file {
        append_file_part(&mut buf, boundary, "file", filename, content_type, data);
    }

    buf.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    buf
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, header, method, path},
    };

    use crate::{
        ElevenLabsClient,
        config::ClientConfig,
        types::{
            AddLanguageRequest, CreateDubbingRequest, CreateSpeakerRequest, DubSegmentsRequest,
            MigrateSegmentsRequest, RenderDubbingRequest, RenderType, SegmentCreatePayload,
            SegmentUpdatePayload, TranscribeSegmentsRequest, TranslateSegmentsRequest,
            UpdateSpeakerRequest,
        },
    };

    /// Helper to create a test client pointed at a mock server.
    fn test_client(uri: &str) -> ElevenLabsClient {
        let config = ClientConfig::builder("test-key").base_url(uri).build();
        ElevenLabsClient::new(config).unwrap()
    }

    // -- create (multipart) ------------------------------------------------

    #[tokio::test]
    async fn create_dubbing_from_url() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/dubbing"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "dubbing_id": "dub_123",
                "expected_duration_sec": 60.0
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = CreateDubbingRequest {
            name: Some("Test Dub".into()),
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
        let result = client.dubbing().create(&req, None).await.unwrap();
        assert_eq!(result.dubbing_id, "dub_123");
        assert!((result.expected_duration_sec - 60.0).abs() < f64::EPSILON);
    }

    // -- list ---------------------------------------------------------------

    #[tokio::test]
    async fn list_dubbing_projects() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/dubbing"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "dubs": [{
                    "dubbing_id": "d1",
                    "name": "Dub One",
                    "status": "dubbed",
                    "source_language": "en",
                    "target_languages": ["es"],
                    "created_at": "2025-01-01T00:00:00Z"
                }],
                "next_cursor": null,
                "has_more": false
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.dubbing().list(None, None).await.unwrap();
        assert_eq!(result.dubs.len(), 1);
        assert_eq!(result.dubs[0].dubbing_id, "d1");
        assert!(!result.has_more);
    }

    // -- get ----------------------------------------------------------------

    #[tokio::test]
    async fn get_dubbing_metadata() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/dubbing/dub_123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "dubbing_id": "dub_123",
                "name": "My Dub",
                "status": "dubbed",
                "source_language": "en",
                "target_languages": ["es", "fr"],
                "editable": true,
                "created_at": "2025-01-15T10:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.dubbing().get("dub_123").await.unwrap();
        assert_eq!(result.dubbing_id, "dub_123");
        assert_eq!(result.name, "My Dub");
        assert!(result.editable);
    }

    // -- delete -------------------------------------------------------------

    #[tokio::test]
    async fn delete_dubbing_project() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/dubbing/dub_123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.dubbing().delete("dub_123").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    // -- get_audio ----------------------------------------------------------

    #[tokio::test]
    async fn get_dubbed_audio() {
        let mock_server = MockServer::start().await;
        let audio_data = b"fake-audio-bytes";

        Mock::given(method("GET"))
            .and(path("/v1/dubbing/dub_123/audio/es"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(audio_data.as_slice(), "audio/mpeg"),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.dubbing().get_audio("dub_123", "es").await.unwrap();
        assert_eq!(result.as_ref(), audio_data);
    }

    // -- get_transcript -----------------------------------------------------

    #[tokio::test]
    async fn get_transcript() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/dubbing/dub_123/transcript/en"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "language": "en",
                "utterances": [{
                    "text": "Hello world",
                    "speaker_id": "spk1",
                    "start_s": 0.0,
                    "end_s": 1.5,
                    "words": []
                }]
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.dubbing().get_transcript("dub_123", "en").await.unwrap();
        assert_eq!(result.language, "en");
        assert_eq!(result.utterances.len(), 1);
    }

    // -- get_resource -------------------------------------------------------

    #[tokio::test]
    async fn get_dubbing_resource() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/dubbing/resource/dub_123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "dub_123",
                "version": 1,
                "source_language": "en",
                "target_languages": ["es"],
                "input": {
                    "src": "/path/input.mp4",
                    "content_type": "video/mp4",
                    "bucket_name": "bucket",
                    "random_path_slug": "slug",
                    "duration_secs": 120.0,
                    "is_audio": false,
                    "url": "https://cdn.example.com/input.mp4"
                },
                "background": null,
                "foreground": null,
                "speaker_tracks": {},
                "speaker_segments": {},
                "renders": {}
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.dubbing().get_resource("dub_123").await.unwrap();
        assert_eq!(result.id, "dub_123");
        assert_eq!(result.source_language, "en");
    }

    // -- add_language -------------------------------------------------------

    #[tokio::test]
    async fn add_language_returns_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/dubbing/resource/dub_123/language"))
            .and(header("xi-api-key", "test-key"))
            .and(body_json(serde_json::json!({"language": "fr"})))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(serde_json::json!({"version": 2})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = AddLanguageRequest { language: "fr".into() };
        let result = client.dubbing().add_language("dub_123", &req).await.unwrap();
        assert_eq!(result.version, 2);
    }

    // -- create_speaker -----------------------------------------------------

    #[tokio::test]
    async fn create_speaker_returns_id() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/dubbing/resource/dub_123/speaker"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "version": 3,
                "speaker_id": "spk_new"
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = CreateSpeakerRequest {
            speaker_name: Some("Speaker A".into()),
            voice_id: None,
            voice_stability: None,
            voice_similarity: None,
            voice_style: None,
        };
        let result = client.dubbing().create_speaker("dub_123", &req).await.unwrap();
        assert_eq!(result.version, 3);
        assert_eq!(result.speaker_id, "spk_new");
    }

    // -- update_speaker -----------------------------------------------------

    #[tokio::test]
    async fn update_speaker_returns_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path("/v1/dubbing/resource/dub_123/speaker/spk_1"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": 4})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = UpdateSpeakerRequest {
            speaker_name: Some("Updated Name".into()),
            voice_id: None,
            voice_stability: None,
            voice_similarity: None,
            voice_style: None,
            languages: None,
        };
        let result = client.dubbing().update_speaker("dub_123", "spk_1", &req).await.unwrap();
        assert_eq!(result.version, 4);
    }

    // -- create_segment -----------------------------------------------------

    #[tokio::test]
    async fn create_segment_returns_id() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/dubbing/resource/dub_123/speaker/spk_1/segment"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "version": 5,
                "new_segment": "seg_new"
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = SegmentCreatePayload {
            start_time: 1.0,
            end_time: 5.0,
            text: Some("Hello".into()),
            translations: None,
        };
        let result = client.dubbing().create_segment("dub_123", "spk_1", &req).await.unwrap();
        assert_eq!(result.version, 5);
        assert_eq!(result.new_segment, "seg_new");
    }

    // -- update_segment -----------------------------------------------------

    #[tokio::test]
    async fn update_segment_returns_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path("/v1/dubbing/resource/dub_123/segment/seg_1/es"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": 6})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req =
            SegmentUpdatePayload { start_time: None, end_time: None, text: Some("Hola".into()) };
        let result = client.dubbing().update_segment("dub_123", "seg_1", "es", &req).await.unwrap();
        assert_eq!(result.version, 6);
    }

    // -- delete_segment -----------------------------------------------------

    #[tokio::test]
    async fn delete_segment_returns_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/dubbing/resource/dub_123/segment/seg_1"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": 7})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.dubbing().delete_segment("dub_123", "seg_1").await.unwrap();
        assert_eq!(result.version, 7);
    }

    // -- dub_segments -------------------------------------------------------

    #[tokio::test]
    async fn dub_segments_returns_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/dubbing/resource/dub_123/dub"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": 8})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = DubSegmentsRequest {
            segments: vec!["seg_1".into()],
            languages: Some(vec!["es".into()]),
        };
        let result = client.dubbing().dub_segments("dub_123", &req).await.unwrap();
        assert_eq!(result.version, 8);
    }

    // -- render -------------------------------------------------------------

    #[tokio::test]
    async fn render_returns_render_id() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/dubbing/resource/dub_123/render/es"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": 9,
                "render_id": "render_abc"
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req =
            RenderDubbingRequest { render_type: RenderType::Mp4, normalize_volume: Some(true) };
        let result = client.dubbing().render("dub_123", "es", &req).await.unwrap();
        assert_eq!(result.version, 9);
        assert_eq!(result.render_id, "render_abc");
    }

    // -- transcribe_segments ------------------------------------------------

    #[tokio::test]
    async fn transcribe_segments_returns_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/dubbing/resource/dub_123/transcribe"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": 10})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = TranscribeSegmentsRequest { segments: vec!["seg_1".into(), "seg_2".into()] };
        let result = client.dubbing().transcribe_segments("dub_123", &req).await.unwrap();
        assert_eq!(result.version, 10);
    }

    // -- translate_segments -------------------------------------------------

    #[tokio::test]
    async fn translate_segments_returns_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/dubbing/resource/dub_123/translate"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": 11})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = TranslateSegmentsRequest { segments: vec!["seg_1".into()], languages: None };
        let result = client.dubbing().translate_segments("dub_123", &req).await.unwrap();
        assert_eq!(result.version, 11);
    }

    // -- migrate_segments ---------------------------------------------------

    #[tokio::test]
    async fn migrate_segments_returns_version() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/dubbing/resource/dub_123/migrate-segments"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": 12})),
            )
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let req = MigrateSegmentsRequest {
            segment_ids: vec!["seg_1".into(), "seg_2".into()],
            speaker_id: "spk_target".into(),
        };
        let result = client.dubbing().migrate_segments("dub_123", &req).await.unwrap();
        assert_eq!(result.version, 12);
    }

    // -- get_similar_voices -------------------------------------------------

    #[tokio::test]
    async fn get_similar_voices_returns_list() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/dubbing/resource/dub_123/speaker/spk_1/similar-voices"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voices": [{
                    "voice_id": "v1",
                    "name": "Voice One",
                    "category": "premade",
                    "description": null,
                    "preview_url": null
                }]
            })))
            .mount(&mock_server)
            .await;

        let client = test_client(&mock_server.uri());
        let result = client.dubbing().get_similar_voices("dub_123", "spk_1").await.unwrap();
        assert_eq!(result.voices.len(), 1);
        assert_eq!(result.voices[0].voice_id, "v1");
    }

    // -- multipart helpers --------------------------------------------------

    #[test]
    fn uuid_v4_simple_returns_32_char_hex() {
        let id = super::uuid_v4_simple();
        assert_eq!(id.len(), 32);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn build_create_dubbing_multipart_contains_fields() {
        let req = CreateDubbingRequest {
            name: Some("Test".into()),
            source_url: Some("https://example.com/v.mp4".into()),
            source_lang: None,
            target_lang: Some("es".into()),
            target_accent: None,
            num_speakers: Some(2),
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
        let boundary = "test-boundary";
        let body = super::build_create_dubbing_multipart(boundary, &req, None);
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("Test"));
        assert!(body_str.contains("source_url"));
        assert!(body_str.contains("target_lang"));
        assert!(body_str.contains("num_speakers"));
        assert!(body_str.contains("--test-boundary--"));
    }

    #[test]
    fn build_create_dubbing_multipart_with_file() {
        let req = CreateDubbingRequest {
            name: Some("With File".into()),
            source_url: None,
            source_lang: None,
            target_lang: Some("fr".into()),
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
        let boundary = "test-boundary";
        let body = super::build_create_dubbing_multipart(
            boundary,
            &req,
            Some(("video.mp4", "video/mp4", b"fake-video-data")),
        );
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("video.mp4"));
        assert!(body_str.contains("video/mp4"));
        assert!(body_str.contains("fake-video-data"));
    }
}
