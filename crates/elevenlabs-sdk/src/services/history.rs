//! History service providing access to speech history endpoints.
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`list`](HistoryService::list) | `GET /v1/history` | List speech history items |
//! | [`get`](HistoryService::get) | `GET /v1/history/{history_item_id}` | Get a single history item |
//! | [`get_audio`](HistoryService::get_audio) | `GET /v1/history/{history_item_id}/audio` | Download audio |
//! | [`delete`](HistoryService::delete) | `DELETE /v1/history/{history_item_id}` | Delete a history item |
//! | [`download`](HistoryService::download) | `POST /v1/history/download` | Download multiple items |
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
//! let history = client.history().list(None, None, None).await?;
//! println!("Found {} history items", history.history.len());
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{
        DeleteHistoryItemResponse, DownloadHistoryItemsRequest, GetSpeechHistoryResponse,
        SpeechHistoryItem,
    },
};

/// History service providing typed access to speech history endpoints.
///
/// Obtained via [`ElevenLabsClient::history`].
#[derive(Debug)]
pub struct HistoryService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> HistoryService<'a> {
    /// Creates a new `HistoryService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Lists speech history items with optional filters.
    ///
    /// Calls `GET /v1/history`.
    ///
    /// # Arguments
    ///
    /// * `page_size` — Maximum items per page.
    /// * `start_after_history_item_id` — Cursor for pagination.
    /// * `voice_id` — Filter by voice ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn list(
        &self,
        page_size: Option<u32>,
        start_after_history_item_id: Option<&str>,
        voice_id: Option<&str>,
    ) -> Result<GetSpeechHistoryResponse> {
        let mut path = "/v1/history".to_owned();
        let mut sep = '?';
        if let Some(ps) = page_size {
            path.push_str(&format!("{sep}page_size={ps}"));
            sep = '&';
        }
        if let Some(after) = start_after_history_item_id {
            path.push_str(&format!("{sep}start_after_history_item_id={after}"));
            sep = '&';
        }
        if let Some(vid) = voice_id {
            path.push_str(&format!("{sep}voice_id={vid}"));
        }
        self.client.get(&path).await
    }

    /// Gets a single speech history item by its ID.
    ///
    /// Calls `GET /v1/history/{history_item_id}`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get(&self, history_item_id: &str) -> Result<SpeechHistoryItem> {
        let path = format!("/v1/history/{history_item_id}");
        self.client.get(&path).await
    }

    /// Downloads the audio for a single history item.
    ///
    /// Calls `GET /v1/history/{history_item_id}/audio`.
    ///
    /// Returns the raw audio bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get_audio(&self, history_item_id: &str) -> Result<Bytes> {
        let path = format!("/v1/history/{history_item_id}/audio");
        self.client.get_bytes(&path).await
    }

    /// Deletes a speech history item.
    ///
    /// Calls `DELETE /v1/history/{history_item_id}`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete(&self, history_item_id: &str) -> Result<DeleteHistoryItemResponse> {
        let path = format!("/v1/history/{history_item_id}");
        self.client.delete_json(&path).await
    }

    /// Downloads multiple history items as audio.
    ///
    /// Calls `POST /v1/history/download` with a JSON body containing
    /// the item IDs to download. Returns raw bytes (typically a zip archive
    /// when multiple items are requested).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn download(&self, request: &DownloadHistoryItemsRequest) -> Result<Bytes> {
        self.client.post_bytes("/v1/history/download", request).await
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path, query_param},
    };

    use crate::{ElevenLabsClient, config::ClientConfig, types::DownloadHistoryItemsRequest};

    #[tokio::test]
    async fn list_returns_history() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/history"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "history": [
                    {
                        "history_item_id": "item1",
                        "date_unix": 1714650306,
                        "character_count_change_from": 100,
                        "character_count_change_to": 150,
                        "content_type": "audio/mpeg",
                        "state": "created"
                    }
                ],
                "last_history_item_id": "item1",
                "has_more": false
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.history().list(None, None, None).await.unwrap();
        assert_eq!(result.history.len(), 1);
        assert!(!result.has_more);
    }

    #[tokio::test]
    async fn list_with_page_size() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/history"))
            .and(query_param("page_size", "5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "history": [],
                "has_more": false
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.history().list(Some(5), None, None).await.unwrap();
        assert!(result.history.is_empty());
    }

    #[tokio::test]
    async fn get_returns_item() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/history/item123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "history_item_id": "item123",
                "date_unix": 1714650306,
                "character_count_change_from": 100,
                "character_count_change_to": 150,
                "content_type": "audio/mpeg",
                "state": "created"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let item = client.history().get("item123").await.unwrap();
        assert_eq!(item.history_item_id, "item123");
    }

    #[tokio::test]
    async fn get_audio_returns_bytes() {
        let mock_server = MockServer::start().await;
        let audio_data = b"fake-audio-data";

        Mock::given(method("GET"))
            .and(path("/v1/history/item123/audio"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(audio_data.as_slice(), "audio/mpeg"),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let bytes = client.history().get_audio("item123").await.unwrap();
        assert_eq!(bytes.as_ref(), audio_data);
    }

    #[tokio::test]
    async fn delete_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/history/item123"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.history().delete("item123").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    #[tokio::test]
    async fn download_returns_bytes() {
        let mock_server = MockServer::start().await;
        let zip_data = b"PK\x03\x04fake-zip";

        Mock::given(method("POST"))
            .and(path("/v1/history/download"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(zip_data.as_slice(), "application/zip"),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let req = DownloadHistoryItemsRequest {
            history_item_ids: vec!["id1".into(), "id2".into()],
            output_format: None,
        };
        let bytes = client.history().download(&req).await.unwrap();
        assert_eq!(bytes.as_ref(), zip_data);
    }
}
