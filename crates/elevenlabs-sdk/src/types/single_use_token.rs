//! Types for the ElevenLabs Single-Use Token endpoint.
//!
//! Covers `POST /v1/single-use-token/{token_type}` — generate a single-use
//! token that can be embedded in client-side code.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Response
// ---------------------------------------------------------------------------

/// Response from `POST /v1/single-use-token/{token_type}`.
///
/// Contains a short-lived, single-use token string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SingleUseTokenResponse {
    /// The single-use token string.
    pub token: String,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn single_use_token_response_deserialize() {
        let json = r#"{"token": "abc123xyz"}"#;
        let resp: SingleUseTokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.token, "abc123xyz");
    }
}
