//! Types for the ElevenLabs User endpoints.
//!
//! Covers:
//! - `GET /v1/user` — retrieve the current user's profile and subscription
//! - `GET /v1/user/subscription` — retrieve extended subscription details
//! - `GET /v1/usage/character-stats` — retrieve character usage statistics

use serde::{Deserialize, Serialize};

use super::common::Subscription;

// ---------------------------------------------------------------------------
// Response
// ---------------------------------------------------------------------------

/// Response from `GET /v1/user`.
///
/// Contains user profile information and subscription details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserResponse {
    /// Unique user identifier.
    pub user_id: String,
    /// The user's subscription details.
    pub subscription: Subscription,
    /// Whether this is a new user (deprecated — use `created_at` instead).
    pub is_new_user: bool,
    /// The user's API key (only returned on certain requests).
    #[serde(default)]
    pub xi_api_key: Option<String>,
    /// Whether the user can use delayed payment methods (deprecated).
    pub can_use_delayed_payment_methods: bool,
    /// Whether onboarding has been completed.
    pub is_onboarding_completed: bool,
    /// Whether the onboarding checklist has been completed.
    pub is_onboarding_checklist_completed: bool,
    /// Whether to show compliance terms during onboarding.
    #[serde(default)]
    pub show_compliance_terms: Option<bool>,
    /// User's first name.
    #[serde(default)]
    pub first_name: Option<String>,
    /// Whether the user's API key is stored hashed.
    #[serde(default)]
    pub is_api_key_hashed: Option<bool>,
    /// Preview of the user's API key (masked).
    #[serde(default)]
    pub xi_api_key_preview: Option<String>,
    /// Referral link code.
    #[serde(default)]
    pub referral_link_code: Option<String>,
    /// PartnerStack partner default link.
    #[serde(default)]
    pub partnerstack_partner_default_link: Option<String>,
    /// Unix timestamp of user creation.
    pub created_at: i64,
}

// ---------------------------------------------------------------------------
// Extended Subscription
// ---------------------------------------------------------------------------

/// Response from `GET /v1/user/subscription`.
///
/// Extends the base subscription information with invoice and billing
/// details. Uses [`serde_json::Value`] for complex nested types like
/// invoices and pending changes to remain forward-compatible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtendedSubscriptionResponse {
    /// Subscription tier name (e.g. `"trial"`, `"creator"`).
    pub tier: String,
    /// Number of characters used in the current billing period.
    pub character_count: i64,
    /// Maximum characters allowed in the current billing period.
    pub character_limit: i64,
    /// Whether the user can extend their character limit.
    pub can_extend_character_limit: bool,
    /// Whether the user is allowed to extend their character limit.
    pub allowed_to_extend_character_limit: bool,
    /// Unix timestamp of next character count reset.
    #[serde(default)]
    pub next_character_count_reset_unix: Option<i64>,
    /// Number of voice slots in use.
    pub voice_slots_used: i64,
    /// Number of professional voice slots in use.
    pub professional_voice_slots_used: i64,
    /// Maximum number of voice slots allowed.
    pub voice_limit: i64,
    /// Number of voice add/edit operations performed.
    pub voice_add_edit_counter: i64,
    /// Maximum number of professional voices allowed.
    pub professional_voice_limit: i64,
    /// Whether the user can extend their voice limit.
    pub can_extend_voice_limit: bool,
    /// Whether the user can use instant voice cloning.
    pub can_use_instant_voice_cloning: bool,
    /// Whether the user can use professional voice cloning.
    pub can_use_professional_voice_cloning: bool,
    /// Current subscription status.
    #[serde(default)]
    pub status: Option<serde_json::Value>,
    /// Whether there are open invoices.
    #[serde(default)]
    pub has_open_invoices: Option<bool>,
    /// Next invoice details.
    #[serde(default)]
    pub next_invoice: Option<serde_json::Value>,
    /// List of open invoices.
    #[serde(default)]
    pub open_invoices: Option<Vec<serde_json::Value>>,
    /// Pending subscription change (switch or cancellation).
    #[serde(default)]
    pub pending_change: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Usage / Character Stats
// ---------------------------------------------------------------------------

/// Response from `GET /v1/usage/character-stats`.
///
/// Contains time-series character usage data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageCharactersResponse {
    /// Unix timestamps for each data point.
    pub time: Vec<i64>,
    /// Usage breakdown by category. Keys are metric names, values are
    /// arrays of counts aligned to the `time` vector.
    pub usage: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn user_response_deserialize() {
        let json = r#"{
            "user_id": "user123",
            "subscription": {
                "tier": "creator",
                "character_count": 5000,
                "character_limit": 100000,
                "can_extend_character_limit": true,
                "allowed_to_extend_character_limit": true,
                "voice_slots_used": 3,
                "professional_voice_slots_used": 0,
                "voice_limit": 30,
                "voice_add_edit_counter": 5,
                "professional_voice_limit": 1,
                "can_extend_voice_limit": true,
                "can_use_instant_voice_cloning": true,
                "can_use_professional_voice_cloning": true,
                "status": "active"
            },
            "is_new_user": false,
            "can_use_delayed_payment_methods": false,
            "is_onboarding_completed": true,
            "is_onboarding_checklist_completed": true,
            "created_at": 1700000000
        }"#;
        let user: UserResponse = serde_json::from_str(json).unwrap();
        assert_eq!(user.user_id, "user123");
        assert!(!user.is_new_user);
        assert!(user.is_onboarding_completed);
        assert_eq!(user.created_at, 1700000000);
    }

    #[test]
    fn user_response_with_optional_fields() {
        let json = r#"{
            "user_id": "user456",
            "subscription": {
                "tier": "free",
                "character_count": 0,
                "character_limit": 10000,
                "can_extend_character_limit": false,
                "allowed_to_extend_character_limit": false,
                "voice_slots_used": 0,
                "professional_voice_slots_used": 0,
                "voice_limit": 10,
                "voice_add_edit_counter": 0,
                "professional_voice_limit": 0,
                "can_extend_voice_limit": false,
                "can_use_instant_voice_cloning": false,
                "can_use_professional_voice_cloning": false,
                "status": "free"
            },
            "is_new_user": true,
            "xi_api_key": "xi_key_123",
            "can_use_delayed_payment_methods": false,
            "is_onboarding_completed": false,
            "is_onboarding_checklist_completed": false,
            "first_name": "John",
            "xi_api_key_preview": "xi_...23",
            "created_at": 1710000000
        }"#;
        let user: UserResponse = serde_json::from_str(json).unwrap();
        assert_eq!(user.xi_api_key, Some("xi_key_123".into()));
        assert_eq!(user.first_name, Some("John".into()));
    }
}
