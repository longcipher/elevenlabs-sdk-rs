//! Types for the ElevenLabs Voice Generation endpoints.
//!
//! Covers voice generation and design operations:
//! - `GET /v1/voice-generation/generate-voice/parameters` — list generation parameters
//! - `POST /v1/voice-generation/generate-voice` — generate a random voice
//! - `POST /v1/voice-generation/create-voice` — create a voice from a generated preview

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Voice Generation Parameters (response)
// ---------------------------------------------------------------------------

/// A selectable option for a voice generation parameter.
///
/// Options describe available genders, accents, and ages with
/// human-readable names and machine-readable codes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceGenerationParameterOption {
    /// Human-readable name (e.g. `"Female"`, `"British"`).
    pub name: String,
    /// Machine-readable code (e.g. `"female"`, `"british"`).
    pub code: String,
}

/// Response from `GET /v1/voice-generation/generate-voice/parameters`.
///
/// Lists available parameter options (gender, accent, age) and their
/// allowed ranges for voice generation requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceGenerationParameters {
    /// Available gender options.
    pub genders: Vec<VoiceGenerationParameterOption>,
    /// Available accent options.
    pub accents: Vec<VoiceGenerationParameterOption>,
    /// Available age options.
    pub ages: Vec<VoiceGenerationParameterOption>,
    /// Minimum number of characters in the generation text.
    pub minimum_characters: i64,
    /// Maximum number of characters in the generation text.
    pub maximum_characters: i64,
    /// Minimum accent strength value.
    pub minimum_accent_strength: f64,
    /// Maximum accent strength value.
    pub maximum_accent_strength: f64,
}

// ---------------------------------------------------------------------------
// Generate Random Voice (request)
// ---------------------------------------------------------------------------

/// Gender for voice generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerateVoiceGender {
    /// Female voice.
    Female,
    /// Male voice.
    Male,
}

/// Age category for voice generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerateVoiceAge {
    /// Young-sounding voice.
    Young,
    /// Middle-aged voice.
    MiddleAged,
    /// Older-sounding voice.
    Old,
}

/// Request body for `POST /v1/voice-generation/generate-voice`.
///
/// Generates a random voice with the specified characteristics. The
/// response is audio bytes of the generated voice speaking the provided
/// text.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::{
///     GenerateRandomVoiceRequest, GenerateVoiceAge, GenerateVoiceGender,
/// };
///
/// let req = GenerateRandomVoiceRequest {
///     gender: GenerateVoiceGender::Female,
///     accent: "british".into(),
///     age: GenerateVoiceAge::Young,
///     accent_strength: 1.0,
///     text: "Every act of kindness carries value and can make a difference.".repeat(2),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GenerateRandomVoiceRequest {
    /// Gender of the generated voice.
    pub gender: GenerateVoiceGender,
    /// Accent code (e.g. `"british"`, `"american"`, `"african"`, `"australian"`, `"indian"`).
    pub accent: String,
    /// Age category of the generated voice.
    pub age: GenerateVoiceAge,
    /// Accent strength. Must be between 0.3 and 2.0.
    pub accent_strength: f64,
    /// Text to speak for the generation preview (100–1000 characters).
    pub text: String,
}

// ---------------------------------------------------------------------------
// Create Voice from Generated Preview (request)
// ---------------------------------------------------------------------------

/// Request body for `POST /v1/voice-generation/create-voice`.
///
/// Creates a persistent voice from a previously generated voice preview.
/// The `generated_voice_id` is obtained from the response headers of
/// text-to-voice preview endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateGeneratedVoiceRequest {
    /// Display name for the new voice.
    pub voice_name: String,
    /// Description for the new voice.
    pub voice_description: String,
    /// ID of the generated voice preview to create from.
    pub generated_voice_id: String,
    /// Voice IDs that were played but not selected (used for RLHF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub played_not_selected_voice_ids: Option<Vec<String>>,
    /// Optional metadata labels for the new voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn voice_generation_parameters_deserialize() {
        let json = r#"{
            "genders": [
                {"name": "Female", "code": "female"},
                {"name": "Male", "code": "male"}
            ],
            "accents": [
                {"name": "British", "code": "british"},
                {"name": "American", "code": "american"}
            ],
            "ages": [
                {"name": "Young", "code": "young"},
                {"name": "Middle Aged", "code": "middle_aged"},
                {"name": "Old", "code": "old"}
            ],
            "minimum_characters": 100,
            "maximum_characters": 1000,
            "minimum_accent_strength": 0.3,
            "maximum_accent_strength": 2.0
        }"#;
        let params: VoiceGenerationParameters = serde_json::from_str(json).unwrap();
        assert_eq!(params.genders.len(), 2);
        assert_eq!(params.genders[0].name, "Female");
        assert_eq!(params.genders[0].code, "female");
        assert_eq!(params.accents.len(), 2);
        assert_eq!(params.ages.len(), 3);
        assert_eq!(params.minimum_characters, 100);
        assert_eq!(params.maximum_characters, 1000);
        assert!((params.minimum_accent_strength - 0.3).abs() < f64::EPSILON);
        assert!((params.maximum_accent_strength - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn generate_random_voice_request_serialize() {
        let req = GenerateRandomVoiceRequest {
            gender: GenerateVoiceGender::Female,
            accent: "british".into(),
            age: GenerateVoiceAge::Young,
            accent_strength: 1.5,
            text: "a]".repeat(60),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["gender"], "female");
        assert_eq!(json["accent"], "british");
        assert_eq!(json["age"], "young");
        assert_eq!(json["accent_strength"], 1.5);
    }

    #[test]
    fn generate_voice_gender_serialize() {
        assert_eq!(serde_json::to_string(&GenerateVoiceGender::Female).unwrap(), "\"female\"");
        assert_eq!(serde_json::to_string(&GenerateVoiceGender::Male).unwrap(), "\"male\"");
    }

    #[test]
    fn generate_voice_age_serialize() {
        assert_eq!(serde_json::to_string(&GenerateVoiceAge::Young).unwrap(), "\"young\"");
        assert_eq!(
            serde_json::to_string(&GenerateVoiceAge::MiddleAged).unwrap(),
            "\"middle_aged\""
        );
        assert_eq!(serde_json::to_string(&GenerateVoiceAge::Old).unwrap(), "\"old\"");
    }

    #[test]
    fn create_generated_voice_request_serialize() {
        let req = CreateGeneratedVoiceRequest {
            voice_name: "Sassy Mouse".into(),
            voice_description: "A sassy squeaky mouse".into(),
            generated_voice_id: "37HceQefKmEi3bGovXjL".into(),
            played_not_selected_voice_ids: Some(vec!["id1".into(), "id2".into()]),
            labels: Some(HashMap::from([("language".into(), "en".into())])),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["voice_name"], "Sassy Mouse");
        assert_eq!(json["voice_description"], "A sassy squeaky mouse");
        assert_eq!(json["generated_voice_id"], "37HceQefKmEi3bGovXjL");
        assert_eq!(json["played_not_selected_voice_ids"], serde_json::json!(["id1", "id2"]));
        assert_eq!(json["labels"]["language"], "en");
    }

    #[test]
    fn create_generated_voice_request_omits_none_fields() {
        let req = CreateGeneratedVoiceRequest {
            voice_name: "Voice".into(),
            voice_description: "Desc".into(),
            generated_voice_id: "id123".into(),
            played_not_selected_voice_ids: None,
            labels: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("voice_name"));
        assert!(!json.contains("played_not_selected_voice_ids"));
        assert!(!json.contains("labels"));
    }
}
