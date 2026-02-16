//! Error types for the ElevenLabs SDK.
//!
//! Provides [`ElevenLabsError`] as the primary error enum for all SDK
//! operations, along with a convenient [`Result`] type alias.

/// A convenient `Result` type alias that defaults to [`ElevenLabsError`].
pub type Result<T> = std::result::Result<T, ElevenLabsError>;

/// All possible errors returned by the ElevenLabs SDK.
///
/// Each variant carries enough context to produce a meaningful
/// [`Display`](std::fmt::Display) message and, where applicable, structured
/// data that callers can use for programmatic error handling (e.g. retry-after
/// headers, HTTP status codes).
#[derive(Debug, thiserror::Error)]
pub enum ElevenLabsError {
    /// The API returned an error response.
    #[error("API error (HTTP {status}): {message}")]
    Api {
        /// HTTP status code from the API.
        status: u16,
        /// Human-readable error message from the API.
        message: String,
        /// Optional raw response body for further inspection.
        body: Option<String>,
    },

    /// Authentication failed (invalid or missing API key).
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// The request was rate-limited by the API.
    #[error("Rate limited (retry after {retry_after:?}s)")]
    RateLimited {
        /// Optional number of seconds to wait before retrying.
        retry_after: Option<u64>,
    },

    /// The request timed out before a response was received.
    #[error("Request timeout")]
    Timeout,

    /// An error occurred at the HTTP transport layer.
    #[error("Transport error: {0}")]
    Transport(#[from] hpx::Error),

    /// Failed to deserialize a JSON response body.
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),

    /// A caller-provided input failed validation.
    #[error("Invalid input: {0}")]
    Validation(String),

    /// A URL could not be parsed.
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// WebSocket communication error.
    #[error("WebSocket error: {0}")]
    WebSocket(String),
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    /// Compile-time proof that `ElevenLabsError` is `Send + Sync + 'static`.
    const fn assert_send_sync<T: Send + Sync + 'static>() {}
    const _: () = assert_send_sync::<ElevenLabsError>();

    #[test]
    fn display_api_error() {
        let err = ElevenLabsError::Api {
            status: 422,
            message: "invalid voice id".to_owned(),
            body: Some("{\"detail\":\"not found\"}".to_owned()),
        };
        assert_eq!(err.to_string(), "API error (HTTP 422): invalid voice id");
    }

    #[test]
    fn display_auth_error() {
        let err = ElevenLabsError::Auth("invalid api key".to_owned());
        assert_eq!(err.to_string(), "Authentication failed: invalid api key");
    }

    #[test]
    fn display_rate_limited_with_retry() {
        let err = ElevenLabsError::RateLimited { retry_after: Some(30) };
        assert_eq!(err.to_string(), "Rate limited (retry after Some(30)s)");
    }

    #[test]
    fn display_rate_limited_without_retry() {
        let err = ElevenLabsError::RateLimited { retry_after: None };
        assert_eq!(err.to_string(), "Rate limited (retry after Nones)");
    }

    #[test]
    fn display_timeout() {
        let err = ElevenLabsError::Timeout;
        assert_eq!(err.to_string(), "Request timeout");
    }

    #[test]
    fn display_validation_error() {
        let err = ElevenLabsError::Validation("text is empty".to_owned());
        assert_eq!(err.to_string(), "Invalid input: text is empty");
    }

    #[test]
    fn display_invalid_url() {
        let err: ElevenLabsError = url::Url::parse("://bad").unwrap_err().into();
        assert!(err.to_string().starts_with("Invalid URL:"));
    }

    #[test]
    fn from_serde_json_error() {
        let json_err: serde_json::Error =
            serde_json::from_str::<serde_json::Value>("not json").unwrap_err();
        let err: ElevenLabsError = json_err.into();
        assert!(err.to_string().starts_with("Deserialization error:"));
    }

    #[test]
    fn from_url_parse_error() {
        let url_err: url::ParseError = url::Url::parse("://bad").unwrap_err();
        let err = ElevenLabsError::from(url_err);
        assert!(matches!(err, ElevenLabsError::InvalidUrl(_)));
    }

    #[test]
    fn display_websocket_error() {
        let err = ElevenLabsError::WebSocket("connection refused".to_owned());
        assert_eq!(err.to_string(), "WebSocket error: connection refused");
    }
}
