# CLI Crate & WebSocket hpx-transport Migration — Implementation Tasks

| Metadata | Details |
| :--- | :--- |
| **Design Doc** | specs/2026-02-16-01-cli-and-ws-transport/design.md |
| **Owner** | — |
| **Start Date** | 2026-02-16 |
| **Target Date** | 2026-02-23 |
| **Status** | Planning |

## Summary & Phasing

This implementation has two parallel tracks:

- **Track A (SDK):** Migrate WebSocket modules from raw `hpx::ws` to `hpx_transport::websocket`.
- **Track B (CLI):** Create the `elevenlabs-cli` binary crate with subcommands for all REST and WebSocket APIs.

Track A must complete before Track B's WebSocket subcommands can be wired, but Track B's REST subcommands can proceed in parallel.

- **Phase 1: Foundation & Scaffolding** — CLI crate scaffold, workspace config, clap skeleton.
- **Phase 2: Core Logic** — hpx-transport migration, CLI REST subcommand handlers.
- **Phase 3: Integration & Features** — CLI WebSocket subcommands, output formatting, end-to-end testing.
- **Phase 4: Polish, QA & Docs** — Lint, format, documentation, Justfile updates, cleanup.

---

## Phase 1: Foundation & Scaffolding

### Task 1.1: Scaffold `elevenlabs-cli` Crate

> **Context:** Create the new binary crate under `bin/elevenlabs-cli/` and add it to the workspace. The CLI uses `clap` for argument parsing, `eyre` for error handling (binary crate convention), `tracing-subscriber` for logging, and depends on `elevenlabs-sdk`. Reuse workspace dependency versions.
> **Verification:** `cargo check -p elevenlabs-cli` compiles.

- **Priority:** P0
- **Scope:** Crate scaffold + workspace membership
- **Status:** � DONE

- [x] **Step 1:** Create `bin/elevenlabs-cli/Cargo.toml` with `workspace = true` for version/edition. Dependencies: `elevenlabs-sdk` (path), `clap` (workspace, features `["derive", "env"]`), `eyre` (workspace), `tokio` (workspace, features `["rt-multi-thread", "macros"]`), `tracing` (workspace), `tracing-subscriber` (workspace), `serde_json` (workspace).
- [x] **Step 2:** Update root `Cargo.toml` workspace members to include `bin/*` alongside `crates/*`.
- [x] **Step 3:** Create `bin/elevenlabs-cli/src/main.rs` with a minimal `#[tokio::main]` entry point that parses `Cli` args and prints help.
- [x] **Step 4:** Create `bin/elevenlabs-cli/src/cli.rs` with the top-level `Cli` struct (clap derive) including `--api-key`, `--base-url`, `--format`, `--verbose` global args and a `Commands` enum stub.
- [x] **Step 5:** Create `bin/elevenlabs-cli/src/output.rs` with `OutputFormat` enum (`Json` / `Pretty`) and a `print_json<T: Serialize>(value: &T, format: OutputFormat)` helper.
- [x] **Step 6:** Create `bin/elevenlabs-cli/src/commands/mod.rs` with empty command module declarations.
- [x] **Verification:** `cargo check -p elevenlabs-cli` compiles. `cargo run -p elevenlabs-cli -- --help` prints usage.

---

### Task 1.2: Define CLI Subcommand Structure

> **Context:** Define the full `Commands` enum with a subcommand per SDK service group (20 REST services + 1 `ws` group). Each subcommand module gets a struct with `#[derive(clap::Args)]` and an inner subcommand enum for individual operations. Reference the 20 service accessor methods on `ElevenLabsClient`.
> **Verification:** `cargo check -p elevenlabs-cli` compiles with all subcommand stubs.

- **Priority:** P0
- **Scope:** CLI command structure definition
- **Status:** � DONE

- [x] **Step 1:** Create command module files for all service groups under `bin/elevenlabs-cli/src/commands/`: `tts.rs`, `voices.rs`, `models.rs`, `user.rs`, `workspace.rs`, `agents.rs`, `audio_isolation.rs`, `audio_native.rs`, `dubbing.rs`, `forced_alignment.rs`, `history.rs`, `music.rs`, `pvc_voices.rs`, `single_use_token.rs`, `sound_generation.rs`, `speech_to_speech.rs`, `speech_to_text.rs`, `studio.rs`, `text_to_dialogue.rs`, `text_to_voice.rs`, `voice_generation.rs`, `ws.rs`.
- [x] **Step 2:** In each command module, define the clap `Args` struct and inner `Subcommand` enum mapping to the corresponding SDK service methods. Start with the most commonly used operations (list, get, convert) and add all others.
- [x] **Step 3:** Wire all subcommands into the `Commands` enum in `cli.rs`.
- [x] **Step 4:** Add placeholder `execute` async functions that return `Ok(())` for each command module.
- [x] **Verification:** `cargo check -p elevenlabs-cli` builds. `cargo run -p elevenlabs-cli -- tts --help` shows TTS subcommands.

---

## Phase 2: Core Logic

### Task 2.1: Implement `ProtocolHandler` for TTS WebSocket

> **Context:** Create a `TtsProtocolHandler` implementing `hpx_transport::websocket::ProtocolHandler`. The TTS protocol is simple fire-and-forget: text messages in, JSON responses out. No request-response correlation or subscriptions. The handler classifies all incoming messages as `Unsolicited`. This enables replacing raw `hpx::ws` with managed `WsClient` in `tts.rs`.
> **Verification:** Unit test for `classify_message` returns correct `MessageKind`.

- **Priority:** P0
- **Scope:** New file `ws/tts_handler.rs`
- **Status:** � DONE

- [x] **Step 1:** Create `crates/elevenlabs-sdk/src/ws/tts_handler.rs`.
- [x] **Step 2:** Implement `ProtocolHandler` trait for `TtsProtocolHandler`: `classify_message` → `MessageKind::Unknown` (routes to event stream), `extract_request_id` → `None`, `extract_topic` → `None`, `build_subscribe` / `build_unsubscribe` → `WsMessage::text("{}")`.
- [x] **Step 3:** BOS message sent from `TtsWebSocket::connect()` via `handle.send()` (not `on_connect`).
- [x] **Step 4:** Auth handled via BOS message `xi_api_key` field (existing approach preserved).
- [x] **Step 5:** Register module in `ws/mod.rs` as `pub(crate) mod tts_handler`.
- [x] **Verification:** 6 unit tests for handler methods pass. `cargo test -p elevenlabs-sdk --lib`.

---

### Task 2.2: Implement `ProtocolHandler` for Conversation WebSocket

> **Context:** Create a `ConversationProtocolHandler` implementing `hpx_transport::websocket::ProtocolHandler`. The Conversation protocol uses JSON text frames with a `"type"` field for event classification. Ping/pong events are application-level (not WebSocket control frames) and must be handled by the protocol handler. No request-response correlation; all messages are unsolicited events.
> **Verification:** Unit test for `classify_message` and `is_server_ping`.

- **Priority:** P0
- **Scope:** New file `ws/conversation_handler.rs`
- **Status:** � DONE

- [x] **Step 1:** Create `crates/elevenlabs-sdk/src/ws/conversation_handler.rs`.
- [x] **Step 2:** Implement `ProtocolHandler` trait: `classify_message` → `MessageKind::Unknown` for all events (routes to event stream), `extract_request_id` → `None`, `extract_topic` → `None`.
- [x] **Step 3:** `is_server_ping` returns false — pings flow through event stream to preserve existing public API where caller handles pings.
- [x] **Step 4:** `build_pong` returns None — pong messages sent by caller via `ConversationWebSocket::send_pong()`.
- [x] **Step 5:** Register module in `ws/mod.rs` as `pub(crate) mod conversation_handler`.
- [x] **Verification:** 7 unit tests pass. `cargo test -p elevenlabs-sdk --lib`.

---

### Task 2.3: Migrate `TtsWebSocket` to hpx-transport

> **Context:** Replace the raw `hpx::ws::WebSocketWrite` / `WebSocketRead` split-stream implementation in `tts.rs` with `hpx_transport::websocket::WsClient<TtsProtocolHandler>`. The public API (`connect`, `send_text`, `flush`, `recv`, `close`) must remain unchanged. Internally, `connect` builds a `WsConfig`, creates a `WsClient`, and connects. Send methods use `client.send()` / `client.send_json()`. Receive uses the managed connection's incoming message stream.
> **Verification:** All existing unit tests in `ws/tts.rs` still pass. `cargo test -p elevenlabs-sdk`.

- **Priority:** P0
- **Scope:** Rewrite `ws/tts.rs` internals
- **Status:** � DONE

- [x] **Step 1:** Replaced struct fields with `handle: ConnectionHandle, stream: ConnectionStream` using `Connection::connect()` API.
- [x] **Step 2:** Rewrote `connect()` to build `WsConfig`, call `Connection::connect()`, send BOS via `handle.send()`.
- [x] **Step 3:** Rewrote `send_text()`, `flush()` to use `handle.send(WsMessage::text(...))`.
- [x] **Step 4:** Rewrote `recv()` to receive from `ConnectionStream::next()`, match `Event::Message` for text.
- [x] **Step 5:** Rewrote `close()` to send EOS via `handle.send()` then `handle.close()`.
- [x] **Step 6:** Updated imports: removed `hpx::ws::*`, `futures_util::TryStreamExt`. Added `hpx_transport::websocket::{Connection, ConnectionHandle, ConnectionStream, Event, WsConfig, WsMessage}`.
- [x] **Verification:** All 10 existing `ws::tts::tests` pass. `cargo test -p elevenlabs-sdk --lib`.

---

### Task 2.4: Migrate `ConversationWebSocket` to hpx-transport

> **Context:** Same migration pattern as Task 2.3 but for `conversation.rs`. Replace raw `hpx::ws` with `WsClient<ConversationProtocolHandler>`. Public API unchanged. The conversation protocol's application-level ping/pong may be handled automatically by the `ProtocolHandler` or still exposed to the caller, depending on `hpx-transport`'s behavior — verify and document.
> **Verification:** All existing unit tests in `ws/conversation.rs` still pass.

- **Priority:** P0
- **Scope:** Rewrite `ws/conversation.rs` internals
- **Status:** � DONE

- [x] **Step 1:** Replaced struct fields with `handle: ConnectionHandle, stream: ConnectionStream`.
- [x] **Step 2:** Rewrote `connect()` and `connect_with_agent()` using `Connection::connect()`.
- [x] **Step 3:** Rewrote `send_audio()` and `send_pong()` using `handle.send(WsMessage::text(...))`.
- [x] **Step 4:** Rewrote `recv()` to use `ConnectionStream::next()`, match `Event::Message`.
- [x] **Step 5:** Rewrote `close()` using `handle.close()`.
- [x] **Step 6:** Updated imports: removed `hpx::ws::*`, `futures_util::TryStreamExt`. Added `hpx_transport::websocket::*`.
- [x] **Verification:** All 10 existing `ws::conversation::tests` pass. `cargo test -p elevenlabs-sdk --lib`.

---

### Task 2.5: Update `ws/mod.rs` Re-exports

> **Context:** The current `ws/mod.rs` re-exports `hpx::ws::message::{CloseCode, CloseFrame, Message, Utf8Bytes}`. After migration, these may no longer be needed or should be replaced with `hpx_transport::websocket::WsMessage` equivalents. Also register the new handler modules.
> **Verification:** `cargo check -p elevenlabs-sdk` and `cargo doc -p elevenlabs-sdk` compile.

- **Priority:** P0
- **Scope:** Update module re-exports
- **Status:** � DONE

- [x] **Step 1:** Added `pub(crate) mod tts_handler;` and `pub(crate) mod conversation_handler;` to `ws/mod.rs`.
- [x] **Step 2:** Removed `pub use hpx::ws::message::{CloseCode, CloseFrame, Message, Utf8Bytes}` — no longer needed.
- [x] **Step 3:** Verified `lib.rs` public exports compile. Also removed `futures-util` from SDK deps (no longer used).
- [x] **Step 4:** Updated doc comment in `ws/mod.rs` to reference `hpx_transport::websocket`.
- [x] **Verification:** `cargo check -p elevenlabs-sdk` succeeds. `cargo clippy -p elevenlabs-sdk --lib` clean.

---

### Task 2.6: Implement CLI REST Subcommand Handlers

> **Context:** Implement the `execute` functions for all REST service subcommands. Each handler: (1) creates `ClientConfig` from CLI args, (2) creates `ElevenLabsClient`, (3) calls the matching SDK service method, (4) prints the result. Start with high-priority services (TTS, voices, models, user) then add the rest. Reuse the `output.rs` helpers.
> **Verification:** `cargo build -p elevenlabs-cli` succeeds. Each subcommand's `--help` works.

- **Priority:** P1
- **Scope:** Implement all REST command handlers
- **Status:** 🔴 TODO

- [ ] **Step 1:** Implement `tts` commands: `convert` (with `--voice-id`, `--text`, `--output-format`, `-o`), `stream`, `convert-with-timestamps`.
- [ ] **Step 2:** Implement `voices` commands: `list`, `get`, `delete`, `edit`, `add`.
- [ ] **Step 3:** Implement `models` commands: `list`.
- [ ] **Step 4:** Implement `user` commands: `info`, `subscription`.
- [ ] **Step 5:** Implement `workspace` commands: available SDK methods.
- [ ] **Step 6:** Implement remaining service commands: `agents`, `audio-isolation`, `audio-native`, `dubbing`, `forced-alignment`, `history`, `music`, `pvc-voices`, `single-use-token`, `sound-generation`, `speech-to-speech`, `speech-to-text`, `studio`, `text-to-dialogue`, `text-to-voice`, `voice-generation`.
- [ ] **Verification:** `cargo build -p elevenlabs-cli` compiles. All `--help` outputs correct.

---

## Phase 3: Integration & Features

### Task 3.1: Implement CLI WebSocket Subcommands

> **Context:** Wire the `ws tts` and `ws conversation` CLI subcommands to the SDK's `TtsWebSocket` and `ConversationWebSocket` (now backed by hpx-transport). The TTS subcommand reads text from `--text` or stdin, streams audio to `-o` file. The conversation subcommand connects to an agent and prints events as JSON.
> **Verification:** `cargo run -p elevenlabs-cli -- ws tts --help` shows correct args.

- **Priority:** P1
- **Scope:** CLI WebSocket integration
- **Status:** 🔴 TODO

- [ ] **Step 1:** Implement `ws tts` command: parse `--voice-id`, `--model-id`, `--text`, `-o`. Call `TtsWebSocket::connect()`, send text, receive audio, write to file.
- [ ] **Step 2:** Implement `ws conversation` command: parse `--agent-id`. Call `ConversationWebSocket::connect_with_agent()`, loop receiving events, print as JSON, handle pings.
- [ ] **Step 3:** Add progress output to stderr for streaming operations.
- [ ] **Verification:** `cargo build -p elevenlabs-cli` succeeds. Manual test with API key.

---

### Task 3.2: End-to-End Verification

> **Context:** Run the full verification harness from the design doc. Ensure all VP-01 through VP-07 pass. Run against Prism mock server for REST endpoints. WebSocket endpoints require a real API key or manual verification.
> **Verification:** All verification steps pass.

- **Priority:** P1
- **Scope:** E2E testing
- **Status:** 🔴 TODO

- [ ] **Step 1:** Run `cargo build -p elevenlabs-cli` (VP-01).
- [ ] **Step 2:** Run `cargo test -p elevenlabs-sdk` (VP-02) — verify all existing tests pass including WS migration.
- [ ] **Step 3:** Run `cargo clippy --all -- -D warnings` (VP-03).
- [ ] **Step 4:** Run `cargo +nightly fmt --all -- --check` (VP-04).
- [ ] **Step 5:** Run `cargo run -p elevenlabs-cli -- --help` (VP-05) — verify all subcommands listed.
- [ ] **Step 6:** Manual test: `elevenlabs voices list` against Prism or live API (VP-06).
- [ ] **Step 7:** Manual test: WS TTS round-trip (VP-07).
- [ ] **Verification:** All VP steps green.

---

## Phase 4: Polish, QA & Docs

### Task 4.1: Update Justfile with CLI Commands

> **Context:** Add `cli-build`, `cli-run`, `cli-lint` recipes to the Justfile for developer convenience. Also update the top-level `lint`, `test`, `build` recipes to include the CLI crate.
> **Verification:** `just cli-build` and `just lint` succeed.

- **Priority:** P2
- **Scope:** Justfile updates
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add `cli-build: cargo build -p elevenlabs-cli`.
- [ ] **Step 2:** Add `cli-lint: cargo clippy -p elevenlabs-cli -- -D warnings`.
- [ ] **Step 3:** Ensure `just lint` and `just build` include the CLI crate (they should via `--workspace` / `--all`).
- [ ] **Verification:** `just cli-build` and `just lint` succeed.

---

### Task 4.2: Update SDK Dependency — Remove `hpx` `ws` Feature If Possible

> **Context:** After migration, check if `hpx`'s `ws` feature is still needed by `elevenlabs-sdk`. If all WebSocket types now come from `hpx-transport`, remove the `ws` feature from `hpx` in `crates/elevenlabs-sdk/Cargo.toml` to reduce compile time and binary size. If some re-exported types (e.g., `Message`) are still needed from `hpx::ws`, keep the feature.
> **Verification:** `cargo check -p elevenlabs-sdk` compiles. `cargo doc -p elevenlabs-sdk` succeeds.

- **Priority:** P2
- **Scope:** Dependency cleanup
- **Status:** 🔴 TODO

- [ ] **Step 1:** Grep for any remaining `hpx::ws` usage in `elevenlabs-sdk` source.
- [ ] **Step 2:** If none remain, remove `"ws"` from `hpx` features in `crates/elevenlabs-sdk/Cargo.toml`.
- [ ] **Step 3:** Run `cargo check -p elevenlabs-sdk` and `cargo test -p elevenlabs-sdk` to verify.
- [ ] **Verification:** Compiles and tests pass with reduced features.

---

### Task 4.3: Documentation & AGENTS.md Update

> **Context:** Update `AGENTS.md` to document the new CLI crate, update the workspace layout diagram, and add CLI-related `just` commands. Add doc comments to all new public types in the CLI crate. Update the SDK's `ws/mod.rs` doc comments to reference `hpx-transport`.
> **Verification:** `cargo doc --workspace --no-deps` builds without warnings.

- **Priority:** P2
- **Scope:** Documentation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Update `AGENTS.md` workspace layout to include `bin/elevenlabs-cli/`.
- [ ] **Step 2:** Add CLI commands section to `AGENTS.md` (e.g., `just cli-build`, usage examples).
- [ ] **Step 3:** Add `#![doc = "..."]` module-level docs to CLI crate.
- [ ] **Step 4:** Verify `cargo doc --workspace --no-deps` produces clean output.
- [ ] **Verification:** Docs build without warnings. `AGENTS.md` reflects current workspace.

---

### Task 4.4: Final Lint, Format & CI Check

> **Context:** Run the full project quality checklist: format, lint, test, check for Chinese characters, run `cargo machete` for unused deps.
> **Verification:** `just ci` passes.

- **Priority:** P2
- **Scope:** Quality assurance
- **Status:** 🔴 TODO

- [ ] **Step 1:** Run `just format` (or `cargo +nightly fmt --all`).
- [ ] **Step 2:** Run `just lint` — fix any clippy warnings.
- [ ] **Step 3:** Run `just test` — all tests pass.
- [ ] **Step 4:** Run `cargo machete` — no unused dependencies.
- [ ] **Step 5:** Run `just check-cn` — no Chinese characters.
- [ ] **Verification:** `just ci` exits 0.

---

## Summary & Timeline

| Phase | Tasks | Target Date |
| :--- | :---: | :--- |
| **1. Foundation** | 2 | 02-17 |
| **2. Core Logic** | 6 | 02-20 |
| **3. Integration** | 2 | 02-21 |
| **4. Polish** | 4 | 02-23 |
| **Total** | **14** | |

## Definition of Done

1. [ ] **Linted:** `cargo clippy --all -- -D warnings` clean.
2. [ ] **Tested:** All unit tests pass (`cargo test --all-features`).
3. [ ] **Formatted:** `cargo +nightly fmt --all -- --check` clean.
4. [ ] **Verified:** Each task's specific Verification criterion met.
5. [ ] **No unused deps:** `cargo machete` clean.
6. [ ] **English only:** No Chinese characters in comments/docs.
7. [ ] **hpx-transport used:** No direct `hpx::ws` usage in `elevenlabs-sdk` WebSocket modules.
8. [ ] **CLI functional:** `elevenlabs --help` lists all subcommands. Core commands work against live or mock API.
