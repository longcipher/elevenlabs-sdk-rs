//! Retry middleware utilities for the ElevenLabs SDK.
//!
//! Provides helpers for determining whether a failed HTTP request should be
//! retried and computing the appropriate delay between attempts.

use std::time::Duration;

use hpx::StatusCode;

/// Maximum delay cap for retry backoff (30 seconds).
const MAX_RETRY_DELAY: Duration = Duration::from_secs(30);

/// Returns `true` if the given HTTP status code is transient and the request
/// should be retried.
///
/// Retryable status codes:
/// - **429** Too Many Requests (rate limited)
/// - **500** Internal Server Error
/// - **502** Bad Gateway
/// - **503** Service Unavailable
pub(crate) const fn should_retry(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS |
            StatusCode::INTERNAL_SERVER_ERROR |
            StatusCode::BAD_GATEWAY |
            StatusCode::SERVICE_UNAVAILABLE
    )
}

/// Parses the `Retry-After` header from an HTTP response as an integer number
/// of seconds.
///
/// Returns `None` if the header is absent, not valid UTF-8, or not a valid
/// integer.
pub(crate) fn parse_retry_after(response: &hpx::Response) -> Option<u64> {
    response
        .headers()
        .get(hpx::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
}

/// Computes the delay before the next retry attempt.
///
/// Uses exponential backoff: `base_backoff * 2^attempt`, capped at 30 seconds.
/// If `retry_after` is provided (from a `Retry-After` header), the delay is
/// the **maximum** of the computed backoff and the server-requested wait time.
pub(crate) fn compute_delay(
    attempt: u32,
    base_backoff: Duration,
    retry_after: Option<u64>,
) -> Duration {
    let exponential = base_backoff.saturating_mul(2u32.saturating_pow(attempt));
    let delay = match retry_after {
        Some(secs) => exponential.max(Duration::from_secs(secs)),
        None => exponential,
    };
    delay.min(MAX_RETRY_DELAY)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_retry_returns_true_for_retryable_statuses() {
        assert!(should_retry(StatusCode::TOO_MANY_REQUESTS));
        assert!(should_retry(StatusCode::INTERNAL_SERVER_ERROR));
        assert!(should_retry(StatusCode::BAD_GATEWAY));
        assert!(should_retry(StatusCode::SERVICE_UNAVAILABLE));
    }

    #[test]
    fn should_retry_returns_false_for_non_retryable() {
        assert!(!should_retry(StatusCode::OK));
        assert!(!should_retry(StatusCode::BAD_REQUEST));
        assert!(!should_retry(StatusCode::UNAUTHORIZED));
        assert!(!should_retry(StatusCode::NOT_FOUND));
        assert!(!should_retry(StatusCode::FORBIDDEN));
    }

    #[test]
    fn compute_delay_exponential_backoff() {
        let base = Duration::from_secs(1);
        assert_eq!(compute_delay(0, base, None), Duration::from_secs(1));
        assert_eq!(compute_delay(1, base, None), Duration::from_secs(2));
        assert_eq!(compute_delay(2, base, None), Duration::from_secs(4));
        assert_eq!(compute_delay(3, base, None), Duration::from_secs(8));
    }

    #[test]
    fn compute_delay_caps_at_30_seconds() {
        let base = Duration::from_secs(1);
        assert_eq!(compute_delay(10, base, None), Duration::from_secs(30));
    }

    #[test]
    fn compute_delay_respects_retry_after() {
        let base = Duration::from_millis(100);
        // retry_after is larger than exponential — use retry_after
        assert_eq!(compute_delay(0, base, Some(5)), Duration::from_secs(5));
        // exponential is larger than retry_after — use exponential
        assert_eq!(compute_delay(0, Duration::from_secs(10), Some(5)), Duration::from_secs(10));
    }

    #[test]
    fn compute_delay_retry_after_capped_at_30s() {
        let base = Duration::from_millis(100);
        assert_eq!(compute_delay(0, base, Some(60)), Duration::from_secs(30));
    }
}
