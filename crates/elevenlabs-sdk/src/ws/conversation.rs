//! Conversational AI WebSocket client.
//!
//! Provides bidirectional real-time communication with an ElevenLabs
//! conversational agent. Audio frames are sent as binary messages while
//! control/event messages use JSON text frames.
//!
//! # Protocol
//!
//! 1. Obtain a signed URL via
//!    [`AgentsService::get_conversation_signed_url`](crate::services::AgentsService::get_conversation_signed_url).
//! 2. Connect to the signed URL with [`ConversationWebSocket::connect`].
//! 3. Send audio via [`ConversationWebSocket::send_audio`].
//! 4. Receive events via [`ConversationWebSocket::recv`].
//! 5. Respond to [`ConversationEvent::Ping`] with [`ConversationWebSocket::send_pong`] to keep the
//!    connection alive.

use base64::Engine;
use hpx_transport::websocket::{
    Connection, ConnectionHandle, ConnectionStream, Event, WsConfig, WsMessage,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    client::ElevenLabsClient,
    error::{ElevenLabsError, Result},
    ws::conversation_handler::ConversationProtocolHandler,
};

/// Events received from the Conversational AI WebSocket.
///
/// Each variant corresponds to a server-sent event type identified by the
/// `"type"` field in the JSON payload.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum ConversationEvent {
    /// Initial metadata sent when the conversation begins.
    #[serde(rename = "conversation_initiation_metadata")]
    InitiationMetadata {
        /// Raw metadata payload.
        #[serde(flatten)]
        metadata: serde_json::Value,
    },

    /// An audio chunk from the agent (base64-encoded).
    #[serde(rename = "audio")]
    Audio {
        /// Base64-encoded audio data.
        audio: AudioEvent,
    },

    /// A text response from the agent.
    #[serde(rename = "agent_response")]
    AgentResponse {
        /// The agent's response text.
        agent_response_text: String,
    },

    /// A transcript of the user's speech.
    #[serde(rename = "user_transcript")]
    UserTranscript {
        /// The transcribed user text.
        user_transcript_text: String,
    },

    /// The agent was interrupted by the user.
    #[serde(rename = "interruption")]
    Interruption {
        /// Raw interruption payload.
        #[serde(flatten)]
        data: serde_json::Value,
    },

    /// A keep-alive ping from the server. Respond with [`ConversationWebSocket::send_pong`].
    #[serde(rename = "ping")]
    Ping {
        /// The ping event payload containing an event ID.
        ping_event: PingEvent,
    },

    /// Pong acknowledgement from the server.
    #[serde(rename = "pong")]
    Pong {
        /// Raw pong payload.
        #[serde(flatten)]
        data: serde_json::Value,
    },

    /// An event type not yet modelled by this SDK.
    #[serde(other)]
    Unknown,
}

/// Payload of an audio event from the server.
#[derive(Debug, Clone, Deserialize)]
pub struct AudioEvent {
    /// Base64-encoded audio chunk.
    pub chunk: Option<String>,
}

/// Payload of a ping event from the server.
#[derive(Debug, Clone, Deserialize)]
pub struct PingEvent {
    /// The event ID to echo back in a pong response.
    pub event_id: i64,
}

// -- Client messages ----------------------------------------------------------

/// Messages sent from the client to the server.
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum ClientMessage {
    /// An audio chunk from the user's microphone.
    #[serde(rename = "user_audio_chunk")]
    UserAudioChunk {
        /// Base64-encoded audio data.
        user_audio_chunk: String,
    },

    /// Pong response to a server ping.
    #[serde(rename = "pong")]
    Pong {
        /// The event ID from the original ping.
        event_id: i64,
    },
}

/// Conversational AI WebSocket client for real-time agent interaction.
///
/// Supports sending audio frames and receiving typed conversation events
/// (transcripts, agent responses, audio, pings, etc.).
///
/// # Example
///
/// ```no_run
/// use elevenlabs_sdk::{ClientConfig, ConversationWebSocket, ElevenLabsClient};
///
/// # async fn example() -> elevenlabs_sdk::Result<()> {
/// let config = ClientConfig::builder("your-api-key").build();
/// let client = ElevenLabsClient::new(config)?;
///
/// let mut conv = ConversationWebSocket::connect_with_agent(&client, "agent-id").await?;
///
/// while let Some(event) = conv.recv().await? {
///     match event {
///         elevenlabs_sdk::ConversationEvent::AgentResponse { agent_response_text } => {
///             println!("Agent: {agent_response_text}");
///         }
///         elevenlabs_sdk::ConversationEvent::Ping { ping_event } => {
///             conv.send_pong(ping_event.event_id).await?;
///         }
///         _ => {}
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub struct ConversationWebSocket {
    handle: ConnectionHandle,
    stream: ConnectionStream,
}

impl std::fmt::Debug for ConversationWebSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConversationWebSocket").finish_non_exhaustive()
    }
}

impl ConversationWebSocket {
    /// Connect using a pre-obtained signed URL.
    ///
    /// The signed URL is typically retrieved via
    /// [`AgentsService::get_conversation_signed_url`](crate::services::AgentsService::get_conversation_signed_url).
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::WebSocket`] if the connection or upgrade
    /// fails.
    pub async fn connect(signed_url: &str) -> Result<Self> {
        debug!(url = %signed_url, "connecting to Conversational AI WebSocket");

        let handler = ConversationProtocolHandler;
        let transport_config =
            WsConfig::new(signed_url).reconnect_max_attempts(Some(0)).use_websocket_ping(true);

        let (handle, stream) = Connection::connect(transport_config, handler)
            .await
            .map_err(|e| ElevenLabsError::WebSocket(format!("connection failed: {e}")))?;

        debug!("Conversational AI WebSocket connected");
        Ok(Self { handle, stream })
    }

    /// Connect by agent ID.
    ///
    /// Automatically fetches a signed URL via the Agents service and connects.
    ///
    /// # Errors
    ///
    /// Returns an error if the signed-URL request or the WebSocket connection
    /// fails.
    pub async fn connect_with_agent(client: &ElevenLabsClient, agent_id: &str) -> Result<Self> {
        debug!(agent_id, "fetching signed URL for conversation");
        let resp = client.agents().get_conversation_signed_url(agent_id).await?;
        Self::connect(&resp.signed_url).await
    }

    /// Send an audio chunk (raw PCM bytes) to the agent.
    ///
    /// The audio is base64-encoded before sending.
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::WebSocket`] if the send fails.
    pub async fn send_audio(&mut self, audio: &[u8]) -> Result<()> {
        let encoded = base64::engine::general_purpose::STANDARD.encode(audio);
        let msg = ClientMessage::UserAudioChunk { user_audio_chunk: encoded };
        let json = serde_json::to_string(&msg)?;
        self.handle
            .send(WsMessage::text(json))
            .await
            .map_err(|e| ElevenLabsError::WebSocket(format!("send_audio failed: {e}")))?;
        Ok(())
    }

    /// Receive the next conversation event from the server.
    ///
    /// Returns `Ok(None)` when the connection is closed.
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::WebSocket`] on transport errors or
    /// [`ElevenLabsError::Deserialization`] if the JSON payload is malformed.
    pub async fn recv(&mut self) -> Result<Option<ConversationEvent>> {
        loop {
            match self.stream.next().await {
                Some(Event::Message(incoming)) => {
                    if let Some(text) = incoming.text {
                        let event: ConversationEvent = serde_json::from_str(&text)?;
                        return Ok(Some(event));
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

    /// Send a pong response to keep the connection alive.
    ///
    /// Should be called whenever a [`ConversationEvent::Ping`] is received.
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::WebSocket`] if the send fails.
    pub async fn send_pong(&mut self, event_id: i64) -> Result<()> {
        let msg = ClientMessage::Pong { event_id };
        let json = serde_json::to_string(&msg)?;
        self.handle
            .send(WsMessage::text(json))
            .await
            .map_err(|e| ElevenLabsError::WebSocket(format!("send_pong failed: {e}")))?;
        Ok(())
    }

    /// Close the conversation.
    ///
    /// # Errors
    ///
    /// Returns [`ElevenLabsError::WebSocket`] if the close handshake fails.
    pub async fn close(self) -> Result<()> {
        self.handle
            .close()
            .await
            .map_err(|e| ElevenLabsError::WebSocket(format!("close failed: {e}")))?;
        debug!("Conversational AI WebSocket closed");
        Ok(())
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn deserialize_initiation_metadata() {
        let json = r#"{
            "type": "conversation_initiation_metadata",
            "conversation_id": "conv-123",
            "agent_output_audio_format": "pcm_16000"
        }"#;
        let event: ConversationEvent = serde_json::from_str(json).unwrap();
        assert!(matches!(event, ConversationEvent::InitiationMetadata { .. }));
    }

    #[test]
    fn deserialize_audio_event() {
        let json = r#"{
            "type": "audio",
            "audio": {"chunk": "SGVsbG8="}
        }"#;
        let event: ConversationEvent = serde_json::from_str(json).unwrap();
        match event {
            ConversationEvent::Audio { audio } => {
                assert_eq!(audio.chunk.as_deref(), Some("SGVsbG8="));
            }
            _ => panic!("expected Audio event"),
        }
    }

    #[test]
    fn deserialize_agent_response() {
        let json = r#"{
            "type": "agent_response",
            "agent_response_text": "Hello! How can I help?"
        }"#;
        let event: ConversationEvent = serde_json::from_str(json).unwrap();
        match event {
            ConversationEvent::AgentResponse { agent_response_text } => {
                assert_eq!(agent_response_text, "Hello! How can I help?");
            }
            _ => panic!("expected AgentResponse event"),
        }
    }

    #[test]
    fn deserialize_user_transcript() {
        let json = r#"{
            "type": "user_transcript",
            "user_transcript_text": "Hi there"
        }"#;
        let event: ConversationEvent = serde_json::from_str(json).unwrap();
        match event {
            ConversationEvent::UserTranscript { user_transcript_text } => {
                assert_eq!(user_transcript_text, "Hi there");
            }
            _ => panic!("expected UserTranscript event"),
        }
    }

    #[test]
    fn deserialize_interruption() {
        let json = r#"{"type": "interruption"}"#;
        let event: ConversationEvent = serde_json::from_str(json).unwrap();
        assert!(matches!(event, ConversationEvent::Interruption { .. }));
    }

    #[test]
    fn deserialize_ping() {
        let json = r#"{
            "type": "ping",
            "ping_event": {"event_id": 42}
        }"#;
        let event: ConversationEvent = serde_json::from_str(json).unwrap();
        match event {
            ConversationEvent::Ping { ping_event } => {
                assert_eq!(ping_event.event_id, 42);
            }
            _ => panic!("expected Ping event"),
        }
    }

    #[test]
    fn deserialize_pong() {
        let json = r#"{"type": "pong"}"#;
        let event: ConversationEvent = serde_json::from_str(json).unwrap();
        assert!(matches!(event, ConversationEvent::Pong { .. }));
    }

    #[test]
    fn deserialize_unknown_event() {
        let json = r#"{"type": "some_future_event", "data": 123}"#;
        let event: ConversationEvent = serde_json::from_str(json).unwrap();
        assert!(matches!(event, ConversationEvent::Unknown));
    }

    #[test]
    fn serialize_user_audio_chunk() {
        let msg = ClientMessage::UserAudioChunk { user_audio_chunk: "AAAA".to_owned() };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"user_audio_chunk\""));
        assert!(json.contains("\"user_audio_chunk\":\"AAAA\""));
    }

    #[test]
    fn serialize_pong() {
        let msg = ClientMessage::Pong { event_id: 42 };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"pong\""));
        assert!(json.contains("\"event_id\":42"));
    }
}
