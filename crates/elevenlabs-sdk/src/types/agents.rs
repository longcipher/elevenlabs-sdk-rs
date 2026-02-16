//! Types for the ElevenLabs Agents Platform & ConvAI endpoints.
//!
//! Covers the largest API group (98 endpoints) including:
//! - Agents: CRUD, avatars, branches, deployments, drafts, duplication
//! - Conversations: list, detail, transcript, audio, feedback
//! - Knowledge Base: documents (URL, file, text, folder), RAG indexes
//! - Tools: webhook, client, system, MCP tool configs
//! - Phone Numbers: Twilio & SIP trunk management
//! - MCP Servers: create, list, get, update, delete, tool configs
//! - Batch Calling: submit, list, get, cancel, retry
//! - Secrets: workspace & user secrets
//! - Settings: workspace ConvAI settings
//! - SIP Trunk: outbound calls
//! - WhatsApp: accounts, outbound calls/messages
//!
//! Complex nested configuration objects (prompt config, LLM config,
//! workflow nodes, tool configs) are represented as `serde_json::Value`
//! to keep the type surface manageable while still providing fully typed
//! wrappers for the most commonly used request/response shapes.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ===========================================================================
// Common Enums (used across multiple agent sub-resources)
// ===========================================================================

/// Role of a user in relation to a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceRole {
    /// Full administrative access.
    Admin,
    /// Can edit the resource.
    Editor,
    /// Can comment on the resource.
    Commenter,
    /// Read-only access.
    Viewer,
}

/// Access information for a shared resource (agent, tool, document, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceAccessInfo {
    /// Whether the requesting user is the creator.
    pub is_creator: bool,
    /// Name of the resource creator.
    pub creator_name: String,
    /// Email of the resource creator.
    pub creator_email: String,
    /// Role of the requesting user.
    pub role: ResourceRole,
}

/// Sort order for listing agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentSortBy {
    /// Sort by agent name.
    Name,
    /// Sort by creation time.
    CreatedAt,
}

/// Status of a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversationStatus {
    /// Conversation has been initiated but not yet started.
    Initiated,
    /// Conversation is actively in progress.
    #[serde(rename = "in-progress")]
    InProgress,
    /// Conversation is being post-processed.
    Processing,
    /// Conversation has completed.
    Done,
    /// Conversation ended with an error.
    Failed,
}

/// Result of a conversation evaluation criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationSuccessResult {
    /// The criterion was met.
    Success,
    /// The criterion was not met.
    Failure,
    /// The result could not be determined.
    Unknown,
}

/// User feedback score for a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserFeedbackScore {
    /// Positive feedback.
    Like,
    /// Negative feedback.
    Dislike,
}

/// Type of conversation feedback collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversationFeedbackType {
    /// Thumbs up/down feedback.
    Thumbs,
    /// Numeric rating feedback.
    Rating,
}

/// Source from which a conversation was initiated.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversationInitiationSource {
    /// Source is unknown.
    Unknown,
    /// Initiated via the Android SDK.
    AndroidSdk,
    /// Initiated via the Node.js SDK.
    NodeJsSdk,
    /// Initiated via the React Native SDK.
    ReactNativeSdk,
    /// Initiated via the React SDK.
    ReactSdk,
    /// Initiated via the JavaScript SDK.
    JsSdk,
    /// Initiated via the Python SDK.
    PythonSdk,
    /// Initiated via the embeddable widget.
    Widget,
    /// Initiated via SIP trunk.
    SipTrunk,
    /// Initiated via Twilio.
    Twilio,
    /// Initiated via Genesys.
    Genesys,
    /// Initiated via the Swift SDK.
    SwiftSdk,
    /// Initiated via WhatsApp.
    Whatsapp,
    /// Initiated via the Flutter SDK.
    FlutterSdk,
    /// Initiated via Zendesk integration.
    ZendeskIntegration,
    /// Initiated via Slack integration.
    SlackIntegration,
}

/// Authorization method used for a conversation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationMethod {
    /// Invalid or unspecified.
    Invalid,
    /// Public access (no auth required).
    Public,
    /// Via Authorization header.
    AuthorizationHeader,
    /// Via signed URL.
    SignedUrl,
    /// Via shareable link.
    ShareableLink,
    /// Via LiveKit token.
    LivekitToken,
    /// Via LiveKit token (website variant).
    LivekitTokenWebsite,
    /// Via Genesys API key.
    GenesysApiKey,
    /// Via WhatsApp.
    Whatsapp,
}

/// Telephony provider for phone numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelephonyProvider {
    /// Twilio provider.
    Twilio,
    /// SIP trunk provider.
    SipTrunk,
}

/// Call direction for phone-based conversations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallDirection {
    /// Incoming call to the agent.
    Inbound,
    /// Outgoing call from the agent.
    Outbound,
}

/// Protection status for an agent branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchProtectionStatus {
    /// Requires writer permissions to modify.
    WriterPermsRequired,
    /// Requires admin permissions to modify.
    AdminPermsRequired,
}

/// Status of a batch call job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchCallStatus {
    /// Batch call is waiting to start.
    Pending,
    /// Batch call is currently executing.
    InProgress,
    /// Batch call has finished.
    Completed,
    /// Batch call encountered an error.
    Failed,
    /// Batch call was cancelled.
    Cancelled,
}

/// Status of an individual recipient within a batch call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchCallRecipientStatus {
    /// Waiting to be called.
    Pending,
    /// Call has been dispatched.
    Dispatched,
    /// Call has been initiated.
    Initiated,
    /// Call is in progress.
    InProgress,
    /// Call completed successfully.
    Completed,
    /// Call failed.
    Failed,
    /// Call was cancelled.
    Cancelled,
    /// Call reached voicemail.
    Voicemail,
}

/// Transport type for MCP server connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum McpServerTransport {
    /// Server-Sent Events transport.
    #[serde(rename = "SSE")]
    Sse,
    /// Streamable HTTP transport.
    #[serde(rename = "STREAMABLE_HTTP")]
    StreamableHttp,
}

/// Usage mode for a knowledge base document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentUsageMode {
    /// Document is used as prompt context.
    Prompt,
    /// Document usage is determined automatically.
    Auto,
}

/// Webhook event types for ConvAI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEventType {
    /// Transcript events.
    Transcript,
    /// Audio events.
    Audio,
    /// Call initiation failure events.
    CallInitiationFailure,
}

/// Type of secret dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretDependencyType {
    /// Used by a conversation initiation webhook.
    ConversationInitiationWebhook,
}

// ===========================================================================
// Agents — Core Types
// ===========================================================================

/// Agent metadata (timestamps).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// Creation time in Unix seconds.
    pub created_at_unix_secs: i64,
    /// Last update time in Unix seconds.
    pub updated_at_unix_secs: i64,
}

/// Summary information for an agent (returned in list responses).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSummary {
    /// Unique agent identifier.
    pub agent_id: String,
    /// Display name of the agent.
    pub name: String,
    /// Tags used to categorize the agent.
    pub tags: Vec<String>,
    /// Creation time in Unix seconds.
    pub created_at_unix_secs: i64,
    /// Access information for the requesting user.
    pub access_info: ResourceAccessInfo,
    /// Time of the most recent call in Unix seconds, if any.
    pub last_call_time_unix_secs: Option<i64>,
    /// Whether the agent is archived.
    #[serde(default)]
    pub archived: bool,
}

/// Paginated response for listing agents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetAgentsResponse {
    /// List of agents.
    pub agents: Vec<AgentSummary>,
    /// Cursor for the next page, if any.
    pub next_cursor: Option<String>,
    /// Whether more pages exist.
    pub has_more: bool,
}

/// Full agent detail response.
///
/// The `conversation_config`, `platform_settings`, and `workflow` fields
/// are represented as opaque JSON values because they contain deeply
/// nested configuration objects with many optional sub-types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetAgentResponse {
    /// Unique agent identifier.
    pub agent_id: String,
    /// Display name of the agent.
    pub name: String,
    /// Conversation configuration (prompt, LLM, TTS, STT, turn-taking, etc.).
    pub conversation_config: serde_json::Value,
    /// Agent metadata (timestamps).
    pub metadata: AgentMetadata,
    /// Platform settings (evaluation, widget, data collection, guardrails, etc.).
    pub platform_settings: serde_json::Value,
    /// Phone numbers assigned to this agent.
    #[serde(default)]
    pub phone_numbers: Vec<serde_json::Value>,
    /// WhatsApp accounts assigned to this agent.
    #[serde(default)]
    pub whatsapp_accounts: Vec<WhatsAppAccount>,
    /// Multi-agent workflow definition.
    #[serde(default)]
    pub workflow: Option<serde_json::Value>,
    /// Access information for the requesting user.
    pub access_info: Option<ResourceAccessInfo>,
    /// Tags used to categorize the agent.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Request body for creating a new agent.
///
/// Uses `serde_json::Value` for complex config objects (conversation_config,
/// platform_settings, workflow) since they contain deeply nested optional
/// fields better handled as free-form JSON.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAgentRequest {
    /// Conversation configuration (prompt, LLM, TTS, STT, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_config: Option<serde_json::Value>,
    /// Platform settings (evaluation, widget, data collection, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_settings: Option<serde_json::Value>,
    /// Multi-agent workflow definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<serde_json::Value>,
    /// Display name for the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags for categorizing the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Request body for updating (patching) an agent.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateAgentRequest {
    /// Conversation configuration updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_config: Option<serde_json::Value>,
    /// Platform settings updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_settings: Option<serde_json::Value>,
    /// Workflow updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<serde_json::Value>,
    /// Updated name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Version description for this update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
    /// Procedure references for this update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_refs: Option<Vec<serde_json::Value>>,
}

/// Agent call limits configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCallLimits {
    /// Maximum concurrent conversations. `-1` means no limit.
    #[serde(default = "default_concurrency_limit")]
    pub agent_concurrency_limit: i64,
    /// Maximum conversations per day.
    #[serde(default = "default_daily_limit")]
    pub daily_limit: i64,
    /// Whether burst mode is enabled (allows exceeding concurrency limit at
    /// double rate).
    #[serde(default = "default_true")]
    pub bursting_enabled: bool,
}

const fn default_concurrency_limit() -> i64 {
    -1
}

const fn default_daily_limit() -> i64 {
    100_000
}

const fn default_true() -> bool {
    true
}

// ===========================================================================
// Agents — Branches
// ===========================================================================

/// Basic branch identifier (id + name).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentBranchBasicInfo {
    /// Branch identifier.
    pub id: String,
    /// Branch display name.
    pub name: String,
}

/// Version parent references for branch merge tracking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentVersionParents {
    /// Parent version ID within the same branch.
    pub in_branch_parent_id: Option<String>,
    /// Parent version ID from another branch.
    pub out_of_branch_parent_id: Option<String>,
    /// Branch ID this version was merged into.
    pub merged_into_branch_id: Option<String>,
    /// Branch ID this version was merged from.
    pub merged_from_branch_id: Option<String>,
}

/// Metadata for a specific agent version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentVersionMetadata {
    /// Version identifier.
    pub id: String,
    /// Parent agent identifier.
    pub agent_id: String,
    /// Branch this version belongs to.
    pub branch_id: String,
    /// Description of changes in this version.
    pub version_description: String,
    /// Sequence number within the branch.
    pub seq_no_in_branch: i64,
    /// Commit time in Unix seconds.
    pub time_committed_secs: i64,
    /// Parent version references.
    pub parents: AgentVersionParents,
    /// Access information for the requesting user.
    pub access_info: Option<ResourceAccessInfo>,
}

/// Full branch response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentBranchResponse {
    /// Branch identifier.
    pub id: String,
    /// Branch display name.
    pub name: String,
    /// Parent agent identifier.
    pub agent_id: String,
    /// Branch description.
    pub description: String,
    /// Creation time in Unix seconds.
    pub created_at: i64,
    /// Time of last commit in Unix seconds.
    pub last_committed_at: i64,
    /// Whether the branch is archived.
    pub is_archived: bool,
    /// Branch protection status.
    #[serde(default)]
    pub protection_status: Option<BranchProtectionStatus>,
    /// Access information for the requesting user.
    pub access_info: Option<ResourceAccessInfo>,
    /// Percentage of live traffic routed to this branch.
    #[serde(default)]
    pub current_live_percentage: f64,
    /// Parent branch info.
    pub parent_branch: Option<AgentBranchBasicInfo>,
    /// Most recent versions on this branch.
    #[serde(default)]
    pub most_recent_versions: Vec<AgentVersionMetadata>,
}

/// Response from creating or updating deployments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentDeploymentResponse {
    /// Map of branch IDs to traffic percentages.
    #[serde(default)]
    pub traffic_percentage_branch_id_map: HashMap<String, f64>,
}

// ===========================================================================
// Conversations
// ===========================================================================

/// Summary information for a conversation (returned in list responses).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationSummary {
    /// Agent that handled the conversation.
    pub agent_id: String,
    /// Branch used, if any.
    pub branch_id: Option<String>,
    /// Agent version used, if any.
    pub version_id: Option<String>,
    /// Agent display name, if available.
    pub agent_name: Option<String>,
    /// Unique conversation identifier.
    pub conversation_id: String,
    /// Start time in Unix seconds.
    pub start_time_unix_secs: i64,
    /// Duration in seconds.
    pub call_duration_secs: i64,
    /// Number of messages exchanged.
    pub message_count: i64,
    /// Current conversation status.
    pub status: ConversationStatus,
    /// Whether the call was successful.
    pub call_successful: EvaluationSuccessResult,
    /// AI-generated summary of the transcript.
    pub transcript_summary: Option<String>,
    /// Short title summarizing the call.
    pub call_summary_title: Option<String>,
    /// Primary language detected.
    pub main_language: Option<String>,
    /// Source that initiated the conversation.
    pub conversation_initiation_source: Option<ConversationInitiationSource>,
}

/// Paginated response for listing conversations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetConversationsResponse {
    /// List of conversation summaries.
    pub conversations: Vec<ConversationSummary>,
    /// Cursor for the next page, if any.
    pub next_cursor: Option<String>,
    /// Whether more pages exist.
    pub has_more: bool,
}

/// Role in a conversation transcript entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptRole {
    /// User (caller) message.
    User,
    /// Agent message.
    Agent,
}

/// A single entry in the conversation transcript.
///
/// Tool calls and tool results are represented as `serde_json::Value`
/// due to their polymorphic nature (webhook, client, system, MCP tools).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationTranscriptEntry {
    /// Role of the message sender.
    pub role: TranscriptRole,
    /// Metadata about which agent/node generated this message.
    pub agent_metadata: Option<serde_json::Value>,
    /// Text content of the message.
    pub message: Option<String>,
    /// Multi-voice message parts (for multi-speaker scenarios).
    pub multivoice_message: Option<serde_json::Value>,
    /// Tool calls made during this turn.
    #[serde(default)]
    pub tool_calls: Vec<serde_json::Value>,
    /// Results from tool executions.
    #[serde(default)]
    pub tool_results: Vec<serde_json::Value>,
    /// User feedback provided during this turn.
    pub feedback: Option<serde_json::Value>,
    /// LLM override applied, if any.
    pub llm_override: Option<String>,
    /// Time in the call (seconds) when this entry occurred.
    #[serde(default)]
    pub time_in_call_secs: Option<i64>,
}

/// Deletion settings for a conversation's data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationDeletionSettings {
    /// Scheduled deletion time in Unix seconds.
    pub deletion_time_unix_secs: Option<i64>,
    /// When logs were deleted (Unix seconds).
    pub deleted_logs_at_time_unix_secs: Option<i64>,
    /// When audio was deleted (Unix seconds).
    pub deleted_audio_at_time_unix_secs: Option<i64>,
    /// When transcript was deleted (Unix seconds).
    pub deleted_transcript_at_time_unix_secs: Option<i64>,
    /// Whether to delete transcript and PII data.
    #[serde(default)]
    pub delete_transcript_and_pii: bool,
    /// Whether to delete audio data.
    #[serde(default)]
    pub delete_audio: bool,
}

/// Feedback information for a conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationFeedback {
    /// Type of feedback collection.
    #[serde(rename = "type")]
    pub feedback_type: Option<ConversationFeedbackType>,
    /// Overall feedback score.
    pub overall_score: Option<UserFeedbackScore>,
    /// Number of positive feedback actions.
    #[serde(default)]
    pub likes: i64,
    /// Number of negative feedback actions.
    #[serde(default)]
    pub dislikes: i64,
    /// Numeric rating value, if applicable.
    pub rating: Option<i64>,
    /// Optional text comment.
    pub comment: Option<String>,
}

/// Charging/billing information for a conversation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversationCharging {
    /// Whether a developer discount was applied.
    #[serde(default)]
    pub dev_discount: bool,
    /// Whether burst pricing was applied.
    #[serde(default)]
    pub is_burst: bool,
    /// Pricing tier name.
    pub tier: Option<String>,
    /// LLM usage breakdown.
    #[serde(default)]
    pub llm_usage: Option<serde_json::Value>,
    /// LLM price charged.
    pub llm_price: Option<f64>,
    /// LLM charge in credits.
    pub llm_charge: Option<i64>,
    /// Total call charge in credits.
    pub call_charge: Option<i64>,
    /// Free minutes consumed by this call.
    #[serde(default)]
    pub free_minutes_consumed: f64,
    /// Free LLM dollars consumed by this call.
    #[serde(default)]
    pub free_llm_dollars_consumed: f64,
}

/// Metadata about a conversation's execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversationMetadata {
    /// Start time in Unix seconds.
    pub start_time_unix_secs: i64,
    /// Time the call was accepted in Unix seconds.
    pub accepted_time_unix_secs: Option<i64>,
    /// Duration in seconds.
    pub call_duration_secs: i64,
    /// Cost in credits.
    pub cost: Option<i64>,
    /// Data deletion settings.
    pub deletion_settings: ConversationDeletionSettings,
    /// User feedback.
    pub feedback: ConversationFeedback,
    /// Authorization method used.
    #[serde(default = "default_authorization_method")]
    pub authorization_method: AuthorizationMethod,
    /// Billing/charging information.
    pub charging: ConversationCharging,
    /// Phone call details, if applicable (opaque due to Twilio/SIP variants).
    pub phone_call: Option<serde_json::Value>,
    /// Batch call reference, if applicable.
    pub batch_call: Option<ConversationBatchCallRef>,
    /// Reason the conversation ended.
    pub termination_reason: Option<String>,
}

const fn default_authorization_method() -> AuthorizationMethod {
    AuthorizationMethod::Public
}

/// Reference to a batch call from within conversation metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationBatchCallRef {
    /// Batch call identifier.
    pub batch_call_id: String,
    /// Recipient identifier within the batch.
    pub batch_call_recipient_id: String,
}

/// Result of a single evaluation criterion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationCriteriaResult {
    /// Criterion identifier.
    pub criteria_id: String,
    /// Evaluation result.
    pub result: EvaluationSuccessResult,
    /// Rationale for the evaluation result.
    pub rationale: String,
}

/// Analysis results for a conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationAnalysis {
    /// Per-criterion evaluation results (keyed by criterion ID).
    #[serde(default)]
    pub evaluation_criteria_results: HashMap<String, EvaluationCriteriaResult>,
    /// Data collection results (opaque).
    #[serde(default)]
    pub data_collection_results: HashMap<String, serde_json::Value>,
    /// Evaluation criteria results as a flat list.
    #[serde(default)]
    pub evaluation_criteria_results_list: Vec<EvaluationCriteriaResult>,
    /// Data collection results as a flat list.
    #[serde(default)]
    pub data_collection_results_list: Vec<serde_json::Value>,
    /// Overall call success evaluation.
    pub call_successful: EvaluationSuccessResult,
    /// AI-generated summary of the transcript.
    pub transcript_summary: String,
    /// Short title summarizing the call.
    pub call_summary_title: Option<String>,
}

/// Full conversation detail response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetConversationResponse {
    /// Agent that handled the conversation.
    pub agent_id: String,
    /// Agent display name.
    pub agent_name: Option<String>,
    /// Conversation status.
    pub status: ConversationStatus,
    /// User identifier, if available.
    pub user_id: Option<String>,
    /// Branch used, if any.
    pub branch_id: Option<String>,
    /// Agent version used.
    pub version_id: Option<String>,
    /// Full conversation transcript.
    #[serde(default)]
    pub transcript: Vec<ConversationTranscriptEntry>,
    /// Conversation execution metadata.
    pub metadata: ConversationMetadata,
    /// Post-call analysis, if available.
    pub analysis: Option<ConversationAnalysis>,
    /// Client data provided at conversation initiation.
    #[serde(default)]
    pub conversation_initiation_client_data: Option<serde_json::Value>,
    /// Unique conversation identifier.
    pub conversation_id: String,
    /// Whether full audio is available.
    pub has_audio: bool,
    /// Whether user audio is available.
    pub has_user_audio: bool,
    /// Whether response audio is available.
    pub has_response_audio: bool,
}

/// Request body for submitting conversation feedback.
#[derive(Debug, Clone, Serialize)]
pub struct ConversationFeedbackRequest {
    /// Feedback score (`like` or `dislike`), or `null` to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<UserFeedbackScore>,
}

// ===========================================================================
// Knowledge Base
// ===========================================================================

/// Metadata for a knowledge base document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeBaseDocumentMetadata {
    /// Creation time in Unix seconds.
    pub created_at_unix_secs: i64,
    /// Last update time in Unix seconds.
    pub last_updated_at_unix_secs: i64,
    /// Document size in bytes.
    pub size_bytes: i64,
}

/// A path segment in the knowledge base folder hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeBaseFolderPathSegment {
    /// Folder segment identifier.
    pub id: String,
}

/// Summary for a knowledge base document.
///
/// This is a unified type covering URL, file, text, and folder document
/// types. The `document_type` field discriminates between them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeBaseDocumentSummary {
    /// Document identifier.
    pub id: String,
    /// Document display name.
    pub name: String,
    /// Document metadata (timestamps, size).
    pub metadata: KnowledgeBaseDocumentMetadata,
    /// Supported usage modes.
    #[serde(default)]
    pub supported_usages: Vec<DocumentUsageMode>,
    /// Access information for the requesting user.
    pub access_info: ResourceAccessInfo,
    /// Parent folder ID, or `None` if at root.
    pub folder_parent_id: Option<String>,
    /// Folder path segments from root to parent.
    #[serde(default)]
    pub folder_path: Vec<KnowledgeBaseFolderPathSegment>,
    /// Deprecated: list of dependent agents.
    #[serde(default)]
    pub dependent_agents: Vec<serde_json::Value>,
    /// Document type discriminator (`url`, `file`, `text`, `folder`).
    #[serde(rename = "type")]
    pub document_type: String,
    /// URL for URL-type documents.
    #[serde(default)]
    pub url: Option<String>,
    /// Number of children for folder-type documents.
    #[serde(default)]
    pub children_count: Option<i64>,
}

/// Paginated response for listing knowledge base documents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetKnowledgeBaseListResponse {
    /// List of documents.
    pub documents: Vec<KnowledgeBaseDocumentSummary>,
    /// Cursor for the next page, if any.
    pub next_cursor: Option<String>,
    /// Whether more pages exist.
    pub has_more: bool,
}

/// Response from adding a document to the knowledge base.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddKnowledgeBaseResponse {
    /// New document identifier.
    pub id: String,
    /// Document display name.
    pub name: String,
    /// Folder path to the document.
    #[serde(default)]
    pub folder_path: Vec<KnowledgeBaseFolderPathSegment>,
}

/// Request to create a URL-based knowledge base document.
#[derive(Debug, Clone, Serialize)]
pub struct CreateKnowledgeBaseUrlRequest {
    /// URL to import content from.
    pub url: String,
    /// Display name for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Parent folder ID for organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

/// Request to create a text-based knowledge base document.
#[derive(Debug, Clone, Serialize)]
pub struct CreateKnowledgeBaseTextRequest {
    /// Text content of the document.
    pub text: String,
    /// Display name for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Parent folder ID for organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

/// Request to create a knowledge base folder.
#[derive(Debug, Clone, Serialize)]
pub struct CreateKnowledgeBaseFolderRequest {
    /// Folder display name.
    pub name: String,
    /// Parent folder ID for nesting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

/// Request to update a knowledge base document's name.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateKnowledgeBaseDocumentRequest {
    /// New display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// ===========================================================================
// Phone Numbers
// ===========================================================================

/// Agent info attached to a phone number.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumberAgentInfo {
    /// Assigned agent identifier.
    pub agent_id: String,
    /// Assigned agent name.
    pub agent_name: String,
}

/// Twilio phone number configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumberTwilio {
    /// Phone number string (E.164 format).
    pub phone_number: String,
    /// Display label for the number.
    pub label: String,
    /// Unique phone number identifier.
    pub phone_number_id: String,
    /// Agent assigned to this number, if any.
    pub assigned_agent: Option<PhoneNumberAgentInfo>,
    /// Provider type (always `"twilio"`).
    #[serde(default)]
    pub provider: Option<String>,
}

/// SIP trunk phone number configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumberSipTrunk {
    /// Phone number string.
    pub phone_number: String,
    /// Display label for the number.
    pub label: String,
    /// Unique phone number identifier.
    pub phone_number_id: String,
    /// Agent assigned to this number, if any.
    pub assigned_agent: Option<PhoneNumberAgentInfo>,
    /// Provider type (always `"sip_trunk"`).
    #[serde(default)]
    pub provider: Option<String>,
    /// Outbound SIP trunk configuration.
    pub outbound_trunk: Option<serde_json::Value>,
    /// Inbound SIP trunk configuration.
    pub inbound_trunk: Option<serde_json::Value>,
}

/// Response from creating a phone number.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatePhoneNumberResponse {
    /// New phone number entity identifier.
    pub phone_number_id: String,
}

// ===========================================================================
// Tools
// ===========================================================================

/// Tool usage statistics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolUsageStats {
    /// Usage statistics as opaque JSON (varies by tool type).
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Response model for a tool.
///
/// The `tool_config` is represented as `serde_json::Value` because it's
/// a discriminated union of webhook, client, system, and MCP tool configs
/// with deeply nested sub-types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolResponse {
    /// Tool identifier.
    pub id: String,
    /// Tool configuration (webhook, client, system, or MCP).
    pub tool_config: serde_json::Value,
    /// Access information for the requesting user.
    pub access_info: ResourceAccessInfo,
    /// Tool usage statistics.
    pub usage_stats: serde_json::Value,
}

// ===========================================================================
// MCP Servers
// ===========================================================================

/// Metadata for an MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpServerMetadata {
    /// Creation time in Unix seconds.
    pub created_at: i64,
    /// Owner user identifier.
    pub owner_user_id: Option<String>,
}

/// MCP server configuration (output/response variant).
///
/// Complex sub-fields (URL variants, secret tokens, request headers) are
/// represented as `serde_json::Value` for flexibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Approval policy for tool invocations.
    #[serde(default)]
    pub approval_policy: Option<String>,
    /// Transport type used to connect.
    pub transport: Option<McpServerTransport>,
    /// Server URL (may be a string or secret locator).
    pub url: Option<serde_json::Value>,
    /// Secret token for authorization.
    pub secret_token: Option<serde_json::Value>,
    /// Custom request headers.
    #[serde(default)]
    pub request_headers: HashMap<String, serde_json::Value>,
    /// Server display name.
    pub name: Option<String>,
    /// Server description.
    pub description: Option<String>,
}

/// Response model for a single MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpServerResponse {
    /// MCP server identifier.
    pub id: String,
    /// Server configuration.
    pub config: McpServerConfig,
    /// Access information for the requesting user.
    pub access_info: Option<ResourceAccessInfo>,
    /// List of dependent agents (opaque due to discriminated union).
    #[serde(default)]
    pub dependent_agents: Vec<serde_json::Value>,
    /// Server metadata.
    pub metadata: McpServerMetadata,
}

/// Response model for listing MCP servers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpServersResponse {
    /// List of MCP server entries.
    pub mcp_servers: Vec<McpServerResponse>,
}

// ===========================================================================
// Batch Calling
// ===========================================================================

/// WhatsApp parameters for batch calls.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchCallWhatsAppParams {
    /// WhatsApp phone number identifier.
    pub whatsapp_phone_number_id: String,
    /// Template name for call permission request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_call_permission_request_template_name: Option<String>,
    /// Language code for the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_call_permission_request_template_language_code: Option<String>,
}

/// Response model for a batch call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchCallResponse {
    /// Batch call identifier.
    pub id: String,
    /// Phone number used for calling.
    pub phone_number_id: Option<String>,
    /// Telephony provider.
    pub phone_provider: Option<TelephonyProvider>,
    /// WhatsApp parameters, if applicable.
    pub whatsapp_params: Option<BatchCallWhatsAppParams>,
    /// Batch call display name.
    pub name: String,
    /// Agent used for the calls.
    pub agent_id: String,
    /// Creation time in Unix seconds.
    pub created_at_unix: i64,
    /// Scheduled execution time in Unix seconds.
    pub scheduled_time_unix: i64,
    /// Timezone for scheduling.
    pub timezone: Option<String>,
    /// Number of calls dispatched.
    #[serde(default)]
    pub total_calls_dispatched: i64,
    /// Number of calls scheduled.
    #[serde(default)]
    pub total_calls_scheduled: i64,
    /// Number of calls completed.
    #[serde(default)]
    pub total_calls_finished: i64,
    /// Last update time in Unix seconds.
    pub last_updated_at_unix: i64,
    /// Batch call status.
    pub status: BatchCallStatus,
    /// Number of retry attempts.
    #[serde(default)]
    pub retry_count: i64,
    /// Agent display name.
    pub agent_name: String,
}

/// Paginated response for listing workspace batch calls.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceBatchCallsResponse {
    /// List of batch call entries.
    pub batch_calls: Vec<BatchCallResponse>,
    /// Cursor for the next page (named `next_doc` in the API).
    pub next_doc: Option<String>,
    /// Whether more pages exist.
    #[serde(default)]
    pub has_more: bool,
}

/// Request body for submitting a batch call.
#[derive(Debug, Clone, Serialize)]
pub struct SubmitBatchCallRequest {
    /// Display name for the batch call job.
    pub call_name: String,
    /// Agent to use for the calls.
    pub agent_id: String,
    /// List of recipients (opaque — includes phone/name/metadata per recipient).
    pub recipients: Vec<serde_json::Value>,
    /// Scheduled execution time in Unix seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time_unix: Option<i64>,
    /// Phone number to call from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_phone_number_id: Option<String>,
    /// WhatsApp parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_params: Option<BatchCallWhatsAppParams>,
    /// Timezone for scheduling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

// ===========================================================================
// Secrets
// ===========================================================================

/// A workspace-stored secret with dependency information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStoredSecret {
    /// Secret type discriminator (always `"stored"`).
    #[serde(rename = "type")]
    pub secret_type: String,
    /// Secret identifier.
    pub secret_id: String,
    /// Secret display name.
    pub name: String,
    /// Resources that depend on this secret.
    pub used_by: serde_json::Value,
}

/// Request body for creating a secret.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSecretRequest {
    /// Secret display name.
    pub name: String,
    /// Secret value (will be encrypted at rest).
    pub value: String,
}

/// Request body for updating a secret.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSecretRequest {
    /// Updated secret name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated secret value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

// ===========================================================================
// Settings
// ===========================================================================

/// ConvAI webhook configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvAiWebhooks {
    /// Post-call webhook identifier.
    pub post_call_webhook_id: Option<String>,
    /// Event types to emit via webhook.
    #[serde(default)]
    pub events: Vec<WebhookEventType>,
}

/// Workspace-level ConvAI settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetConvAiSettingsResponse {
    /// Conversation initiation data webhook configuration.
    pub conversation_initiation_client_data_webhook: Option<serde_json::Value>,
    /// Webhook configuration.
    pub webhooks: ConvAiWebhooks,
    /// Whether MCP servers are enabled for the workspace.
    #[serde(default)]
    pub can_use_mcp_servers: bool,
    /// RAG data retention period in days.
    #[serde(default = "default_rag_retention")]
    pub rag_retention_period_days: i64,
    /// Default LiveKit stack type.
    #[serde(default)]
    pub default_livekit_stack: Option<String>,
}

const fn default_rag_retention() -> i64 {
    10
}

// ===========================================================================
// WhatsApp
// ===========================================================================

/// WhatsApp business account assigned to an agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhatsAppAccount {
    /// WhatsApp Business Account ID.
    pub business_account_id: String,
    /// Phone number ID within WhatsApp Business.
    pub phone_number_id: String,
    /// Business account display name.
    pub business_account_name: String,
    /// Phone number display name.
    pub phone_number_name: String,
    /// Phone number string.
    pub phone_number: String,
    /// Agent assigned to this WhatsApp number.
    pub assigned_agent_id: Option<String>,
    /// Agent name assigned to this WhatsApp number.
    pub assigned_agent_name: Option<String>,
}

// ===========================================================================
// SIP Trunk
// ===========================================================================

/// Request body for making an outbound call via SIP trunk.
#[derive(Debug, Clone, Serialize)]
pub struct SipTrunkOutboundCallRequest {
    /// Agent to handle the call.
    pub agent_id: String,
    /// Phone number entity to call from.
    pub agent_phone_number_id: String,
    /// Destination phone number.
    pub to_number: String,
    /// Client data to pass at conversation initiation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<serde_json::Value>,
}

// ===========================================================================
// Agent Summaries (additional response)
// ===========================================================================

/// Response for listing agent summaries (compact — no pagination).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetAgentSummariesResponse {
    /// List of agent summaries.
    pub agents: Vec<AgentSummary>,
}

// ===========================================================================
// Agent Link & Widget Responses
// ===========================================================================

/// Response for retrieving an agent's shareable link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentLinkResponse {
    /// The shareable link URL.
    pub url: String,
}

// ===========================================================================
// Conversations — Signed URL & Token
// ===========================================================================

/// Response for getting a signed conversation URL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedUrlResponse {
    /// The signed URL.
    pub signed_url: String,
}

/// Response for getting a LiveKit conversation token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationTokenResponse {
    /// The LiveKit token.
    pub token: String,
    /// Additional fields returned by the API.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

// ===========================================================================
// Phone Numbers — Request / list types
// ===========================================================================

/// Response for listing phone numbers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListPhoneNumbersResponse {
    /// List of phone numbers (polymorphic — Twilio or SIP trunk).
    pub phone_numbers: Vec<serde_json::Value>,
}

// ===========================================================================
// Tools — List response
// ===========================================================================

/// Response for listing tools.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetToolsResponse {
    /// List of tool entries.
    pub tools: Vec<ToolResponse>,
}

// ===========================================================================
// WhatsApp — Additional request / response types
// ===========================================================================

/// Response for listing WhatsApp accounts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListWhatsAppAccountsResponse {
    /// List of WhatsApp accounts.
    pub accounts: Vec<WhatsAppAccount>,
}

/// Request for making an outbound WhatsApp call.
#[derive(Debug, Clone, Serialize)]
pub struct WhatsAppOutboundCallRequest {
    /// Agent to handle the call.
    pub agent_id: String,
    /// WhatsApp phone number identifier to call from.
    pub agent_phone_number_id: String,
    /// Destination phone number (E.164).
    pub to: String,
    /// Client data to pass at conversation initiation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<serde_json::Value>,
}

/// Request for sending an outbound WhatsApp message.
#[derive(Debug, Clone, Serialize)]
pub struct WhatsAppOutboundMessageRequest {
    /// Agent that will handle the conversation.
    pub agent_id: String,
    /// WhatsApp phone number identifier.
    pub whatsapp_phone_number_id: String,
    /// Destination phone number (E.164).
    pub to: String,
    /// Optional initial message text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Client data to pass at conversation initiation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<serde_json::Value>,
}

// ===========================================================================
// Analytics
// ===========================================================================

/// Response for the live conversation count endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveCountResponse {
    /// Number of currently active conversations.
    pub count: i64,
}

// ===========================================================================
// Branches — Request types
// ===========================================================================

/// Request to create a new agent branch.
#[derive(Debug, Clone, Serialize)]
pub struct CreateBranchRequest {
    /// Branch display name.
    pub name: String,
    /// Optional branch description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to update an agent branch.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateBranchRequest {
    /// Updated branch name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated branch description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to merge a source branch into a target branch.
#[derive(Debug, Clone, Serialize)]
pub struct MergeBranchRequest {
    /// Target branch to merge into.
    pub target_branch_id: String,
}

// ===========================================================================
// Deployments — Request type
// ===========================================================================

/// Request to create or update an agent deployment (traffic split).
#[derive(Debug, Clone, Serialize)]
pub struct CreateDeploymentRequest {
    /// Mapping of branch IDs to traffic percentages (0.0–1.0).
    pub traffic_percentage_branch_id_map: HashMap<String, f64>,
}

// ===========================================================================
// Knowledge Base — Move operations
// ===========================================================================

/// Request to move a knowledge base document into a folder.
#[derive(Debug, Clone, Serialize)]
pub struct KnowledgeBaseMoveRequest {
    /// Target folder ID, or `None` for root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
}

/// Request to bulk-move knowledge base documents.
#[derive(Debug, Clone, Serialize)]
pub struct KnowledgeBaseBulkMoveRequest {
    /// Document IDs to move.
    pub document_ids: Vec<String>,
    /// Target folder ID, or `None` for root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
}

// ===========================================================================
// Secrets — List response
// ===========================================================================

/// Response for listing workspace secrets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetSecretsResponse {
    /// List of stored secrets.
    pub secrets: Vec<WorkspaceStoredSecret>,
}

// ===========================================================================
// Twilio
// ===========================================================================

/// Request body for making an outbound call via Twilio.
#[derive(Debug, Clone, Serialize)]
pub struct TwilioOutboundCallRequest {
    /// Agent to handle the call.
    pub agent_id: String,
    /// Phone number entity to call from.
    pub agent_phone_number_id: String,
    /// Destination phone number.
    pub to_number: String,
    /// Client data to pass at conversation initiation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<serde_json::Value>,
}

/// Response from an outbound Twilio call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwilioOutboundCallResponse {
    /// Whether the call was initiated successfully.
    pub success: bool,
    /// Status message.
    pub message: String,
    /// Conversation ID for the call.
    pub conversation_id: Option<String>,
    /// Twilio Call SID.
    #[serde(rename = "callSid")]
    pub call_sid: Option<String>,
}

/// Request body for registering a Twilio call.
#[derive(Debug, Clone, Serialize)]
pub struct TwilioRegisterCallRequest {
    /// Agent to handle the call.
    pub agent_id: String,
    /// Caller phone number.
    pub from_number: String,
    /// Destination phone number.
    pub to_number: String,
    /// Call direction (`"inbound"` or `"outbound"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Client data to pass at conversation initiation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<serde_json::Value>,
}

// ===========================================================================
// Users
// ===========================================================================

/// A conversation user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationUser {
    /// User identifier.
    pub user_id: String,
    /// Last contact time in Unix seconds.
    pub last_contact_unix_secs: i64,
    /// First contact time in Unix seconds.
    pub first_contact_unix_secs: i64,
    /// Number of conversations.
    pub conversation_count: i64,
    /// Last agent the user interacted with.
    pub last_agent_id: Option<String>,
    /// Name of the last agent the user interacted with.
    pub last_agent_name: Option<String>,
}

/// Paginated response for listing conversation users.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetConversationUsersResponse {
    /// List of users.
    pub users: Vec<ConversationUser>,
    /// Cursor for the next page, if any.
    pub next_cursor: Option<String>,
    /// Whether more pages exist.
    pub has_more: bool,
}

// ===========================================================================
// Tool Dependent Agents
// ===========================================================================

/// Paginated response for listing agents dependent on a tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetToolDependentAgentsResponse {
    /// List of dependent agents (polymorphic — available or unknown).
    pub agents: Vec<serde_json::Value>,
    /// Cursor for the next page, if any.
    pub next_cursor: Option<String>,
    /// Whether more pages exist.
    pub has_more: bool,
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    // -- Enums ---------------------------------------------------------------

    #[test]
    fn conversation_status_serde_round_trip() {
        let status = ConversationStatus::InProgress;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"in-progress\"");
        let back: ConversationStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, back);
    }

    #[test]
    fn evaluation_success_result_serde_round_trip() {
        let result = EvaluationSuccessResult::Success;
        let json = serde_json::to_string(&result).unwrap();
        assert_eq!(json, "\"success\"");
        let back: EvaluationSuccessResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, back);
    }

    #[test]
    fn batch_call_status_serde_round_trip() {
        let status = BatchCallStatus::InProgress;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"in_progress\"");
        let back: BatchCallStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, back);
    }

    #[test]
    fn mcp_server_transport_serde_round_trip() {
        let transport = McpServerTransport::Sse;
        let json = serde_json::to_string(&transport).unwrap();
        assert_eq!(json, "\"SSE\"");
        let back: McpServerTransport = serde_json::from_str(&json).unwrap();
        assert_eq!(transport, back);
    }

    #[test]
    fn telephony_provider_serde_round_trip() {
        let provider = TelephonyProvider::SipTrunk;
        let json = serde_json::to_string(&provider).unwrap();
        assert_eq!(json, "\"sip_trunk\"");
        let back: TelephonyProvider = serde_json::from_str(&json).unwrap();
        assert_eq!(provider, back);
    }

    #[test]
    fn branch_protection_status_serde_round_trip() {
        let status = BranchProtectionStatus::AdminPermsRequired;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"admin_perms_required\"");
        let back: BranchProtectionStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, back);
    }

    #[test]
    fn agent_sort_by_serde_round_trip() {
        let sort = AgentSortBy::CreatedAt;
        let json = serde_json::to_string(&sort).unwrap();
        assert_eq!(json, "\"created_at\"");
        let back: AgentSortBy = serde_json::from_str(&json).unwrap();
        assert_eq!(sort, back);
    }

    #[test]
    fn conversation_initiation_source_serde_round_trip() {
        let source = ConversationInitiationSource::PythonSdk;
        let json = serde_json::to_string(&source).unwrap();
        assert_eq!(json, "\"python_sdk\"");
        let back: ConversationInitiationSource = serde_json::from_str(&json).unwrap();
        assert_eq!(source, back);
    }

    #[test]
    fn user_feedback_score_serde_round_trip() {
        let score = UserFeedbackScore::Like;
        let json = serde_json::to_string(&score).unwrap();
        assert_eq!(json, "\"like\"");
        let back: UserFeedbackScore = serde_json::from_str(&json).unwrap();
        assert_eq!(score, back);
    }

    // -- Resource Access Info ------------------------------------------------

    #[test]
    fn resource_access_info_deserialize() {
        let json = r#"{
            "is_creator": true,
            "creator_name": "John Doe",
            "creator_email": "john@example.com",
            "role": "admin"
        }"#;
        let info: ResourceAccessInfo = serde_json::from_str(json).unwrap();
        assert!(info.is_creator);
        assert_eq!(info.creator_name, "John Doe");
        assert_eq!(info.role, ResourceRole::Admin);
    }

    // -- Agent Summary (list item) -------------------------------------------

    #[test]
    fn agent_summary_deserialize() {
        let json = r#"{
            "agent_id": "J3Pbu5gP6NNKBscdCdwB",
            "name": "My Agent",
            "tags": ["Customer Support", "Technical Help"],
            "created_at_unix_secs": 1716153600,
            "access_info": {
                "is_creator": true,
                "creator_name": "John Doe",
                "creator_email": "john@example.com",
                "role": "admin"
            },
            "last_call_time_unix_secs": 1716240000,
            "archived": false
        }"#;
        let agent: AgentSummary = serde_json::from_str(json).unwrap();
        assert_eq!(agent.agent_id, "J3Pbu5gP6NNKBscdCdwB");
        assert_eq!(agent.name, "My Agent");
        assert_eq!(agent.tags.len(), 2);
        assert_eq!(agent.created_at_unix_secs, 1716153600);
        assert!(agent.access_info.is_creator);
        assert_eq!(agent.last_call_time_unix_secs, Some(1716240000));
        assert!(!agent.archived);
    }

    // -- Agents Page Response ------------------------------------------------

    #[test]
    fn get_agents_response_deserialize() {
        let json = r#"{
            "agents": [
                {
                    "agent_id": "agent_1",
                    "name": "Agent 1",
                    "tags": [],
                    "created_at_unix_secs": 1700000000,
                    "access_info": {
                        "is_creator": true,
                        "creator_name": "Test",
                        "creator_email": "test@test.com",
                        "role": "admin"
                    }
                }
            ],
            "next_cursor": "abc123",
            "has_more": true
        }"#;
        let resp: GetAgentsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.agents.len(), 1);
        assert_eq!(resp.agents[0].agent_id, "agent_1");
        assert_eq!(resp.next_cursor.as_deref(), Some("abc123"));
        assert!(resp.has_more);
    }

    // -- Full Agent Response -------------------------------------------------

    #[test]
    fn get_agent_response_deserialize() {
        let json = r#"{
            "agent_id": "agent_xyz",
            "name": "Support Bot",
            "conversation_config": {"prompt": {"prompt": "You are a helpful assistant."}},
            "metadata": {
                "created_at_unix_secs": 1700000000,
                "updated_at_unix_secs": 1700001000
            },
            "platform_settings": {},
            "phone_numbers": [],
            "whatsapp_accounts": [],
            "tags": ["support"]
        }"#;
        let resp: GetAgentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.agent_id, "agent_xyz");
        assert_eq!(resp.name, "Support Bot");
        assert_eq!(resp.metadata.created_at_unix_secs, 1700000000);
        assert_eq!(resp.tags, vec!["support"]);
    }

    // -- Create Agent Request ------------------------------------------------

    #[test]
    fn create_agent_request_serialize_minimal() {
        let req = CreateAgentRequest {
            conversation_config: None,
            platform_settings: None,
            workflow: None,
            name: Some("New Agent".into()),
            tags: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"name\":\"New Agent\""));
        assert!(!json.contains("conversation_config"));
        assert!(!json.contains("platform_settings"));
    }

    // -- Agent Call Limits ---------------------------------------------------

    #[test]
    fn agent_call_limits_serde_round_trip() {
        let limits = AgentCallLimits {
            agent_concurrency_limit: 10,
            daily_limit: 5000,
            bursting_enabled: false,
        };
        let json = serde_json::to_string(&limits).unwrap();
        let back: AgentCallLimits = serde_json::from_str(&json).unwrap();
        assert_eq!(limits, back);
    }

    #[test]
    fn agent_call_limits_defaults() {
        let json = r#"{}"#;
        let limits: AgentCallLimits = serde_json::from_str(json).unwrap();
        assert_eq!(limits.agent_concurrency_limit, -1);
        assert_eq!(limits.daily_limit, 100_000);
        assert!(limits.bursting_enabled);
    }

    // -- Branch Response -----------------------------------------------------

    #[test]
    fn agent_branch_response_deserialize() {
        let json = r#"{
            "id": "branch_1",
            "name": "main",
            "agent_id": "agent_xyz",
            "description": "The main branch",
            "created_at": 1700000000,
            "last_committed_at": 1700001000,
            "is_archived": false,
            "protection_status": "writer_perms_required",
            "current_live_percentage": 100.0,
            "most_recent_versions": []
        }"#;
        let branch: AgentBranchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(branch.id, "branch_1");
        assert_eq!(branch.name, "main");
        assert!(!branch.is_archived);
        assert_eq!(branch.protection_status, Some(BranchProtectionStatus::WriterPermsRequired));
        assert!((branch.current_live_percentage - 100.0).abs() < f64::EPSILON);
    }

    // -- Deployment Response -------------------------------------------------

    #[test]
    fn agent_deployment_response_deserialize() {
        let json = r#"{
            "traffic_percentage_branch_id_map": {
                "branch_abc": 0.5,
                "branch_def": 0.5
            }
        }"#;
        let resp: AgentDeploymentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.traffic_percentage_branch_id_map.len(), 2);
        assert!((resp.traffic_percentage_branch_id_map["branch_abc"] - 0.5).abs() < f64::EPSILON);
    }

    // -- Conversation Summary ------------------------------------------------

    #[test]
    fn conversation_summary_deserialize() {
        let json = r#"{
            "agent_id": "agent_1",
            "conversation_id": "conv_123",
            "start_time_unix_secs": 1700000000,
            "call_duration_secs": 120,
            "message_count": 10,
            "status": "done",
            "call_successful": "success",
            "transcript_summary": "User asked about order status."
        }"#;
        let summary: ConversationSummary = serde_json::from_str(json).unwrap();
        assert_eq!(summary.agent_id, "agent_1");
        assert_eq!(summary.conversation_id, "conv_123");
        assert_eq!(summary.status, ConversationStatus::Done);
        assert_eq!(summary.call_successful, EvaluationSuccessResult::Success);
        assert_eq!(summary.call_duration_secs, 120);
    }

    // -- Get Conversations Response ------------------------------------------

    #[test]
    fn get_conversations_response_deserialize() {
        let json = r#"{
            "conversations": [
                {
                    "agent_id": "a1",
                    "conversation_id": "c1",
                    "start_time_unix_secs": 1700000000,
                    "call_duration_secs": 60,
                    "message_count": 5,
                    "status": "done",
                    "call_successful": "unknown"
                }
            ],
            "next_cursor": null,
            "has_more": false
        }"#;
        let resp: GetConversationsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.conversations.len(), 1);
        assert!(!resp.has_more);
        assert!(resp.next_cursor.is_none());
    }

    // -- Conversation Detail Response ----------------------------------------

    #[test]
    fn get_conversation_response_deserialize() {
        let json = r#"{
            "agent_id": "agent_1",
            "agent_name": "Bot",
            "status": "done",
            "user_id": null,
            "branch_id": null,
            "version_id": null,
            "transcript": [
                {
                    "role": "user",
                    "message": "Hello",
                    "tool_calls": [],
                    "tool_results": [],
                    "time_in_call_secs": 0
                },
                {
                    "role": "agent",
                    "message": "Hi! How can I help?",
                    "tool_calls": [],
                    "tool_results": [],
                    "time_in_call_secs": 1
                }
            ],
            "metadata": {
                "start_time_unix_secs": 1700000000,
                "call_duration_secs": 30,
                "cost": 5,
                "deletion_settings": {},
                "feedback": {"likes": 1, "dislikes": 0},
                "charging": {}
            },
            "conversation_id": "conv_456",
            "has_audio": true,
            "has_user_audio": false,
            "has_response_audio": true
        }"#;
        let resp: GetConversationResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.agent_id, "agent_1");
        assert_eq!(resp.agent_name.as_deref(), Some("Bot"));
        assert_eq!(resp.status, ConversationStatus::Done);
        assert_eq!(resp.transcript.len(), 2);
        assert_eq!(resp.transcript[0].role, TranscriptRole::User);
        assert_eq!(resp.transcript[0].message.as_deref(), Some("Hello"));
        assert_eq!(resp.metadata.call_duration_secs, 30);
        assert_eq!(resp.metadata.feedback.likes, 1);
        assert!(resp.has_audio);
        assert!(!resp.has_user_audio);
    }

    // -- Conversation Feedback Request ---------------------------------------

    #[test]
    fn conversation_feedback_request_serialize() {
        let req = ConversationFeedbackRequest { feedback: Some(UserFeedbackScore::Like) };
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, r#"{"feedback":"like"}"#);
    }

    // -- Knowledge Base Document Summary -------------------------------------

    #[test]
    fn knowledge_base_document_summary_deserialize() {
        let json = r#"{
            "id": "doc_1",
            "name": "FAQ",
            "metadata": {
                "created_at_unix_secs": 1700000000,
                "last_updated_at_unix_secs": 1700001000,
                "size_bytes": 4096
            },
            "supported_usages": ["auto"],
            "access_info": {
                "is_creator": true,
                "creator_name": "Jane",
                "creator_email": "jane@co.com",
                "role": "editor"
            },
            "folder_path": [{"id": "folder_root"}],
            "dependent_agents": [],
            "type": "text"
        }"#;
        let doc: KnowledgeBaseDocumentSummary = serde_json::from_str(json).unwrap();
        assert_eq!(doc.id, "doc_1");
        assert_eq!(doc.name, "FAQ");
        assert_eq!(doc.document_type, "text");
        assert_eq!(doc.metadata.size_bytes, 4096);
        assert_eq!(doc.supported_usages, vec![DocumentUsageMode::Auto]);
        assert_eq!(doc.folder_path.len(), 1);
    }

    // -- Knowledge Base List Response ----------------------------------------

    #[test]
    fn get_knowledge_base_list_response_deserialize() {
        let json = r#"{
            "documents": [],
            "next_cursor": null,
            "has_more": false
        }"#;
        let resp: GetKnowledgeBaseListResponse = serde_json::from_str(json).unwrap();
        assert!(resp.documents.is_empty());
        assert!(!resp.has_more);
    }

    // -- Add Knowledge Base Response -----------------------------------------

    #[test]
    fn add_knowledge_base_response_deserialize() {
        let json = r#"{"id": "doc_new", "name": "My Doc"}"#;
        let resp: AddKnowledgeBaseResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, "doc_new");
        assert_eq!(resp.name, "My Doc");
        assert!(resp.folder_path.is_empty());
    }

    // -- Create Knowledge Base Requests --------------------------------------

    #[test]
    fn create_knowledge_base_url_request_serialize() {
        let req = CreateKnowledgeBaseUrlRequest {
            url: "https://example.com/faq".into(),
            name: Some("FAQ Page".into()),
            parent_folder_id: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"url\":\"https://example.com/faq\""));
        assert!(json.contains("\"name\":\"FAQ Page\""));
        assert!(!json.contains("parent_folder_id"));
    }

    #[test]
    fn create_knowledge_base_text_request_serialize() {
        let req = CreateKnowledgeBaseTextRequest {
            text: "Some content here.".into(),
            name: None,
            parent_folder_id: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"text\":\"Some content here.\""));
        assert!(!json.contains("name"));
    }

    // -- Phone Numbers -------------------------------------------------------

    #[test]
    fn phone_number_twilio_deserialize() {
        let json = r#"{
            "phone_number": "+1234567890",
            "label": "Customer Support",
            "phone_number_id": "phone_123",
            "assigned_agent": {
                "agent_id": "agent_1",
                "agent_name": "Support Bot"
            },
            "provider": "twilio"
        }"#;
        let phone: PhoneNumberTwilio = serde_json::from_str(json).unwrap();
        assert_eq!(phone.phone_number, "+1234567890");
        assert_eq!(phone.label, "Customer Support");
        assert_eq!(phone.phone_number_id, "phone_123");
        assert!(phone.assigned_agent.is_some());
        let agent = phone.assigned_agent.unwrap();
        assert_eq!(agent.agent_id, "agent_1");
    }

    #[test]
    fn create_phone_number_response_deserialize() {
        let json = r#"{"phone_number_id": "phone_new"}"#;
        let resp: CreatePhoneNumberResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.phone_number_id, "phone_new");
    }

    // -- MCP Server ----------------------------------------------------------

    #[test]
    fn mcp_server_response_deserialize() {
        let json = r#"{
            "id": "mcp_1",
            "config": {
                "transport": "SSE",
                "name": "My MCP Server",
                "description": "A test MCP server",
                "request_headers": {}
            },
            "dependent_agents": [],
            "metadata": {
                "created_at": 1700000000
            }
        }"#;
        let resp: McpServerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, "mcp_1");
        assert_eq!(resp.config.name.as_deref(), Some("My MCP Server"));
        assert_eq!(resp.config.transport, Some(McpServerTransport::Sse));
        assert_eq!(resp.metadata.created_at, 1700000000);
    }

    #[test]
    fn mcp_servers_response_deserialize() {
        let json = r#"{
            "mcp_servers": [
                {
                    "id": "mcp_1",
                    "config": {
                        "name": "Server 1",
                        "request_headers": {}
                    },
                    "metadata": {
                        "created_at": 1700000000
                    }
                }
            ]
        }"#;
        let resp: McpServersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.mcp_servers.len(), 1);
        assert_eq!(resp.mcp_servers[0].id, "mcp_1");
    }

    // -- Batch Call -----------------------------------------------------------

    #[test]
    fn batch_call_response_deserialize() {
        let json = r#"{
            "id": "batch_1",
            "phone_number_id": "phone_1",
            "phone_provider": "twilio",
            "whatsapp_params": null,
            "name": "Outreach Campaign",
            "agent_id": "agent_1",
            "created_at_unix": 1700000000,
            "scheduled_time_unix": 1700010000,
            "timezone": "America/New_York",
            "total_calls_dispatched": 50,
            "total_calls_scheduled": 100,
            "total_calls_finished": 50,
            "last_updated_at_unix": 1700005000,
            "status": "in_progress",
            "retry_count": 0,
            "agent_name": "Outreach Bot"
        }"#;
        let resp: BatchCallResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, "batch_1");
        assert_eq!(resp.name, "Outreach Campaign");
        assert_eq!(resp.status, BatchCallStatus::InProgress);
        assert_eq!(resp.total_calls_dispatched, 50);
        assert_eq!(resp.total_calls_scheduled, 100);
        assert_eq!(resp.phone_provider, Some(TelephonyProvider::Twilio));
    }

    #[test]
    fn workspace_batch_calls_response_deserialize() {
        let json = r#"{
            "batch_calls": [],
            "next_doc": null,
            "has_more": false
        }"#;
        let resp: WorkspaceBatchCallsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.batch_calls.is_empty());
        assert!(!resp.has_more);
    }

    // -- Secrets --------------------------------------------------------------

    #[test]
    fn workspace_stored_secret_deserialize() {
        let json = r#"{
            "type": "stored",
            "secret_id": "sec_1",
            "name": "API Key",
            "used_by": {"tools": [], "agents": [], "others": []}
        }"#;
        let secret: WorkspaceStoredSecret = serde_json::from_str(json).unwrap();
        assert_eq!(secret.secret_type, "stored");
        assert_eq!(secret.secret_id, "sec_1");
        assert_eq!(secret.name, "API Key");
    }

    #[test]
    fn create_secret_request_serialize() {
        let req =
            CreateSecretRequest { name: "My Secret".into(), value: "super-secret-value".into() };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"name\":\"My Secret\""));
        assert!(json.contains("\"value\":\"super-secret-value\""));
    }

    // -- Settings -------------------------------------------------------------

    #[test]
    fn convai_settings_response_deserialize() {
        let json = r#"{
            "webhooks": {
                "events": ["transcript", "audio"]
            },
            "can_use_mcp_servers": true,
            "rag_retention_period_days": 15
        }"#;
        let resp: GetConvAiSettingsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.can_use_mcp_servers);
        assert_eq!(resp.rag_retention_period_days, 15);
        assert_eq!(resp.webhooks.events.len(), 2);
        assert_eq!(resp.webhooks.events[0], WebhookEventType::Transcript);
    }

    // -- WhatsApp -------------------------------------------------------------

    #[test]
    fn whatsapp_account_deserialize() {
        let json = r#"{
            "business_account_id": "ba_1",
            "phone_number_id": "pn_1",
            "business_account_name": "ACME Corp",
            "phone_number_name": "Main Line",
            "phone_number": "+1234567890",
            "assigned_agent_name": "Bot"
        }"#;
        let acct: WhatsAppAccount = serde_json::from_str(json).unwrap();
        assert_eq!(acct.business_account_id, "ba_1");
        assert_eq!(acct.business_account_name, "ACME Corp");
        assert_eq!(acct.phone_number, "+1234567890");
        assert_eq!(acct.assigned_agent_name.as_deref(), Some("Bot"));
    }

    // -- SIP Trunk Request ---------------------------------------------------

    #[test]
    fn sip_trunk_outbound_call_request_serialize() {
        let req = SipTrunkOutboundCallRequest {
            agent_id: "agent_1".into(),
            agent_phone_number_id: "phone_1".into(),
            to_number: "+9876543210".into(),
            conversation_initiation_client_data: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"agent_id\":\"agent_1\""));
        assert!(json.contains("\"to_number\":\"+9876543210\""));
        assert!(!json.contains("conversation_initiation_client_data"));
    }

    // -- Twilio ---------------------------------------------------------------

    #[test]
    fn twilio_outbound_call_request_serialize() {
        let req = TwilioOutboundCallRequest {
            agent_id: "agent_1".into(),
            agent_phone_number_id: "phone_1".into(),
            to_number: "+1234567890".into(),
            conversation_initiation_client_data: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"agent_id\":\"agent_1\""));
        assert!(json.contains("\"to_number\":\"+1234567890\""));
        assert!(!json.contains("conversation_initiation_client_data"));
    }

    #[test]
    fn twilio_outbound_call_response_deserialize() {
        let json = r#"{
            "success": true,
            "message": "Call initiated",
            "conversation_id": "conv_123",
            "callSid": "CA123"
        }"#;
        let resp: TwilioOutboundCallResponse = serde_json::from_str(json).unwrap();
        assert!(resp.success);
        assert_eq!(resp.message, "Call initiated");
        assert_eq!(resp.conversation_id.as_deref(), Some("conv_123"));
        assert_eq!(resp.call_sid.as_deref(), Some("CA123"));
    }

    #[test]
    fn twilio_register_call_request_serialize() {
        let req = TwilioRegisterCallRequest {
            agent_id: "agent_1".into(),
            from_number: "+1111111111".into(),
            to_number: "+2222222222".into(),
            direction: Some("inbound".into()),
            conversation_initiation_client_data: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"from_number\":\"+1111111111\""));
        assert!(json.contains("\"direction\":\"inbound\""));
    }

    // -- Conversation Users ---------------------------------------------------

    #[test]
    fn conversation_user_deserialize() {
        let json = r#"{
            "user_id": "user_1",
            "last_contact_unix_secs": 1700001000,
            "first_contact_unix_secs": 1700000000,
            "conversation_count": 5,
            "last_agent_id": "agent_1",
            "last_agent_name": "Bot"
        }"#;
        let user: ConversationUser = serde_json::from_str(json).unwrap();
        assert_eq!(user.user_id, "user_1");
        assert_eq!(user.conversation_count, 5);
        assert_eq!(user.last_agent_name.as_deref(), Some("Bot"));
    }

    #[test]
    fn get_conversation_users_response_deserialize() {
        let json = r#"{
            "users": [],
            "next_cursor": null,
            "has_more": false
        }"#;
        let resp: GetConversationUsersResponse = serde_json::from_str(json).unwrap();
        assert!(resp.users.is_empty());
        assert!(!resp.has_more);
    }

    // -- Tool Dependent Agents ------------------------------------------------

    #[test]
    fn get_tool_dependent_agents_response_deserialize() {
        let json = r#"{
            "agents": [{"type": "available", "agent_id": "a1", "name": "Bot"}],
            "next_cursor": null,
            "has_more": false
        }"#;
        let resp: GetToolDependentAgentsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.agents.len(), 1);
        assert!(!resp.has_more);
    }
}
