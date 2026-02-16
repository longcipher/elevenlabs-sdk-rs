//! Integration tests against a Prism mock server.
//!
//! These tests are `#[ignore]`d by default. Run with:
//!
//! ```bash
//! just sdk-test-integration
//! ```
//!
//! Or manually:
//!
//! ```bash
//! # Terminal 1 — start Prism
//! npx @stoplight/prism-cli mock docs/openapi.json --port 4010 --host 127.0.0.1
//!
//! # Terminal 2 — run tests
//! cargo test -p elevenlabs-sdk --test integration_test -- --ignored
//! ```

#[expect(clippy::unwrap_used, reason = "integration tests use unwrap")]
mod prism {
    use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, types::TextToSpeechRequest};

    /// Base URL of the Prism mock server.
    const PRISM_BASE_URL: &str = "http://127.0.0.1:4010";

    /// Creates an [`ElevenLabsClient`] pointing at the local Prism mock server.
    fn integration_client() -> ElevenLabsClient {
        let config =
            ClientConfig::builder("test-key").base_url(PRISM_BASE_URL).max_retries(0_u32).build();
        ElevenLabsClient::new(config).unwrap()
    }

    // ===================================================================
    // Models
    // ===================================================================

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_models_list() {
        let client = integration_client();
        let result = client.models().list().await;
        assert!(result.is_ok(), "models().list() failed: {result:?}");
    }

    // ===================================================================
    // Voices
    // ===================================================================

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_voices_list() {
        let client = integration_client();
        let result = client.voices().list(None).await;
        assert!(result.is_ok(), "voices().list() failed: {result:?}");
    }

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_voices_get_default_settings() {
        let client = integration_client();
        let result = client.voices().get_default_settings().await;
        assert!(result.is_ok(), "voices().get_default_settings() failed: {result:?}");
    }

    // ===================================================================
    // User
    // ===================================================================

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_user_get() {
        let client = integration_client();
        let result = client.user().get().await;
        assert!(result.is_ok(), "user().get() failed: {result:?}");
    }

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_user_get_subscription() {
        let client = integration_client();
        let result = client.user().get_subscription().await;
        assert!(result.is_ok(), "user().get_subscription() failed: {result:?}");
    }

    // ===================================================================
    // History
    // ===================================================================

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_history_list() {
        let client = integration_client();
        let result = client.history().list(None, None, None).await;
        assert!(result.is_ok(), "history().list() failed: {result:?}");
    }

    // ===================================================================
    // Text-to-Speech
    // ===================================================================

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_tts_convert() {
        let client = integration_client();
        let request = TextToSpeechRequest::new("Hello, integration test.");
        let result =
            client.text_to_speech().convert("21m00Tcm4TlvDq8ikWAM", &request, None, None).await;
        assert!(result.is_ok(), "text_to_speech().convert() failed: {result:?}");
    }

    // ===================================================================
    // Voice Generation
    // ===================================================================

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_voice_generation_get_parameters() {
        let client = integration_client();
        let result = client.voice_generation().get_parameters().await;
        assert!(result.is_ok(), "voice_generation().get_parameters() failed: {result:?}");
    }

    // ===================================================================
    // Studio (Projects)
    // ===================================================================

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_studio_get_projects() {
        let client = integration_client();
        let result = client.studio().get_projects().await;
        assert!(result.is_ok(), "studio().get_projects() failed: {result:?}");
    }

    // ===================================================================
    // Dubbing
    // ===================================================================

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_dubbing_list() {
        let client = integration_client();
        let result = client.dubbing().list(None, None).await;
        assert!(result.is_ok(), "dubbing().list() failed: {result:?}");
    }

    // ===================================================================
    // Agents (ConvAI)
    // ===================================================================

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_agents_list() {
        let client = integration_client();
        let result = client.agents().list_agents(None).await;
        assert!(result.is_ok(), "agents().list_agents() failed: {result:?}");
    }

    // ===================================================================
    // Workspace
    // ===================================================================

    #[tokio::test]
    #[ignore = "requires Prism mock server on port 4010"]
    async fn test_workspace_get_service_accounts() {
        let client = integration_client();
        let result = client.workspace().get_service_accounts().await;
        assert!(result.is_ok(), "workspace().get_service_accounts() failed: {result:?}");
    }
}
