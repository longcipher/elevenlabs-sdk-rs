# AGENTS.md — ElevenLabs Rust SDK

Project conventions and structure for AI coding agents.

## Overview

Unofficial Rust SDK for the ElevenLabs API with full REST and WebSocket coverage.

## Workspace Layout

```text
elevenlabs-sdk-rs/
├── Cargo.toml                # Root workspace manifest (versions defined here)
├── Justfile                  # Task runner (just)
├── docs/openapi.json         # ElevenLabs OpenAPI spec (source of truth)
├── scripts/check_coverage.py # Endpoint coverage checker
├── bin/
│   ├── elevenlabs-cli/       # CLI binary (clap-based)
│   │   └── src/
│   │       ├── main.rs        # Entry point & command dispatch
│   │       ├── cli.rs         # Cli struct & Commands enum
│   │       ├── context.rs     # Client construction helper
│   │       ├── output.rs      # JSON output formatting
│   │       └── commands/      # One module per API group (22 commands)
│   └── leptos-csr-app/       # Frontend demo app (Leptos CSR)
└── crates/
    ├── common/               # Shared utilities
    └── elevenlabs-sdk/       # Core SDK crate
        ├── src/
        │   ├── lib.rs        # Public re-exports
        │   ├── client.rs     # ElevenLabsClient (HTTP, retry)
        │   ├── config.rs     # ClientConfig builder
        │   ├── auth.rs       # API key handling
        │   ├── error.rs      # Error types
        │   ├── middleware.rs  # Request middleware
        │   ├── services/     # One module per API group (20 services)
        │   ├── types/        # Request/response structs (from OpenAPI)
        │   └── ws/           # WebSocket (TTS streaming, Conversational AI)
        ├── examples/         # Runnable examples
        └── tests/            # Integration tests (require Prism mock server)
```

## Key Conventions

### Dependencies

- All versions in root `Cargo.toml` under `[workspace.dependencies]`
- Sub-crates use `workspace = true`; features specified per-crate
- Add deps with `cargo add <crate> --workspace` then `cargo add <crate> -p <crate-name>`

### Preferred Libraries

| Use | Instead of |
|-----|------------|
| `hpx` (with `rustls`) | `reqwest` |
| `thiserror` | `anyhow` |
| `tracing` | `log` |
| `scc` | `dashmap`, `RwLock<HashMap>` |

### Error Handling

- SDK crate uses `thiserror` for typed errors
- Never use `anyhow` in library code
- Never use `unwrap()`/`expect()` — use `?` or proper error types

### Code Style

- English only in comments and docs
- `cargo +nightly fmt --all` for formatting
- Pedantic clippy lints enabled workspace-wide

## Common Tasks

```bash
just sdk-test             # Run SDK unit tests
just sdk-lint             # Run clippy on SDK
just sdk-doc              # Generate SDK docs
just sdk-build-examples   # Build all SDK examples
just sdk-check-coverage   # Check endpoint coverage vs OpenAPI spec
just sdk-test-integration # Integration tests with Prism mock server
just cli-build            # Build the CLI binary
just cli-lint             # Run clippy on CLI
just cli-run              # Run the CLI (pass args after --)
just cli-install           # Install the CLI locally
just lint                 # Full workspace lint
just test                 # Full workspace tests
just format               # Format everything
```

## Testing

- **Unit tests**: `cargo test -p elevenlabs-sdk` — uses `wiremock` for HTTP mocking
- **Integration tests**: Require Prism mock server (`just sdk-test-integration`)
- **Coverage check**: `python3 scripts/check_coverage.py` compares implemented endpoints against `docs/openapi.json`

## Adding a New Service

1. Add the service module in `crates/elevenlabs-sdk/src/services/<name>.rs`
2. Add request/response types in `crates/elevenlabs-sdk/src/types/<name>.rs`
3. Re-export from `services/mod.rs` and `types/mod.rs`
4. Add accessor method on `ElevenLabsClient` in `client.rs`
5. Re-export the service type from `lib.rs`
6. Add CLI command module in `bin/elevenlabs-cli/src/commands/<name>.rs`
7. Register the command in `cli.rs` and `main.rs`
8. Run `just sdk-test && just sdk-lint && just cli-lint`
