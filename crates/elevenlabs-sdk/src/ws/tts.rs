//! TTS WebSocket client for real-time text-to-speech streaming.
//!
//! Connects to the ElevenLabs input-streaming TTS endpoint and allows
//! sending text chunks incrementally while receiving base64-encoded audio
//! responses in real time.
//!
//! # Protocol
//!
//! 1. Open a WebSocket to `wss://api.elevenlabs.io/v1/text-to-speech/{voice_id}/stream-input`
//! 2. Send a **BOS** (beginning-of-stream) message with voice settings and generation config.
//! 3. Send text chunks via [`TtsWebSocket::send_text`].
//! 4. Optionally flush with [`TtsWebSocket::flush`].
//! 5. Receive [`TtsWsResponse`] messages containing base64 audio.
//! 6. Close with [`TtsWebSocket::close`] (sends an EOS message).

use hpx_transport::websocket::{
    Connection, ConnectionHandle, ConnectionStream, Event, WsConfig, WsMessage,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    config::ClientConfig,
    error::{ElevenLabsError, Result},
    types::{OutputFormat, VoiceSettings},
    ws::{build_ws_url, tts_handler::TtsProtocolHandler},
};

/// Configuration for a TTS WebSocket connection.
#[derive(Debug, Clone)]
pub struct TtsWsConfig {
    /// The voice ID to use for synthesis.
    pub voice_id: String,
    /// The model ID (e.g. `"eleven_turbo_v2"`).
    pub model_id: String,
    /// Optional voice settings (stability, similarity, etc.).
    pub voice_settings: Option<VoiceSettings>,
    /// Optional generation configuration.
    pub generation_config: Option<TtsWsGenerationConfig>,
    /// Optional output format override.
    pub output_format: Option<OutputFormat>,
}

/// Generation configuration for TTS WebSocket streaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsWsGenerationConfig {
    /// Schedule of chunk lengths in characters. The server uses progressively
    /// larger chunks as more text is buffered.
    pub chunk_length_schedule: Vec<u32>,
}

impl Default for TtsWsGenerationConfig {
    fn default() -> Self {
        Self { chunk_length_schedule: vec![120, 160, 250, 290] }
    }
}

/// Response from the TTS WebSocket.
///
/// Each message may contain a base64-encoded audio chunk, alignment data,
/// or a final marker.
#[derive(Debug, Clone, Deserialize)]
pub struct TtsWsResponse {
    /// Base64-encoded audio data. `None` on the final acknowledgement.
    pub audio: Option<String>,
    /// Whether this is the final response for the current generation.
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    /// Character-level alignment information.
    pub alignment: Option<TtsWsAlignment>,
    /// Normalised character-level alignment information.
    #[serde(rename = "normalizedAlignment")]
    pub normalized_alignment: Option<TtsWsAlignment>,
}

/// Character-level alignment data returned alongside audio chunks.
#[derive(Debug, Clone, Deserialize)]
pub struct TtsWsAlignment {
    /// The characters that were synthesised.
    pub chars: Option<Vec<String>>,
    /// Start time in milliseconds for each character.
    #[serde(rename = "charStartTimesMs")]
    pub char_start_times_ms: Option<Vec<f64>>,
    /// Duration in milliseconds for each character.
    #[serde(rename = "charDurationsMs")]
    pub char_durations_ms: Option<Vec<f64>>,
}

// -- Internal message types sent to the server --------------------------------

/// BOS (beginning-of-stream) message.
#[derive(Serialize)]
struct BosMessage<'a> {
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice_settings: Option<&'a VoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<&'a TtsWsGenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xi_api_key: Option<&'a str>,
}

/// Text chunk message.
#[derive(Serialize)]
struct TextChunkMessage<'a> {
    text: &'a str,
    try_trigger_generation: bool,
}

/// Flush message.
#[derive(Serialize)]
struct FlushMessage<'a> {
    text: &'a str,
    flush: bool,
}

/// EOS (end-of-stream) message.
#[derive(Serialize)]
struct EosMessage<'a> {
    text: &'a str,
}

/// TTS WebSocket client for real-time text-to-speech streaming.
///
/// Wraps an `hpx_transport` managed connection, providing typed methods for
/// the ElevenLabs input-streaming TTS protocol.
///
/// # Example
///
/// ```no_run
/// use elevenlabs_sdk::{ClientConfig, TtsWebSocket, TtsWsConfig};
///
/// # async fn example() -> elevenlabs_sdk::Result<()> {
/// let config = ClientConfig::builder("your-api-key").build();
/// let ws_config = TtsWsConfig {
///     voice_id: "voice123".into(),
///     model_id: "eleven_turbo_v2".into(),
///     voice_settings: None,
///     generation_config: None,
///     output_format: None,
/// };
///
/// let mut ws = TtsWebSocket::connect(&config, &ws_config).await?;
/// ws.send_text("Hello, world!").await?;
/// ws.flush().await?;
///
/// while let Some(resp) = ws.recv().await? {
///     if resp.is_final == Some(true) {
///         break;
///     }
/// }
///
/// ws.close().await?;
/// # Ok(())
/// # }
/// ```
pub struct TtsWebSocket {
    handle: ConnectionHandle,
    stream: ConnectionStream,
}

impl std::fmt::Debug for TtsWebSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TtsWebSocket").finish_non_exhaustive()
    }
}

impl TtsWebSocket {
    /// Connect to the TTS WebSocket endpoint.
    ///
    /// Establishes the connection and sends the BOS (beginning-of-stream)
    /// message automatically.
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::WebSocket`] if the connection or the BOS
    /// handshake fails.
    pub async fn connect(client_config: &ClientConfig, ws_config: &TtsWsConfig) -> Result<Self> {
        let path = format!("/v1/text-to-speech/{}/stream-input", ws_config.voice_id);

        let mut params: Vec<(&str, String)> = vec![("model_id", ws_config.model_id.clone())];

        if let Some(ref fmt) = ws_config.output_format {
            params.push(("output_format", fmt.to_string()));
        }

        // Build param refs for the URL builder.
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let url = build_ws_url(&client_config.base_url, &path, &param_refs)?;
        debug!(url = %url, "connecting to TTS WebSocket");

        let handler = TtsProtocolHandler;
        let transport_config =
            WsConfig::new(url.to_string()).reconnect_max_attempts(Some(0)).use_websocket_ping(true);

        let (handle, stream) = Connection::connect(transport_config, handler)
            .await
            .map_err(|e| ElevenLabsError::WebSocket(format!("connection failed: {e}")))?;

        // Send BOS message.
        let bos = BosMessage {
            text: " ",
            voice_settings: ws_config.voice_settings.as_ref(),
            generation_config: ws_config.generation_config.as_ref(),
            xi_api_key: Some(client_config.api_key.as_str()),
        };
        let bos_json = serde_json::to_string(&bos)?;
        handle
            .send(WsMessage::text(bos_json))
            .await
            .map_err(|e| ElevenLabsError::WebSocket(format!("BOS send failed: {e}")))?;

        debug!("TTS WebSocket connected and BOS sent");
        Ok(Self { handle, stream })
    }

    /// Send a text chunk for conversion.
    ///
    /// The text is queued on the server side and synthesis is triggered
    /// according to the generation config's chunk schedule.
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::WebSocket`] if the send fails.
    pub async fn send_text(&mut self, text: &str) -> Result<()> {
        let msg = TextChunkMessage { text, try_trigger_generation: true };
        let json = serde_json::to_string(&msg)?;
        self.handle
            .send(WsMessage::text(json))
            .await
            .map_err(|e| ElevenLabsError::WebSocket(format!("send_text failed: {e}")))?;
        Ok(())
    }

    /// Flush the current audio generation buffer.
    ///
    /// Forces the server to synthesise any buffered text immediately.
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::WebSocket`] if the send fails.
    pub async fn flush(&mut self) -> Result<()> {
        let msg = FlushMessage { text: " ", flush: true };
        let json = serde_json::to_string(&msg)?;
        self.handle
            .send(WsMessage::text(json))
            .await
            .map_err(|e| ElevenLabsError::WebSocket(format!("flush failed: {e}")))?;
        Ok(())
    }

    /// Receive the next audio response from the server.
    ///
    /// Returns `Ok(None)` when the connection is closed.
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::WebSocket`] on transport errors or
    /// [`ElevenLabsError::Deserialization`] if the JSON payload is malformed.
    pub async fn recv(&mut self) -> Result<Option<TtsWsResponse>> {
        loop {
            match self.stream.next().await {
                Some(Event::Message(incoming)) => {
                    if let Some(text) = incoming.text {
                        let resp: TtsWsResponse = serde_json::from_str(&text)?;
                        return Ok(Some(resp));
                    }
                    // Binary message without decodable text — keep receiving.
                }
                Some(Event::Connected { .. }) => {
                    // Connection lifecycle event — keep receiving.
                }
                Some(Event::Disconnected { .. }) | None => return Ok(None),
            }
        }
    }

    /// Send EOS (end-of-stream) and close the connection.
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::WebSocket`] if the close handshake fails.
    pub async fn close(self) -> Result<()> {
        // Send EOS message.
        let eos = EosMessage { text: "" };
        let json = serde_json::to_string(&eos)?;
        self.handle
            .send(WsMessage::text(json))
            .await
            .map_err(|e| ElevenLabsError::WebSocket(format!("EOS send failed: {e}")))?;

        // Close the managed connection.
        self.handle
            .close()
            .await
            .map_err(|e| ElevenLabsError::WebSocket(format!("close failed: {e}")))?;

        debug!("TTS WebSocket closed");
        Ok(())
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn deserialize_tts_response_with_audio() {
        let json = r#"{
            "audio": "SGVsbG8gV29ybGQ=",
            "isFinal": false,
            "alignment": {
                "chars": ["H", "e", "l", "l", "o"],
                "charStartTimesMs": [0.0, 50.0, 100.0, 150.0, 200.0],
                "charDurationsMs": [50.0, 50.0, 50.0, 50.0, 50.0]
            }
        }"#;

        let resp: TtsWsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.audio.as_deref(), Some("SGVsbG8gV29ybGQ="));
        assert_eq!(resp.is_final, Some(false));
        let alignment = resp.alignment.unwrap();
        assert_eq!(alignment.chars.as_ref().unwrap().len(), 5);
        assert_eq!(alignment.char_start_times_ms.as_ref().unwrap().len(), 5);
        assert_eq!(alignment.char_durations_ms.as_ref().unwrap().len(), 5);
    }

    #[test]
    fn deserialize_tts_response_final() {
        let json = r#"{"audio": null, "isFinal": true}"#;
        let resp: TtsWsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.audio.is_none());
        assert_eq!(resp.is_final, Some(true));
        assert!(resp.alignment.is_none());
    }

    #[test]
    fn deserialize_tts_response_empty_audio() {
        let json = r#"{"audio": ""}"#;
        let resp: TtsWsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.audio.as_deref(), Some(""));
        assert!(resp.is_final.is_none());
    }

    #[test]
    fn deserialize_alignment() {
        let json = r#"{
            "chars": ["a", "b"],
            "charStartTimesMs": [0.0, 100.0],
            "charDurationsMs": [100.0, 200.0]
        }"#;
        let alignment: TtsWsAlignment = serde_json::from_str(json).unwrap();
        assert_eq!(alignment.chars.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn serialize_bos_message() {
        let bos = BosMessage {
            text: " ",
            voice_settings: Some(&VoiceSettings {
                stability: Some(0.5),
                similarity_boost: Some(0.8),
                style: Some(0.0),
                use_speaker_boost: Some(true),
                speed: None,
            }),
            generation_config: Some(&TtsWsGenerationConfig::default()),
            xi_api_key: Some("sk-test"),
        };
        let json = serde_json::to_string(&bos).unwrap();
        assert!(json.contains("\"text\":\" \""));
        assert!(json.contains("\"stability\":0.5"));
        assert!(json.contains("\"chunk_length_schedule\""));
        assert!(json.contains("\"xi_api_key\":\"sk-test\""));
    }

    #[test]
    fn serialize_text_chunk() {
        let msg = TextChunkMessage { text: "Hello ", try_trigger_generation: true };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"text\":\"Hello \""));
        assert!(json.contains("\"try_trigger_generation\":true"));
    }

    #[test]
    fn serialize_flush_message() {
        let msg = FlushMessage { text: " ", flush: true };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"flush\":true"));
    }

    #[test]
    fn serialize_eos_message() {
        let msg = EosMessage { text: "" };
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, r#"{"text":""}"#);
    }

    #[test]
    fn generation_config_default() {
        let config = TtsWsGenerationConfig::default();
        assert_eq!(config.chunk_length_schedule, vec![120, 160, 250, 290]);
    }

    #[test]
    fn deserialize_tts_response_with_normalized_alignment() {
        let json = r#"{
            "audio": "AAAA",
            "isFinal": false,
            "normalizedAlignment": {
                "chars": ["h", "i"],
                "charStartTimesMs": [0.0, 50.0],
                "charDurationsMs": [50.0, 50.0]
            }
        }"#;
        let resp: TtsWsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.normalized_alignment.is_some());
        assert!(resp.alignment.is_none());
    }
}
