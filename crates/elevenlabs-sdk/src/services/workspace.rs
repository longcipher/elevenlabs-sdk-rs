//! Workspace service providing access to workspace management endpoints.
//!
//! Covers groups, invites, members, service accounts, API keys, webhooks,
//! and resource sharing.
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
//! let accounts = client.workspace().get_service_accounts().await?;
//! println!("Found {} service accounts", accounts.service_accounts.len());
//! # Ok(())
//! # }
//! ```

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{
        AddGroupMemberRequest, CreateServiceAccountApiKeyRequest, CreateWorkspaceWebhookRequest,
        DeleteInviteRequest, EditServiceAccountApiKeyRequest, InviteBulkRequest,
        InviteWorkspaceMemberRequest, RemoveGroupMemberRequest, ResourceMetadataResponse,
        SearchGroupsResponse, ShareWorkspaceResourceRequest, UnshareWorkspaceResourceRequest,
        UpdateWorkspaceMemberRequest, UpdateWorkspaceWebhookRequest, WorkspaceApiKeyList,
        WorkspaceCreateApiKeyResponse, WorkspaceCreateWebhookResponse, WorkspaceServiceAccountList,
        WorkspaceStatusResponse, WorkspaceWebhookList,
    },
};

/// Workspace service providing typed access to workspace management endpoints.
///
/// Obtained via [`ElevenLabsClient::workspace`].
#[derive(Debug)]
pub struct WorkspaceService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> WorkspaceService<'a> {
    /// Creates a new `WorkspaceService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    // ── Service Accounts ──────────────────────────────────────────────

    /// Lists all workspace service accounts.
    ///
    /// Calls `GET /v1/service-accounts`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get_service_accounts(&self) -> Result<WorkspaceServiceAccountList> {
        self.client.get("/v1/service-accounts").await
    }

    /// Lists API keys for a service account.
    ///
    /// Calls `GET /v1/service-accounts/{service_account_user_id}/api-keys`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get_service_account_api_keys(
        &self,
        service_account_user_id: &str,
    ) -> Result<WorkspaceApiKeyList> {
        let path = format!("/v1/service-accounts/{service_account_user_id}/api-keys");
        self.client.get(&path).await
    }

    /// Creates a new API key for a service account.
    ///
    /// Calls `POST /v1/service-accounts/{service_account_user_id}/api-keys`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn create_service_account_api_key(
        &self,
        service_account_user_id: &str,
        request: &CreateServiceAccountApiKeyRequest,
    ) -> Result<WorkspaceCreateApiKeyResponse> {
        let path = format!("/v1/service-accounts/{service_account_user_id}/api-keys");
        self.client.post(&path, request).await
    }

    /// Edits a service account API key.
    ///
    /// Calls `PATCH /v1/service-accounts/{service_account_user_id}/api-keys/{api_key_id}`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn edit_service_account_api_key(
        &self,
        service_account_user_id: &str,
        api_key_id: &str,
        request: &EditServiceAccountApiKeyRequest,
    ) -> Result<WorkspaceStatusResponse> {
        let path = format!("/v1/service-accounts/{service_account_user_id}/api-keys/{api_key_id}");
        self.client.patch(&path, request).await
    }

    /// Deletes a service account API key.
    ///
    /// Calls `DELETE /v1/service-accounts/{service_account_user_id}/api-keys/{api_key_id}`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete_service_account_api_key(
        &self,
        service_account_user_id: &str,
        api_key_id: &str,
    ) -> Result<WorkspaceStatusResponse> {
        let path = format!("/v1/service-accounts/{service_account_user_id}/api-keys/{api_key_id}");
        self.client.delete_json(&path).await
    }

    // ── Groups ────────────────────────────────────────────────────────

    /// Searches workspace groups by name.
    ///
    /// Calls `GET /v1/workspace/groups/search`.
    ///
    /// # Arguments
    ///
    /// * `name` — Group name to search for (required).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn search_groups(&self, name: &str) -> Result<SearchGroupsResponse> {
        let path = format!("/v1/workspace/groups/search?name={name}");
        self.client.get(&path).await
    }

    /// Adds a member to a workspace group.
    ///
    /// Calls `POST /v1/workspace/groups/{group_id}/members`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn add_group_member(
        &self,
        group_id: &str,
        request: &AddGroupMemberRequest,
    ) -> Result<WorkspaceStatusResponse> {
        let path = format!("/v1/workspace/groups/{group_id}/members");
        self.client.post(&path, request).await
    }

    /// Removes a member from a workspace group.
    ///
    /// Calls `POST /v1/workspace/groups/{group_id}/members/remove`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn remove_group_member(
        &self,
        group_id: &str,
        request: &RemoveGroupMemberRequest,
    ) -> Result<WorkspaceStatusResponse> {
        let path = format!("/v1/workspace/groups/{group_id}/members/remove");
        self.client.post(&path, request).await
    }

    // ── Invites ───────────────────────────────────────────────────────

    /// Invites a user to the workspace.
    ///
    /// Calls `POST /v1/workspace/invites/add`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn invite_user(
        &self,
        request: &InviteWorkspaceMemberRequest,
    ) -> Result<WorkspaceStatusResponse> {
        self.client.post("/v1/workspace/invites/add", request).await
    }

    /// Invites multiple users to the workspace.
    ///
    /// Calls `POST /v1/workspace/invites/add-bulk`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn invite_users_bulk(
        &self,
        request: &InviteBulkRequest,
    ) -> Result<WorkspaceStatusResponse> {
        self.client.post("/v1/workspace/invites/add-bulk", request).await
    }

    /// Deletes an existing workspace invitation.
    ///
    /// Calls `DELETE /v1/workspace/invites`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete_invite(
        &self,
        request: &DeleteInviteRequest,
    ) -> Result<WorkspaceStatusResponse> {
        self.client.delete_with_body("/v1/workspace/invites", request).await
    }

    // ── Members ───────────────────────────────────────────────────────

    /// Updates a workspace member.
    ///
    /// Calls `POST /v1/workspace/members`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn update_member(
        &self,
        request: &UpdateWorkspaceMemberRequest,
    ) -> Result<WorkspaceStatusResponse> {
        self.client.post("/v1/workspace/members", request).await
    }

    // ── Resources ─────────────────────────────────────────────────────

    /// Gets metadata for a workspace resource.
    ///
    /// Calls `GET /v1/workspace/resources/{resource_id}`.
    ///
    /// # Arguments
    ///
    /// * `resource_id` — ID of the resource.
    /// * `resource_type` — Type of the resource (required query parameter).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get_resource_metadata(
        &self,
        resource_id: &str,
        resource_type: &str,
    ) -> Result<ResourceMetadataResponse> {
        let path = format!("/v1/workspace/resources/{resource_id}?resource_type={resource_type}");
        self.client.get(&path).await
    }

    /// Shares a workspace resource.
    ///
    /// Calls `POST /v1/workspace/resources/{resource_id}/share`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn share_resource(
        &self,
        resource_id: &str,
        request: &ShareWorkspaceResourceRequest,
    ) -> Result<WorkspaceStatusResponse> {
        let path = format!("/v1/workspace/resources/{resource_id}/share");
        self.client.post(&path, request).await
    }

    /// Unshares a workspace resource.
    ///
    /// Calls `POST /v1/workspace/resources/{resource_id}/unshare`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn unshare_resource(
        &self,
        resource_id: &str,
        request: &UnshareWorkspaceResourceRequest,
    ) -> Result<WorkspaceStatusResponse> {
        let path = format!("/v1/workspace/resources/{resource_id}/unshare");
        self.client.post(&path, request).await
    }

    // ── Webhooks ──────────────────────────────────────────────────────

    /// Lists all workspace webhooks.
    ///
    /// Calls `GET /v1/workspace/webhooks`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get_webhooks(&self) -> Result<WorkspaceWebhookList> {
        self.client.get("/v1/workspace/webhooks").await
    }

    /// Creates a new workspace webhook.
    ///
    /// Calls `POST /v1/workspace/webhooks`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn create_webhook(
        &self,
        request: &CreateWorkspaceWebhookRequest,
    ) -> Result<WorkspaceCreateWebhookResponse> {
        self.client.post("/v1/workspace/webhooks", request).await
    }

    /// Updates an existing workspace webhook.
    ///
    /// Calls `PATCH /v1/workspace/webhooks/{webhook_id}`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn edit_webhook(
        &self,
        webhook_id: &str,
        request: &UpdateWorkspaceWebhookRequest,
    ) -> Result<WorkspaceStatusResponse> {
        let path = format!("/v1/workspace/webhooks/{webhook_id}");
        self.client.patch(&path, request).await
    }

    /// Deletes a workspace webhook.
    ///
    /// Calls `DELETE /v1/workspace/webhooks/{webhook_id}`.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete_webhook(&self, webhook_id: &str) -> Result<WorkspaceStatusResponse> {
        let path = format!("/v1/workspace/webhooks/{webhook_id}");
        self.client.delete_json(&path).await
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

    use crate::{
        ElevenLabsClient,
        config::ClientConfig,
        types::{
            AddGroupMemberRequest, CreateWorkspaceWebhookRequest, DeleteInviteRequest,
            InviteWorkspaceMemberRequest, UpdateWorkspaceMemberRequest,
        },
    };

    #[tokio::test]
    async fn get_service_accounts_returns_list() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/service-accounts"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "service-accounts": [
                    {
                        "service_account_user_id": "sa1",
                        "name": "Bot",
                        "api-keys": []
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.workspace().get_service_accounts().await.unwrap();
        assert_eq!(result.service_accounts.len(), 1);
        assert_eq!(result.service_accounts[0].name, "Bot");
    }

    #[tokio::test]
    async fn search_groups_returns_results() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/workspace/groups/search"))
            .and(query_param("name", "devs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "name": "Developers",
                    "id": "grp1",
                    "members_emails": ["a@b.com"]
                }
            ])))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.workspace().search_groups("devs").await.unwrap();
        assert_eq!(result.0.len(), 1);
        assert_eq!(result.0[0].name, "Developers");
    }

    #[tokio::test]
    async fn invite_user_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/workspace/invites/add"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let req = InviteWorkspaceMemberRequest {
            email: "test@example.com".into(),
            workspace_permission: None,
            seat_type: None,
            group_ids: None,
        };
        let result = client.workspace().invite_user(&req).await.unwrap();
        assert_eq!(result.status, "ok");
    }

    #[tokio::test]
    async fn delete_invite_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/workspace/invites"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let req = DeleteInviteRequest { email: "test@example.com".into() };
        let result = client.workspace().delete_invite(&req).await.unwrap();
        assert_eq!(result.status, "ok");
    }

    #[tokio::test]
    async fn update_member_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/workspace/members"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let req = UpdateWorkspaceMemberRequest {
            email: "member@example.com".into(),
            is_locked: Some(false),
            workspace_role: None,
            workspace_seat_type: None,
        };
        let result = client.workspace().update_member(&req).await.unwrap();
        assert_eq!(result.status, "ok");
    }

    #[tokio::test]
    async fn add_group_member_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/workspace/groups/grp1/members"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let req = AddGroupMemberRequest { email: "user@example.com".into() };
        let result = client.workspace().add_group_member("grp1", &req).await.unwrap();
        assert_eq!(result.status, "ok");
    }

    #[tokio::test]
    async fn get_webhooks_returns_list() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/workspace/webhooks"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "webhooks": [
                    {
                        "name": "My Webhook",
                        "webhook_id": "wh1",
                        "webhook_url": "https://example.com/cb",
                        "is_disabled": false,
                        "is_auto_disabled": false,
                        "created_at_unix": 1700000000,
                        "auth_type": "hmac"
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.workspace().get_webhooks().await.unwrap();
        assert_eq!(result.webhooks.len(), 1);
        assert_eq!(result.webhooks[0].name, "My Webhook");
    }

    #[tokio::test]
    async fn create_webhook_returns_id() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/workspace/webhooks"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "webhook_id": "wh_new",
                "webhook_secret": "secret123"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let req = CreateWorkspaceWebhookRequest {
            auth_type: "hmac".into(),
            name: "New Webhook".into(),
            webhook_url: "https://example.com/new".into(),
        };
        let result = client.workspace().create_webhook(&req).await.unwrap();
        assert_eq!(result.webhook_id, "wh_new");
    }

    #[tokio::test]
    async fn delete_webhook_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/workspace/webhooks/wh1"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result = client.workspace().delete_webhook("wh1").await.unwrap();
        assert_eq!(result.status, "ok");
    }

    #[tokio::test]
    async fn delete_service_account_api_key_returns_ok() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/v1/service-accounts/sa1/api-keys/key1"))
            .and(header("xi-api-key", "test-key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("test-key").base_url(mock_server.uri()).build();
        let client = ElevenLabsClient::new(config).unwrap();

        let result =
            client.workspace().delete_service_account_api_key("sa1", "key1").await.unwrap();
        assert_eq!(result.status, "ok");
    }
}
