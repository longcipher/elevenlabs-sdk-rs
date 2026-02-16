//! Agents Platform (ConvAI) service providing access to ~110 endpoints.
//!
//! This module covers the full Conversational AI surface of the ElevenLabs
//! API, organised into the following groups:
//!
//! - **Agents** — CRUD, avatars, branches, deployments, drafts, duplication, link, widget
//! - **Conversations** — list, get, delete, audio, feedback, signed URL, token
//! - **Knowledge Base** — CRUD, documents, folders, RAG indexes, move/bulk-move
//! - **Tools** — CRUD
//! - **Phone Numbers** — CRUD
//! - **MCP Servers** — CRUD, tool configs, approval policies
//! - **Batch Calling** — submit, list, get, cancel, retry
//! - **Secrets** — CRUD
//! - **Settings** — workspace ConvAI settings, dashboard settings
//! - **Agent Testing** — test CRUD, summaries, invocations
//! - **Misc** — SIP trunk, analytics, LLM usage, WhatsApp

use bytes::Bytes;
use futures_core::Stream;

use crate::{
    client::ElevenLabsClient,
    error::Result,
    types::{
        AddKnowledgeBaseResponse, AgentBranchResponse, AgentDeploymentResponse, AgentLinkResponse,
        BatchCallResponse, ConversationFeedbackRequest, ConversationTokenResponse,
        CreateAgentRequest, CreateBranchRequest, CreateDeploymentRequest,
        CreateKnowledgeBaseFolderRequest, CreateKnowledgeBaseTextRequest,
        CreateKnowledgeBaseUrlRequest, CreatePhoneNumberResponse, CreateSecretRequest,
        GetAgentResponse, GetAgentSummariesResponse, GetAgentsResponse, GetConvAiSettingsResponse,
        GetConversationResponse, GetConversationUsersResponse, GetConversationsResponse,
        GetKnowledgeBaseListResponse, GetSecretsResponse, GetToolDependentAgentsResponse,
        GetToolsResponse, KnowledgeBaseBulkMoveRequest, KnowledgeBaseMoveRequest,
        ListPhoneNumbersResponse, ListWhatsAppAccountsResponse, LiveCountResponse,
        McpServerResponse, McpServersResponse, MergeBranchRequest, SignedUrlResponse,
        SipTrunkOutboundCallRequest, SubmitBatchCallRequest, ToolResponse,
        TwilioOutboundCallRequest, TwilioOutboundCallResponse, TwilioRegisterCallRequest,
        UpdateAgentRequest, UpdateBranchRequest, UpdateKnowledgeBaseDocumentRequest,
        UpdateSecretRequest, WhatsAppAccount, WhatsAppOutboundCallRequest,
        WhatsAppOutboundMessageRequest, WorkspaceBatchCallsResponse,
    },
};

/// Service for the ElevenLabs Agents Platform / ConvAI endpoints.
///
/// Obtained via [`ElevenLabsClient::agents`].
#[derive(Debug)]
pub struct AgentsService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> AgentsService<'a> {
    /// Creates a new `AgentsService` bound to the given client.
    pub(crate) const fn new(client: &'a ElevenLabsClient) -> Self {
        Self { client }
    }

    // =======================================================================
    // Agents — CRUD
    // =======================================================================

    /// Lists agents in the workspace.
    ///
    /// `GET /v1/convai/agents`
    ///
    /// Pass `cursor` to paginate through results.
    pub async fn list_agents(&self, cursor: Option<&str>) -> Result<GetAgentsResponse> {
        let mut path = "/v1/convai/agents".to_owned();
        if let Some(c) = cursor {
            append_query(&mut path, "cursor", c);
        }
        self.client.get(&path).await
    }

    /// Creates a new agent.
    ///
    /// `POST /v1/convai/agents/create`
    pub async fn create_agent(&self, request: &CreateAgentRequest) -> Result<GetAgentResponse> {
        self.client.post("/v1/convai/agents/create", request).await
    }

    /// Retrieves agent summaries (compact list, no full config).
    ///
    /// `GET /v1/convai/agents/summaries`
    pub async fn get_agent_summaries(
        &self,
        cursor: Option<&str>,
    ) -> Result<GetAgentSummariesResponse> {
        let mut path = "/v1/convai/agents/summaries".to_owned();
        if let Some(c) = cursor {
            append_query(&mut path, "cursor", c);
        }
        self.client.get(&path).await
    }

    /// Retrieves full agent details.
    ///
    /// `GET /v1/convai/agents/{agent_id}`
    pub async fn get_agent(&self, agent_id: &str) -> Result<GetAgentResponse> {
        let path = format!("/v1/convai/agents/{agent_id}");
        self.client.get(&path).await
    }

    /// Updates (patches) an existing agent.
    ///
    /// `PATCH /v1/convai/agents/{agent_id}`
    pub async fn update_agent(
        &self,
        agent_id: &str,
        request: &UpdateAgentRequest,
    ) -> Result<GetAgentResponse> {
        let path = format!("/v1/convai/agents/{agent_id}");
        self.client.patch(&path, request).await
    }

    /// Deletes an agent.
    ///
    /// `DELETE /v1/convai/agents/{agent_id}`
    pub async fn delete_agent(&self, agent_id: &str) -> Result<()> {
        let path = format!("/v1/convai/agents/{agent_id}");
        self.client.delete(&path).await
    }

    // =======================================================================
    // Agents — Avatar
    // =======================================================================

    /// Uploads an avatar image for an agent (multipart/form-data).
    ///
    /// `POST /v1/convai/agents/{agent_id}/avatar`
    ///
    /// # Arguments
    ///
    /// * `agent_id` — The agent to update.
    /// * `filename` — File name for the avatar image.
    /// * `content_type_value` — MIME type of the image (e.g. `image/png`).
    /// * `data` — Raw image bytes.
    pub async fn upload_avatar(
        &self,
        agent_id: &str,
        filename: &str,
        content_type_value: &str,
        data: &[u8],
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agents/{agent_id}/avatar");
        let boundary = multipart_boundary();
        let body =
            build_single_file_multipart(&boundary, "avatar", filename, content_type_value, data);
        let ct = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart(&path, body, &ct).await
    }

    // =======================================================================
    // Agents — Branches
    // =======================================================================

    /// Creates a new branch for an agent.
    ///
    /// `POST /v1/convai/agents/{agent_id}/branches`
    pub async fn create_branch(
        &self,
        agent_id: &str,
        request: &CreateBranchRequest,
    ) -> Result<AgentBranchResponse> {
        let path = format!("/v1/convai/agents/{agent_id}/branches");
        self.client.post(&path, request).await
    }

    /// Lists branches for an agent.
    ///
    /// `GET /v1/convai/agents/{agent_id}/branches`
    pub async fn list_branches(&self, agent_id: &str) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agents/{agent_id}/branches");
        self.client.get(&path).await
    }

    /// Retrieves a specific branch.
    ///
    /// `GET /v1/convai/agents/{agent_id}/branches/{branch_id}`
    pub async fn get_branch(&self, agent_id: &str, branch_id: &str) -> Result<AgentBranchResponse> {
        let path = format!("/v1/convai/agents/{agent_id}/branches/{branch_id}");
        self.client.get(&path).await
    }

    /// Updates a branch.
    ///
    /// `PATCH /v1/convai/agents/{agent_id}/branches/{branch_id}`
    pub async fn update_branch(
        &self,
        agent_id: &str,
        branch_id: &str,
        request: &UpdateBranchRequest,
    ) -> Result<AgentBranchResponse> {
        let path = format!("/v1/convai/agents/{agent_id}/branches/{branch_id}");
        self.client.patch(&path, request).await
    }

    /// Merges a source branch into a target branch.
    ///
    /// `POST /v1/convai/agents/{agent_id}/branches/{source_branch_id}/merge`
    pub async fn merge_branch(
        &self,
        agent_id: &str,
        source_branch_id: &str,
        request: &MergeBranchRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agents/{agent_id}/branches/{source_branch_id}/merge");
        self.client.post(&path, request).await
    }

    // =======================================================================
    // Agents — Deployments
    // =======================================================================

    /// Creates or updates a deployment (traffic split) for an agent.
    ///
    /// `POST /v1/convai/agents/{agent_id}/deployments`
    pub async fn create_deployment(
        &self,
        agent_id: &str,
        request: &CreateDeploymentRequest,
    ) -> Result<AgentDeploymentResponse> {
        let path = format!("/v1/convai/agents/{agent_id}/deployments");
        self.client.post(&path, request).await
    }

    // =======================================================================
    // Agents — Drafts
    // =======================================================================

    /// Creates a draft for an agent.
    ///
    /// `POST /v1/convai/agents/{agent_id}/drafts`
    pub async fn create_draft(
        &self,
        agent_id: &str,
        request: &UpdateAgentRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agents/{agent_id}/drafts");
        self.client.post(&path, request).await
    }

    /// Deletes the current draft for an agent.
    ///
    /// `DELETE /v1/convai/agents/{agent_id}/drafts`
    pub async fn delete_draft(&self, agent_id: &str) -> Result<()> {
        let path = format!("/v1/convai/agents/{agent_id}/drafts");
        self.client.delete(&path).await
    }

    // =======================================================================
    // Agents — Duplicate
    // =======================================================================

    /// Duplicates an agent.
    ///
    /// `POST /v1/convai/agents/{agent_id}/duplicate`
    pub async fn duplicate_agent(&self, agent_id: &str) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agents/{agent_id}/duplicate");
        self.client.post(&path, &serde_json::json!({})).await
    }

    // =======================================================================
    // Agents — Link & Widget
    // =======================================================================

    /// Retrieves the shareable link for an agent.
    ///
    /// `GET /v1/convai/agents/{agent_id}/link`
    pub async fn get_agent_link(&self, agent_id: &str) -> Result<AgentLinkResponse> {
        let path = format!("/v1/convai/agents/{agent_id}/link");
        self.client.get(&path).await
    }

    /// Retrieves the widget configuration for an agent.
    ///
    /// `GET /v1/convai/agents/{agent_id}/widget`
    pub async fn get_agent_widget(&self, agent_id: &str) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agents/{agent_id}/widget");
        self.client.get(&path).await
    }

    // =======================================================================
    // Agents — Test Suite & Simulation
    // =======================================================================

    /// Runs the test suite for an agent.
    ///
    /// `POST /v1/convai/agents/{agent_id}/run-tests`
    pub async fn run_agent_test_suite(
        &self,
        agent_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agents/{agent_id}/run-tests");
        self.client.post(&path, request).await
    }

    /// Runs a conversation simulation for an agent.
    ///
    /// `POST /v1/convai/agents/{agent_id}/simulate-conversation`
    pub async fn simulate_conversation(
        &self,
        agent_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agents/{agent_id}/simulate-conversation");
        self.client.post(&path, request).await
    }

    /// Runs a conversation simulation with streaming response.
    ///
    /// `POST /v1/convai/agents/{agent_id}/simulate-conversation/stream`
    pub async fn simulate_conversation_stream(
        &self,
        agent_id: &str,
        request: &serde_json::Value,
    ) -> Result<impl Stream<Item = std::result::Result<Bytes, hpx::Error>> + use<'_>> {
        let path = format!("/v1/convai/agents/{agent_id}/simulate-conversation/stream");
        self.client.post_stream(&path, request).await
    }

    // =======================================================================
    // Agents — Knowledge Base Size & LLM Usage (per-agent)
    // =======================================================================

    /// Retrieves the knowledge base size for an agent.
    ///
    /// `GET /v1/convai/agent/{agent_id}/knowledge-base/size`
    pub async fn get_agent_knowledge_base_size(&self, agent_id: &str) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agent/{agent_id}/knowledge-base/size");
        self.client.get(&path).await
    }

    /// Calculates expected LLM usage cost for an agent.
    ///
    /// `POST /v1/convai/agent/{agent_id}/llm-usage/calculate`
    pub async fn calculate_agent_llm_cost(
        &self,
        agent_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agent/{agent_id}/llm-usage/calculate");
        self.client.post(&path, request).await
    }

    // =======================================================================
    // Analytics
    // =======================================================================

    /// Retrieves the current live conversation count.
    ///
    /// `GET /v1/convai/analytics/live-count`
    pub async fn get_live_count(&self) -> Result<LiveCountResponse> {
        self.client.get("/v1/convai/analytics/live-count").await
    }

    // =======================================================================
    // Batch Calling
    // =======================================================================

    /// Submits a new batch call job.
    ///
    /// `POST /v1/convai/batch-calling/submit`
    pub async fn submit_batch_call(
        &self,
        request: &SubmitBatchCallRequest,
    ) -> Result<BatchCallResponse> {
        self.client.post("/v1/convai/batch-calling/submit", request).await
    }

    /// Lists workspace batch calls.
    ///
    /// `GET /v1/convai/batch-calling/workspace`
    pub async fn list_batch_calls(
        &self,
        cursor: Option<&str>,
    ) -> Result<WorkspaceBatchCallsResponse> {
        let mut path = "/v1/convai/batch-calling/workspace".to_owned();
        if let Some(c) = cursor {
            append_query(&mut path, "cursor", c);
        }
        self.client.get(&path).await
    }

    /// Retrieves a specific batch call.
    ///
    /// `GET /v1/convai/batch-calling/{batch_id}`
    pub async fn get_batch_call(&self, batch_id: &str) -> Result<BatchCallResponse> {
        let path = format!("/v1/convai/batch-calling/{batch_id}");
        self.client.get(&path).await
    }

    /// Deletes a batch call.
    ///
    /// `DELETE /v1/convai/batch-calling/{batch_id}`
    pub async fn delete_batch_call(&self, batch_id: &str) -> Result<()> {
        let path = format!("/v1/convai/batch-calling/{batch_id}");
        self.client.delete(&path).await
    }

    /// Cancels a running batch call.
    ///
    /// `POST /v1/convai/batch-calling/{batch_id}/cancel`
    pub async fn cancel_batch_call(&self, batch_id: &str) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/batch-calling/{batch_id}/cancel");
        self.client.post(&path, &serde_json::json!({})).await
    }

    /// Retries a failed batch call.
    ///
    /// `POST /v1/convai/batch-calling/{batch_id}/retry`
    pub async fn retry_batch_call(&self, batch_id: &str) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/batch-calling/{batch_id}/retry");
        self.client.post(&path, &serde_json::json!({})).await
    }

    // =======================================================================
    // Conversations
    // =======================================================================

    /// Retrieves a signed URL for a conversation.
    ///
    /// `GET /v1/convai/conversation/get-signed-url`
    pub async fn get_conversation_signed_url(&self, agent_id: &str) -> Result<SignedUrlResponse> {
        let mut path = "/v1/convai/conversation/get-signed-url".to_owned();
        append_query(&mut path, "agent_id", agent_id);
        self.client.get(&path).await
    }

    /// Retrieves a LiveKit token for a conversation.
    ///
    /// `GET /v1/convai/conversation/token`
    pub async fn get_conversation_token(
        &self,
        agent_id: &str,
    ) -> Result<ConversationTokenResponse> {
        let mut path = "/v1/convai/conversation/token".to_owned();
        append_query(&mut path, "agent_id", agent_id);
        self.client.get(&path).await
    }

    /// Lists conversation histories.
    ///
    /// `GET /v1/convai/conversations`
    pub async fn list_conversations(
        &self,
        agent_id: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<GetConversationsResponse> {
        let mut path = "/v1/convai/conversations".to_owned();
        if let Some(id) = agent_id {
            append_query(&mut path, "agent_id", id);
        }
        if let Some(c) = cursor {
            append_query(&mut path, "cursor", c);
        }
        self.client.get(&path).await
    }

    /// Retrieves a single conversation history.
    ///
    /// `GET /v1/convai/conversations/{conversation_id}`
    pub async fn get_conversation(&self, conversation_id: &str) -> Result<GetConversationResponse> {
        let path = format!("/v1/convai/conversations/{conversation_id}");
        self.client.get(&path).await
    }

    /// Deletes a conversation.
    ///
    /// `DELETE /v1/convai/conversations/{conversation_id}`
    pub async fn delete_conversation(&self, conversation_id: &str) -> Result<()> {
        let path = format!("/v1/convai/conversations/{conversation_id}");
        self.client.delete(&path).await
    }

    /// Retrieves conversation audio as raw bytes.
    ///
    /// `GET /v1/convai/conversations/{conversation_id}/audio`
    pub async fn get_conversation_audio(&self, conversation_id: &str) -> Result<Bytes> {
        let path = format!("/v1/convai/conversations/{conversation_id}/audio");
        self.client.get_bytes(&path).await
    }

    /// Posts feedback for a conversation.
    ///
    /// `POST /v1/convai/conversations/{conversation_id}/feedback`
    pub async fn post_conversation_feedback(
        &self,
        conversation_id: &str,
        request: &ConversationFeedbackRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/conversations/{conversation_id}/feedback");
        self.client.post(&path, request).await
    }

    // =======================================================================
    // Knowledge Base
    // =======================================================================

    /// Adds a documentation entry to the knowledge base.
    ///
    /// `POST /v1/convai/knowledge-base`
    pub async fn add_knowledge_base_document(
        &self,
        request: &serde_json::Value,
    ) -> Result<AddKnowledgeBaseResponse> {
        self.client.post("/v1/convai/knowledge-base", request).await
    }

    /// Lists knowledge base documents.
    ///
    /// `GET /v1/convai/knowledge-base`
    pub async fn list_knowledge_base(
        &self,
        cursor: Option<&str>,
        folder_id: Option<&str>,
    ) -> Result<GetKnowledgeBaseListResponse> {
        let mut path = "/v1/convai/knowledge-base".to_owned();
        if let Some(c) = cursor {
            append_query(&mut path, "cursor", c);
        }
        if let Some(f) = folder_id {
            append_query(&mut path, "folder_id", f);
        }
        self.client.get(&path).await
    }

    /// Bulk-moves knowledge base documents to a folder.
    ///
    /// `POST /v1/convai/knowledge-base/bulk-move`
    pub async fn bulk_move_knowledge_base(
        &self,
        request: &KnowledgeBaseBulkMoveRequest,
    ) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/knowledge-base/bulk-move", request).await
    }

    /// Creates a file-based knowledge base document (multipart upload).
    ///
    /// `POST /v1/convai/knowledge-base/file`
    ///
    /// # Arguments
    ///
    /// * `filename` — Name of the file.
    /// * `content_type_value` — MIME type (e.g. `application/pdf`).
    /// * `data` — Raw file bytes.
    /// * `name` — Optional display name for the document.
    /// * `parent_folder_id` — Optional parent folder ID.
    pub async fn create_knowledge_base_file(
        &self,
        filename: &str,
        content_type_value: &str,
        data: &[u8],
        name: Option<&str>,
        parent_folder_id: Option<&str>,
    ) -> Result<AddKnowledgeBaseResponse> {
        let boundary = multipart_boundary();
        let mut buf = Vec::new();

        if let Some(n) = name {
            append_text_field(&mut buf, &boundary, "name", n);
        }
        if let Some(f) = parent_folder_id {
            append_text_field(&mut buf, &boundary, "parent_folder_id", f);
        }
        append_file_part(&mut buf, &boundary, "file", filename, content_type_value, data);
        buf.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

        let ct = format!("multipart/form-data; boundary={boundary}");
        self.client.post_multipart("/v1/convai/knowledge-base/file", buf, &ct).await
    }

    /// Creates a knowledge base folder.
    ///
    /// `POST /v1/convai/knowledge-base/folder`
    pub async fn create_knowledge_base_folder(
        &self,
        request: &CreateKnowledgeBaseFolderRequest,
    ) -> Result<AddKnowledgeBaseResponse> {
        self.client.post("/v1/convai/knowledge-base/folder", request).await
    }

    /// Gets or creates a RAG index.
    ///
    /// `POST /v1/convai/knowledge-base/rag-index`
    pub async fn get_or_create_rag_index(
        &self,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/knowledge-base/rag-index", request).await
    }

    /// Retrieves the RAG index overview.
    ///
    /// `GET /v1/convai/knowledge-base/rag-index`
    pub async fn get_rag_index_overview(&self) -> Result<serde_json::Value> {
        self.client.get("/v1/convai/knowledge-base/rag-index").await
    }

    /// Retrieves knowledge base document summaries.
    ///
    /// `GET /v1/convai/knowledge-base/summaries`
    pub async fn get_knowledge_base_summaries(&self) -> Result<serde_json::Value> {
        self.client.get("/v1/convai/knowledge-base/summaries").await
    }

    /// Creates a text-based knowledge base document.
    ///
    /// `POST /v1/convai/knowledge-base/text`
    pub async fn create_knowledge_base_text(
        &self,
        request: &CreateKnowledgeBaseTextRequest,
    ) -> Result<AddKnowledgeBaseResponse> {
        self.client.post("/v1/convai/knowledge-base/text", request).await
    }

    /// Creates a URL-based knowledge base document.
    ///
    /// `POST /v1/convai/knowledge-base/url`
    pub async fn create_knowledge_base_url(
        &self,
        request: &CreateKnowledgeBaseUrlRequest,
    ) -> Result<AddKnowledgeBaseResponse> {
        self.client.post("/v1/convai/knowledge-base/url", request).await
    }

    /// Moves a knowledge base document to a folder.
    ///
    /// `POST /v1/convai/knowledge-base/{document_id}/move`
    pub async fn move_knowledge_base_document(
        &self,
        document_id: &str,
        request: &KnowledgeBaseMoveRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/knowledge-base/{document_id}/move");
        self.client.post(&path, request).await
    }

    /// Updates a knowledge base document's metadata.
    ///
    /// `PATCH /v1/convai/knowledge-base/{documentation_id}`
    pub async fn update_knowledge_base_document(
        &self,
        documentation_id: &str,
        request: &UpdateKnowledgeBaseDocumentRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/knowledge-base/{documentation_id}");
        self.client.patch(&path, request).await
    }

    /// Retrieves a knowledge base document.
    ///
    /// `GET /v1/convai/knowledge-base/{documentation_id}`
    pub async fn get_knowledge_base_document(
        &self,
        documentation_id: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/knowledge-base/{documentation_id}");
        self.client.get(&path).await
    }

    /// Deletes a knowledge base document.
    ///
    /// `DELETE /v1/convai/knowledge-base/{documentation_id}`
    pub async fn delete_knowledge_base_document(&self, documentation_id: &str) -> Result<()> {
        let path = format!("/v1/convai/knowledge-base/{documentation_id}");
        self.client.delete(&path).await
    }

    /// Retrieves a specific chunk from a knowledge base document.
    ///
    /// `GET /v1/convai/knowledge-base/{documentation_id}/chunk/{chunk_id}`
    pub async fn get_knowledge_base_chunk(
        &self,
        documentation_id: &str,
        chunk_id: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/knowledge-base/{documentation_id}/chunk/{chunk_id}");
        self.client.get(&path).await
    }

    /// Retrieves the content of a knowledge base document.
    ///
    /// `GET /v1/convai/knowledge-base/{documentation_id}/content`
    pub async fn get_knowledge_base_content(
        &self,
        documentation_id: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/knowledge-base/{documentation_id}/content");
        self.client.get(&path).await
    }

    /// Retrieves agents that depend on a knowledge base document.
    ///
    /// `GET /v1/convai/knowledge-base/{documentation_id}/dependent-agents`
    pub async fn get_knowledge_base_dependent_agents(
        &self,
        documentation_id: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/knowledge-base/{documentation_id}/dependent-agents");
        self.client.get(&path).await
    }

    /// Creates or checks a RAG index for a document.
    ///
    /// `POST /v1/convai/knowledge-base/{documentation_id}/rag-index`
    pub async fn create_document_rag_index(
        &self,
        documentation_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/knowledge-base/{documentation_id}/rag-index");
        self.client.post(&path, request).await
    }

    /// Retrieves RAG indexes for a document.
    ///
    /// `GET /v1/convai/knowledge-base/{documentation_id}/rag-index`
    pub async fn get_document_rag_indexes(
        &self,
        documentation_id: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/knowledge-base/{documentation_id}/rag-index");
        self.client.get(&path).await
    }

    /// Deletes a RAG index for a document.
    ///
    /// `DELETE /v1/convai/knowledge-base/{documentation_id}/rag-index/{rag_index_id}`
    pub async fn delete_document_rag_index(
        &self,
        documentation_id: &str,
        rag_index_id: &str,
    ) -> Result<()> {
        let path = format!("/v1/convai/knowledge-base/{documentation_id}/rag-index/{rag_index_id}");
        self.client.delete(&path).await
    }

    /// Retrieves the source file URL for a knowledge base document.
    ///
    /// `GET /v1/convai/knowledge-base/{documentation_id}/source-file-url`
    pub async fn get_knowledge_base_source_file_url(
        &self,
        documentation_id: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/knowledge-base/{documentation_id}/source-file-url");
        self.client.get(&path).await
    }

    // =======================================================================
    // LLM Usage (public)
    // =======================================================================

    /// Calculates public LLM expected cost.
    ///
    /// `POST /v1/convai/llm-usage/calculate`
    pub async fn calculate_public_llm_cost(
        &self,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/llm-usage/calculate", request).await
    }

    // =======================================================================
    // MCP Servers
    // =======================================================================

    /// Creates a new MCP server.
    ///
    /// `POST /v1/convai/mcp-servers`
    pub async fn create_mcp_server(
        &self,
        request: &serde_json::Value,
    ) -> Result<McpServerResponse> {
        self.client.post("/v1/convai/mcp-servers", request).await
    }

    /// Lists MCP servers in the workspace.
    ///
    /// `GET /v1/convai/mcp-servers`
    pub async fn list_mcp_servers(&self) -> Result<McpServersResponse> {
        self.client.get("/v1/convai/mcp-servers").await
    }

    /// Retrieves a specific MCP server.
    ///
    /// `GET /v1/convai/mcp-servers/{mcp_server_id}`
    pub async fn get_mcp_server(&self, mcp_server_id: &str) -> Result<McpServerResponse> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}");
        self.client.get(&path).await
    }

    /// Deletes an MCP server.
    ///
    /// `DELETE /v1/convai/mcp-servers/{mcp_server_id}`
    pub async fn delete_mcp_server(&self, mcp_server_id: &str) -> Result<()> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}");
        self.client.delete(&path).await
    }

    /// Updates an MCP server configuration.
    ///
    /// `PATCH /v1/convai/mcp-servers/{mcp_server_id}`
    pub async fn update_mcp_server(
        &self,
        mcp_server_id: &str,
        request: &serde_json::Value,
    ) -> Result<McpServerResponse> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}");
        self.client.patch(&path, request).await
    }

    /// Updates the approval policy for an MCP server.
    ///
    /// `PATCH /v1/convai/mcp-servers/{mcp_server_id}/approval-policy`
    pub async fn update_mcp_server_approval_policy(
        &self,
        mcp_server_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}/approval-policy");
        self.client.patch(&path, request).await
    }

    /// Adds a tool approval to an MCP server.
    ///
    /// `POST /v1/convai/mcp-servers/{mcp_server_id}/tool-approvals`
    pub async fn add_mcp_server_tool_approval(
        &self,
        mcp_server_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}/tool-approvals");
        self.client.post(&path, request).await
    }

    /// Removes a tool approval from an MCP server.
    ///
    /// `DELETE /v1/convai/mcp-servers/{mcp_server_id}/tool-approvals/{tool_name}`
    pub async fn remove_mcp_server_tool_approval(
        &self,
        mcp_server_id: &str,
        tool_name: &str,
    ) -> Result<()> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}/tool-approvals/{tool_name}");
        self.client.delete(&path).await
    }

    /// Adds a tool config override to an MCP server.
    ///
    /// `POST /v1/convai/mcp-servers/{mcp_server_id}/tool-configs`
    pub async fn add_mcp_tool_config(
        &self,
        mcp_server_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}/tool-configs");
        self.client.post(&path, request).await
    }

    /// Retrieves a tool config override from an MCP server.
    ///
    /// `GET /v1/convai/mcp-servers/{mcp_server_id}/tool-configs/{tool_name}`
    pub async fn get_mcp_tool_config(
        &self,
        mcp_server_id: &str,
        tool_name: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}/tool-configs/{tool_name}");
        self.client.get(&path).await
    }

    /// Updates a tool config override on an MCP server.
    ///
    /// `PATCH /v1/convai/mcp-servers/{mcp_server_id}/tool-configs/{tool_name}`
    pub async fn update_mcp_tool_config(
        &self,
        mcp_server_id: &str,
        tool_name: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}/tool-configs/{tool_name}");
        self.client.patch(&path, request).await
    }

    /// Removes a tool config override from an MCP server.
    ///
    /// `DELETE /v1/convai/mcp-servers/{mcp_server_id}/tool-configs/{tool_name}`
    pub async fn remove_mcp_tool_config(&self, mcp_server_id: &str, tool_name: &str) -> Result<()> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}/tool-configs/{tool_name}");
        self.client.delete(&path).await
    }

    /// Lists tools available on an MCP server.
    ///
    /// `GET /v1/convai/mcp-servers/{mcp_server_id}/tools`
    pub async fn list_mcp_server_tools(&self, mcp_server_id: &str) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/mcp-servers/{mcp_server_id}/tools");
        self.client.get(&path).await
    }

    // =======================================================================
    // Phone Numbers
    // =======================================================================

    /// Creates a phone number.
    ///
    /// `POST /v1/convai/phone-numbers`
    pub async fn create_phone_number(
        &self,
        request: &serde_json::Value,
    ) -> Result<CreatePhoneNumberResponse> {
        self.client.post("/v1/convai/phone-numbers", request).await
    }

    /// Lists phone numbers in the workspace.
    ///
    /// `GET /v1/convai/phone-numbers`
    pub async fn list_phone_numbers(&self) -> Result<ListPhoneNumbersResponse> {
        self.client.get("/v1/convai/phone-numbers").await
    }

    /// Retrieves a specific phone number.
    ///
    /// `GET /v1/convai/phone-numbers/{phone_number_id}`
    pub async fn get_phone_number(&self, phone_number_id: &str) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/phone-numbers/{phone_number_id}");
        self.client.get(&path).await
    }

    /// Deletes a phone number.
    ///
    /// `DELETE /v1/convai/phone-numbers/{phone_number_id}`
    pub async fn delete_phone_number(&self, phone_number_id: &str) -> Result<()> {
        let path = format!("/v1/convai/phone-numbers/{phone_number_id}");
        self.client.delete(&path).await
    }

    /// Updates a phone number.
    ///
    /// `PATCH /v1/convai/phone-numbers/{phone_number_id}`
    pub async fn update_phone_number(
        &self,
        phone_number_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/phone-numbers/{phone_number_id}");
        self.client.patch(&path, request).await
    }

    // =======================================================================
    // Secrets
    // =======================================================================

    /// Creates a workspace secret.
    ///
    /// `POST /v1/convai/secrets`
    pub async fn create_secret(&self, request: &CreateSecretRequest) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/secrets", request).await
    }

    /// Lists workspace secrets.
    ///
    /// `GET /v1/convai/secrets`
    pub async fn list_secrets(&self) -> Result<GetSecretsResponse> {
        self.client.get("/v1/convai/secrets").await
    }

    /// Deletes a workspace secret.
    ///
    /// `DELETE /v1/convai/secrets/{secret_id}`
    pub async fn delete_secret(&self, secret_id: &str) -> Result<()> {
        let path = format!("/v1/convai/secrets/{secret_id}");
        self.client.delete(&path).await
    }

    /// Updates a workspace secret.
    ///
    /// `PATCH /v1/convai/secrets/{secret_id}`
    pub async fn update_secret(
        &self,
        secret_id: &str,
        request: &UpdateSecretRequest,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/secrets/{secret_id}");
        self.client.patch(&path, request).await
    }

    // =======================================================================
    // Settings
    // =======================================================================

    /// Retrieves workspace ConvAI settings.
    ///
    /// `GET /v1/convai/settings`
    pub async fn get_settings(&self) -> Result<GetConvAiSettingsResponse> {
        self.client.get("/v1/convai/settings").await
    }

    /// Updates workspace ConvAI settings.
    ///
    /// `PATCH /v1/convai/settings`
    pub async fn update_settings(
        &self,
        request: &serde_json::Value,
    ) -> Result<GetConvAiSettingsResponse> {
        self.client.patch("/v1/convai/settings", request).await
    }

    /// Retrieves dashboard settings.
    ///
    /// `GET /v1/convai/settings/dashboard`
    pub async fn get_dashboard_settings(&self) -> Result<serde_json::Value> {
        self.client.get("/v1/convai/settings/dashboard").await
    }

    /// Updates dashboard settings.
    ///
    /// `PATCH /v1/convai/settings/dashboard`
    pub async fn update_dashboard_settings(
        &self,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.client.patch("/v1/convai/settings/dashboard", request).await
    }

    // =======================================================================
    // SIP Trunk
    // =======================================================================

    /// Makes an outbound call via SIP trunk.
    ///
    /// `POST /v1/convai/sip-trunk/outbound-call`
    pub async fn sip_trunk_outbound_call(
        &self,
        request: &SipTrunkOutboundCallRequest,
    ) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/sip-trunk/outbound-call", request).await
    }

    // =======================================================================
    // Agent Testing
    // =======================================================================

    /// Lists agent response tests.
    ///
    /// `GET /v1/convai/agent-testing`
    pub async fn list_agent_tests(&self, agent_id: Option<&str>) -> Result<serde_json::Value> {
        let mut path = "/v1/convai/agent-testing".to_owned();
        if let Some(id) = agent_id {
            append_query(&mut path, "agent_id", id);
        }
        self.client.get(&path).await
    }

    /// Creates an agent response test.
    ///
    /// `POST /v1/convai/agent-testing/create`
    pub async fn create_agent_test(
        &self,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/agent-testing/create", request).await
    }

    /// Retrieves agent response test summaries.
    ///
    /// `POST /v1/convai/agent-testing/summaries`
    pub async fn get_agent_test_summaries(
        &self,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/agent-testing/summaries", request).await
    }

    /// Retrieves a specific agent response test.
    ///
    /// `GET /v1/convai/agent-testing/{test_id}`
    pub async fn get_agent_test(&self, test_id: &str) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agent-testing/{test_id}");
        self.client.get(&path).await
    }

    /// Updates an agent response test.
    ///
    /// `PUT /v1/convai/agent-testing/{test_id}`
    pub async fn update_agent_test(
        &self,
        test_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/agent-testing/{test_id}");
        self.client.put(&path, request).await
    }

    /// Deletes an agent response test.
    ///
    /// `DELETE /v1/convai/agent-testing/{test_id}`
    pub async fn delete_agent_test(&self, test_id: &str) -> Result<()> {
        let path = format!("/v1/convai/agent-testing/{test_id}");
        self.client.delete(&path).await
    }

    // =======================================================================
    // Test Invocations
    // =======================================================================

    /// Lists test invocations.
    ///
    /// `GET /v1/convai/test-invocations`
    pub async fn list_test_invocations(&self, agent_id: Option<&str>) -> Result<serde_json::Value> {
        let mut path = "/v1/convai/test-invocations".to_owned();
        if let Some(id) = agent_id {
            append_query(&mut path, "agent_id", id);
        }
        self.client.get(&path).await
    }

    /// Retrieves a specific test invocation.
    ///
    /// `GET /v1/convai/test-invocations/{test_invocation_id}`
    pub async fn get_test_invocation(&self, test_invocation_id: &str) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/test-invocations/{test_invocation_id}");
        self.client.get(&path).await
    }

    /// Resubmits tests from a test invocation.
    ///
    /// `POST /v1/convai/test-invocations/{test_invocation_id}/resubmit`
    pub async fn resubmit_test_invocation(
        &self,
        test_invocation_id: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1/convai/test-invocations/{test_invocation_id}/resubmit");
        self.client.post(&path, &serde_json::json!({})).await
    }

    // =======================================================================
    // Tools
    // =======================================================================

    /// Creates a new tool.
    ///
    /// `POST /v1/convai/tools`
    pub async fn create_tool(&self, request: &serde_json::Value) -> Result<ToolResponse> {
        self.client.post("/v1/convai/tools", request).await
    }

    /// Lists all tools in the workspace.
    ///
    /// `GET /v1/convai/tools`
    pub async fn list_tools(&self) -> Result<GetToolsResponse> {
        self.client.get("/v1/convai/tools").await
    }

    /// Retrieves a specific tool.
    ///
    /// `GET /v1/convai/tools/{tool_id}`
    pub async fn get_tool(&self, tool_id: &str) -> Result<ToolResponse> {
        let path = format!("/v1/convai/tools/{tool_id}");
        self.client.get(&path).await
    }

    /// Updates a tool.
    ///
    /// `PATCH /v1/convai/tools/{tool_id}`
    pub async fn update_tool(
        &self,
        tool_id: &str,
        request: &serde_json::Value,
    ) -> Result<ToolResponse> {
        let path = format!("/v1/convai/tools/{tool_id}");
        self.client.patch(&path, request).await
    }

    /// Deletes a tool.
    ///
    /// `DELETE /v1/convai/tools/{tool_id}`
    pub async fn delete_tool(&self, tool_id: &str) -> Result<()> {
        let path = format!("/v1/convai/tools/{tool_id}");
        self.client.delete(&path).await
    }

    // =======================================================================
    // WhatsApp
    // =======================================================================

    /// Creates a WhatsApp account connection.
    ///
    /// `POST /v1/convai/whatsapp-accounts`
    pub async fn create_whatsapp_account(
        &self,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/whatsapp-accounts", request).await
    }

    /// Lists WhatsApp accounts.
    ///
    /// `GET /v1/convai/whatsapp-accounts`
    pub async fn list_whatsapp_accounts(&self) -> Result<ListWhatsAppAccountsResponse> {
        self.client.get("/v1/convai/whatsapp-accounts").await
    }

    /// Retrieves a specific WhatsApp account by phone number.
    ///
    /// `GET /v1/convai/whatsapp-accounts/{phone_number}`
    pub async fn get_whatsapp_account(&self, phone_number: &str) -> Result<WhatsAppAccount> {
        let path = format!("/v1/convai/whatsapp-accounts/{phone_number}");
        self.client.get(&path).await
    }

    /// Deletes a WhatsApp account.
    ///
    /// `DELETE /v1/convai/whatsapp-accounts/{phone_number}`
    pub async fn delete_whatsapp_account(&self, phone_number: &str) -> Result<()> {
        let path = format!("/v1/convai/whatsapp-accounts/{phone_number}");
        self.client.delete(&path).await
    }

    /// Makes an outbound WhatsApp call.
    ///
    /// `POST /v1/convai/whatsapp/outbound-call`
    pub async fn whatsapp_outbound_call(
        &self,
        request: &WhatsAppOutboundCallRequest,
    ) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/whatsapp/outbound-call", request).await
    }

    /// Sends an outbound WhatsApp message.
    ///
    /// `POST /v1/convai/whatsapp/outbound-message`
    pub async fn whatsapp_outbound_message(
        &self,
        request: &WhatsAppOutboundMessageRequest,
    ) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/whatsapp/outbound-message", request).await
    }

    // =======================================================================
    // Twilio
    // =======================================================================

    /// Makes an outbound call via Twilio.
    ///
    /// `POST /v1/convai/twilio/outbound-call`
    pub async fn twilio_outbound_call(
        &self,
        request: &TwilioOutboundCallRequest,
    ) -> Result<TwilioOutboundCallResponse> {
        self.client.post("/v1/convai/twilio/outbound-call", request).await
    }

    /// Registers a Twilio call and returns TwiML.
    ///
    /// `POST /v1/convai/twilio/register-call`
    ///
    /// The response is returned as a raw string containing TwiML XML.
    pub async fn twilio_register_call(
        &self,
        request: &TwilioRegisterCallRequest,
    ) -> Result<serde_json::Value> {
        self.client.post("/v1/convai/twilio/register-call", request).await
    }

    // =======================================================================
    // Users
    // =======================================================================

    /// Lists distinct conversation users with pagination.
    ///
    /// `GET /v1/convai/users`
    pub async fn get_conversation_users(
        &self,
        agent_id: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<GetConversationUsersResponse> {
        let mut path = "/v1/convai/users".to_owned();
        if let Some(id) = agent_id {
            append_query(&mut path, "agent_id", id);
        }
        if let Some(c) = cursor {
            append_query(&mut path, "cursor", c);
        }
        self.client.get(&path).await
    }

    // =======================================================================
    // Tool Dependent Agents
    // =======================================================================

    /// Gets a list of agents depending on a specific tool.
    ///
    /// `GET /v1/convai/tools/{tool_id}/dependent-agents`
    pub async fn get_tool_dependent_agents(
        &self,
        tool_id: &str,
        cursor: Option<&str>,
    ) -> Result<GetToolDependentAgentsResponse> {
        let mut path = format!("/v1/convai/tools/{tool_id}/dependent-agents");
        if let Some(c) = cursor {
            append_query(&mut path, "cursor", c);
        }
        self.client.get(&path).await
    }
}

// ---------------------------------------------------------------------------
// Query-string helper
// ---------------------------------------------------------------------------

/// Appends a query parameter to a URL path string.
fn append_query(path: &mut String, key: &str, value: &str) {
    if path.contains('?') {
        path.push('&');
    } else {
        path.push('?');
    }
    path.push_str(key);
    path.push('=');
    path.push_str(value);
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

    use super::*;

    fn test_config(base_url: &str) -> crate::config::ClientConfig {
        crate::config::ClientConfig::builder("test-key")
            .base_url(base_url)
            .max_retries(0_u32)
            .build()
    }

    // -- Agents CRUD ---------------------------------------------------------

    #[tokio::test]
    async fn test_list_agents() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/agents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "agents": [],
                "next_cursor": null,
                "has_more": false
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().list_agents(None).await.unwrap();
        assert!(result.agents.is_empty());
        assert!(!result.has_more);
    }

    #[tokio::test]
    async fn test_create_agent() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("POST"))
            .and(path("/v1/convai/agents/create"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "agent_id": "new_agent_123",
                "name": "Test Agent",
                "conversation_config": {},
                "metadata": {
                    "created_at_unix_secs": 1700000000,
                    "updated_at_unix_secs": 1700000000
                },
                "platform_settings": {},
                "tags": []
            })))
            .mount(&mock_server)
            .await;

        let req = CreateAgentRequest {
            conversation_config: None,
            platform_settings: None,
            workflow: None,
            name: Some("Test Agent".into()),
            tags: None,
        };
        let result = client.agents().create_agent(&req).await.unwrap();
        assert_eq!(result.agent_id, "new_agent_123");
        assert_eq!(result.name, "Test Agent");
    }

    #[tokio::test]
    async fn test_get_agent() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/agents/agent_xyz"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "agent_id": "agent_xyz",
                "name": "Support Bot",
                "conversation_config": {},
                "metadata": {
                    "created_at_unix_secs": 1700000000,
                    "updated_at_unix_secs": 1700001000
                },
                "platform_settings": {},
                "tags": []
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().get_agent("agent_xyz").await.unwrap();
        assert_eq!(result.agent_id, "agent_xyz");
        assert_eq!(result.name, "Support Bot");
    }

    #[tokio::test]
    async fn test_delete_agent() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("DELETE"))
            .and(path("/v1/convai/agents/agent_xyz"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        client.agents().delete_agent("agent_xyz").await.unwrap();
    }

    // -- Conversations -------------------------------------------------------

    #[tokio::test]
    async fn test_list_conversations() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/conversations"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "conversations": [],
                "next_cursor": null,
                "has_more": false
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().list_conversations(None, None).await.unwrap();
        assert!(result.conversations.is_empty());
    }

    #[tokio::test]
    async fn test_get_conversation() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/conversations/conv_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "agent_id": "agent_1",
                "status": "done",
                "transcript": [],
                "metadata": {
                    "start_time_unix_secs": 1700000000,
                    "call_duration_secs": 30,
                    "deletion_settings": {},
                    "feedback": {"likes": 0, "dislikes": 0},
                    "charging": {}
                },
                "conversation_id": "conv_1",
                "has_audio": false,
                "has_user_audio": false,
                "has_response_audio": false
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().get_conversation("conv_1").await.unwrap();
        assert_eq!(result.conversation_id, "conv_1");
    }

    // -- Knowledge Base ------------------------------------------------------

    #[tokio::test]
    async fn test_list_knowledge_base() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/knowledge-base"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "documents": [],
                "next_cursor": null,
                "has_more": false
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().list_knowledge_base(None, None).await.unwrap();
        assert!(result.documents.is_empty());
    }

    #[tokio::test]
    async fn test_create_knowledge_base_url() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("POST"))
            .and(path("/v1/convai/knowledge-base/url"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "doc_new",
                "name": "FAQ Page"
            })))
            .mount(&mock_server)
            .await;

        let req = CreateKnowledgeBaseUrlRequest {
            url: "https://example.com/faq".into(),
            name: Some("FAQ Page".into()),
            parent_folder_id: None,
        };
        let result = client.agents().create_knowledge_base_url(&req).await.unwrap();
        assert_eq!(result.id, "doc_new");
        assert_eq!(result.name, "FAQ Page");
    }

    // -- Tools ---------------------------------------------------------------

    #[tokio::test]
    async fn test_list_tools() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/tools"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "tools": []
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().list_tools().await.unwrap();
        assert!(result.tools.is_empty());
    }

    // -- MCP Servers ---------------------------------------------------------

    #[tokio::test]
    async fn test_list_mcp_servers() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/mcp-servers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "mcp_servers": []
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().list_mcp_servers().await.unwrap();
        assert!(result.mcp_servers.is_empty());
    }

    // -- Batch Calling -------------------------------------------------------

    #[tokio::test]
    async fn test_list_batch_calls() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/batch-calling/workspace"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "batch_calls": [],
                "next_doc": null,
                "has_more": false
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().list_batch_calls(None).await.unwrap();
        assert!(result.batch_calls.is_empty());
    }

    // -- Secrets -------------------------------------------------------------

    #[tokio::test]
    async fn test_list_secrets() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/secrets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "secrets": []
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().list_secrets().await.unwrap();
        assert!(result.secrets.is_empty());
    }

    // -- Settings ------------------------------------------------------------

    #[tokio::test]
    async fn test_get_settings() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "webhooks": {
                    "events": []
                },
                "can_use_mcp_servers": false,
                "rag_retention_period_days": 10
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().get_settings().await.unwrap();
        assert!(!result.can_use_mcp_servers);
        assert_eq!(result.rag_retention_period_days, 10);
    }

    // -- Phone Numbers -------------------------------------------------------

    #[tokio::test]
    async fn test_list_phone_numbers() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/phone-numbers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "phone_numbers": []
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().list_phone_numbers().await.unwrap();
        assert!(result.phone_numbers.is_empty());
    }

    // -- WhatsApp ------------------------------------------------------------

    #[tokio::test]
    async fn test_list_whatsapp_accounts() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/whatsapp-accounts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "accounts": []
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().list_whatsapp_accounts().await.unwrap();
        assert!(result.accounts.is_empty());
    }

    // -- Analytics -----------------------------------------------------------

    #[tokio::test]
    async fn test_get_live_count() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/analytics/live-count"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "count": 42
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().get_live_count().await.unwrap();
        assert_eq!(result.count, 42);
    }

    // -- Agent Testing -------------------------------------------------------

    #[tokio::test]
    async fn test_delete_agent_test() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("DELETE"))
            .and(path("/v1/convai/agent-testing/test_1"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        client.agents().delete_agent_test("test_1").await.unwrap();
    }

    // -- Query parameter helper -----------------------------------------------

    #[test]
    fn append_query_builds_correct_path() {
        let mut path = "/v1/test".to_owned();
        append_query(&mut path, "foo", "bar");
        assert_eq!(path, "/v1/test?foo=bar");
        append_query(&mut path, "baz", "qux");
        assert_eq!(path, "/v1/test?foo=bar&baz=qux");
    }

    // -- Twilio ---------------------------------------------------------------

    #[tokio::test]
    async fn test_twilio_outbound_call() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("POST"))
            .and(path("/v1/convai/twilio/outbound-call"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "message": "Call initiated",
                "conversation_id": "conv_tw_1",
                "callSid": "CA123"
            })))
            .mount(&mock_server)
            .await;

        let req = crate::types::TwilioOutboundCallRequest {
            agent_id: "agent_1".into(),
            agent_phone_number_id: "phone_1".into(),
            to_number: "+1234567890".into(),
            conversation_initiation_client_data: None,
        };
        let result = client.agents().twilio_outbound_call(&req).await.unwrap();
        assert!(result.success);
        assert_eq!(result.conversation_id.as_deref(), Some("conv_tw_1"));
    }

    #[tokio::test]
    async fn test_twilio_register_call() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("POST"))
            .and(path("/v1/convai/twilio/register-call"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok"
            })))
            .mount(&mock_server)
            .await;

        let req = crate::types::TwilioRegisterCallRequest {
            agent_id: "agent_1".into(),
            from_number: "+1111111111".into(),
            to_number: "+2222222222".into(),
            direction: Some("inbound".into()),
            conversation_initiation_client_data: None,
        };
        let result = client.agents().twilio_register_call(&req).await.unwrap();
        assert_eq!(result["status"], "ok");
    }

    // -- Users ----------------------------------------------------------------

    #[tokio::test]
    async fn test_get_conversation_users() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "users": [],
                "next_cursor": null,
                "has_more": false
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().get_conversation_users(None, None).await.unwrap();
        assert!(result.users.is_empty());
        assert!(!result.has_more);
    }

    // -- Tool Dependent Agents ------------------------------------------------

    #[tokio::test]
    async fn test_get_tool_dependent_agents() {
        let mock_server = MockServer::start().await;
        let client = crate::client::ElevenLabsClient::new(test_config(&mock_server.uri())).unwrap();

        Mock::given(method("GET"))
            .and(path("/v1/convai/tools/tool_1/dependent-agents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "agents": [],
                "next_cursor": null,
                "has_more": false
            })))
            .mount(&mock_server)
            .await;

        let result = client.agents().get_tool_dependent_agents("tool_1", None).await.unwrap();
        assert!(result.agents.is_empty());
        assert!(!result.has_more);
    }
}
