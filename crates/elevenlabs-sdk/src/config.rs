//! Client configuration and builder for the ElevenLabs SDK.
//!
//! Provides [`ClientConfig`] with a builder pattern for configuring API
//! connections, including base URL, API key, timeout, and retry settings.

use std::time::Duration;

use crate::auth::ApiKey;

/// Default base URL for the ElevenLabs API.
pub const DEFAULT_BASE_URL: &str = "https://api.elevenlabs.io";

/// Default request timeout duration.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Default maximum number of retry attempts.
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Default retry backoff duration.
pub const DEFAULT_RETRY_BACKOFF: Duration = Duration::from_secs(1);

/// Environment variable name for the ElevenLabs API key.
pub const ENV_API_KEY: &str = "ELEVENLABS_API_KEY";

/// Environment variable name for the ElevenLabs base URL.
pub const ENV_BASE_URL: &str = "ELEVENLABS_BASE_URL";

/// Errors that can occur when building a [`ClientConfig`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConfigError {
    /// A required environment variable is missing.
    #[error("missing required environment variable: {0}")]
    MissingEnvVar(String),
}

/// Configuration for the ElevenLabs API client.
///
/// Created via [`ClientConfig::builder`] or [`ClientConfig::from_env`].
///
/// # Examples
///
/// ```
/// use elevenlabs_sdk::config::ClientConfig;
///
/// let config = ClientConfig::builder("your-api-key").build();
/// assert_eq!(config.base_url, "https://api.elevenlabs.io");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientConfig {
    /// Base URL for the ElevenLabs API.
    pub base_url: String,
    /// API key for authentication.
    pub api_key: ApiKey,
    /// Request timeout duration.
    pub timeout: Duration,
    /// Maximum number of retry attempts for failed requests.
    pub max_retries: u32,
    /// Duration to wait between retry attempts.
    pub retry_backoff: Duration,
}

impl ClientConfig {
    /// Creates a new [`ClientConfigBuilder`] with the given API key.
    ///
    /// The API key is required; all other fields use sensible defaults.
    pub fn builder(api_key: impl Into<ApiKey>) -> ClientConfigBuilder {
        ClientConfigBuilder::new(api_key)
    }

    /// Creates a [`ClientConfig`] from environment variables.
    ///
    /// Reads `ELEVENLABS_API_KEY` (required) and `ELEVENLABS_BASE_URL` (optional)
    /// from the process environment. All other fields use their defaults.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::MissingEnvVar`] if `ELEVENLABS_API_KEY` is not set.
    pub fn from_env() -> Result<Self, ConfigError> {
        let api_key = std::env::var(ENV_API_KEY)
            .map_err(|_| ConfigError::MissingEnvVar(ENV_API_KEY.to_owned()))?;

        let mut builder = Self::builder(api_key);

        if let Ok(base_url) = std::env::var(ENV_BASE_URL) {
            builder = builder.base_url(base_url);
        }

        Ok(builder.build())
    }
}

/// Builder for constructing a [`ClientConfig`].
///
/// Created via [`ClientConfig::builder`]. Use chained setter methods to
/// customize configuration, then call [`build`](ClientConfigBuilder::build)
/// to produce the final [`ClientConfig`].
#[derive(Debug, Clone)]
pub struct ClientConfigBuilder {
    api_key: ApiKey,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    retry_backoff: Option<Duration>,
}

impl ClientConfigBuilder {
    /// Creates a new builder with the given API key.
    pub fn new(api_key: impl Into<ApiKey>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: None,
            timeout: None,
            max_retries: None,
            retry_backoff: None,
        }
    }

    /// Sets the base URL for the API.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Sets the request timeout duration.
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets the maximum number of retry attempts.
    pub const fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Sets the duration to wait between retry attempts.
    pub const fn retry_backoff(mut self, backoff: Duration) -> Self {
        self.retry_backoff = Some(backoff);
        self
    }

    /// Builds the [`ClientConfig`], applying defaults for any unset fields.
    ///
    /// Default values:
    /// - `base_url`: `"https://api.elevenlabs.io"`
    /// - `timeout`: 30 seconds
    /// - `max_retries`: 3
    /// - `retry_backoff`: 1 second
    pub fn build(self) -> ClientConfig {
        ClientConfig {
            base_url: self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_owned()),
            api_key: self.api_key,
            timeout: self.timeout.unwrap_or(DEFAULT_TIMEOUT),
            max_retries: self.max_retries.unwrap_or(DEFAULT_MAX_RETRIES),
            retry_backoff: self.retry_backoff.unwrap_or(DEFAULT_RETRY_BACKOFF),
        }
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap for concise assertions")]
mod tests {
    use super::*;

    /// RAII guard that restores an environment variable to its previous value on drop.
    struct EnvGuard {
        key: &'static str,
        prev: Option<String>,
    }

    impl EnvGuard {
        /// Sets an environment variable and returns a guard that restores the
        /// previous value when dropped.
        fn set(key: &'static str, value: &str) -> Self {
            let prev = std::env::var(key).ok();
            // SAFETY: These tests do not run concurrently with other threads
            // that read or write these specific environment variables.
            unsafe { std::env::set_var(key, value) };
            Self { key, prev }
        }

        /// Removes an environment variable and returns a guard that restores
        /// the previous value when dropped.
        fn remove(key: &'static str) -> Self {
            let prev = std::env::var(key).ok();
            // SAFETY: See `set` above.
            unsafe { std::env::remove_var(key) };
            Self { key, prev }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.prev {
                // SAFETY: Only called during test teardown; same thread-safety
                // reasoning as `set`/`remove`.
                Some(val) => unsafe { std::env::set_var(self.key, val) },
                None => unsafe { std::env::remove_var(self.key) },
            }
        }
    }

    #[test]
    fn builder_creates_config_with_defaults() {
        let config = ClientConfig::builder("test-api-key").build();

        assert_eq!(config.api_key.as_str(), "test-api-key");
        assert_eq!(config.base_url, DEFAULT_BASE_URL);
        assert_eq!(config.timeout, DEFAULT_TIMEOUT);
        assert_eq!(config.max_retries, DEFAULT_MAX_RETRIES);
        assert_eq!(config.retry_backoff, DEFAULT_RETRY_BACKOFF);
    }

    #[test]
    fn builder_with_all_custom_values() {
        let config = ClientConfig::builder("custom-key")
            .base_url("https://custom.api.com")
            .timeout(Duration::from_secs(60))
            .max_retries(5)
            .retry_backoff(Duration::from_secs(2))
            .build();

        assert_eq!(config.api_key.as_str(), "custom-key");
        assert_eq!(config.base_url, "https://custom.api.com");
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.retry_backoff, Duration::from_secs(2));
    }

    #[test]
    fn builder_with_partial_custom_values() {
        let config = ClientConfig::builder("partial-key").timeout(Duration::from_secs(10)).build();

        assert_eq!(config.api_key.as_str(), "partial-key");
        assert_eq!(config.base_url, DEFAULT_BASE_URL);
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert_eq!(config.max_retries, DEFAULT_MAX_RETRIES);
        assert_eq!(config.retry_backoff, DEFAULT_RETRY_BACKOFF);
    }

    #[test]
    fn from_env_reads_api_key() {
        let _key_guard = EnvGuard::set(ENV_API_KEY, "env-api-key");
        let _url_guard = EnvGuard::remove(ENV_BASE_URL);

        let config = ClientConfig::from_env().unwrap();

        assert_eq!(config.api_key.as_str(), "env-api-key");
        assert_eq!(config.base_url, DEFAULT_BASE_URL);
    }

    #[test]
    fn from_env_reads_base_url() {
        let _key_guard = EnvGuard::set(ENV_API_KEY, "env-api-key");
        let _url_guard = EnvGuard::set(ENV_BASE_URL, "https://custom.env.api.com");

        let config = ClientConfig::from_env().unwrap();

        assert_eq!(config.api_key.as_str(), "env-api-key");
        assert_eq!(config.base_url, "https://custom.env.api.com");
    }

    #[test]
    fn from_env_missing_api_key_returns_error() {
        let _key_guard = EnvGuard::remove(ENV_API_KEY);

        let result = ClientConfig::from_env();

        assert_eq!(result.unwrap_err(), ConfigError::MissingEnvVar(ENV_API_KEY.to_owned()),);
    }

    #[test]
    fn config_is_clone_and_debug() {
        let config = ClientConfig::builder("secret-value").build();
        let cloned = config.clone();
        assert_eq!(config, cloned);

        let debug_str = format!("{config:?}");
        assert!(debug_str.contains("ApiKey(****)"));
        assert!(!debug_str.contains("secret-value"));
    }
}
