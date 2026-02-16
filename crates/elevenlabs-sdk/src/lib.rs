//! # ElevenLabs Rust SDK
//!
//! A comprehensive, async Rust SDK for the [ElevenLabs](https://elevenlabs.io) API,
//! providing text-to-speech, voice management, audio isolation, speech-to-text,
//! and other audio AI capabilities.
//!
//! ## Quick Start
//!
//! ```no_run
//! use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, types::TextToSpeechRequest};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create client from environment variable ELEVENLABS_API_KEY
//! let config = ClientConfig::from_env()?;
//! let client = ElevenLabsClient::new(config)?;
//!
//! // Text-to-speech
//! let request = TextToSpeechRequest::new("Hello from Rust!");
//! let audio_bytes = client.text_to_speech().convert("voice_id", &request, None, None).await?;
//! println!("Received {} bytes of audio", audio_bytes.len());
//!
//! // List available voices
//! let voices = client.voices().list(None).await?;
//! for voice in &voices.voices {
//!     println!("  {} ({})", voice.name, voice.voice_id);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - **Text-to-Speech** — Convert text to natural-sounding speech with full and streaming
//!   endpoints, including character-level timestamp alignment.
//! - **Voice Management** — List, create, edit, and delete voices; manage voice settings and
//!   samples.
//! - **WebSocket Streaming** — Real-time TTS via input-streaming WebSocket, and conversational AI
//!   with bidirectional audio/text.
//! - **Audio Isolation** — Remove background noise from audio files.
//! - **Speech-to-Text** — Transcribe audio to text.
//! - **Speech-to-Speech** — Convert speech with a different voice.
//! - **Sound Generation** — Generate sound effects from text prompts.
//! - **Models** — Query available synthesis models.
//! - **User & Workspace** — Account and workspace management.
//! - **Retry & Error Handling** — Automatic retry with exponential backoff, structured error types
//!   with status codes and rate-limit info.
//!
//! ## Module Organization
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`auth`] | API key authentication and secure key handling |
//! | [`config`] | Client configuration builder with env-var support |
//! | [`error`] | Error types ([`ElevenLabsError`]) and `Result` alias |
//! | [`client`] | HTTP client ([`ElevenLabsClient`]) with automatic auth |
//! | [`types`] | Shared request/response types mirroring the OpenAPI spec |
//! | [`services`] | Typed endpoint wrappers (TTS, voices, models, etc.) |
//! | [`ws`] | WebSocket streaming (TTS input-streaming, conversational AI) |

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
mod middleware;
pub mod services;
pub mod types;
pub mod ws;

pub use auth::ApiKey;
pub use client::ElevenLabsClient;
pub use config::{ClientConfig, ClientConfigBuilder, ConfigError};
pub use error::{ElevenLabsError, Result};
pub use services::{
    AgentsService, AudioIsolationService, AudioNativeService, ForcedAlignmentService,
    HistoryService, ModelsService, MusicService, PvcVoicesService, SingleUseTokenService,
    SoundGenerationService, SpeechToSpeechService, SpeechToTextService, StudioService,
    TextToDialogueService, TextToSpeechService, TextToVoiceService, UserService,
    VoiceGenerationService, VoicesService, WorkspaceService,
};
pub use ws::{
    conversation::{ConversationEvent, ConversationWebSocket},
    tts::{TtsWebSocket, TtsWsConfig, TtsWsResponse},
};
