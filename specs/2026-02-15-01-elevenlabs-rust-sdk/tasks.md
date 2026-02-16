# ElevenLabs Rust SDK — Implementation Tasks

| Metadata | Details |
| :--- | :--- |
| **Design Doc** | specs/2026-02-15-01-elevenlabs-rust-sdk/design.md |
| **Owner** | — |
| **Start Date** | 2026-02-15 |
| **Target Date** | 2026-03-15 |
| **Status** | Planning |

## Summary & Phasing

Build a comprehensive Rust SDK for the ElevenLabs API from the ground up. The work is split into 6 phases to ensure a solid base before layering more features. Each phase produces working, tested code.

- **Phase 1: Foundation & Scaffolding** — Crate setup, `hpx` client, auth, error types, base request infrastructure
- **Phase 2: Core Types** — All OpenAPI schema types grouped by module (875 schemas)
- **Phase 3: Service Implementation** — All 253 REST endpoints as typed service methods
- **Phase 4: WebSocket Support** — Real-time TTS and Conversational AI WebSocket clients
- **Phase 5: Testing & Validation** — wiremock unit tests, Prism integration tests, 100% endpoint coverage
- **Phase 6: Polish, QA & Docs** — Documentation, examples, CI recipes, README

---

## Phase 1: Foundation & Scaffolding

### Task 1.1: Create SDK Crate & Workspace Integration

> **Context:** Set up the `elevenlabs-sdk` crate in `crates/elevenlabs-sdk/` and integrate it into the Cargo workspace. Add all required dependencies to the workspace root using `cargo add --workspace`. Reuse existing workspace lint config and dependency patterns from `crates/common/Cargo.toml`.
> **Verification:** `cargo check -p elevenlabs-sdk` compiles successfully.

- **Priority:** P0
- **Scope:** Crate scaffolding
- **Status:** � DONE

- [x] **Step 1:** Create `crates/elevenlabs-sdk/Cargo.toml` with `version.workspace = true`, `edition.workspace = true`, `[lints] workspace = true`.
- [x] **Step 2:** Add workspace dependencies via `cargo add`:
  - `cargo add hpx@1.4.0 --workspace` (with `rustls` feature)
  - `cargo add hpx-transport@1.4.0 --workspace`
  - `cargo add serde_json --workspace`
  - `cargo add bytes --workspace`
  - `cargo add futures-core --workspace` (for `Stream` trait)
  - `cargo add tokio --workspace` (already may exist)
  - `cargo add url --workspace`
  - Verify `serde`, `thiserror`, `tracing` already in workspace deps.
- [x] **Step 3:** Create `crates/elevenlabs-sdk/src/lib.rs` with module declarations and crate-level docs.
- [x] **Step 4:** Verify `cargo check -p elevenlabs-sdk` passes.
- [x] **Verification:** `cargo check -p elevenlabs-sdk` exits 0.

---

### Task 1.2: Implement Client Config & Builder

> **Context:** Create `ClientConfig` with builder pattern for base_url, api_key, timeout, retry settings. The config drives all HTTP and WebSocket connections. Follow the builder pattern where required params (api_key) are constructor args and optional params are builder methods.
> **Verification:** Unit test creates config with defaults and custom values.

- **Priority:** P0
- **Scope:** Client configuration
- **Status:** � DONE

- [x] **Step 1:** Create `src/config.rs` with `ClientConfig` struct and `ClientConfigBuilder`.
- [x] **Step 2:** Implement `Default` for optional fields (base_url = `https://api.elevenlabs.io`, timeout = 30s, max_retries = 3).
- [x] **Step 3:** Support `ELEVENLABS_API_KEY` and `ELEVENLABS_BASE_URL` env vars as fallbacks.
- [x] **Step 4:** Add unit tests for builder, defaults, and env var loading.
- [x] **Verification:** `cargo test -p elevenlabs-sdk config` passes.

---

### Task 1.3: Implement Auth & API Key Type

> **Context:** Create `ApiKey` newtype wrapping `String` with redacted `Debug` impl (prints `ApiKey(****)` not the actual key). Implement header injection for `xi-api-key`. All 248 endpoints use this header.
> **Verification:** `Debug` formatting of `ApiKey` does not reveal the key.

- **Priority:** P0
- **Scope:** Authentication
- **Status:** � DONE

- [x] **Step 1:** Create `src/auth.rs` with `ApiKey` newtype.
- [x] **Step 2:** Implement `Debug` that redacts the key value.
- [x] **Step 3:** Implement `From<String>`, `From<&str>`, `AsRef<str>` for ergonomic construction.
- [x] **Step 4:** Add unit tests verifying debug redaction and conversions.
- [x] **Verification:** `cargo test -p elevenlabs-sdk auth` passes; debug output is redacted.

---

### Task 1.4: Implement Error Types

> **Context:** Create `ElevenLabsError` enum using `thiserror` with variants for API errors, auth errors, rate limiting, timeouts, transport errors, deserialization errors, and validation errors. Must be `Send + Sync + 'static`.
> **Verification:** All error variants can be constructed and display meaningful messages.

- **Priority:** P0
- **Scope:** Error handling
- **Status:** � DONE

- [x] **Step 1:** Create `src/error.rs` with `ElevenLabsError` enum and `thiserror` derives.
- [x] **Step 2:** Add `Result<T>` type alias: `pub type Result<T> = std::result::Result<T, ElevenLabsError>`.
- [x] **Step 3:** Implement `From` conversions for `hpx::Error`, `serde_json::Error`.
- [x] **Step 4:** Add unit tests for each error variant's `Display` output.
- [x] **Verification:** `cargo test -p elevenlabs-sdk error` passes.

---

### Task 1.5: Implement HTTP Client Core

> **Context:** Create `ElevenLabsClient` wrapping `hpx::Client` (with `rustls` feature). Implement base request methods (`get`, `post`, `put`, `patch`, `delete`) that handle: URL construction, `xi-api-key` header injection, JSON serialization/deserialization, error response parsing, and tracing spans.
> **Verification:** A simple GET request to a wiremock server returns expected response.

- **Priority:** P0
- **Scope:** HTTP client core
- **Status:** � DONE

- [x] **Step 1:** Create `src/client.rs` with `ElevenLabsClient` struct holding `ClientConfig` and `hpx::Client`.
- [x] **Step 2:** Implement `new(config: ClientConfig) -> Result<Self>` that builds the `hpx::Client`.
- [x] **Step 3:** Implement private helper methods: `request`, `get`, `post`, `post_bytes`, `post_stream`, `delete`, `patch`, `put`, `get_bytes`.
- [x] **Step 4:** Implement response error mapping: parse error JSON body, map status codes to error variants.
- [x] **Step 5:** Add `tracing::instrument` on request method with `method`, `path`, `status` fields.
- [x] **Step 6:** Add wiremock test: mock a GET endpoint, verify client returns deserialized response.
- [x] **Verification:** `cargo test -p elevenlabs-sdk client` passes with wiremock test.

---

### Task 1.6: Implement Retry Middleware

> **Context:** Add retry logic with exponential backoff for transient errors (HTTP 429, 500, 502, 503). Respect `Retry-After` header on 429 responses. Configurable via `ClientConfig.max_retries` and `ClientConfig.retry_backoff`.
> **Verification:** wiremock test returns 429 twice then 200; client retries and succeeds.

- **Priority:** P1
- **Scope:** Retry middleware
- **Status:** � DONE

- [x] **Step 1:** Create `src/middleware.rs` with retry logic integrated into the base `request` method in `client.rs`.
- [x] **Step 2:** Implement exponential backoff: `backoff * 2^attempt` capped at 30s.
- [x] **Step 3:** Parse `Retry-After` header (integer seconds) for 429 responses.
- [x] **Step 4:** Add wiremock test: mock returns 429 with Retry-After then 200.
- [x] **Step 5:** Add wiremock test: mock returns 500 twice then 200 with max_retries=3.
- [x] **Verification:** `cargo test -p elevenlabs-sdk middleware` passes.

---

## Phase 2: Core Types

### Task 2.1: Implement Common & Shared Types

> **Context:** Define shared types used across multiple API groups: pagination types, common enums (e.g., `OutputFormat`, `ModelId`), and utility types. These are referenced by many schemas in the OpenAPI spec.
> **Verification:** Types compile and can be serialized/deserialized round-trip.

- **Priority:** P0
- **Scope:** Shared type definitions
- **Status:** � DONE

- [x] **Step 1:** Create `src/types/mod.rs` and `src/types/common.rs`.
- [x] **Step 2:** Define common enums from OpenAPI spec: `OutputFormat`, `VoiceSettings`, `PronunciationDictionaryLocator`, pagination types.
- [x] **Step 3:** Ensure all types derive `Debug, Clone, Serialize, Deserialize` as appropriate.
- [x] **Step 4:** Use `#[serde(rename_all = "snake_case")]` or explicit `#[serde(rename = "...")]` to match JSON field names exactly.
- [x] **Step 5:** Add round-trip serde unit tests for each type with sample JSON from the OpenAPI spec.
- [x] **Verification:** `cargo test -p elevenlabs-sdk types::common` passes.

---

### Task 2.2: Implement Text-to-Speech Types

> **Context:** Define request/response types for the 4 TTS endpoints: `TextToSpeechRequest`, `TextToSpeechTimestampResponse`, `VoiceSettings`, etc. These are core types that many users will use first.
> **Verification:** JSON from the OpenAPI spec examples deserializes correctly.

- **Priority:** P0
- **Scope:** TTS type definitions
- **Status:** � DONE

- [x] **Step 1:** Create `src/types/text_to_speech.rs`.
- [x] **Step 2:** Define `TextToSpeechRequest`, `TextToSpeechTimestampResponse`, `NormalizedAlignment`, `Alignment` from spec.
- [x] **Step 3:** Add serde round-trip tests with example JSON.
- [x] **Verification:** `cargo test -p elevenlabs-sdk types::text_to_speech` passes.

---

### Task 2.3: Implement Voice & Voice Generation Types

> **Context:** Define types for voices (12 endpoints) and voice generation (3 endpoints): `Voice`, `VoiceResponse`, `GetVoicesResponse`, `VoiceSettings`, `EditVoiceRequest`, etc.
> **Verification:** Types serialize/deserialize matching the spec's JSON structure.

- **Priority:** P0
- **Scope:** Voice type definitions
- **Status:** � DONE

- [x] **Step 1:** Create `src/types/voices.rs` and `src/types/voice_generation.rs`.
- [x] **Step 2:** Define all voice-related schemas from the OpenAPI spec.
- [x] **Step 3:** Add serde tests.
- [x] **Verification:** `cargo test -p elevenlabs-sdk types::voices` passes.

---

### Task 2.4: Implement Speech-to-Speech & Speech-to-Text Types

> **Context:** Define types for S2S (2 endpoints) and STT (3 endpoints).
> **Verification:** Serde round-trip tests pass.

- **Priority:** P0
- **Scope:** S2S and STT type definitions
- **Status:** � DONE

- [x] **Step 1:** Create `src/types/speech_to_speech.rs` and `src/types/speech_to_text.rs`.
- [x] **Step 2:** Define request/response types from spec.
- [x] **Step 3:** Add serde tests.
- [x] **Verification:** `cargo test -p elevenlabs-sdk types::speech_to_speech types::speech_to_text` passes.

---

### Task 2.5: Implement Audio, Sound & Music Types

> **Context:** Define types for audio-isolation (2 endpoints), audio-native (3 endpoints), sound-generation (1 endpoint), and music (5 endpoints).
> **Verification:** Serde round-trip tests pass.

- **Priority:** P0
- **Scope:** Audio/music type definitions
- **Status:** � DONE

- [x] **Step 1:** Create `src/types/audio_isolation.rs`, `src/types/audio_native.rs`, `src/types/sound_generation.rs`, `src/types/music.rs`.
- [x] **Step 2:** Define request/response types from spec.
- [x] **Step 3:** Add serde tests.
- [x] **Verification:** `cargo test -p elevenlabs-sdk types::audio` passes.

---

### Task 2.6: Implement Dubbing & Studio Types

> **Context:** Define types for dubbing (~15 unique endpoints, 875+ schemas for dubbing resources), studio (23 endpoints). These are the largest type groups.
> **Verification:** Serde round-trip tests pass.

- **Priority:** P0
- **Scope:** Dubbing and studio type definitions
- **Status:** � DONE

- [x] **Step 1:** Create `src/types/dubbing.rs` and `src/types/studio.rs`.
- [x] **Step 2:** Define all dubbing resource types, speaker types, segment types, render types.
- [x] **Step 3:** Define all studio project, chapter, snapshot types.
- [x] **Step 4:** Add serde tests.
- [x] **Verification:** `cargo test -p elevenlabs-sdk types::dubbing types::studio` passes.

---

### Task 2.7: Implement Agents Platform & ConvAI Types

> **Context:** Define types for the Agents Platform (98 endpoints) — the largest API group. Includes agents, conversations, knowledge base, tools, phone numbers, MCP servers, batch calling, secrets, settings, branches, deployments, whatsapp, and SIP trunk types.
> **Verification:** Serde round-trip tests pass.

- **Priority:** P0
- **Scope:** Agents Platform type definitions
- **Status:** � DONE

- [x] **Step 1:** Create `src/types/agents.rs`, knowledge base, tools, phone numbers types.
- [x] **Step 2:** Define agent config types (complex nested structure with serde_json::Value for variant configs).
- [x] **Step 3:** Define conversation, knowledge base, tool, phone number, MCP server, batch calling types.
- [x] **Step 4:** Define settings, secrets, branches, deployment types.
- [x] **Step 5:** Add serde tests.
- [x] **Verification:** `cargo test -p elevenlabs-sdk types::agents` passes.

---

### Task 2.8: Implement Remaining API Types

> **Context:** Define types for the remaining smaller API groups: text-to-dialogue (4), text-to-voice (5), models (1), history (5), samples (2), pronunciation (8), workspace (19), user (3), PVC voices (14), forced-alignment (1), single-use-token (1), webhooks.
> **Verification:** Serde round-trip tests pass.

- **Priority:** P0
- **Scope:** Remaining type definitions
- **Status:** � DONE

- [x] **Step 1:** Create type files: `text_to_dialogue.rs`, `text_to_voice.rs`, `models.rs`, `history.rs`, `samples.rs`, `pronunciation.rs`, `workspace.rs`, `user.rs`, `pvc_voices.rs`, `forced_alignment.rs`.
- [x] **Step 2:** Define request/response types from spec for each.
- [x] **Step 3:** Add serde tests for each.
- [x] **Verification:** `cargo test -p elevenlabs-sdk types` passes — all type modules.

---

## Phase 3: Service Implementation

### Task 3.1: Implement Text-to-Speech Service

> **Context:** Implement 4 TTS endpoints: `text_to_speech_full`, `text_to_speech_full_with_timestamps`, `text_to_speech_stream`, `text_to_speech_stream_with_timestamps`. These are the most commonly used endpoints. Uses typed request/response from Task 2.2. Reuses client base methods from Task 1.5.
> **Verification:** wiremock test for each endpoint passes.

- **Priority:** P0
- **Scope:** TTS service (4 endpoints)
- **Status:** � DONE

- [x] **Step 1:** Create `src/services/mod.rs` and `src/services/text_to_speech.rs`.
- [x] **Step 2:** Define `TextToSpeechService<'a>` struct with `client: &'a ElevenLabsClient`.
- [x] **Step 3:** Implement `convert()` → `POST /v1/text-to-speech/{voice_id}`, returns `Bytes`.
- [x] **Step 4:** Implement `convert_with_timestamps()` → `POST /v1/text-to-speech/{voice_id}/with-timestamps`.
- [x] **Step 5:** Implement `convert_stream()` → `POST /v1/text-to-speech/{voice_id}/stream`, returns `Stream`.
- [x] **Step 6:** Implement `convert_stream_with_timestamps()` → `POST /v1/text-to-speech/{voice_id}/stream/with-timestamps`.
- [x] **Step 7:** Add wiremock tests for each endpoint.
- [x] **Verification:** `cargo test -p elevenlabs-sdk services::text_to_speech` passes.

---

### Task 3.2: Implement Voices Service

> **Context:** Implement 12 voice endpoints: list, get, create, edit, delete, settings, shared voices, etc. Uses types from Task 2.3.
> **Verification:** wiremock test for each endpoint passes.

- **Priority:** P0
- **Scope:** Voices service (12 endpoints)
- **Status:** � DONE

- [x] **Step 1:** Create `src/services/voices.rs`.
- [x] **Step 2:** Implement all 11 endpoints matching operationIds from the spec.
- [x] **Step 3:** Handle multipart/form-data for `add_voice` and `edit_voice` (file uploads).
- [x] **Step 4:** Add wiremock tests for each endpoint.
- [x] **Verification:** `cargo test -p elevenlabs-sdk services::voices` passes.

---

### Task 3.3: Implement Speech-to-Speech & Speech-to-Text Services

> **Context:** Implement S2S (2 endpoints) and STT (3 endpoints). S2S endpoints handle file upload (audio input). Uses types from Task 2.4.
> **Verification:** wiremock tests pass.

- **Priority:** P0
- **Scope:** S2S and STT services (5 endpoints)
- **Status:** � DONE

- [x] **Step 1:** Create `src/services/speech_to_speech.rs` and `src/services/speech_to_text.rs`.
- [x] **Step 2:** Implement endpoints. Handle multipart for audio uploads.
- [x] **Step 3:** Add wiremock tests.
- [x] **Verification:** `cargo test -p elevenlabs-sdk services::speech_to_speech services::speech_to_text` passes.

---

### Task 3.4: Implement Audio, Sound & Music Services

> **Context:** Implement audio-isolation (2), audio-native (3), sound-generation (1), music (5) services. Includes streaming endpoints for audio-isolation and music.
> **Verification:** wiremock tests pass.

- **Priority:** P0
- **Scope:** Audio/music services (11 endpoints)
- **Status:** � DONE

- [x] **Step 1:** Create `src/services/audio_isolation.rs`, `src/services/audio_native.rs`, `src/services/sound_generation.rs`, `src/services/music.rs`.
- [x] **Step 2:** Implement all endpoints.
- [x] **Step 3:** Handle streaming responses for `audio_isolation_stream` and `stream_compose`.
- [x] **Step 4:** Add wiremock tests.
- [x] **Verification:** `cargo test -p elevenlabs-sdk services::audio services::music` passes.

---

### Task 3.5: Implement Text-to-Dialogue & Text-to-Voice Services

> **Context:** Implement text-to-dialogue (4 endpoints, including streaming) and text-to-voice (5 endpoints).
> **Verification:** wiremock tests pass.

- **Priority:** P0
- **Scope:** Dialogue and voice design services (9 endpoints)
- **Status:** � DONE

- [x] **Step 1:** Create `src/services/text_to_dialogue.rs` and `src/services/text_to_voice.rs`.
- [x] **Step 2:** Implement all endpoints including streaming variants.
- [x] **Step 3:** Add wiremock tests.
- [x] **Verification:** `cargo test -p elevenlabs-sdk services::text_to_dialogue services::text_to_voice` passes.

---

### Task 3.6: Implement Dubbing Service

> **Context:** Implement ~15 unique dubbing endpoints (deduplicated from dubbing/enterprise/resource/segment tags which share the same endpoints). Handles complex CRUD for dubbing resources, speakers, segments, rendering.
> **Verification:** wiremock tests pass.

- **Priority:** P0
- **Scope:** Dubbing service (~20 unique endpoints)
- **Status:** 🟢 DONE

- [x] **Step 1:** Create `src/services/dubbing.rs`.
- [x] **Step 2:** Deduplicate endpoints across dubbing/enterprise/resource/segment tags.
- [x] **Step 3:** Implement all 20 unique endpoints.
- [x] **Step 4:** Handle multipart upload for `create_dubbing`.
- [x] **Step 5:** Add wiremock tests.
- [x] **Verification:** `cargo test -p elevenlabs-sdk services::dubbing` passes.

---

### Task 3.7: Implement Studio Service

> **Context:** Implement 23 studio endpoints: projects, chapters, snapshots, podcasts, pronunciation dictionaries, muted tracks. Includes streaming for snapshot audio.
> **Verification:** wiremock tests pass.

- **Priority:** P0
- **Scope:** Studio service (23 endpoints)
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/services/studio.rs`.
- [ ] **Step 2:** Implement all 23 endpoints.
- [ ] **Step 3:** Handle streaming responses for snapshot audio/archive endpoints.
- [ ] **Step 4:** Add wiremock tests.
- [ ] **Verification:** `cargo test -p elevenlabs-sdk services::studio` passes.

---

### Task 3.8: Implement Agents Platform Service

> **Context:** Implement the Agents Platform service — the largest group with ~98 endpoints covering agents, conversations, phone numbers, knowledge base, tools, MCP servers, batch calling, secrets, settings, branches, deployments, whatsapp, SIP trunk, and testing. Split into logical sub-services if needed.
> **Verification:** wiremock tests pass for all endpoints.

- **Priority:** P0
- **Scope:** Agents Platform service (~98 endpoints)
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/services/agents.rs` as the main service module.
- [ ] **Step 2:** Consider sub-service pattern for organization:
  - `client.agents().agents()` — Agent CRUD
  - `client.agents().conversations()` — Conversation history
  - `client.agents().knowledge_base()` — KB management
  - `client.agents().tools()` — Tool management
  - `client.agents().phone_numbers()` — Phone number management
  - `client.agents().mcp_servers()` — MCP server management
  - `client.agents().batch_calling()` — Batch call management
  - `client.agents().secrets()` — Secret management
  - `client.agents().settings()` — Settings management
  - `client.agents().branches()` — Branch management
  - `client.agents().whatsapp()` — WhatsApp management
  - `client.agents().testing()` — Agent test management
- [ ] **Step 3:** Implement all endpoints grouped by sub-service.
- [ ] **Step 4:** Add wiremock tests for each sub-group.
- [ ] **Verification:** `cargo test -p elevenlabs-sdk services::agents` passes.

---

### Task 3.9: Implement Remaining Services

> **Context:** Implement all remaining services: models (1), history (5), samples (2), pronunciation (8), workspace (19), user (3), PVC voices (14), forced-alignment (1), single-use-token (1), voice-generation (3).
> **Verification:** wiremock tests pass for all endpoints.

- **Priority:** P0
- **Scope:** Remaining services (~57 endpoints)
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create service files: `models.rs`, `history.rs`, `samples.rs`, `pronunciation.rs`, `workspace.rs`, `user.rs`, `pvc_voices.rs`, `forced_alignment.rs`, `single_use_token.rs`, `voice_generation.rs`.
- [ ] **Step 2:** Implement all endpoints for each service.
- [ ] **Step 3:** Handle multipart uploads where needed (PVC voice samples, pronunciation dictionaries).
- [ ] **Step 4:** Add wiremock tests for each service.
- [ ] **Step 5:** Wire up all services in `ElevenLabsClient` with accessor methods.
- [ ] **Verification:** `cargo test -p elevenlabs-sdk services` passes — all service modules.

---

## Phase 4: WebSocket Support

### Task 4.1: Implement WebSocket Base Infrastructure

> **Context:** Create WebSocket connection infrastructure using `hpx-transport` 1.4.0. Handle connection lifecycle (connect, send, receive, close), authentication via query parameter or first message, and error handling. This is the foundation for both TTS and Conversational AI WebSocket clients.
> **Verification:** Can establish a WebSocket connection and exchange messages in a test.

- **Priority:** P0
- **Scope:** WebSocket infrastructure
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/ws/mod.rs` with shared WebSocket types and connection helpers.
- [ ] **Step 2:** Implement `connect_ws(url, api_key) -> Result<WsConnection>` using `hpx-transport`.
- [ ] **Step 3:** Handle WebSocket frame types: text (JSON), binary (audio), ping/pong.
- [ ] **Step 4:** Add connection-level error handling and reconnection support.
- [ ] **Step 5:** Add unit test with a local WebSocket server mock.
- [ ] **Verification:** `cargo test -p elevenlabs-sdk ws` passes.

---

### Task 4.2: Implement TTS WebSocket Client

> **Context:** Implement the real-time Text-to-Speech WebSocket client. Protocol: connect → send BOS message → stream text chunks → receive audio chunks → send EOS → close. URL: `wss://api.elevenlabs.io/v1/text-to-speech/{voice_id}/stream-input`. Uses `hpx-transport` from Task 4.1.
> **Verification:** Can send text and receive audio frames in a test.

- **Priority:** P0
- **Scope:** TTS WebSocket client
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/ws/tts.rs` with `TtsWebSocket` struct.
- [ ] **Step 2:** Implement `connect(config, voice_id, model_id)` with query params for auth.
- [ ] **Step 3:** Implement `send_text(text)` — sends `{"text": "...", "try_trigger_generation": true}`.
- [ ] **Step 4:** Implement `flush()` — sends `{"text": " ", "flush": true}`.
- [ ] **Step 5:** Implement `recv_audio()` — receives `{"audio": "<base64>", "isFinal": bool, "alignment": {...}}`.
- [ ] **Step 6:** Implement `close()` — sends EOS `{"text": ""}` and closes.
- [ ] **Step 7:** Define `TtsWsConfig`, `TtsWsResponse`, `TtsWsAlignment` types.
- [ ] **Step 8:** Add test with mock WebSocket server.
- [ ] **Verification:** `cargo test -p elevenlabs-sdk ws::tts` passes.

---

### Task 4.3: Implement Conversational AI WebSocket Client

> **Context:** Implement the Conversational AI WebSocket client for real-time agent conversations. URL: `wss://api.elevenlabs.io/v1/convai/conversation`. Protocol: bidirectional JSON/binary frames for audio in/out and conversation events.
> **Verification:** Can establish conversation session and exchange messages in a test.

- **Priority:** P1
- **Scope:** ConvAI WebSocket client
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/ws/conversation.rs` with `ConversationWebSocket` struct.
- [ ] **Step 2:** Implement `connect(config, agent_id)` with signed URL auth flow.
- [ ] **Step 3:** Implement `send_audio(bytes)` — sends binary frames with audio input.
- [ ] **Step 4:** Implement `recv_event()` — receives conversation events (agent response, audio, metadata).
- [ ] **Step 5:** Define `ConversationEvent`, `ConversationConfig`, `AgentResponse` types.
- [ ] **Step 6:** Implement `close()` — graceful shutdown.
- [ ] **Step 7:** Add test with mock WebSocket server.
- [ ] **Verification:** `cargo test -p elevenlabs-sdk ws::conversation` passes.

---

## Phase 5: Testing & Validation

### Task 5.1: Set Up Prism Mock Server Integration

> **Context:** Configure Prism (Stoplight) to run as a mock server from `docs/openapi.json`. Prism validates that requests match the OpenAPI spec and returns spec-compliant responses. Add `Justfile` recipes for starting Prism and running integration tests.
> **Verification:** `npx @stoplight/prism-cli mock docs/openapi.json --port 4010` starts successfully.

- **Priority:** P0
- **Scope:** Test infrastructure
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add `package.json` (or update existing) with `@stoplight/prism-cli` as a dev dependency.
- [ ] **Step 2:** Add `just sdk-prism-start` recipe: `npx @stoplight/prism-cli mock docs/openapi.json --port 4010 --host 127.0.0.1`.
- [ ] **Step 3:** Add `just sdk-test-integration` recipe: starts Prism, runs integration tests, stops Prism.
- [ ] **Step 4:** Create `crates/elevenlabs-sdk/tests/` directory with test harness that connects to `http://127.0.0.1:4010`.
- [ ] **Step 5:** Verify Prism starts and responds to a test request.
- [ ] **Verification:** `just sdk-prism-start` starts Prism; `curl http://127.0.0.1:4010/v1/models` returns JSON.

---

### Task 5.2: Write Integration Tests for All Endpoints

> **Context:** Write integration tests that call every SDK endpoint against the Prism mock server. Each test verifies: correct HTTP method, correct URL path, request body accepted by Prism (field validation), response deserialized correctly. Group tests by service module.
> **Verification:** All 253 endpoints tested; `cargo test --test integration` passes.

- **Priority:** P0
- **Scope:** Integration test suite
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `tests/integration/mod.rs` with shared test setup (client pointing to Prism).
- [ ] **Step 2:** Create test file per service module matching service structure.
- [ ] **Step 3:** Write one test per endpoint — call the SDK method, assert no errors.
- [ ] **Step 4:** For endpoints requiring path params, use valid dummy IDs.
- [ ] **Step 5:** For endpoints requiring request bodies, use minimal valid payloads from OpenAPI examples.
- [ ] **Step 6:** Run full suite against Prism: `just sdk-test-integration`.
- [ ] **Verification:** All tests pass against Prism; no Prism validation errors in logs.

---

### Task 5.3: Create Endpoint Coverage Script

> **Context:** Create a Python script (`scripts/check_coverage.py`) that: 1) parses `docs/openapi.json` to extract all operationIds, 2) scans SDK test code for references to each operationId or corresponding method name, 3) reports any untested endpoints. Ensures 100% endpoint coverage.
> **Verification:** Script reports 0 missing endpoints.

- **Priority:** P1
- **Scope:** Coverage validation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `scripts/check_coverage.py`.
- [ ] **Step 2:** Parse openapi.json for all unique `(method, path, operationId)` triples.
- [ ] **Step 3:** Deduplicate (same endpoint in multiple tags).
- [ ] **Step 4:** Scan `crates/elevenlabs-sdk/src/services/` for method implementations matching each operationId.
- [ ] **Step 5:** Scan `crates/elevenlabs-sdk/tests/` for test coverage of each endpoint.
- [ ] **Step 6:** Report missing implementations and missing tests.
- [ ] **Verification:** `python3 scripts/check_coverage.py` reports 100% coverage.

---

### Task 5.4: Comprehensive wiremock Unit Tests

> **Context:** Ensure every service method has wiremock-based unit tests that verify: 1) correct HTTP method used, 2) correct path constructed (including path parameters), 3) correct request headers (`xi-api-key`, `Content-Type`), 4) request body JSON matches expected schema, 5) response deserialization is correct, 6) error cases (401, 404, 429, 500) are handled. This uses wiremock 0.6.5 without needing Prism.
> **Verification:** `cargo test -p elevenlabs-sdk` passes with >200 test cases.

- **Priority:** P0
- **Scope:** Unit test completeness
- **Status:** 🔴 TODO

- [ ] **Step 1:** Review each service module for test coverage gaps.
- [ ] **Step 2:** Add missing tests — minimum 1 success + 1 error test per endpoint.
- [ ] **Step 3:** Add edge case tests: empty responses, large payloads, optional fields missing.
- [ ] **Step 4:** Add auth tests: missing API key, invalid API key.
- [ ] **Step 5:** Verify all tests pass: `cargo test -p elevenlabs-sdk`.
- [ ] **Verification:** `cargo test -p elevenlabs-sdk -- --list | wc -l` shows >200 tests.

---

## Phase 6: Polish, QA & Docs

### Task 6.1: Add Documentation

> **Context:** Add `///` doc comments to all public types, methods, and modules. Module-level docs explain the API group and link to ElevenLabs documentation. Examples in doc comments where helpful.
> **Verification:** `cargo doc -p elevenlabs-sdk --no-deps` generates docs without warnings.

- **Priority:** P1
- **Scope:** API documentation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add crate-level docs in `lib.rs` with usage example.
- [ ] **Step 2:** Add module docs for each service and type module.
- [ ] **Step 3:** Add method docs for each service method with parameter descriptions.
- [ ] **Step 4:** Add doc examples for the most common use cases (TTS, voices, STT).
- [ ] **Step 5:** Run `cargo doc` and fix any warnings.
- [ ] **Verification:** `cargo doc -p elevenlabs-sdk --no-deps 2>&1 | grep -c warning` is 0.

---

### Task 6.2: Add Examples

> **Context:** Create runnable example programs in `crates/elevenlabs-sdk/examples/` demonstrating key SDK features: TTS, streaming, voice listing, WebSocket TTS.
> **Verification:** Examples compile with `cargo build --examples -p elevenlabs-sdk`.

- **Priority:** P2
- **Scope:** Example programs
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `examples/text_to_speech.rs` — basic TTS usage.
- [ ] **Step 2:** Create `examples/streaming.rs` — streaming audio to file.
- [ ] **Step 3:** Create `examples/voices.rs` — list and manage voices.
- [ ] **Step 4:** Create `examples/websocket_tts.rs` — real-time TTS via WebSocket.
- [ ] **Step 5:** Create `examples/conversation.rs` — Conversational AI WebSocket.
- [ ] **Verification:** `cargo build --examples -p elevenlabs-sdk` compiles all examples.

---

### Task 6.3: Update Justfile & CI

> **Context:** Add SDK-specific recipes to the Justfile and ensure lint/format/test commands cover the SDK crate. Reuses existing just recipes pattern.
> **Verification:** `just lint`, `just test`, `just format` all include the SDK crate.

- **Priority:** P1
- **Scope:** Build tooling
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add `sdk-test` recipe: `cargo test -p elevenlabs-sdk --all-features`.
- [ ] **Step 2:** Add `sdk-test-integration` recipe with Prism lifecycle management.
- [ ] **Step 3:** Add `sdk-coverage` recipe: `python3 scripts/check_coverage.py`.
- [ ] **Step 4:** Verify existing `just lint`, `just test`, `just format` include the new crate (they should via workspace).
- [ ] **Verification:** `just lint && just test` pass cleanly.

---

### Task 6.4: Update README & Project Documentation

> **Context:** Update the project README with SDK usage instructions, installation, and quick-start guide. Add AGENTS.md for project context.
> **Verification:** README contains SDK documentation.

- **Priority:** P2
- **Scope:** Project documentation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Update `README.md` with SDK overview, installation, and usage examples.
- [ ] **Step 2:** Document authentication setup (API key via env var or constructor).
- [ ] **Step 3:** Document available services and their methods.
- [ ] **Step 4:** Add link to generated API docs.
- [ ] **Step 5:** Create `AGENTS.md` with project structure and conventions.
- [ ] **Verification:** README is comprehensive and accurate.

---

### Task 6.5: Final Validation & Cleanup

> **Context:** Run full CI pipeline, fix any remaining issues, and verify all quality checklist items from the project instructions are met.
> **Verification:** All checks pass clean.

- **Priority:** P0
- **Scope:** Final QA
- **Status:** 🔴 TODO

- [ ] **Step 1:** Run `just format` — fix any formatting issues.
- [ ] **Step 2:** Run `just lint` — fix all clippy warnings and typos.
- [ ] **Step 3:** Run `just test` — all tests pass.
- [ ] **Step 4:** Run `just sdk-test-integration` — all integration tests pass against Prism.
- [ ] **Step 5:** Run `python3 scripts/check_coverage.py` — 100% endpoint coverage.
- [ ] **Step 6:** Verify no Chinese comments in codebase.
- [ ] **Step 7:** Verify no forbidden crates (`anyhow`, `log`, `reqwest`, `dashmap`).
- [ ] **Step 8:** Verify HTTP uses `hpx` with `rustls`, not `reqwest`.
- [ ] **Step 9:** Verify `thiserror` for errors, `tracing` for logging.
- [ ] **Verification:** All quality checklist items pass.

---

## Summary & Timeline

| Phase | Tasks | Target Date |
| :--- | :---: | :--- |
| **1. Foundation** | 6 | 02-22 |
| **2. Core Types** | 8 | 03-01 |
| **3. Service Implementation** | 9 | 03-08 |
| **4. WebSocket Support** | 3 | 03-11 |
| **5. Testing & Validation** | 4 | 03-14 |
| **6. Polish** | 5 | 03-15 |
| **Total** | **35** | |

## Definition of Done

1. [ ] **Linted:** `just lint` passes with no errors or warnings.
2. [ ] **Tested:** Unit tests (wiremock) covering all 253 endpoints + integration tests (Prism).
3. [ ] **Formatted:** `just format` leaves no changes.
4. [ ] **Verified:** Every task's specific Verification criterion met.
5. [ ] **Coverage:** `check_coverage.py` reports 100% endpoint coverage.
6. [ ] **No forbidden crates:** `anyhow`, `log`, `reqwest`, `dashmap`, `trunk` not used.
7. [ ] **HTTP via hpx:** All HTTP done via `hpx` 1.4.0 with `rustls`.
8. [ ] **WebSocket via hpx-transport:** All WS done via `hpx-transport` 1.4.0.
9. [ ] **English only:** No Chinese in comments or docs.
10. [ ] **Documented:** All public APIs have doc comments.
