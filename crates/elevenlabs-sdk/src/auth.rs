//! API key authentication for the ElevenLabs API.
//!
//! Provides the [`ApiKey`] newtype for securely handling API keys with
//! redacted [`Debug`] output, and the [`API_KEY_HEADER`] constant used
//! for authenticating all API requests.

use std::fmt;

/// HTTP header name used to send the API key to ElevenLabs.
///
/// All 248 endpoints in the ElevenLabs API use this header for authentication.
pub const API_KEY_HEADER: &str = "xi-api-key";

/// A newtype wrapper around an API key string.
///
/// `ApiKey` intentionally redacts its [`Debug`] output to prevent
/// accidental key leakage in logs or error messages. The formatted
/// output is always `ApiKey(****)` regardless of the actual key value.
///
/// # Examples
///
/// ```
/// use elevenlabs_sdk::auth::ApiKey;
///
/// let key = ApiKey::from("sk-test-1234");
/// assert_eq!(key.as_str(), "sk-test-1234");
/// assert_eq!(format!("{key:?}"), "ApiKey(****)");
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct ApiKey(String);

impl ApiKey {
    /// Returns the API key value as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ApiKey(****)")
    }
}

impl From<String> for ApiKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ApiKey {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl AsRef<str> for ApiKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap for concise assertions")]
mod tests {
    use super::*;

    #[test]
    fn debug_output_is_redacted() {
        let key = ApiKey::from("sk-secret-12345");
        let debug = format!("{key:?}");

        assert_eq!(debug, "ApiKey(****)");
        assert!(!debug.contains("sk-secret-12345"));
    }

    #[test]
    fn from_string_conversion() {
        let key = ApiKey::from(String::from("my-api-key"));
        assert_eq!(key.as_str(), "my-api-key");
    }

    #[test]
    fn from_str_conversion() {
        let key = ApiKey::from("my-api-key");
        assert_eq!(key.as_str(), "my-api-key");
    }

    #[test]
    fn as_ref_str_returns_inner_value() {
        let key = ApiKey::from("ref-key");
        let s: &str = key.as_ref();
        assert_eq!(s, "ref-key");
    }

    #[test]
    fn as_str_returns_inner_value() {
        let key = ApiKey::from("str-key");
        assert_eq!(key.as_str(), "str-key");
    }

    #[test]
    fn clone_produces_equal_key() {
        let key = ApiKey::from("clone-key");
        let cloned = key.clone();
        assert_eq!(key, cloned);
        assert_eq!(cloned.as_str(), "clone-key");
    }

    #[test]
    fn equality_works() {
        let a = ApiKey::from("same");
        let b = ApiKey::from("same");
        let c = ApiKey::from("different");

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn api_key_header_constant() {
        assert_eq!(API_KEY_HEADER, "xi-api-key");
    }
}
