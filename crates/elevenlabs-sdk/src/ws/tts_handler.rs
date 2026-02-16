//! Protocol handler for TTS WebSocket streaming.
//!
//! Implements the [`ProtocolHandler`] trait for the ElevenLabs input-streaming
//! TTS protocol. The TTS protocol is a simple fire-and-forget pattern: text
//! messages are sent in, and JSON responses containing base64-encoded audio
//! are received. There is no request-response correlation or subscription
//! mechanism at the protocol level.

use hpx_transport::websocket::{MessageKind, ProtocolHandler, RequestId, Topic, WsMessage};

/// Protocol handler for the ElevenLabs TTS streaming WebSocket.
///
/// Classifies all incoming messages as [`MessageKind::Unknown`] so they flow
/// through the event stream rather than subscriptions or request matching.
/// The TTS protocol has no concept of topics or request IDs.
pub(crate) struct TtsProtocolHandler;

impl ProtocolHandler for TtsProtocolHandler {
    fn classify_message(&self, _message: &str) -> MessageKind {
        // All TTS responses are unsolicited — route them through the event
        // stream so the caller can consume them directly.
        MessageKind::Unknown
    }

    fn extract_request_id(&self, _message: &str) -> Option<RequestId> {
        None
    }

    fn extract_topic(&self, _message: &str) -> Option<Topic> {
        None
    }

    fn build_subscribe(&self, _topics: &[Topic], _request_id: RequestId) -> WsMessage {
        // TTS protocol has no subscription mechanism.
        WsMessage::text("{}")
    }

    fn build_unsubscribe(&self, _topics: &[Topic], _request_id: RequestId) -> WsMessage {
        // TTS protocol has no subscription mechanism.
        WsMessage::text("{}")
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn classify_message_returns_unknown() {
        let handler = TtsProtocolHandler;
        let kind = handler.classify_message(r#"{"audio":"AAAA","isFinal":false}"#);
        assert_eq!(kind, MessageKind::Unknown);
    }

    #[test]
    fn classify_message_final_returns_unknown() {
        let handler = TtsProtocolHandler;
        let kind = handler.classify_message(r#"{"audio":null,"isFinal":true}"#);
        assert_eq!(kind, MessageKind::Unknown);
    }

    #[test]
    fn extract_request_id_returns_none() {
        let handler = TtsProtocolHandler;
        assert!(handler.extract_request_id(r#"{"audio":"AAAA"}"#).is_none());
    }

    #[test]
    fn extract_topic_returns_none() {
        let handler = TtsProtocolHandler;
        assert!(handler.extract_topic(r#"{"audio":"AAAA"}"#).is_none());
    }

    #[test]
    fn is_server_ping_returns_false() {
        let handler = TtsProtocolHandler;
        assert!(!handler.is_server_ping(r#"{"type":"ping"}"#));
    }

    #[test]
    fn build_subscribe_returns_empty_json() {
        let handler = TtsProtocolHandler;
        let msg = handler.build_subscribe(&[], RequestId::new());
        assert!(matches!(msg, WsMessage::Text(s) if s == "{}"));
    }
}
