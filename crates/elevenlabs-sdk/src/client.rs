//! HTTP client core for the ElevenLabs API.
//!
//! Provides [`ElevenLabsClient`], which wraps an [`hpx::Client`] and handles
//! URL construction, API key header injection, JSON (de)serialization,
//! error response parsing, and tracing instrumentation.

use bytes::Bytes;
use futures_core::Stream;
use hpx::{
    Method, StatusCode,
    header::{HeaderMap, HeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    auth::API_KEY_HEADER,
    config::ClientConfig,
    error::{ElevenLabsError, Result},
    middleware,
};

/// The main ElevenLabs API client.
///
/// Wraps an [`hpx::Client`] with ElevenLabs-specific configuration, including
/// automatic API key injection, base URL handling, and error mapping.
///
/// Created via [`ElevenLabsClient::new`] with a [`ClientConfig`].
///
/// # Examples
///
/// ```no_run
/// use elevenlabs_sdk::{ClientConfig, ElevenLabsClient};
///
/// # async fn example() -> elevenlabs_sdk::Result<()> {
/// let config = ClientConfig::builder("your-api-key").build();
/// let client = ElevenLabsClient::new(config)?;
/// # Ok(())
/// # }
/// ```
pub struct ElevenLabsClient {
    config: ClientConfig,
    http: hpx::Client,
    base_url: url::Url,
}

impl std::fmt::Debug for ElevenLabsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElevenLabsClient")
            .field("config", &self.config)
            .field("base_url", &self.base_url)
            .finish_non_exhaustive()
    }
}

/// Shape of error responses returned by the ElevenLabs API.
#[derive(serde::Deserialize)]
struct ApiErrorBody {
    /// Top-level detail field.
    detail: Option<ApiErrorDetail>,
}

/// Inner detail of an API error — can be a plain string or structured object.
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ApiErrorDetail {
    /// A plain error message string.
    Message(String),
    /// A structured error object with a message field.
    Structured {
        /// The error message.
        message: String,
    },
}

impl ElevenLabsClient {
    /// Creates a new [`ElevenLabsClient`] from the given configuration.
    ///
    /// Builds an internal HTTP client with default headers (including the
    /// `xi-api-key` authentication header) and the configured timeout.
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::InvalidUrl`] if `config.base_url` cannot be parsed,
    /// or [`ElevenLabsError::Transport`] if the HTTP client fails to build.
    pub fn new(config: ClientConfig) -> Result<Self> {
        let base_url = url::Url::parse(&config.base_url)?;

        let mut default_headers = HeaderMap::new();
        let mut api_key_value = HeaderValue::from_str(config.api_key.as_str()).map_err(|e| {
            ElevenLabsError::Validation(format!("invalid API key header value: {e}"))
        })?;
        api_key_value.set_sensitive(true);
        default_headers.insert(API_KEY_HEADER, api_key_value);

        let http = hpx::Client::builder()
            .default_headers(default_headers)
            .timeout(config.timeout)
            .build()
            .map_err(ElevenLabsError::Transport)?;

        Ok(Self { config, http, base_url })
    }

    /// Returns a reference to the underlying [`ClientConfig`].
    pub const fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Returns an [`AgentsService`](crate::services::AgentsService) scoped to
    /// this client.
    pub const fn agents(&self) -> crate::services::AgentsService<'_> {
        crate::services::AgentsService::new(self)
    }

    /// Returns a [`TextToSpeechService`](crate::services::TextToSpeechService)
    /// scoped to this client.
    pub const fn text_to_speech(&self) -> crate::services::TextToSpeechService<'_> {
        crate::services::TextToSpeechService::new(self)
    }

    /// Returns a [`VoicesService`](crate::services::VoicesService) scoped to
    /// this client.
    pub const fn voices(&self) -> crate::services::VoicesService<'_> {
        crate::services::VoicesService::new(self)
    }

    /// Returns a [`SpeechToSpeechService`](crate::services::SpeechToSpeechService)
    /// scoped to this client.
    pub const fn speech_to_speech(&self) -> crate::services::SpeechToSpeechService<'_> {
        crate::services::SpeechToSpeechService::new(self)
    }

    /// Returns a [`SpeechToTextService`](crate::services::SpeechToTextService)
    /// scoped to this client.
    pub const fn speech_to_text(&self) -> crate::services::SpeechToTextService<'_> {
        crate::services::SpeechToTextService::new(self)
    }

    /// Returns an [`AudioIsolationService`](crate::services::AudioIsolationService)
    /// scoped to this client.
    pub const fn audio_isolation(&self) -> crate::services::AudioIsolationService<'_> {
        crate::services::AudioIsolationService::new(self)
    }

    /// Returns an [`AudioNativeService`](crate::services::AudioNativeService)
    /// scoped to this client.
    pub const fn audio_native(&self) -> crate::services::AudioNativeService<'_> {
        crate::services::AudioNativeService::new(self)
    }

    /// Returns a [`SoundGenerationService`](crate::services::SoundGenerationService)
    /// scoped to this client.
    pub const fn sound_generation(&self) -> crate::services::SoundGenerationService<'_> {
        crate::services::SoundGenerationService::new(self)
    }

    /// Returns a [`TextToDialogueService`](crate::services::TextToDialogueService)
    /// scoped to this client.
    pub const fn text_to_dialogue(&self) -> crate::services::TextToDialogueService<'_> {
        crate::services::TextToDialogueService::new(self)
    }

    /// Returns a [`TextToVoiceService`](crate::services::TextToVoiceService)
    /// scoped to this client.
    pub const fn text_to_voice(&self) -> crate::services::TextToVoiceService<'_> {
        crate::services::TextToVoiceService::new(self)
    }

    /// Returns a [`VoiceGenerationService`](crate::services::VoiceGenerationService)
    /// scoped to this client.
    pub const fn voice_generation(&self) -> crate::services::VoiceGenerationService<'_> {
        crate::services::VoiceGenerationService::new(self)
    }

    /// Returns a [`DubbingService`](crate::services::DubbingService) scoped to
    /// this client.
    pub const fn dubbing(&self) -> crate::services::DubbingService<'_> {
        crate::services::DubbingService::new(self)
    }

    /// Returns a [`StudioService`](crate::services::StudioService) scoped to
    /// this client.
    pub const fn studio(&self) -> crate::services::StudioService<'_> {
        crate::services::StudioService::new(self)
    }

    /// Returns a [`MusicService`](crate::services::MusicService) scoped to
    /// this client.
    pub const fn music(&self) -> crate::services::MusicService<'_> {
        crate::services::MusicService::new(self)
    }

    /// Returns a [`ModelsService`](crate::services::ModelsService) scoped to
    /// this client.
    pub const fn models(&self) -> crate::services::ModelsService<'_> {
        crate::services::ModelsService::new(self)
    }

    /// Returns a [`HistoryService`](crate::services::HistoryService) scoped to
    /// this client.
    pub const fn history(&self) -> crate::services::HistoryService<'_> {
        crate::services::HistoryService::new(self)
    }

    /// Returns a [`UserService`](crate::services::UserService) scoped to
    /// this client.
    pub const fn user(&self) -> crate::services::UserService<'_> {
        crate::services::UserService::new(self)
    }

    /// Returns a [`WorkspaceService`](crate::services::WorkspaceService) scoped
    /// to this client.
    pub const fn workspace(&self) -> crate::services::WorkspaceService<'_> {
        crate::services::WorkspaceService::new(self)
    }

    /// Returns a [`ForcedAlignmentService`](crate::services::ForcedAlignmentService)
    /// scoped to this client.
    pub const fn forced_alignment(&self) -> crate::services::ForcedAlignmentService<'_> {
        crate::services::ForcedAlignmentService::new(self)
    }

    /// Returns a [`SingleUseTokenService`](crate::services::SingleUseTokenService)
    /// scoped to this client.
    pub const fn single_use_token(&self) -> crate::services::SingleUseTokenService<'_> {
        crate::services::SingleUseTokenService::new(self)
    }

    /// Returns a [`PvcVoicesService`](crate::services::PvcVoicesService) scoped
    /// to this client.
    pub const fn pvc_voices(&self) -> crate::services::PvcVoicesService<'_> {
        crate::services::PvcVoicesService::new(self)
    }

    /// Sends an HTTP request and returns the raw [`hpx::Response`].
    ///
    /// Constructs the full URL by joining `path` onto the base URL,
    /// optionally attaches a pre-serialized JSON body, and maps
    /// transport/timeout errors.
    #[tracing::instrument(
        skip(self, body),
        fields(method = %method, path = %path)
    )]
    async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<hpx::Response> {
        let url = self.base_url.join(path)?;

        let mut last_error: Option<ElevenLabsError> = None;

        for attempt in 0..=self.config.max_retries {
            let mut builder = self.http.request(method.clone(), url.as_str());
            if let Some(ref json_body) = body {
                builder = builder.json(json_body);
            }

            match builder.send().await {
                Ok(response) => {
                    let status = response.status();

                    if middleware::should_retry(status) && attempt < self.config.max_retries {
                        let retry_after = middleware::parse_retry_after(&response);
                        let delay = middleware::compute_delay(
                            attempt,
                            self.config.retry_backoff,
                            retry_after,
                        );
                        tracing::warn!(
                            attempt,
                            status = %status,
                            delay_ms = delay.as_millis() as u64,
                            "retrying request"
                        );
                        tokio::time::sleep(delay).await;
                        continue;
                    }

                    tracing::debug!(status = %status, "received API response");
                    return Ok(response);
                }
                Err(e) if e.is_timeout() && attempt < self.config.max_retries => {
                    let delay = middleware::compute_delay(attempt, self.config.retry_backoff, None);
                    tracing::warn!(
                        attempt,
                        delay_ms = delay.as_millis() as u64,
                        "request timed out, retrying"
                    );
                    tokio::time::sleep(delay).await;
                    last_error = Some(ElevenLabsError::Timeout);
                }
                Err(e) if e.is_timeout() => {
                    return Err(ElevenLabsError::Timeout);
                }
                Err(e) => {
                    return Err(ElevenLabsError::Transport(e));
                }
            }
        }

        Err(last_error.unwrap_or(ElevenLabsError::Timeout))
    }

    /// Checks an HTTP response for errors and maps them to [`ElevenLabsError`]
    /// variants.
    async fn handle_error_response(response: hpx::Response) -> Result<hpx::Response> {
        let status = response.status();

        if status.is_success() {
            return Ok(response);
        }

        // 401 Unauthorized
        if status == StatusCode::UNAUTHORIZED {
            let body = response.text().await.unwrap_or_default();
            let message = Self::extract_error_message(&body)
                .unwrap_or_else(|| "invalid or missing API key".to_owned());
            return Err(ElevenLabsError::Auth(message));
        }

        // 429 Rate Limited
        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get(hpx::header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            return Err(ElevenLabsError::RateLimited { retry_after });
        }

        // Other 4xx / 5xx
        let status_code = status.as_u16();
        let body = response.text().await.unwrap_or_default();
        let message = Self::extract_error_message(&body)
            .unwrap_or_else(|| status.canonical_reason().unwrap_or("Unknown error").to_owned());

        Err(ElevenLabsError::Api {
            status: status_code,
            message,
            body: if body.is_empty() { None } else { Some(body) },
        })
    }

    /// Attempts to extract a human-readable error message from a JSON body.
    fn extract_error_message(body: &str) -> Option<String> {
        let parsed: ApiErrorBody = serde_json::from_str(body).ok()?;
        match parsed.detail? {
            ApiErrorDetail::Message(msg) => Some(msg),
            ApiErrorDetail::Structured { message } => Some(message),
        }
    }

    // ─── Convenience request methods ───────────────────────────────────

    /// Sends a GET request and deserializes the JSON response body.
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self.request(Method::GET, path, None).await?;
        let response = Self::handle_error_response(response).await?;
        let parsed = response.json::<T>().await.map_err(ElevenLabsError::Transport)?;
        Ok(parsed)
    }

    /// Sends a GET request and returns the response as raw bytes.
    pub(crate) async fn get_bytes(&self, path: &str) -> Result<Bytes> {
        let response = self.request(Method::GET, path, None).await?;
        let response = Self::handle_error_response(response).await?;
        let bytes = response.bytes().await.map_err(ElevenLabsError::Transport)?;
        Ok(bytes)
    }

    /// Sends a POST request with a JSON body and deserializes the JSON
    /// response.
    pub(crate) async fn post<T: DeserializeOwned, B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let json_value = serde_json::to_value(body)?;
        let response = self.request(Method::POST, path, Some(json_value)).await?;
        let response = Self::handle_error_response(response).await?;
        let parsed = response.json::<T>().await.map_err(ElevenLabsError::Transport)?;
        Ok(parsed)
    }

    /// Sends a POST request with a JSON body and returns raw bytes (for
    /// audio).
    pub(crate) async fn post_bytes<B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<Bytes> {
        let json_value = serde_json::to_value(body)?;
        let response = self.request(Method::POST, path, Some(json_value)).await?;
        let response = Self::handle_error_response(response).await?;
        let bytes = response.bytes().await.map_err(ElevenLabsError::Transport)?;
        Ok(bytes)
    }

    /// Sends a POST request and returns a streaming response of byte chunks.
    ///
    /// Stream items contain [`hpx::Error`] rather than [`ElevenLabsError`] to
    /// avoid requiring additional stream-mapping dependencies. Callers should
    /// convert errors at the service layer.
    pub(crate) async fn post_stream<B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>> + use<B>> {
        let json_value = serde_json::to_value(body)?;
        let response = self.request(Method::POST, path, Some(json_value)).await?;
        let response = Self::handle_error_response(response).await?;
        Ok(response.bytes_stream())
    }

    /// Sends a DELETE request (expects no response body).
    pub(crate) async fn delete(&self, path: &str) -> Result<()> {
        let response = self.request(Method::DELETE, path, None).await?;
        let _response = Self::handle_error_response(response).await?;
        Ok(())
    }

    /// Sends a DELETE request and deserializes the JSON response body.
    pub(crate) async fn delete_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self.request(Method::DELETE, path, None).await?;
        let response = Self::handle_error_response(response).await?;
        let parsed = response.json::<T>().await.map_err(ElevenLabsError::Transport)?;
        Ok(parsed)
    }

    /// Sends a DELETE request with a JSON body and deserializes the JSON
    /// response.
    pub(crate) async fn delete_with_body<T: DeserializeOwned, B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let json_value = serde_json::to_value(body)?;
        let response = self.request(Method::DELETE, path, Some(json_value)).await?;
        let response = Self::handle_error_response(response).await?;
        let parsed = response.json::<T>().await.map_err(ElevenLabsError::Transport)?;
        Ok(parsed)
    }

    /// Sends a POST request with a raw body and custom content-type, then
    /// deserializes the JSON response.
    ///
    /// Used for multipart/form-data uploads where `hpx` does not provide a
    /// built-in multipart builder.
    pub(crate) async fn post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Vec<u8>,
        content_type: &str,
    ) -> Result<T> {
        let url = self.base_url.join(path)?;
        let response = self
            .http
            .post(url.as_str())
            .header(hpx::header::CONTENT_TYPE, content_type)
            .body(body)
            .send()
            .await
            .map_err(ElevenLabsError::Transport)?;
        let response = Self::handle_error_response(response).await?;
        let parsed = response.json::<T>().await.map_err(ElevenLabsError::Transport)?;
        Ok(parsed)
    }

    /// Sends a POST request with a raw multipart body and returns the
    /// response as raw bytes (for audio endpoints).
    ///
    /// Used for speech-to-speech endpoints that accept `multipart/form-data`
    /// and return audio bytes rather than JSON.
    pub(crate) async fn post_multipart_bytes(
        &self,
        path: &str,
        body: Vec<u8>,
        content_type: &str,
    ) -> Result<Bytes> {
        let url = self.base_url.join(path)?;
        let response = self
            .http
            .post(url.as_str())
            .header(hpx::header::CONTENT_TYPE, content_type)
            .body(body)
            .send()
            .await
            .map_err(ElevenLabsError::Transport)?;
        let response = Self::handle_error_response(response).await?;
        let bytes = response.bytes().await.map_err(ElevenLabsError::Transport)?;
        Ok(bytes)
    }

    /// Sends a POST request with a raw multipart body and returns a streaming
    /// response of byte chunks.
    ///
    /// Used for speech-to-speech streaming endpoints that accept
    /// `multipart/form-data` and return chunked audio.
    pub(crate) async fn post_multipart_stream(
        &self,
        path: &str,
        body: Vec<u8>,
        content_type: &str,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>> + use<'_>> {
        let url = self.base_url.join(path)?;
        let response = self
            .http
            .post(url.as_str())
            .header(hpx::header::CONTENT_TYPE, content_type)
            .body(body)
            .send()
            .await
            .map_err(ElevenLabsError::Transport)?;
        let response = Self::handle_error_response(response).await?;
        Ok(response.bytes_stream())
    }

    /// Sends a PATCH request with a JSON body and deserializes the JSON
    /// response.
    pub(crate) async fn patch<T: DeserializeOwned, B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let json_value = serde_json::to_value(body)?;
        let response = self.request(Method::PATCH, path, Some(json_value)).await?;
        let response = Self::handle_error_response(response).await?;
        let parsed = response.json::<T>().await.map_err(ElevenLabsError::Transport)?;
        Ok(parsed)
    }

    /// Sends a PUT request with a JSON body and deserializes the JSON
    /// response.
    pub(crate) async fn put<T: DeserializeOwned, B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let json_value = serde_json::to_value(body)?;
        let response = self.request(Method::PUT, path, Some(json_value)).await?;
        let response = Self::handle_error_response(response).await?;
        let parsed = response.json::<T>().await.map_err(ElevenLabsError::Transport)?;
        Ok(parsed)
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::*;
    use crate::config::ClientConfig;

    #[derive(Debug, serde::Deserialize, PartialEq, Eq)]
    struct TestResponse {
        message: String,
        count: u32,
    }

    #[tokio::test]
    async fn get_returns_deserialized_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/voices"))
            .and(header("xi-api-key", "test-key-123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "message": "success",
                "count": 42
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key-123").base_url(mock_server.uri()).build();

        let client = ElevenLabsClient::new(config).unwrap();
        let result: TestResponse = client.get("/v1/voices").await.unwrap();

        assert_eq!(result, TestResponse { message: "success".to_owned(), count: 42 });
    }

    #[tokio::test]
    async fn get_handles_401_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/voices"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "detail": {
                    "message": "Invalid API key"
                }
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("bad-key").base_url(mock_server.uri()).build();

        let client = ElevenLabsClient::new(config).unwrap();
        let result: Result<TestResponse> = client.get("/v1/voices").await;

        match result {
            Err(ElevenLabsError::Auth(msg)) => {
                assert_eq!(msg, "Invalid API key");
            }
            other => panic!("expected Auth error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn get_handles_429_rate_limited() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/voices"))
            .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "30"))
            .mount(&mock_server)
            .await;

        let config =
            ClientConfig::builder("test-key").base_url(mock_server.uri()).max_retries(0).build();

        let client = ElevenLabsClient::new(config).unwrap();
        let result: Result<TestResponse> = client.get("/v1/voices").await;

        match result {
            Err(ElevenLabsError::RateLimited { retry_after }) => {
                assert_eq!(retry_after, Some(30));
            }
            other => panic!("expected RateLimited error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn get_handles_500_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/voices"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "detail": "Internal server error"
            })))
            .mount(&mock_server)
            .await;

        let config =
            ClientConfig::builder("test-key").base_url(mock_server.uri()).max_retries(0).build();

        let client = ElevenLabsClient::new(config).unwrap();
        let result: Result<TestResponse> = client.get("/v1/voices").await;

        match result {
            Err(ElevenLabsError::Api { status, message, body }) => {
                assert_eq!(status, 500);
                assert_eq!(message, "Internal server error");
                assert!(body.is_some());
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn delete_succeeds_on_200() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/voices/abc123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();

        let client = ElevenLabsClient::new(config).unwrap();
        let result = client.delete("/v1/voices/abc123").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn retry_on_429_then_succeeds() {
        use std::time::Duration;

        let mock_server = MockServer::start().await;

        // Mount success mock first (checked last due to LIFO ordering)
        Mock::given(method("GET"))
            .and(path("/v1/test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "message": "ok",
                "count": 1
            })))
            .mount(&mock_server)
            .await;

        // Mount 429 mock second (checked first, exhausted after 2 responses)
        Mock::given(method("GET"))
            .and(path("/v1/test"))
            .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "0"))
            .up_to_n_times(2)
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key")
            .base_url(mock_server.uri())
            .max_retries(3)
            .retry_backoff(Duration::from_millis(1))
            .build();

        let client = ElevenLabsClient::new(config).unwrap();
        let result: TestResponse = client.get("/v1/test").await.unwrap();

        assert_eq!(result.message, "ok");
        assert_eq!(result.count, 1);
    }

    #[tokio::test]
    async fn retry_on_500_then_succeeds() {
        use std::time::Duration;

        let mock_server = MockServer::start().await;

        // Mount success mock first (checked last)
        Mock::given(method("GET"))
            .and(path("/v1/test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "message": "recovered",
                "count": 7
            })))
            .mount(&mock_server)
            .await;

        // Mount 500 mock second (checked first, exhausted after 2 responses)
        Mock::given(method("GET"))
            .and(path("/v1/test"))
            .respond_with(ResponseTemplate::new(500))
            .up_to_n_times(2)
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key")
            .base_url(mock_server.uri())
            .max_retries(3)
            .retry_backoff(Duration::from_millis(1))
            .build();

        let client = ElevenLabsClient::new(config).unwrap();
        let result: TestResponse = client.get("/v1/test").await.unwrap();

        assert_eq!(result.message, "recovered");
        assert_eq!(result.count, 7);
    }

    #[tokio::test]
    async fn retry_exhausted_returns_error() {
        use std::time::Duration;

        let mock_server = MockServer::start().await;

        // Always returns 500 — retries will be exhausted
        Mock::given(method("GET"))
            .and(path("/v1/test"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key")
            .base_url(mock_server.uri())
            .max_retries(2)
            .retry_backoff(Duration::from_millis(1))
            .build();

        let client = ElevenLabsClient::new(config).unwrap();
        let result: Result<TestResponse> = client.get("/v1/test").await;

        match result {
            Err(ElevenLabsError::Api { status, .. }) => {
                assert_eq!(status, 500);
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn no_retry_on_non_retryable_status() {
        let mock_server = MockServer::start().await;

        // 400 is not retryable — should fail immediately
        Mock::given(method("GET"))
            .and(path("/v1/test"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "detail": "Bad request"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let config =
            ClientConfig::builder("test-key").base_url(mock_server.uri()).max_retries(3).build();

        let client = ElevenLabsClient::new(config).unwrap();
        let result: Result<TestResponse> = client.get("/v1/test").await;

        match result {
            Err(ElevenLabsError::Api { status, .. }) => {
                assert_eq!(status, 400);
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn post_returns_deserialized_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/text-to-speech/voice123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "message": "created",
                "count": 1
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();

        let client = ElevenLabsClient::new(config).unwrap();

        #[derive(serde::Serialize)]
        struct Req {
            text: String,
        }

        let body = Req { text: "Hello world".to_owned() };
        let result: TestResponse = client.post("/v1/text-to-speech/voice123", &body).await.unwrap();

        assert_eq!(result, TestResponse { message: "created".to_owned(), count: 1 });
    }
}
