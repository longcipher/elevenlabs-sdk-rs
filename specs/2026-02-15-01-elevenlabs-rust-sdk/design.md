# Design Document: ElevenLabs Rust SDK

| Metadata | Details |
| :--- | :--- |
| **Author** | pb-plan agent |
| **Status** | Draft |
| **Created** | 2026-02-15 |
| **Reviewers** | — |
| **Related Issues** | N/A |

## 1. Executive Summary

**Problem:** There is no Rust SDK for the ElevenLabs API. The project has an OpenAPI 3.1.0 spec (`docs/openapi.json`) with 253 endpoints across 28 tag groups and 875 schemas, but no SDK crate to consume it. Developers who need to interact with ElevenLabs from Rust have no typed, ergonomic client.

**Solution:** Build a comprehensive Rust SDK crate (`elevenlabs-sdk`) that provides:

1. A fully typed REST client covering all 253 endpoints with request/response types derived from the OpenAPI spec's 875 schemas.
2. Streaming support for the 11 streaming endpoints (SSE/chunked audio responses).
3. WebSocket client support for real-time TTS and Conversational AI (ElevenLabs WebSocket APIs not captured in the OpenAPI spec but documented externally).
4. `hpx` 1.4.0 + `hpx-transport` 1.4.0 as the HTTP and WebSocket transport layer.
5. TDD verification using Prism (OpenAPI mock server) and `wiremock` 0.6.5 to guarantee all endpoints are implemented and field-matched.

---

## 2. Requirements & Goals

### 2.1 Problem Statement

- No Rust SDK exists for ElevenLabs API.
- The OpenAPI spec is 1.3 MB with 253 endpoints and 875 schemas — manual typing is error-prone.
- Streaming endpoints (audio isolation, TTS, music, speech-to-speech) require chunked/SSE response handling.
- Real-time use cases (conversational AI, real-time TTS) require WebSocket support not in the OpenAPI spec.
- No existing test infrastructure to validate SDK correctness against the spec.

### 2.2 Functional Goals

1. **Complete REST API Coverage:** Every endpoint in `docs/openapi.json` has a corresponding SDK method with typed request/response structs matching the spec.
2. **Streaming Support:** The 11 streaming endpoints return `Stream<Item = Bytes>` or typed streaming responses.
3. **WebSocket Client:** Support for ElevenLabs real-time WebSocket APIs:
   - Text-to-Speech WebSocket (`wss://api.elevenlabs.io/v1/text-to-speech/{voice_id}/stream-input`)
   - Conversational AI WebSocket (`wss://api.elevenlabs.io/v1/convai/conversation`)
4. **Authentication:** `xi-api-key` header-based auth on all 248 endpoints that require it.
5. **Builder Pattern:** Ergonomic request builders for endpoints with many optional parameters.
6. **Error Handling:** Typed error types using `thiserror`, mapping HTTP status codes to enum variants.
7. **Service Modules:** Endpoints grouped by tag into service modules (e.g., `client.text_to_speech()`, `client.voices()`, `client.dubbing()`).
8. **TDD Validated:** Every endpoint has a test that runs against a Prism mock server; all field names match the OpenAPI spec exactly.

### 2.3 Non-Functional Goals

- **Performance:** Zero unnecessary allocations. Use `hpx` with `rustls` for TLS. Stream responses without buffering entire audio in memory.
- **Reliability:** Automatic retry with exponential backoff for transient failures (429, 500, 502, 503). Configurable timeout.
- **Security:** No credential logging. API keys in headers only, never in URLs or logs. TLS-only via `rustls`.
- **Observability:** `tracing` instrumentation on every HTTP request/response cycle with method, path, status, and latency spans.
- **Compile Time:** Use runtime serde, not compile-time macros. Minimize proc-macro dependencies.

### 2.4 Out of Scope

- Code generation from OpenAPI spec (types are hand-written to ensure idiomatic Rust).
- Server-side implementation / mock server logic (Prism handles this).
- The existing `leptos-csr-app` frontend — SDK is a standalone library crate.
- OAuth/OAuth2 flows (ElevenLabs uses API key auth).
- Webhook receiving (the SDK is a client, not a server).

### 2.5 Assumptions

- `hpx` 1.4.0 and `hpx-transport` 1.4.0 provide HTTP client and WebSocket client capabilities with `rustls` support.
- ElevenLabs WebSocket APIs follow the documented protocol: JSON text frames for control, binary frames for audio.
- The OpenAPI spec at `docs/openapi.json` is the authoritative source for REST endpoint definitions.
- `wiremock` 0.6.5 is used for unit tests (fast, no external process); Prism is used for integration tests (full OpenAPI validation).
- Duplicate endpoints (same path appearing in multiple tags like dubbing/enterprise/resource/segment) are deduplicated — one method per unique `(method, path)` pair.

---

## 3. Architecture Overview

### 3.1 System Context

```text
┌──────────────────────────────────────────────────┐
│                User Application                   │
│                                                    │
│  let client = ElevenLabsClient::new(api_key);     │
│  let audio = client.text_to_speech()              │
│      .voice_id("...")                              │
│      .text("Hello")                                │
│      .send().await?;                               │
└──────────┬───────────────────────────┬────────────┘
           │ REST/Streaming            │ WebSocket
           ▼                           ▼
┌──────────────────────┐  ┌─────────────────────────┐
│   hpx HTTP Client    │  │  hpx-transport WS Client│
│  (rustls, h2, retry) │  │  (rustls, tungstenite)  │
└──────────┬───────────┘  └──────────┬──────────────┘
           │                         │
           ▼                         ▼
┌──────────────────────────────────────────────────┐
│            ElevenLabs API                         │
│  https://api.elevenlabs.io/v1/...                │
│  wss://api.elevenlabs.io/v1/...                  │
└──────────────────────────────────────────────────┘
```

The SDK is a library crate (`crates/elevenlabs-sdk`) in the existing Cargo workspace. It depends on:

- `hpx` for HTTP operations
- `hpx-transport` for WebSocket connections
- `serde` / `serde_json` for serialization
- `thiserror` for error types
- `tracing` for observability
- Shared types may live in `crates/common` if needed by other crates

### 3.2 Key Design Principles

1. **Spec-Faithful:** Every type name, field name, and endpoint path matches the OpenAPI spec exactly. No renaming beyond Rust `snake_case` convention for fields (with `#[serde(rename = "...")]` where needed).
2. **Zero-Cost Abstractions:** Service modules are zero-sized types that borrow the client. No runtime overhead for the module grouping.
3. **Builder Ergonomics:** Requests with >3 parameters use the builder pattern. Required params are constructor args; optional params are builder methods.
4. **Streaming First:** Audio endpoints default to streaming responses. Non-streaming wrappers are convenience methods.
5. **No Unsafe:** Entire codebase is `#[forbid(unsafe_code)]`.
6. **Test-Driven:** Every endpoint is tested against Prism mock and wiremock for field correctness.

### 3.3 Existing Components to Reuse

| Component | Location | How to Reuse |
| :--- | :--- | :--- |
| `common` crate | `crates/common/` | Place shared types (e.g., pagination, common enums) here if needed by both SDK and frontend |
| Workspace lint config | Root `Cargo.toml` | SDK crate inherits `[lints] workspace = true` |
| Workspace dependencies | Root `Cargo.toml` | Add `hpx`, `hpx-transport`, `serde`, `thiserror`, `tracing` to `[workspace.dependencies]` |
| `Justfile` tasks | `Justfile` | Add `sdk-test` recipe for running SDK tests with Prism |
| `serde` (workspace dep) | Root `Cargo.toml` | Already defined — use `workspace = true` in SDK crate |
| `thiserror` (workspace dep) | Root `Cargo.toml` | Already defined — use `workspace = true` in SDK crate |
| `tracing` (workspace dep) | Root `Cargo.toml` | Already defined — use `workspace = true` in SDK crate |

---

## 4. Detailed Design

### 4.1 Module Structure

```text
crates/elevenlabs-sdk/
├── Cargo.toml
└── src/
    ├── lib.rs                    # Public API surface, re-exports
    ├── client.rs                 # ElevenLabsClient struct, config, auth
    ├── config.rs                 # ClientConfig builder (base_url, timeout, retries)
    ├── error.rs                  # ElevenLabsError enum (thiserror)
    ├── auth.rs                   # ApiKey type, header injection
    ├── middleware.rs             # Retry, tracing, rate-limit middleware
    │
    ├── types/                    # All request/response types from OpenAPI schemas
    │   ├── mod.rs
    │   ├── common.rs             # Shared types (pagination, enums)
    │   ├── text_to_speech.rs
    │   ├── speech_to_speech.rs
    │   ├── speech_to_text.rs
    │   ├── text_to_dialogue.rs
    │   ├── voices.rs
    │   ├── voice_generation.rs
    │   ├── text_to_voice.rs
    │   ├── models.rs
    │   ├── history.rs
    │   ├── samples.rs
    │   ├── audio_isolation.rs
    │   ├── audio_native.rs
    │   ├── sound_generation.rs
    │   ├── music.rs
    │   ├── dubbing.rs
    │   ├── studio.rs
    │   ├── pronunciation.rs
    │   ├── agents.rs             # Conversational AI / Agents Platform
    │   ├── knowledge_base.rs
    │   ├── tools.rs              # ConvAI tools
    │   ├── phone_numbers.rs
    │   ├── workspace.rs
    │   ├── user.rs
    │   ├── pvc_voices.rs
    │   ├── forced_alignment.rs
    │   ├── single_use_token.rs
    │   └── webhooks.rs
    │
    ├── services/                 # Service modules — one per API tag group
    │   ├── mod.rs
    │   ├── text_to_speech.rs     # 4 endpoints
    │   ├── speech_to_speech.rs   # 2 endpoints
    │   ├── speech_to_text.rs     # 3 endpoints
    │   ├── text_to_dialogue.rs   # 4 endpoints
    │   ├── voices.rs             # 12 endpoints
    │   ├── voice_generation.rs   # 3 endpoints
    │   ├── text_to_voice.rs      # 5 endpoints
    │   ├── models.rs             # 1 endpoint
    │   ├── history.rs            # 5 endpoints
    │   ├── samples.rs            # 2 endpoints
    │   ├── audio_isolation.rs    # 2 endpoints
    │   ├── audio_native.rs       # 3 endpoints
    │   ├── sound_generation.rs   # 1 endpoint
    │   ├── music.rs              # 5 endpoints
    │   ├── dubbing.rs            # ~15 unique endpoints (deduplicated)
    │   ├── studio.rs             # 23 endpoints
    │   ├── pronunciation.rs      # 8 endpoints
    │   ├── agents.rs             # ~98 endpoints (Agents Platform + ConvAI)
    │   ├── workspace.rs          # 19 endpoints
    │   ├── user.rs               # 3 endpoints (user + subscription + usage)
    │   ├── pvc_voices.rs         # 14 endpoints
    │   ├── forced_alignment.rs   # 1 endpoint
    │   ├── single_use_token.rs   # 1 endpoint
    │   └── enterprise.rs         # Merged into dubbing (shared endpoints)
    │
    └── ws/                       # WebSocket client support
        ├── mod.rs
        ├── tts.rs                # Real-time TTS WebSocket
        └── conversation.rs       # Conversational AI WebSocket
```

### 4.2 Data Structures & Types

**Client Configuration:**

```rust
pub struct ClientConfig {
    pub base_url: String,         // Default: "https://api.elevenlabs.io"
    pub api_key: ApiKey,
    pub timeout: Duration,        // Default: 30s
    pub max_retries: u32,         // Default: 3
    pub retry_backoff: Duration,  // Default: 1s
}

pub struct ApiKey(String);        // Newtype, Debug redacted

pub struct ElevenLabsClient {
    config: ClientConfig,
    http: hpx::Client,            // hpx HTTP client
}
```

**Error Type:**

```rust
#[derive(Debug, thiserror::Error)]
pub enum ElevenLabsError {
    #[error("HTTP error: {status} {message}")]
    Api { status: u16, message: String, detail: Option<serde_json::Value> },

    #[error("Authentication failed")]
    Unauthorized,

    #[error("Rate limited, retry after {retry_after_ms}ms")]
    RateLimited { retry_after_ms: u64 },

    #[error("Request timeout")]
    Timeout,

    #[error("Transport error: {0}")]
    Transport(#[from] hpx::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    Validation(String),
}
```

**Example Type (Text-to-Speech):**

```rust
#[derive(Debug, Clone, Serialize)]
pub struct TextToSpeechRequest {
    pub text: String,
    pub model_id: Option<String>,
    pub language_code: Option<String>,
    pub voice_settings: Option<VoiceSettings>,
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
    pub seed: Option<i64>,
    pub previous_text: Option<String>,
    pub next_text: Option<String>,
    pub apply_text_normalization: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    pub stability: f64,
    pub similarity_boost: f64,
    pub style: Option<f64>,
    pub use_speaker_boost: Option<bool>,
}
```

**Service Pattern:**

```rust
impl ElevenLabsClient {
    pub fn text_to_speech(&self) -> TextToSpeechService<'_> {
        TextToSpeechService { client: self }
    }
}

pub struct TextToSpeechService<'a> {
    client: &'a ElevenLabsClient,
}

impl<'a> TextToSpeechService<'a> {
    pub async fn convert(&self, voice_id: &str, req: &TextToSpeechRequest)
        -> Result<Bytes, ElevenLabsError> { ... }

    pub async fn convert_stream(&self, voice_id: &str, req: &TextToSpeechRequest)
        -> Result<impl Stream<Item = Result<Bytes, ElevenLabsError>>, ElevenLabsError> { ... }

    pub async fn convert_with_timestamps(&self, voice_id: &str, req: &TextToSpeechRequest)
        -> Result<TextToSpeechTimestampResponse, ElevenLabsError> { ... }

    pub async fn convert_stream_with_timestamps(&self, voice_id: &str, req: &TextToSpeechRequest)
        -> Result<impl Stream<Item = Result<Bytes, ElevenLabsError>>, ElevenLabsError> { ... }
}
```

### 4.3 Interface Design

**Public API Surface (`lib.rs`):**

```rust
pub use client::ElevenLabsClient;
pub use config::ClientConfig;
pub use error::ElevenLabsError;
pub use auth::ApiKey;

pub mod types;      // All request/response types
pub mod services;   // Service modules
pub mod ws;         // WebSocket clients
```

**WebSocket Interface:**

```rust
pub struct TtsWebSocket {
    // wraps hpx-transport WebSocket connection
}

impl TtsWebSocket {
    pub async fn connect(config: &ClientConfig, voice_id: &str, model_id: &str)
        -> Result<Self, ElevenLabsError> { ... }

    pub async fn send_text(&mut self, text: &str)
        -> Result<(), ElevenLabsError> { ... }

    pub async fn flush(&mut self)
        -> Result<(), ElevenLabsError> { ... }

    pub async fn recv_audio(&mut self)
        -> Result<Option<TtsWsResponse>, ElevenLabsError> { ... }

    pub async fn close(&mut self)
        -> Result<(), ElevenLabsError> { ... }
}
```

### 4.4 Logic Flow

**REST Request Flow:**

1. User calls service method (e.g., `client.text_to_speech().convert(...)`)
2. Service constructs URL from base_url + path template + path params
3. Request builder sets headers (`xi-api-key`, `Content-Type`)
4. Middleware chain: tracing span → retry logic → rate-limit check
5. `hpx` sends request
6. Response is checked: 2xx → deserialize body; 4xx/5xx → map to `ElevenLabsError`
7. Retryable errors (429, 500, 502, 503) trigger exponential backoff retry

**WebSocket Flow (TTS):**

1. Connect via `hpx-transport` to `wss://api.elevenlabs.io/v1/text-to-speech/{voice_id}/stream-input`
2. Send `BOS` (beginning of stream) JSON message with model config
3. Send text chunks as JSON `{"text": "...", "try_trigger_generation": true}`
4. Receive audio chunks as JSON `{"audio": "<base64>", "alignment": {...}}`
5. Send `EOS` (end of stream) `{"text": ""}`
6. Close connection

### 4.5 Configuration

| Config | Env Var | Default | Description |
| :--- | :--- | :--- | :--- |
| `api_key` | `ELEVENLABS_API_KEY` | — (required) | API key for authentication |
| `base_url` | `ELEVENLABS_BASE_URL` | `https://api.elevenlabs.io` | API base URL |
| `timeout_secs` | — | `30` | Request timeout in seconds |
| `max_retries` | — | `3` | Maximum retry attempts for transient errors |
| `retry_backoff_ms` | — | `1000` | Initial backoff between retries |

### 4.6 Error Handling

- **4xx errors:** Parsed from response body JSON (ElevenLabs returns `{"detail": {...}}` on errors). Mapped to `ElevenLabsError::Api`.
- **401:** `ElevenLabsError::Unauthorized`
- **429:** `ElevenLabsError::RateLimited` with `retry_after_ms` from response header.
- **5xx:** Retried up to `max_retries`, then `ElevenLabsError::Api`.
- **Network errors:** `ElevenLabsError::Transport`.
- **Deserialization errors:** `ElevenLabsError::Deserialize` with context about which endpoint/type failed.
- **All errors implement `std::error::Error`** and are `Send + Sync`.

---

## 5. Verification & Testing Strategy

### 5.1 Unit Testing

- **wiremock 0.6.5** for isolated HTTP mock tests.
- Every service method has at least one test.
- Tests verify:
  - Correct URL path construction
  - Correct HTTP method
  - Request body serialization matches expected JSON
  - Response deserialization produces correct types
  - Error responses mapped to correct error variants
- Tests live in `#[cfg(test)] mod tests` in each service module.

### 5.2 Integration Testing

- **Prism** (Stoplight) launches a mock server from `docs/openapi.json`.
- Integration tests in `tests/` directory call every SDK endpoint against Prism.
- Prism validates request/response against the OpenAPI spec → catches field mismatches, missing required fields, wrong types.
- Run via `just sdk-test-integration`.

### 5.3 Critical Path Verification (The "Harness")

| Verification Step | Command | Success Criteria |
| :--- | :--- | :--- |
| **VP-01** | `cargo test -p elevenlabs-sdk --all-features` | All unit tests pass (wiremock) |
| **VP-02** | `npx @stoplight/prism-cli mock docs/openapi.json --port 4010 &` then `cargo test -p elevenlabs-sdk --test integration` | All integration tests pass against Prism mock |
| **VP-03** | `cargo clippy -p elevenlabs-sdk -- -D warnings` | No clippy warnings |
| **VP-04** | `cargo +nightly fmt --check -p elevenlabs-sdk` | Code is formatted |
| **VP-05** | `python3 scripts/check_coverage.py` | Every operationId in openapi.json has a corresponding test |

### 5.4 Validation Rules

| Test Case ID | Action | Expected Outcome | Verification Method |
| :--- | :--- | :--- | :--- |
| **TC-01** | Call `client.text_to_speech().convert(voice_id, req)` | Returns audio bytes, HTTP 200 | wiremock mock returns sample audio, assert `Bytes` non-empty |
| **TC-02** | Call `client.voices().list()` | Returns `Vec<Voice>` with all fields populated | wiremock returns JSON matching schema, assert deserialization |
| **TC-03** | Call with invalid API key | Returns `ElevenLabsError::Unauthorized` | wiremock returns 401 |
| **TC-04** | Call with rate limiting | Returns `ElevenLabsError::RateLimited` | wiremock returns 429 with retry-after |
| **TC-05** | Call streaming endpoint | Returns `Stream<Item=Result<Bytes>>` | wiremock returns chunked response |
| **TC-06** | Every operationId in spec has a test | 100% endpoint coverage | `check_coverage.py` script |

---

## 6. Implementation Plan

- [ ] **Phase 1: Foundation** — Crate scaffolding, `hpx`/`hpx-transport` integration, client config, auth, error types
- [ ] **Phase 2: Core Types** — All 875 schemas from OpenAPI spec as Rust types (grouped by module)
- [ ] **Phase 3: Service Implementation** — All 253 endpoints implemented as service methods
- [ ] **Phase 4: WebSocket Support** — TTS WebSocket and Conversational AI WebSocket clients
- [ ] **Phase 5: Testing & Validation** — wiremock unit tests, Prism integration tests, coverage script
- [ ] **Phase 6: Polish** — Documentation, examples, CI recipes, README

---

## 7. Cross-Functional Concerns

- **Backward Compatibility:** First release — no backward compatibility concerns. Follow semver from 0.1.0.
- **Security:** API key is a newtype with redacted `Debug`. No logging of request/response bodies at default log level (only at `TRACE`). `rustls` only — no OpenSSL.
- **Monitoring:** Every HTTP request emits a `tracing` span with `method`, `path`, `status`, `latency_ms`. WebSocket connections emit connect/disconnect events.
- **CI Integration:** Add `just sdk-test` and `just sdk-test-integration` to `Justfile`. Integration tests require `npx` and Prism.
- **Documentation:** All public types and methods have `///` doc comments. Module-level docs explain the API group. `README.md` updated with SDK usage examples.
