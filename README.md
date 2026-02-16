# ElevenLabs Rust SDK

Unofficial Rust SDK for the [ElevenLabs](https://elevenlabs.io) API. Provides typed access to all REST endpoints and WebSocket streaming.

## Features

- **Full API Coverage**: 220+ endpoints across Text-to-Speech, Speech-to-Text, Voices, Dubbing, Studio, Conversational AI, and more
- **WebSocket Streaming**: Real-time TTS input-streaming and Conversational AI via WebSocket
- **Type-Safe**: Strongly typed request/response types generated from the official OpenAPI spec
- **Async/Await**: Built on `tokio` and `hpx` for async HTTP
- **Automatic Retries**: Configurable retry with exponential backoff
- **Error Handling**: Typed errors with status codes and rate-limit info

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs-sdk = { path = "crates/elevenlabs-sdk" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust,no_run
use elevenlabs_sdk::{ClientConfig, ElevenLabsClient};
use elevenlabs_sdk::types::TextToSpeechRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from the ELEVENLABS_API_KEY environment variable
    let config = ClientConfig::from_env()?;
    let client = ElevenLabsClient::new(config)?;

    // List voices
    let voices = client.voices().list(None).await?;
    for voice in &voices.voices {
        println!("{}: {}", voice.voice_id, voice.name);
    }

    // Text-to-speech
    let request = TextToSpeechRequest::new("Hello, world!");
    let audio = client
        .text_to_speech()
        .convert("21m00Tcm4TlvDq8ikWAM", &request, None, None)
        .await?;
    std::fs::write("output.mp3", &audio)?;

    Ok(())
}
```

## Available Services

| Service | Method | Description |
|---------|--------|-------------|
| Text-to-Speech | `text_to_speech()` | Convert text to speech (full & streaming) |
| Voices | `voices()` | Voice management, library, and settings |
| Speech-to-Speech | `speech_to_speech()` | Voice conversion |
| Speech-to-Text | `speech_to_text()` | Audio transcription |
| Audio Isolation | `audio_isolation()` | Background noise removal |
| Audio Native | `audio_native()` | Audio Native project management |
| Sound Generation | `sound_generation()` | Generate sound effects from text |
| Text-to-Dialogue | `text_to_dialogue()` | Dialogue generation |
| Text-to-Voice | `text_to_voice()` | Voice design from text prompts |
| Voice Generation | `voice_generation()` | Generate random voices |
| Dubbing | `dubbing()` | Video/audio dubbing |
| Studio | `studio()` | Studio project management |
| Music | `music()` | Music generation |
| Models | `models()` | List available models |
| History | `history()` | Speech generation history |
| User | `user()` | User info and subscription |
| Workspace | `workspace()` | Workspace management |
| Forced Alignment | `forced_alignment()` | Audio-text alignment |
| Single-Use Token | `single_use_token()` | Generate ephemeral tokens |
| Agents | `agents()` | Conversational AI agents |

## WebSocket Streaming

```rust,no_run
use elevenlabs_sdk::{ClientConfig, TtsWebSocket, TtsWsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::from_env()?;

    let ws_config = TtsWsConfig {
        voice_id: "21m00Tcm4TlvDq8ikWAM".into(),
        model_id: "eleven_turbo_v2".into(),
        voice_settings: None,
        generation_config: None,
        output_format: None,
    };

    let mut ws = TtsWebSocket::connect(&config, &ws_config).await?;

    ws.send_text("Hello from real-time streaming!").await?;
    ws.flush().await?;

    while let Some(resp) = ws.recv().await? {
        if let Some(ref audio) = resp.audio {
            println!("Received audio chunk: {} chars base64", audio.len());
        }
        if resp.is_final == Some(true) {
            break;
        }
    }

    ws.close().await?;
    Ok(())
}
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ELEVENLABS_API_KEY` | API key (required) | — |
| `ELEVENLABS_BASE_URL` | Custom base URL | `https://api.elevenlabs.io` |

### Builder Pattern

```rust,no_run
use elevenlabs_sdk::{ClientConfig, ClientConfigBuilder, ElevenLabsClient};

let config = ClientConfigBuilder::default()
    .api_key("your-api-key")
    .base_url("https://api.elevenlabs.io")
    .timeout(std::time::Duration::from_secs(60))
    .max_retries(5)
    .build()?;

let client = ElevenLabsClient::new(config)?;
```

## Examples

Run the bundled examples with your API key:

```bash
ELEVENLABS_API_KEY=your-key cargo run -p elevenlabs-sdk --example text_to_speech
ELEVENLABS_API_KEY=your-key cargo run -p elevenlabs-sdk --example voices
ELEVENLABS_API_KEY=your-key cargo run -p elevenlabs-sdk --example streaming
ELEVENLABS_API_KEY=your-key cargo run -p elevenlabs-sdk --example websocket_tts
```

## Development

```bash
just sdk-test             # Run SDK unit tests
just sdk-lint             # Run clippy
just sdk-doc              # Generate docs
just sdk-build-examples   # Build all examples
just sdk-check-coverage   # Check endpoint coverage vs OpenAPI spec
just sdk-test-integration # Run integration tests with Prism mock server
```

## Project Structure

```text
crates/elevenlabs-sdk/
├── src/
│   ├── lib.rs          # Public API re-exports
│   ├── client.rs       # ElevenLabsClient with HTTP + retry logic
│   ├── config.rs       # ClientConfig builder
│   ├── auth.rs         # API key handling
│   ├── error.rs        # Error types
│   ├── middleware.rs    # Request middleware
│   ├── services/       # Typed endpoint wrappers (one module per API group)
│   ├── types/          # Request/response structs from OpenAPI spec
│   └── ws/             # WebSocket streaming (TTS, Conversational AI)
├── examples/           # Runnable examples
└── tests/              # Integration tests
```

## License

Apache-2.0
