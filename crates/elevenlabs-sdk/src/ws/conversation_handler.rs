//! Protocol handler for Conversational AI WebSocket.
//!
//! Implements the [`ProtocolHandler`] trait for the ElevenLabs Conversational AI
//! protocol. This protocol uses JSON text frames with a `"type"` field for event
//! classification. Application-level ping/pong events are intentionally NOT
//! handled automatically — they are passed through to the caller so the existing
//! public API (where the caller receives [`ConversationEvent::Ping`] and calls
//! [`ConversationWebSocket::send_pong`]) is preserved.

use hpx_transport::websocket::{MessageKind, ProtocolHandler, RequestId, Topic, WsMessage};

/// Protocol handler for the ElevenLabs Conversational AI WebSocket.
///
/// All incoming messages (including application-level pings) are classified as
/// [`MessageKind::Unknown`] and flow through the event stream. This preserves
/// the existing public API where the caller handles pings manually.
pub(crate) struct ConversationProtocolHandler;

impl ProtocolHandler for ConversationProtocolHandler {
    fn classify_message(&self, _message: &str) -> MessageKind {
        // All conversation events (including pings) are routed through the
        // event stream so the caller can handle them directly.
        MessageKind::Unknown
    }

    fn extract_request_id(&self, _message: &str) -> Option<RequestId> {
        None
    }

    fn extract_topic(&self, _message: &str) -> Option<Topic> {
        None
    }

    fn build_subscribe(&self, _topics: &[Topic], _request_id: RequestId) -> WsMessage {
        // Conversation protocol has no subscription mechanism.
        WsMessage::text("{}")
    }

    fn build_unsubscribe(&self, _topics: &[Topic], _request_id: RequestId) -> WsMessage {
        // Conversation protocol has no subscription mechanism.
        WsMessage::text("{}")
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn classify_audio_event() {
        let handler = ConversationProtocolHandler;
        let kind = handler.classify_message(r#"{"type":"audio","audio":{"chunk":"SGVsbG8="}}"#);
        assert_eq!(kind, MessageKind::Unknown);
    }

    #[test]
    fn classify_ping_event() {
        let handler = ConversationProtocolHandler;
        let kind = handler.classify_message(r#"{"type":"ping","ping_event":{"event_id":42}}"#);
        assert_eq!(kind, MessageKind::Unknown);
    }

    #[test]
    fn classify_agent_response() {
        let handler = ConversationProtocolHandler;
        let kind =
            handler.classify_message(r#"{"type":"agent_response","agent_response_text":"Hello!"}"#);
        assert_eq!(kind, MessageKind::Unknown);
    }

    #[test]
    fn extract_request_id_returns_none() {
        let handler = ConversationProtocolHandler;
        assert!(handler.extract_request_id(r#"{"type":"audio"}"#).is_none());
    }

    #[test]
    fn extract_topic_returns_none() {
        let handler = ConversationProtocolHandler;
        assert!(handler.extract_topic(r#"{"type":"audio"}"#).is_none());
    }

    #[test]
    fn is_server_ping_returns_false() {
        let handler = ConversationProtocolHandler;
        // We deliberately return false so pings flow through the event stream.
        assert!(!handler.is_server_ping(r#"{"type":"ping","ping_event":{"event_id":1}}"#));
    }

    #[test]
    fn build_subscribe_returns_empty_json() {
        let handler = ConversationProtocolHandler;
        let msg = handler.build_subscribe(&[], RequestId::new());
        assert!(matches!(msg, WsMessage::Text(s) if s == "{}"));
    }
}
