//! Types for the ElevenLabs Workspace endpoints.
//!
//! Covers workspace management: groups, invites, members, service accounts,
//! API keys, webhooks, and resource sharing.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Permission that can be granted to a workspace group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceGroupPermission {
    /// Text-to-speech capability.
    TextToSpeech,
    /// Speech-to-speech capability.
    SpeechToSpeech,
    /// Speech-to-text capability.
    SpeechToText,
    /// Voice lab access.
    VoiceLab,
    /// Sound effects generation.
    SoundEffects,
    /// Projects access.
    Projects,
    /// Voiceover studio access.
    VoiceoverStudio,
    /// Dubbing capability.
    Dubbing,
    /// Audio native access.
    AudioNative,
    /// Conversational AI access.
    ConversationalAi,
    /// Voice isolator access.
    VoiceIsolator,
    /// AI speech classifier access.
    AiSpeechClassifier,
    /// Add voice from voice library.
    AddVoiceFromVoiceLibrary,
    /// Create instant voice clone.
    CreateInstantVoiceClone,
    /// Create professional voice clone.
    CreateProfessionalVoiceClone,
    /// Create user API key.
    CreateUserApiKey,
    /// Publish studio project.
    PublishStudioProject,
    /// Music generation.
    Music,
    /// Share voice externally.
    ShareVoiceExternally,
    /// Publish voice to voice library.
    PublishVoiceToVoiceLibrary,
    /// View fiat balance.
    ViewFiatBalance,
    /// Full read access to workspace analytics.
    WorkspaceAnalyticsFullRead,
    /// Manage service accounts.
    ServiceAccountsManage,
    /// Manage webhooks.
    WebhooksManage,
    /// Manage group members.
    GroupMembersManage,
    /// Invite workspace members.
    WorkspaceMembersInvite,
    /// Remove workspace members.
    WorkspaceMembersRemove,
    /// Accept terms of service.
    TermsOfServiceAccept,
}

/// Type of workspace resource that can be shared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceResourceType {
    /// A voice resource.
    Voice,
    /// A voice collection resource.
    VoiceCollection,
    /// A pronunciation dictionary resource.
    PronunciationDictionary,
    /// A dubbing resource.
    Dubbing,
    /// A project resource.
    Project,
    /// ConvAI agents.
    ConvaiAgents,
    /// ConvAI knowledge base documents.
    ConvaiKnowledgeBaseDocuments,
    /// ConvAI tools.
    ConvaiTools,
    /// ConvAI settings.
    ConvaiSettings,
    /// ConvAI secrets.
    ConvaiSecrets,
    /// Workspace auth connections.
    WorkspaceAuthConnections,
    /// ConvAI phone numbers.
    ConvaiPhoneNumbers,
    /// ConvAI MCP servers.
    ConvaiMcpServers,
    /// ConvAI API integration connections.
    ConvaiApiIntegrationConnections,
    /// ConvAI API integration trigger connections.
    ConvaiApiIntegrationTriggerConnections,
    /// ConvAI batch calls.
    ConvaiBatchCalls,
    /// ConvAI agent response tests.
    ConvaiAgentResponseTests,
    /// ConvAI test suite invocations.
    ConvaiTestSuiteInvocations,
    /// ConvAI crawl jobs.
    ConvaiCrawlJobs,
    /// ConvAI crawl tasks.
    ConvaiCrawlTasks,
    /// ConvAI WhatsApp accounts.
    ConvaiWhatsappAccounts,
    /// ConvAI agent versions.
    ConvaiAgentVersions,
    /// ConvAI agent branches.
    ConvaiAgentBranches,
    /// ConvAI agent versions deployments.
    ConvaiAgentVersionsDeployments,
    /// Dashboard.
    Dashboard,
    /// Dashboard configuration.
    DashboardConfiguration,
    /// ConvAI agent drafts.
    ConvaiAgentDrafts,
}

/// Authentication method type for webhooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookAuthMethod {
    /// HMAC-based authentication.
    Hmac,
    /// OAuth2 authentication.
    Oauth2,
    /// Mutual TLS authentication.
    Mtls,
}

/// Product/service type that triggers webhooks.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WebhookUsageType {
    /// ConvAI agent-level settings.
    #[serde(rename = "ConvAI Agent Settings")]
    ConvAiAgentSettings,
    /// ConvAI global settings.
    #[serde(rename = "ConvAI Settings")]
    ConvAiSettings,
    /// Voice library removal notices.
    #[serde(rename = "Voice Library Removal Notices")]
    VoiceLibraryRemovalNotices,
    /// Speech-to-text transcription.
    #[serde(rename = "Speech to Text")]
    SpeechToText,
}

/// Permission level for shared resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionLevel {
    /// Administrator access.
    Admin,
    /// Editor access.
    Editor,
    /// Read-only access.
    Viewer,
}

// ---------------------------------------------------------------------------
// Groups
// ---------------------------------------------------------------------------

/// A workspace group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceGroup {
    /// Group name.
    pub name: String,
    /// Group unique identifier.
    pub id: String,
    /// User IDs of group members.
    pub members: Vec<String>,
    /// Permissions granted to the group.
    pub permissions: serde_json::Value,
    /// Usage limit for the group.
    #[serde(default)]
    pub group_usage_limit: Option<serde_json::Value>,
    /// Character count for the group.
    #[serde(default)]
    pub character_count: Option<serde_json::Value>,
}

/// A workspace group identified by name (search result).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceGroupByName {
    /// Group name.
    pub name: String,
    /// Group unique identifier.
    pub id: String,
    /// Emails of the group members.
    pub members_emails: Vec<String>,
}

// ---------------------------------------------------------------------------
// Service Accounts & API Keys
// ---------------------------------------------------------------------------

/// A workspace API key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceApiKey {
    /// Display name of the API key.
    pub name: String,
    /// Hint (masked key preview).
    pub hint: String,
    /// Unique key identifier.
    pub key_id: String,
    /// Service account user ID this key belongs to.
    pub service_account_user_id: String,
    /// Hashed API key value.
    pub hashed_xi_api_key: String,
    /// Unix timestamp of creation.
    #[serde(default)]
    pub created_at_unix: Option<i64>,
    /// Whether the key is disabled.
    #[serde(default)]
    pub is_disabled: Option<bool>,
    /// Permissions associated with this key.
    #[serde(default)]
    pub permissions: Option<serde_json::Value>,
    /// Character limit for this key.
    #[serde(default)]
    pub character_limit: Option<serde_json::Value>,
    /// Character count used by this key.
    #[serde(default)]
    pub character_count: Option<serde_json::Value>,
}

/// A workspace service account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceServiceAccount {
    /// Service account user identifier.
    pub service_account_user_id: String,
    /// Service account display name.
    pub name: String,
    /// Unix timestamp of creation.
    #[serde(default)]
    pub created_at_unix: Option<i64>,
    /// API keys associated with this service account.
    #[serde(rename = "api-keys")]
    pub api_keys: Vec<WorkspaceApiKey>,
    /// Default sharing groups configuration.
    #[serde(default)]
    pub default_sharing_groups: Option<Vec<serde_json::Value>>,
}

/// Response from listing service accounts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceServiceAccountList {
    /// List of service accounts.
    #[serde(rename = "service-accounts")]
    pub service_accounts: Vec<WorkspaceServiceAccount>,
}

/// Response from listing API keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceApiKeyList {
    /// List of API keys.
    #[serde(rename = "api-keys")]
    pub api_keys: Vec<WorkspaceApiKey>,
}

/// Response from creating a new API key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCreateApiKeyResponse {
    /// The newly created API key value.
    #[serde(rename = "xi-api-key")]
    pub xi_api_key: String,
    /// Key identifier.
    pub key_id: String,
}

// ---------------------------------------------------------------------------
// Webhooks
// ---------------------------------------------------------------------------

/// A workspace webhook configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWebhook {
    /// Display name for this webhook.
    pub name: String,
    /// Unique webhook identifier.
    pub webhook_id: String,
    /// HTTPS callback URL.
    pub webhook_url: String,
    /// Whether the webhook has been manually disabled.
    pub is_disabled: bool,
    /// Whether the webhook has been auto-disabled due to failures.
    pub is_auto_disabled: bool,
    /// Unix timestamp of creation.
    pub created_at_unix: i64,
    /// Authentication method type.
    pub auth_type: WebhookAuthMethod,
    /// Products configured to trigger this webhook.
    #[serde(default)]
    pub usage: Option<Vec<WorkspaceWebhookUsage>>,
    /// Most recent failure HTTP error code.
    #[serde(default)]
    pub most_recent_failure_error_code: Option<serde_json::Value>,
    /// Unix timestamp of the most recent failure.
    #[serde(default)]
    pub most_recent_failure_timestamp: Option<serde_json::Value>,
}

/// Usage configuration for a workspace webhook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWebhookUsage {
    /// The usage type (product that triggers this webhook).
    pub usage_type: WebhookUsageType,
}

/// Response from listing workspace webhooks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWebhookList {
    /// List of configured webhooks.
    pub webhooks: Vec<WorkspaceWebhook>,
}

/// Response from creating a workspace webhook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCreateWebhookResponse {
    /// Unique identifier of the created webhook.
    pub webhook_id: String,
    /// Webhook secret (only returned on creation).
    #[serde(default)]
    pub webhook_secret: Option<String>,
}

/// Request body for creating a workspace webhook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateWorkspaceWebhookRequest {
    /// Authentication type for the webhook.
    pub auth_type: String,
    /// Display name for the webhook.
    pub name: String,
    /// HTTPS callback URL.
    pub webhook_url: String,
}

/// Request body for updating a workspace webhook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdateWorkspaceWebhookRequest {
    /// New display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New callback URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// Whether to disable the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
}

// ---------------------------------------------------------------------------
// Invites & Members
// ---------------------------------------------------------------------------

/// Request body for inviting a user to the workspace.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InviteWorkspaceMemberRequest {
    /// Email address to invite.
    pub email: String,
    /// Workspace permission string (e.g. `"admin"`, `"member"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_permission: Option<String>,
    /// Seat type for the new member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_type: Option<String>,
    /// Group IDs to add the member to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

/// Request body for bulk-inviting users to the workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InviteBulkRequest {
    /// Email addresses to invite.
    pub emails: Vec<String>,
    /// Group IDs to add the members to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

/// Request body for deleting a workspace invitation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeleteInviteRequest {
    /// Email address of the invitation to delete.
    pub email: String,
}

/// Request body for updating a workspace member.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct UpdateWorkspaceMemberRequest {
    /// Email of the member to update.
    pub email: String,
    /// Whether to lock the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    /// Workspace role for the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_role: Option<String>,
    /// Workspace seat type for the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_seat_type: Option<String>,
}

/// Request body for sharing a workspace resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShareWorkspaceResourceRequest {
    /// Role to grant (e.g. `"editor"`, `"viewer"`).
    pub role: String,
    /// Type of resource to share.
    pub resource_type: WorkspaceResourceType,
    /// Email of the user to share with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    /// Group ID to share with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// Workspace API key ID to share with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_api_key_id: Option<String>,
}

/// Request body for unsharing a workspace resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnshareWorkspaceResourceRequest {
    /// Type of resource to unshare.
    pub resource_type: WorkspaceResourceType,
    /// Email of the user to unshare from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    /// Group ID to unshare from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// Workspace API key ID to unshare from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_api_key_id: Option<String>,
}

/// Request body for adding a member to a workspace group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddGroupMemberRequest {
    /// Email of the member to add.
    pub email: String,
}

/// Request body for removing a member from a workspace group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemoveGroupMemberRequest {
    /// Email of the member to remove.
    pub email: String,
}

/// Request body for creating a service account API key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateServiceAccountApiKeyRequest {
    /// Display name for the API key.
    pub name: String,
    /// Permissions to grant (list of permission types or `"all"`).
    pub permissions: serde_json::Value,
    /// Optional character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_limit: Option<i64>,
}

/// Request body for editing a service account API key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EditServiceAccountApiKeyRequest {
    /// Whether the key is enabled.
    pub is_enabled: bool,
    /// Display name for the API key.
    pub name: String,
    /// Permissions to grant (list of permission types or `"all"`).
    pub permissions: serde_json::Value,
    /// Optional character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_limit: Option<i64>,
}

/// Response from `GET /v1/workspace/groups/search`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchGroupsResponse(pub Vec<WorkspaceGroupByName>);

/// Response from `GET /v1/workspace/resources/{resource_id}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceMetadataResponse {
    /// Unique resource identifier.
    pub resource_id: String,
    /// Display name of the resource.
    #[serde(default)]
    pub resource_name: Option<String>,
    /// Type of resource.
    pub resource_type: WorkspaceResourceType,
    /// Creator user identifier.
    #[serde(default)]
    pub creator_user_id: Option<String>,
    /// Anonymous access level override.
    #[serde(default)]
    pub anonymous_access_level_override: Option<String>,
    /// Mapping of roles to group IDs.
    #[serde(default)]
    pub role_to_group_ids: Option<serde_json::Value>,
    /// Sharing options for this resource.
    #[serde(default)]
    pub share_options: Option<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Simple status responses
// ---------------------------------------------------------------------------

/// Generic status response used by many workspace endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStatusResponse {
    /// Status string, typically `"ok"`.
    pub status: String,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::{super::agents::WebhookEventType, *};

    fn round_trip<T>(value: &T)
    where
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        let json = serde_json::to_string(value).unwrap();
        let back: T = serde_json::from_str(&json).unwrap();
        assert_eq!(*value, back, "round-trip failed for {json}");
    }

    #[test]
    fn workspace_group_permission_round_trip() {
        let variants = [
            WorkspaceGroupPermission::TextToSpeech,
            WorkspaceGroupPermission::Dubbing,
            WorkspaceGroupPermission::WebhooksManage,
            WorkspaceGroupPermission::Music,
        ];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn workspace_resource_type_round_trip() {
        let variants = [
            WorkspaceResourceType::Voice,
            WorkspaceResourceType::PronunciationDictionary,
            WorkspaceResourceType::ConvaiAgents,
        ];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn webhook_auth_method_round_trip() {
        let variants =
            [WebhookAuthMethod::Hmac, WebhookAuthMethod::Oauth2, WebhookAuthMethod::Mtls];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn webhook_usage_type_round_trip() {
        let variants = [
            WebhookUsageType::ConvAiAgentSettings,
            WebhookUsageType::ConvAiSettings,
            WebhookUsageType::VoiceLibraryRemovalNotices,
            WebhookUsageType::SpeechToText,
        ];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn webhook_event_type_round_trip() {
        let variants = [
            WebhookEventType::Transcript,
            WebhookEventType::Audio,
            WebhookEventType::CallInitiationFailure,
        ];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn workspace_group_deserialize() {
        let json = r#"{
            "name": "Developers",
            "id": "grp1",
            "members": ["user1", "user2"],
            "permissions": ["text_to_speech", "dubbing"]
        }"#;
        let group: WorkspaceGroup = serde_json::from_str(json).unwrap();
        assert_eq!(group.name, "Developers");
        assert_eq!(group.members.len(), 2);
    }

    #[test]
    fn workspace_group_by_name_deserialize() {
        let json = r#"{
            "name": "Engineers",
            "id": "grp2",
            "members_emails": ["a@b.com", "c@d.com"]
        }"#;
        let g: WorkspaceGroupByName = serde_json::from_str(json).unwrap();
        assert_eq!(g.members_emails.len(), 2);
    }

    #[test]
    fn workspace_api_key_deserialize() {
        let json = r#"{
            "name": "Prod Key",
            "hint": "xi_...abc",
            "key_id": "key1",
            "service_account_user_id": "sa1",
            "hashed_xi_api_key": "hash123",
            "created_at_unix": 1700000000,
            "is_disabled": false
        }"#;
        let key: WorkspaceApiKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.name, "Prod Key");
        assert_eq!(key.is_disabled, Some(false));
    }

    #[test]
    fn workspace_service_account_deserialize() {
        let json = r#"{
            "service_account_user_id": "sa1",
            "name": "Bot Account",
            "api-keys": [
                {
                    "name": "Key1",
                    "hint": "xi_...1",
                    "key_id": "k1",
                    "service_account_user_id": "sa1",
                    "hashed_xi_api_key": "h1"
                }
            ]
        }"#;
        let sa: WorkspaceServiceAccount = serde_json::from_str(json).unwrap();
        assert_eq!(sa.api_keys.len(), 1);
    }

    #[test]
    fn workspace_create_api_key_response_deserialize() {
        let json = r#"{
            "xi-api-key": "xi_abc123",
            "key_id": "k1"
        }"#;
        let resp: WorkspaceCreateApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.xi_api_key, "xi_abc123");
    }

    #[test]
    fn workspace_webhook_deserialize() {
        let json = r#"{
            "name": "My Webhook",
            "webhook_id": "wh1",
            "webhook_url": "https://example.com/callback",
            "is_disabled": false,
            "is_auto_disabled": false,
            "created_at_unix": 1700000000,
            "auth_type": "hmac"
        }"#;
        let wh: WorkspaceWebhook = serde_json::from_str(json).unwrap();
        assert_eq!(wh.name, "My Webhook");
        assert_eq!(wh.auth_type, WebhookAuthMethod::Hmac);
    }

    #[test]
    fn workspace_create_webhook_response_deserialize() {
        let json = r#"{
            "webhook_id": "wh1",
            "webhook_secret": "secret123"
        }"#;
        let resp: WorkspaceCreateWebhookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.webhook_id, "wh1");
        assert_eq!(resp.webhook_secret, Some("secret123".into()));
    }

    #[test]
    fn create_webhook_request_serialize() {
        let req = CreateWorkspaceWebhookRequest {
            auth_type: "hmac".into(),
            name: "Test Webhook".into(),
            webhook_url: "https://example.com".into(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"auth_type\":\"hmac\""));
    }

    #[test]
    fn update_webhook_request_serialize_omits_none() {
        let req = UpdateWorkspaceWebhookRequest {
            name: Some("Updated".into()),
            webhook_url: None,
            is_disabled: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"name\":\"Updated\""));
        assert!(!json.contains("webhook_url"));
        assert!(!json.contains("is_disabled"));
    }

    #[test]
    fn workspace_status_response_deserialize() {
        let json = r#"{"status": "ok"}"#;
        let resp: WorkspaceStatusResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "ok");
    }

    #[test]
    fn permission_level_round_trip() {
        let variants = [PermissionLevel::Admin, PermissionLevel::Editor, PermissionLevel::Viewer];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn invite_member_request_serialize() {
        let req = InviteWorkspaceMemberRequest {
            email: "test@example.com".into(),
            workspace_permission: None,
            seat_type: None,
            group_ids: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"email\":\"test@example.com\""));
    }
}
