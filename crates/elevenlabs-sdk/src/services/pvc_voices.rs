//! Professional Voice Cloning (PVC) service covering 14 endpoints.
//!
//! Provides typed access to the PVC voice creation, editing, sample
//! management, speaker separation, captcha verification, training,
//! and manual verification endpoints.

use bytes::Bytes;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{
        AddVoiceResponse, CreatePvcVoiceRequest, DeletePvcSampleResponse, EditPvcVoiceRequest,
        EditPvcVoiceSampleRequest, GetPvcCaptchaResponse, RequestPvcManualVerificationResponse,
        SpeakerSeparationResponse, StartPvcTrainingResponse, StartSpeakerSeparationResponse,
        VerifyPvcCaptchaResponse, VoiceSamplePreviewResponse, VoiceSampleWaveformResponse,
    },
};

/// Service for PVC (Professional Voice Cloning) endpoints.
///
/// Obtained via [`ElevenLabsClient::pvc_voices`].
#[derive(Debug)]
pub struct PvcVoicesService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> PvcVoicesService<'a> {
    /// Creates a new `PvcVoicesService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    // =======================================================================
    // Voice CRUD
    // =======================================================================

    /// Creates a new PVC voice with metadata (no samples).
    ///
    /// `POST /v1/voices/pvc`
    pub async fn create_pvc_voice(
        &self,
        request: &CreatePvcVoiceRequest,
    ) -> Result<AddVoiceResponse> {
        self.client.post("/v1/voices/pvc", request).await
    }

    /// Edits PVC voice metadata.
    ///
    /// `POST /v1/voices/pvc/{voice_id}`
    pub async fn edit_pvc_voice(
        &self,
        voice_id: &str,
        request: &EditPvcVoiceRequest,
    ) -> Result<AddVoiceResponse> {
        let path = format!("/v1/voices/pvc/{voice_id}");
        self.client.post(&path, request).await
    }

    // =======================================================================
    // Samples
    // =======================================================================

    /// Adds audio samples to a PVC voice.
    ///
    /// `POST /v1/voices/pvc/{voice_id}/samples`
    ///
    /// The samples must be uploaded as multipart/form-data. Pass the raw
    /// file bytes, filename, and content type for each sample.
    pub async fn add_pvc_voice_samples(
        &self,
        voice_id: &str,
        files: &[(&str, &str, &[u8])],
    ) -> Result<serde_json::Value> {
        let boundary = multipart_boundary();
        let mut buf = Vec::new();
        for (filename, content_type, data) in files {
            append_file_part(&mut buf, &boundary, "files", filename, content_type, data);
        }
        buf.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

        let path = format!("/v1/voices/pvc/{voice_id}/samples");
        let ct = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart(&path, buf, &ct).await
    }

    /// Updates a PVC voice sample (noise removal, speaker selection, trim, rename).
    ///
    /// `POST /v1/voices/pvc/{voice_id}/samples/{sample_id}`
    pub async fn edit_pvc_voice_sample(
        &self,
        voice_id: &str,
        sample_id: &str,
        request: &EditPvcVoiceSampleRequest,
    ) -> Result<AddVoiceResponse> {
        let path = format!("/v1/voices/pvc/{voice_id}/samples/{sample_id}");
        self.client.post(&path, request).await
    }

    /// Deletes a sample from a PVC voice.
    ///
    /// `DELETE /v1/voices/pvc/{voice_id}/samples/{sample_id}`
    pub async fn delete_pvc_voice_sample(
        &self,
        voice_id: &str,
        sample_id: &str,
    ) -> Result<DeletePvcSampleResponse> {
        let path = format!("/v1/voices/pvc/{voice_id}/samples/{sample_id}");
        self.client.delete_json(&path).await
    }

    /// Retrieves the first 30 seconds of a voice sample audio preview.
    ///
    /// `GET /v1/voices/pvc/{voice_id}/samples/{sample_id}/audio`
    pub async fn get_pvc_sample_audio(
        &self,
        voice_id: &str,
        sample_id: &str,
    ) -> Result<VoiceSamplePreviewResponse> {
        let path = format!("/v1/voices/pvc/{voice_id}/samples/{sample_id}/audio");
        self.client.get(&path).await
    }

    /// Retrieves the visual waveform data for a voice sample.
    ///
    /// `GET /v1/voices/pvc/{voice_id}/samples/{sample_id}/waveform`
    pub async fn get_pvc_sample_visual_waveform(
        &self,
        voice_id: &str,
        sample_id: &str,
    ) -> Result<VoiceSampleWaveformResponse> {
        let path = format!("/v1/voices/pvc/{voice_id}/samples/{sample_id}/waveform");
        self.client.get(&path).await
    }

    // =======================================================================
    // Speaker Separation
    // =======================================================================

    /// Retrieves the speakers detected in a voice sample.
    ///
    /// `GET /v1/voices/pvc/{voice_id}/samples/{sample_id}/speakers`
    pub async fn get_pvc_sample_speakers(
        &self,
        voice_id: &str,
        sample_id: &str,
    ) -> Result<SpeakerSeparationResponse> {
        let path = format!("/v1/voices/pvc/{voice_id}/samples/{sample_id}/speakers");
        self.client.get(&path).await
    }

    /// Starts the speaker separation process for a sample.
    ///
    /// `POST /v1/voices/pvc/{voice_id}/samples/{sample_id}/separate-speakers`
    pub async fn start_speaker_separation(
        &self,
        voice_id: &str,
        sample_id: &str,
    ) -> Result<StartSpeakerSeparationResponse> {
        let path = format!("/v1/voices/pvc/{voice_id}/samples/{sample_id}/separate-speakers");
        self.client.post(&path, &serde_json::Value::Object(Default::default())).await
    }

    /// Retrieves the separated audio for a specific speaker.
    ///
    /// `GET /v1/voices/pvc/{voice_id}/samples/{sample_id}/speakers/{speaker_id}/audio`
    pub async fn get_speaker_audio(
        &self,
        voice_id: &str,
        sample_id: &str,
        speaker_id: &str,
    ) -> Result<Bytes> {
        let path =
            format!("/v1/voices/pvc/{voice_id}/samples/{sample_id}/speakers/{speaker_id}/audio");
        self.client.get_bytes(&path).await
    }

    // =======================================================================
    // Captcha
    // =======================================================================

    /// Retrieves the captcha challenge for a PVC voice.
    ///
    /// `GET /v1/voices/pvc/{voice_id}/captcha`
    pub async fn get_pvc_voice_captcha(&self, voice_id: &str) -> Result<GetPvcCaptchaResponse> {
        let path = format!("/v1/voices/pvc/{voice_id}/captcha");
        self.client.get(&path).await
    }

    /// Verifies the captcha for a PVC voice.
    ///
    /// `POST /v1/voices/pvc/{voice_id}/captcha`
    ///
    /// The captcha recording must be uploaded as multipart/form-data.
    pub async fn verify_pvc_voice_captcha(
        &self,
        voice_id: &str,
        recording_data: &[u8],
        filename: &str,
        content_type: &str,
    ) -> Result<VerifyPvcCaptchaResponse> {
        let boundary = multipart_boundary();
        let body = build_single_file_multipart(
            &boundary,
            "recording",
            filename,
            content_type,
            recording_data,
        );
        let path = format!("/v1/voices/pvc/{voice_id}/captcha");
        let ct = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart(&path, body, &ct).await
    }

    // =======================================================================
    // Training & Verification
    // =======================================================================

    /// Starts the PVC training process for a voice.
    ///
    /// `POST /v1/voices/pvc/{voice_id}/train`
    pub async fn run_pvc_voice_training(&self, voice_id: &str) -> Result<StartPvcTrainingResponse> {
        let path = format!("/v1/voices/pvc/{voice_id}/train");
        self.client.post(&path, &serde_json::Value::Object(Default::default())).await
    }

    /// Requests manual verification for a PVC voice.
    ///
    /// `POST /v1/voices/pvc/{voice_id}/verification`
    pub async fn request_pvc_manual_verification(
        &self,
        voice_id: &str,
    ) -> Result<RequestPvcManualVerificationResponse> {
        let path = format!("/v1/voices/pvc/{voice_id}/verification");
        self.client.post(&path, &serde_json::Value::Object(Default::default())).await
    }
}

// ---------------------------------------------------------------------------
// Multipart helpers
// ---------------------------------------------------------------------------

/// Generates a pseudo-random hex string for multipart boundaries.
fn multipart_boundary() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    format!("----ElevenLabsSDK{nanos:032x}")
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

/// Builds a multipart body with a single file part.
fn build_single_file_multipart(
    boundary: &str,
    field_name: &str,
    filename: &str,
    content_type: &str,
    data: &[u8],
) -> Vec<u8> {
    let mut buf = Vec::new();
    append_file_part(&mut buf, boundary, field_name, filename, content_type, data);
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
        matchers::{method, path},
    };

    fn test_config(base_url: &str) -> crate::config::ClientConfig {
        crate::config::ClientConfig::builder("test-key")
            .base_url(base_url)
            .max_retries(0_u32)
            .build()
    }

    #[tokio::test]
    async fn test_create_pvc_voice() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("POST"))
            .and(path("/v1/voices/pvc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voice_id": "pvc_voice_1"
            })))
            .mount(&mock_server)
            .await;

        let req = crate::types::CreatePvcVoiceRequest {
            name: "My PVC Voice".into(),
            description: None,
            labels: None,
        };
        let result = client.pvc_voices().create_pvc_voice(&req).await.unwrap();
        assert_eq!(result.voice_id, "pvc_voice_1");
    }

    #[tokio::test]
    async fn test_edit_pvc_voice() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("POST"))
            .and(path("/v1/voices/pvc/v1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voice_id": "v1"
            })))
            .mount(&mock_server)
            .await;

        let req = crate::types::EditPvcVoiceRequest {
            name: Some("Updated".into()),
            description: None,
            labels: None,
        };
        let result = client.pvc_voices().edit_pvc_voice("v1", &req).await.unwrap();
        assert_eq!(result.voice_id, "v1");
    }

    #[tokio::test]
    async fn test_delete_pvc_voice_sample() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("DELETE"))
            .and(path("/v1/voices/pvc/v1/samples/s1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok"
            })))
            .mount(&mock_server)
            .await;

        let result = client.pvc_voices().delete_pvc_voice_sample("v1", "s1").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    #[tokio::test]
    async fn test_get_pvc_sample_audio() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/voices/pvc/v1/samples/s1/audio"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "audio_base_64": "base64data",
                "voice_id": "v1",
                "sample_id": "s1",
                "media_type": "audio/mpeg"
            })))
            .mount(&mock_server)
            .await;

        let result = client.pvc_voices().get_pvc_sample_audio("v1", "s1").await.unwrap();
        assert_eq!(result.voice_id, "v1");
        assert_eq!(result.audio_base_64, "base64data");
    }

    #[tokio::test]
    async fn test_get_pvc_sample_waveform() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/voices/pvc/v1/samples/s1/waveform"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "sample_id": "s1",
                "visual_waveform": [0.1, 0.5, 0.3]
            })))
            .mount(&mock_server)
            .await;

        let result = client.pvc_voices().get_pvc_sample_visual_waveform("v1", "s1").await.unwrap();
        assert_eq!(result.sample_id, "s1");
        assert_eq!(result.visual_waveform.len(), 3);
    }

    #[tokio::test]
    async fn test_get_pvc_sample_speakers() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/voices/pvc/v1/samples/s1/speakers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "voice_id": "v1",
                "sample_id": "s1",
                "status": "completed"
            })))
            .mount(&mock_server)
            .await;

        let result = client.pvc_voices().get_pvc_sample_speakers("v1", "s1").await.unwrap();
        assert_eq!(result.voice_id, "v1");
    }

    #[tokio::test]
    async fn test_run_pvc_voice_training() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("POST"))
            .and(path("/v1/voices/pvc/v1/train"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok"
            })))
            .mount(&mock_server)
            .await;

        let result = client.pvc_voices().run_pvc_voice_training("v1").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    #[tokio::test]
    async fn test_request_pvc_manual_verification() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("POST"))
            .and(path("/v1/voices/pvc/v1/verification"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok"
            })))
            .mount(&mock_server)
            .await;

        let result = client.pvc_voices().request_pvc_manual_verification("v1").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    #[tokio::test]
    async fn test_get_pvc_voice_captcha() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/voices/pvc/v1/captcha"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "captcha_text": "say hello"
            })))
            .mount(&mock_server)
            .await;

        let result = client.pvc_voices().get_pvc_voice_captcha("v1").await.unwrap();
        assert!(result.extra.contains_key("captcha_text"));
    }

    #[tokio::test]
    async fn test_start_speaker_separation() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("POST"))
            .and(path("/v1/voices/pvc/v1/samples/s1/separate-speakers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok"
            })))
            .mount(&mock_server)
            .await;

        let result = client.pvc_voices().start_speaker_separation("v1", "s1").await.unwrap();
        assert_eq!(result.status, "ok");
    }
}
