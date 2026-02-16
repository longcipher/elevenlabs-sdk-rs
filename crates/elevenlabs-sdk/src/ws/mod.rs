//! WebSocket streaming support for the ElevenLabs API.
//!
//! This module provides real-time streaming clients for:
//!
//! - **Text-to-Speech** ([`tts`]) — stream text and receive audio chunks in real time via the
//!   input-streaming TTS endpoint.
//! - **Conversational AI** ([`conversation`]) — bidirectional audio/text communication with an
//!   ElevenLabs conversational agent.
//!
//! Both clients are built on top of [`hpx_transport::websocket`] for managed
//! WebSocket connections with automatic reconnection and protocol handling.

pub mod conversation;
pub(crate) mod conversation_handler;
pub mod tts;
pub(crate) mod tts_handler;

use url::Url;

use crate::error::Result;

/// Builds a WebSocket URL by appending query parameters to a base path.
///
/// # Errors
///
/// Returns [`ElevenLabsError::InvalidUrl`] if the resulting URL cannot be
/// parsed.
pub(crate) fn build_ws_url(base_url: &str, path: &str, params: &[(&str, &str)]) -> Result<Url> {
    // Replace https:// with wss:// (and http:// with ws://)
    let ws_base = base_url.replace("https://", "wss://").replace("http://", "ws://");

    let mut url = Url::parse(&format!("{ws_base}{path}"))?;

    {
        let mut query = url.query_pairs_mut();
        for &(key, value) in params {
            query.append_pair(key, value);
        }
    }

    Ok(url)
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn build_ws_url_basic() {
        let url = build_ws_url(
            "https://api.elevenlabs.io",
            "/v1/text-to-speech/voice123/stream-input",
            &[("model_id", "eleven_turbo_v2"), ("xi_api_key", "sk-test")],
        )
        .unwrap();

        assert_eq!(url.scheme(), "wss");
        assert_eq!(url.host_str(), Some("api.elevenlabs.io"));
        assert_eq!(url.path(), "/v1/text-to-speech/voice123/stream-input");
        assert!(url.query().unwrap().contains("model_id=eleven_turbo_v2"));
        assert!(url.query().unwrap().contains("xi_api_key=sk-test"));
    }

    #[test]
    fn build_ws_url_empty_params() {
        let url = build_ws_url("https://api.elevenlabs.io", "/v1/ws", &[]).unwrap();

        assert_eq!(url.scheme(), "wss");
        assert!(url.query().is_none() || url.query() == Some(""));
    }

    #[test]
    fn build_ws_url_http_to_ws() {
        let url = build_ws_url("http://localhost:8080", "/ws", &[]).unwrap();
        assert_eq!(url.scheme(), "ws");
    }

    #[test]
    fn build_ws_url_special_chars() {
        let url = build_ws_url(
            "https://api.elevenlabs.io",
            "/v1/ws",
            &[("key", "value with spaces & symbols=!")],
        )
        .unwrap();

        // URL-encodes special characters
        let query = url.query().unwrap();
        assert!(query.contains("key="));
    }
}
