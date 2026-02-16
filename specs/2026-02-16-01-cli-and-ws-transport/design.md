# Design Document: CLI Crate & WebSocket hpx-transport Migration

| Metadata | Details |
| :--- | :--- |
| **Author** | pb-plan agent |
| **Status** | Draft |
| **Created** | 2026-02-16 |
| **Reviewers** | — |
| **Related Issues** | N/A |

## 1. Executive Summary

**Problem:** The workspace lacks a CLI binary for interacting with the ElevenLabs API from the command line. Additionally, the `elevenlabs-sdk` crate's WebSocket modules (`ws/tts.rs`, `ws/conversation.rs`) use low-level `hpx::ws` directly for WebSocket I/O instead of the higher-level `hpx-transport` crate (which is already declared as a dependency but never used). The `hpx-transport` crate provides managed WebSocket connections with built-in reconnection, protocol abstraction, ping/pong handling, and structured message classification — all of which the current hand-rolled implementation must handle manually.

**Solution:**

1. Create a new `bin/elevenlabs-cli/` binary crate that exposes every REST service and WebSocket API in the SDK as CLI subcommands via `clap`.
2. Migrate `ws/tts.rs` and `ws/conversation.rs` from raw `hpx::ws` to `hpx_transport::websocket` (`WsClient` + `ProtocolHandler`), leveraging managed connections and removing manual split-stream handling.

---

## 2. Requirements & Goals

### 2.1 Problem Statement

- **No CLI exists.** Developers and testers must write Rust programs or use `curl` to exercise SDK endpoints. A CLI would enable rapid exploration, scripting, and CI integration.
- **WebSocket layer uses wrong abstraction.** The project conventions and design doc specify `hpx-transport` for WebSocket transport, but the implementation fell back to raw `hpx::ws`. This means no automatic reconnection, no structured protocol handling, and manual message loop boilerplate duplicated across `tts.rs` and `conversation.rs`.

### 2.2 Functional Goals

1. **CLI binary (`elevenlabs-cli`):** A single binary with subcommands covering:
   - All 20 REST service groups (text-to-speech, voices, models, user, workspace, agents, audio-isolation, audio-native, dubbing, forced-alignment, history, music, pvc-voices, single-use-token, sound-generation, speech-to-speech, speech-to-text, studio, text-to-dialogue, text-to-voice, voice-generation).
   - WebSocket TTS streaming (`ws tts`).
   - Conversational AI WebSocket (`ws conversation`).
   - Output as JSON (default) or pretty-printed human-readable format.

2. **hpx-transport migration:** Replace `hpx::ws::{WebSocketWrite, WebSocketRead}` and `hpx::websocket()` usage with `hpx_transport::websocket::{WsClient, WsConfig, ProtocolHandler}` in:
   - `crates/elevenlabs-sdk/src/ws/tts.rs`
   - `crates/elevenlabs-sdk/src/ws/conversation.rs`
   - `crates/elevenlabs-sdk/src/ws/mod.rs` (re-exports)

### 2.3 Non-Functional Goals

- **Performance:** No unnecessary allocations. Stream audio output directly to files or stdout.
- **Reliability:** WebSocket connections benefit from `hpx-transport`'s built-in reconnection and ping/pong management.
- **Security:** API key read from `ELEVENLABS_API_KEY` env var. Never printed in logs.
- **Observability:** CLI uses `tracing-subscriber` for configurable log levels (`--verbose` / `-v` flag).

### 2.4 Out of Scope

- GUI or TUI interface.
- Shell completions (can be added later via `clap_complete`).
- Audio playback from the CLI (output to file only).
- Implementing new SDK service methods — the CLI wraps existing SDK API surface.

### 2.5 Assumptions

- `hpx-transport` 1.4.0's `ProtocolHandler` trait can accommodate ElevenLabs' JSON-text-frame protocols for both TTS and Conversational AI.
- The CLI does not need to maintain backward compatibility (new crate).
- The `hpx::ws` feature on `hpx` can be removed from `elevenlabs-sdk` after migration to `hpx-transport`, or kept if other non-WS features still need it.

---

## 3. Architecture Overview

### 3.1 System Context

```text
┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│  elevenlabs-cli  │────▶│  elevenlabs-sdk  │────▶│ ElevenLabs API   │
│  (bin crate)     │     │  (lib crate)     │     │ REST + WebSocket │
│  clap subcommands│     │  services/ + ws/ │     └──────────────────┘
└──────────────────┘     │  hpx (HTTP)      │
                         │  hpx-transport   │
                         │   (WebSocket)    │
                         └──────────────────┘
```

The CLI is a thin layer: it parses arguments, constructs SDK requests, calls SDK methods, and prints results. All business logic stays in the SDK crate.

### 3.2 Key Design Principles

1. **Thin CLI, thick SDK.** The CLI does argument parsing + output formatting only. No API logic in the CLI.
2. **Subcommand-per-service.** Each REST service maps to a top-level subcommand, each method maps to a sub-subcommand.
3. **Protocol Handler pattern.** Each ElevenLabs WebSocket protocol (TTS, Conversation) gets its own `ProtocolHandler` implementation for `hpx-transport`.
4. **Backward compatible SDK API.** The public API of `TtsWebSocket` and `ConversationWebSocket` retains the same method signatures; only the internal transport changes.

### 3.3 Existing Components to Reuse

| Component | Location | How to Reuse |
| :--- | :--- | :--- |
| `ElevenLabsClient` | `crates/elevenlabs-sdk/src/client.rs` | CLI instantiates this for all REST calls |
| `ClientConfig` / `ClientConfig::from_env()` | `crates/elevenlabs-sdk/src/config.rs` | CLI uses env-based config |
| 20 service modules | `crates/elevenlabs-sdk/src/services/*.rs` | CLI subcommands call these directly |
| `TtsWebSocket` / `ConversationWebSocket` | `crates/elevenlabs-sdk/src/ws/*.rs` | CLI `ws` subcommand calls these; internal transport migrates to `hpx-transport` |
| `build_ws_url()` | `crates/elevenlabs-sdk/src/ws/mod.rs` | Reused by `WsConfig` URL construction |
| All request/response types | `crates/elevenlabs-sdk/src/types/*.rs` | CLI serializes/deserializes these |
| `hpx-transport` 1.4.0 | workspace dep (declared, unused) | WebSocket managed connections |
| `clap` 4.5.56 | workspace dep | CLI argument parsing |
| `tracing` / `tracing-subscriber` | workspace deps | CLI logging |

---

## 4. Detailed Design

### 4.1 Module Structure

**New crate: `bin/elevenlabs-cli/`**

```text
bin/elevenlabs-cli/
├── Cargo.toml
└── src/
    ├── main.rs              # Entry point, clap derive App
    ├── cli.rs               # Top-level CLI enum + global args
    ├── output.rs            # JSON / pretty-print output helpers
    └── commands/
        ├── mod.rs
        ├── tts.rs           # text-to-speech subcommands
        ├── voices.rs        # voices subcommands
        ├── models.rs        # models subcommands
        ├── user.rs          # user subcommands
        ├── workspace.rs     # workspace subcommands
        ├── agents.rs        # agents subcommands
        ├── audio_isolation.rs
        ├── audio_native.rs
        ├── dubbing.rs
        ├── forced_alignment.rs
        ├── history.rs
        ├── music.rs
        ├── pvc_voices.rs
        ├── single_use_token.rs
        ├── sound_generation.rs
        ├── speech_to_speech.rs
        ├── speech_to_text.rs
        ├── studio.rs
        ├── text_to_dialogue.rs
        ├── text_to_voice.rs
        ├── voice_generation.rs
        └── ws.rs            # WebSocket subcommands (tts, conversation)
```

**Modified in `crates/elevenlabs-sdk/`:**

```text
crates/elevenlabs-sdk/src/ws/
├── mod.rs                   # Update re-exports (remove hpx::ws re-exports, add hpx-transport types)
├── tts.rs                   # Migrate from hpx::ws to hpx_transport::websocket
├── conversation.rs          # Migrate from hpx::ws to hpx_transport::websocket
├── tts_handler.rs           # NEW: ProtocolHandler impl for TTS protocol
└── conversation_handler.rs  # NEW: ProtocolHandler impl for Conversation protocol
```

### 4.2 Data Structures & Types

#### CLI Top-Level

```rust
#[derive(clap::Parser)]
#[command(name = "elevenlabs", about = "ElevenLabs API CLI")]
struct Cli {
    /// API key (overrides ELEVENLABS_API_KEY env var)
    #[arg(long, env = "ELEVENLABS_API_KEY", global = true)]
    api_key: String,

    /// Base URL override
    #[arg(long, env = "ELEVENLABS_BASE_URL", global = true)]
    base_url: Option<String>,

    /// Output format
    #[arg(long, default_value = "json", global = true)]
    format: OutputFormat,

    /// Verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Text-to-speech operations
    Tts(TtsArgs),
    /// Voice management
    Voices(VoicesArgs),
    /// List available models
    Models(ModelsArgs),
    // ... all 20 services + ws
    /// WebSocket streaming
    Ws(WsArgs),
}
```

#### Protocol Handlers (hpx-transport)

```rust
/// TTS WebSocket protocol handler for hpx-transport.
pub(crate) struct TtsProtocolHandler;

impl hpx_transport::websocket::ProtocolHandler for TtsProtocolHandler {
    fn classify_message(&self, msg: &hpx_transport::websocket::WsMessage) -> MessageKind {
        // TTS uses simple fire-and-forget text messages; responses are unsolicited
        MessageKind::Unsolicited
    }
    fn extract_request_id(&self, _msg: &hpx_transport::websocket::WsMessage) -> Option<RequestId> {
        None // TTS protocol has no request-response correlation
    }
    fn extract_topic(&self, _msg: &hpx_transport::websocket::WsMessage) -> Option<Topic> {
        None // TTS protocol has no pub/sub topics
    }
    fn build_subscribe(&self, _topic: &Topic) -> hpx_transport::websocket::WsMessage {
        unreachable!("TTS protocol does not use subscriptions")
    }
    fn build_unsubscribe(&self, _topic: &Topic) -> hpx_transport::websocket::WsMessage {
        unreachable!("TTS protocol does not use subscriptions")
    }
}
```

### 4.3 Interface Design

The CLI binary's public interface is its command-line API:

```bash
# REST examples
elevenlabs tts convert --voice-id <ID> --text "Hello" -o output.mp3
elevenlabs tts stream --voice-id <ID> --text "Hello" -o output.mp3
elevenlabs voices list
elevenlabs voices get --voice-id <ID>
elevenlabs models list
elevenlabs user info
elevenlabs user subscription

# WebSocket examples
elevenlabs ws tts --voice-id <ID> --model-id eleven_turbo_v2 --text "Hello" -o output.mp3
elevenlabs ws conversation --agent-id <ID>
```

The SDK's public Rust API for `TtsWebSocket` and `ConversationWebSocket` remains unchanged:

```rust
// These method signatures stay the same:
TtsWebSocket::connect(config, ws_config) -> Result<Self>
TtsWebSocket::send_text(&mut self, text) -> Result<()>
TtsWebSocket::flush(&mut self) -> Result<()>
TtsWebSocket::recv(&mut self) -> Result<Option<TtsWsResponse>>
TtsWebSocket::close(self) -> Result<()>

ConversationWebSocket::connect(signed_url) -> Result<Self>
ConversationWebSocket::connect_with_agent(client, agent_id) -> Result<Self>
ConversationWebSocket::send_audio(&mut self, audio) -> Result<()>
ConversationWebSocket::recv(&mut self) -> Result<Option<ConversationEvent>>
ConversationWebSocket::send_pong(&mut self, event_id) -> Result<()>
ConversationWebSocket::close(self) -> Result<()>
```

### 4.4 Logic Flow

**CLI command execution flow:**

1. Parse CLI args via `clap`.
2. Initialize `tracing-subscriber` (with verbosity level).
3. Build `ClientConfig` from `--api-key` / env + optional `--base-url`.
4. Create `ElevenLabsClient`.
5. Dispatch to the matched subcommand handler.
6. Handler calls the appropriate SDK service method.
7. Format result as JSON and print to stdout.
8. For streaming/WebSocket: write audio bytes to output file, print progress to stderr.

**hpx-transport WebSocket flow (TTS):**

1. Build `WsConfig` with the constructed URL, auth header, and timeouts.
2. Create `TtsProtocolHandler`.
3. Call `WsClient::connect(config, handler)`.
4. Send BOS via `client.send_json(bos)`.
5. Send text chunks via `client.send_json(chunk)`.
6. Receive responses from the managed connection's incoming stream.
7. Send EOS and close.

### 4.5 Configuration

**New CLI-specific configuration (via clap args / env vars):**

| Arg | Env Var | Default | Description |
|-----|---------|---------|-------------|
| `--api-key` | `ELEVENLABS_API_KEY` | (required) | API key |
| `--base-url` | `ELEVENLABS_BASE_URL` | `https://api.elevenlabs.io` | Base URL |
| `--format` | — | `json` | Output format (`json` \| `pretty`) |
| `-v, --verbose` | — | false | Enable debug logging |
| `-o, --output` | — | stdout | Output file for audio data |

### 4.6 Error Handling

- CLI uses `eyre` for top-level error reporting (as per project conventions for binaries).
- SDK continues to use `thiserror`-based `ElevenLabsError`.
- WebSocket errors from `hpx-transport` are mapped to `ElevenLabsError::WebSocket(String)`, same as today.
- CLI exits with code 1 on any error, printing the error chain to stderr.

---

## 5. Verification & Testing Strategy

### 5.1 Unit Testing

- **Protocol handlers:** Test `classify_message`, `extract_request_id` with sample messages.
- **CLI arg parsing:** Test clap derive parsing for critical subcommands.
- **Output formatting:** Test JSON and pretty-print output helpers.

### 5.2 Integration Testing

- **SDK WebSocket tests:** Existing unit tests in `ws/tts.rs` and `ws/conversation.rs` (serialization/deserialization) remain valid.
- **CLI integration:** Manual testing against Prism mock server, or E2E tests with subprocess invocation.

### 5.3 Critical Path Verification (The "Harness")

| Verification Step | Command | Success Criteria |
| :--- | :--- | :--- |
| **VP-01** | `cargo build -p elevenlabs-cli` | Compiles without errors |
| **VP-02** | `cargo test -p elevenlabs-sdk` | All existing tests pass |
| **VP-03** | `cargo clippy --all -- -D warnings` | No warnings |
| **VP-04** | `cargo +nightly fmt --all -- --check` | Formatted |
| **VP-05** | `elevenlabs --help` | Shows all subcommands |
| **VP-06** | `elevenlabs voices list` (with API key) | Returns JSON voice list |
| **VP-07** | `elevenlabs ws tts --voice-id <id> --text "test" -o /dev/null` | Streams audio via hpx-transport |

### 5.4 Validation Rules

| Test Case ID | Action | Expected Outcome | Verification Method |
| :--- | :--- | :--- | :--- |
| **TC-01** | Build CLI crate | Compiles | `cargo build -p elevenlabs-cli` |
| **TC-02** | Run `elevenlabs voices list` | JSON output with voice array | Manual + Prism |
| **TC-03** | Run `elevenlabs tts convert` | MP3 file produced | File size > 0 |
| **TC-04** | TTS WebSocket via hpx-transport | Audio received | Run `websocket_tts` example |
| **TC-05** | Conversation WS via hpx-transport | Events received | Run with agent ID |
| **TC-06** | SDK unit tests pass after migration | All green | `cargo test -p elevenlabs-sdk` |

---

## 6. Implementation Plan

- [ ] **Phase 1: Foundation** — Scaffold CLI crate, add workspace member, define clap structure.
- [ ] **Phase 2: Core Logic** — Implement hpx-transport migration for WebSocket modules. Implement CLI subcommand handlers for all REST services.
- [ ] **Phase 3: Integration** — Wire CLI WebSocket subcommands. End-to-end testing.
- [ ] **Phase 4: Polish** — Format, lint, documentation, Justfile commands.

---

## 7. Cross-Functional Concerns

- **Backward Compatibility:** The SDK's public API (`TtsWebSocket`, `ConversationWebSocket` method signatures) does not change. Only the internal transport layer is swapped. Downstream users are unaffected.
- **Dependency cleanup:** After migration, evaluate whether the `ws` feature on `hpx` can be removed from `elevenlabs-sdk`'s Cargo.toml (it may still be needed for re-exported types like `Message`). If `hpx-transport` re-exports the necessary message types, the `ws` feature can be dropped.
- **Binary size:** The CLI binary pulls in `clap` and `elevenlabs-sdk`. This is acceptable for a developer tool.
- **Security:** API keys are never logged. The CLI reads them from env vars or `--api-key` (marked sensitive in clap).
