//! User service providing access to user profile and usage endpoints.
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`get`](UserService::get) | `GET /v1/user` | Get user profile |
//! | [`get_subscription`](UserService::get_subscription) | `GET /v1/user/subscription` | Get extended subscription info |
//! | [`get_character_usage`](UserService::get_character_usage) | `GET /v1/usage/character-stats` | Get character usage stats |
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
//! let user = client.user().get().await?;
//! println!("User ID: {}", user.user_id);
//!
//! let sub = client.user().get_subscription().await?;
//! println!("Tier: {}", sub.tier);
//! # Ok(())
//! # }
//! ```

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{ExtendedSubscriptionResponse, UsageCharactersResponse, UserResponse},
};

/// User service providing typed access to user profile and usage endpoints.
///
/// Obtained via [`ElevenLabsClient::user`].
#[derive(Debug)]
pub struct UserService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> UserService<'a> {
    /// Creates a new `UserService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    /// Gets the current user's profile.
    ///
    /// Calls `GET /v1/user`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get(&self) -> Result<UserResponse> {
        self.client.get("/v1/user").await
    }

    /// Gets the current user's extended subscription details.
    ///
    /// Calls `GET /v1/user/subscription`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_subscription(&self) -> Result<ExtendedSubscriptionResponse> {
        self.client.get("/v1/user/subscription").await
    }

    /// Gets character usage statistics for a time range.
    ///
    /// Calls `GET /v1/usage/character-stats`.
    ///
    /// # Arguments
    ///
    /// * `start_unix` — Start of the time range (Unix timestamp, required).
    /// * `end_unix` — End of the time range (Unix timestamp, required).
    /// * `include_workspace_metrics` — Whether to include workspace-level metrics.
    /// * `breakdown_type` — Type of breakdown (e.g. `"voice"`, `"user"`).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the response cannot be
    /// deserialized.
    pub async fn get_character_usage(
        &self,
        start_unix: i64,
        end_unix: i64,
        include_workspace_metrics: Option<bool>,
        breakdown_type: Option<&str>,
    ) -> Result<UsageCharactersResponse> {
        let mut path =
            format!("/v1/usage/character-stats?start_unix={start_unix}&end_unix={end_unix}");
        if include_workspace_metrics == Some(true) {
            path.push_str("&include_workspace_metrics=true");
        }
        if let Some(bt) = breakdown_type {
            path.push_str(&format!("&breakdown_type={bt}"));
        }
        self.client.get(&path).await
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

    use crate::{ElevenLabsClient, config::ClientConfig};

    #[tokio::test]
    async fn get_returns_user() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/user"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "user_id": "user123",
                "subscription": {
                    "tier": "creator",
                    "character_count": 5000,
                    "character_limit": 100000,
                    "can_extend_character_limit": true,
                    "allowed_to_extend_character_limit": true,
                    "voice_slots_used": 3,
                    "professional_voice_slots_used": 0,
                    "voice_limit": 30,
                    "voice_add_edit_counter": 5,
                    "professional_voice_limit": 1,
                    "can_extend_voice_limit": true,
                    "can_use_instant_voice_cloning": true,
                    "can_use_professional_voice_cloning": true,
                    "status": "active"
                },
                "is_new_user": false,
                "can_use_delayed_payment_methods": false,
                "is_onboarding_completed": true,
                "is_onboarding_checklist_completed": true,
                "created_at": 1700000000
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let user = client.user().get().await.unwrap();
        assert_eq!(user.user_id, "user123");
        assert!(!user.is_new_user);
    }

    #[tokio::test]
    async fn get_subscription_returns_details() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/user/subscription"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "tier": "creator",
                "character_count": 5000,
                "character_limit": 100000,
                "can_extend_character_limit": true,
                "allowed_to_extend_character_limit": true,
                "voice_slots_used": 3,
                "professional_voice_slots_used": 0,
                "voice_limit": 30,
                "voice_add_edit_counter": 5,
                "professional_voice_limit": 1,
                "can_extend_voice_limit": true,
                "can_use_instant_voice_cloning": true,
                "can_use_professional_voice_cloning": true,
                "status": "active",
                "has_open_invoices": false
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let sub = client.user().get_subscription().await.unwrap();
        assert_eq!(sub.tier, "creator");
        assert_eq!(sub.character_count, 5000);
    }

    #[tokio::test]
    async fn get_character_usage_returns_stats() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/usage/character-stats"))
            .and(query_param("start_unix", "1700000000"))
            .and(query_param("end_unix", "1700100000"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "time": [1700000000, 1700050000, 1700100000],
                "usage": {
                    "tts": [100, 200, 150]
                }
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let usage = client
            .user()
            .get_character_usage(1_700_000_000, 1_700_100_000, None, None)
            .await
            .unwrap();
        assert_eq!(usage.time.len(), 3);
    }
}
