//! Types for the ElevenLabs Models endpoint.
//!
//! Covers `GET /v1/models` which returns a list of available models.
//! The individual [`Model`](super::common::Model) type is defined in
//! [`common`](super::common).

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Response
// ---------------------------------------------------------------------------

/// Response from `GET /v1/models`.
///
/// The API returns a JSON array of [`Model`](super::common::Model) objects
/// directly (not wrapped in an object), so this is a thin new-type wrapper
/// around `Vec<Model>`.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::GetModelsResponse;
///
/// let json = r#"[
///     {
///         "model_id": "eleven_multilingual_v2",
///         "name": "Multilingual v2",
///         "can_be_finetuned": true,
///         "can_do_text_to_speech": true,
///         "can_do_voice_conversion": true,
///         "can_use_style": true,
///         "can_use_speaker_boost": true,
///         "serves_pro_voices": false,
///         "token_cost_factor": 1.0,
///         "description": "State of the art multilingual model.",
///         "requires_alpha_access": false,
///         "max_characters_request_free_user": 2500,
///         "max_characters_request_subscribed_user": 5000,
///         "maximum_text_length_per_request": 1000000,
///         "languages": [{ "language_id": "en", "name": "English" }],
///         "model_rates": { "character_cost_multiplier": 1.0 },
///         "concurrency_group": "standard"
///     }
/// ]"#;
/// let models: GetModelsResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(models.0.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelsResponse(pub Vec<super::common::Model>);

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn get_models_response_deserialize() {
        let json = r#"[
            {
                "model_id": "eleven_multilingual_v2",
                "name": "Multilingual v2",
                "can_be_finetuned": true,
                "can_do_text_to_speech": true,
                "can_do_voice_conversion": true,
                "can_use_style": true,
                "can_use_speaker_boost": true,
                "serves_pro_voices": false,
                "token_cost_factor": 1.0,
                "description": "State of the art multilingual model.",
                "requires_alpha_access": false,
                "max_characters_request_free_user": 2500,
                "max_characters_request_subscribed_user": 5000,
                "maximum_text_length_per_request": 1000000,
                "languages": [{ "language_id": "en", "name": "English" }],
                "model_rates": { "character_cost_multiplier": 1.0 },
                "concurrency_group": "standard"
            }
        ]"#;
        let models: GetModelsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(models.0.len(), 1);
        assert_eq!(models.0[0].model_id, "eleven_multilingual_v2");
    }

    #[test]
    fn get_models_response_empty() {
        let json = "[]";
        let models: GetModelsResponse = serde_json::from_str(json).unwrap();
        assert!(models.0.is_empty());
    }
}
