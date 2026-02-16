//! Types for the ElevenLabs Pronunciation Dictionary endpoints.
//!
//! Covers dictionary CRUD, rule management, and version locators.
//! Endpoints (under `/v1/pronunciation-dictionaries/`):
//! - GET list dictionaries
//! - GET dictionary by ID
//! - POST add from file / add from rules
//! - POST add rules / remove rules
//! - PATCH update dictionary
//! - GET dictionary rules

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Rule Types
// ---------------------------------------------------------------------------

/// A pronunciation alias rule (request).
///
/// Maps one string to another for pronunciation replacement.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PronunciationAliasRuleRequest {
    /// The string to replace. Must be non-empty.
    pub string_to_replace: String,
    /// Rule type — always `"alias"`.
    #[serde(rename = "type")]
    pub rule_type: String,
    /// The alias for the string to be replaced.
    pub alias: String,
}

/// A pronunciation alias rule (response).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PronunciationAliasRuleResponse {
    /// The string being replaced.
    pub string_to_replace: String,
    /// Rule type.
    #[serde(rename = "type")]
    pub rule_type: String,
    /// The alias.
    pub alias: String,
}

/// A pronunciation phoneme rule (request).
///
/// Maps a string to a phonemic representation using a specific alphabet.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PronunciationPhonemeRuleRequest {
    /// The string to replace. Must be non-empty.
    pub string_to_replace: String,
    /// Rule type — always `"phoneme"`.
    #[serde(rename = "type")]
    pub rule_type: String,
    /// The phoneme representation.
    pub phoneme: String,
    /// The phoneme alphabet to use (e.g. `"ipa"`, `"cmu-arpabet"`).
    pub alphabet: String,
}

/// A pronunciation phoneme rule (response).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PronunciationPhonemeRuleResponse {
    /// The string being replaced.
    pub string_to_replace: String,
    /// Rule type.
    #[serde(rename = "type")]
    pub rule_type: String,
    /// The phoneme representation.
    pub phoneme: String,
    /// The phoneme alphabet.
    pub alphabet: String,
}

// ---------------------------------------------------------------------------
// Dictionary Metadata
// ---------------------------------------------------------------------------

/// Metadata for a pronunciation dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PronunciationDictionaryMetadata {
    /// Dictionary unique identifier.
    pub id: String,
    /// ID of the latest version.
    pub latest_version_id: String,
    /// Number of rules in the latest version.
    pub latest_version_rules_num: i64,
    /// Dictionary name.
    pub name: String,
    /// Permission level on this dictionary.
    pub permission_on_resource: serde_json::Value,
    /// User ID of the creator.
    pub created_by: String,
    /// Unix timestamp of creation.
    pub creation_time_unix: i64,
    /// Unix timestamp when archived, if applicable.
    #[serde(default)]
    pub archived_time_unix: Option<i64>,
    /// Optional description.
    #[serde(default)]
    pub description: Option<String>,
}

/// Version information for a pronunciation dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PronunciationDictionaryVersion {
    /// Version identifier.
    pub version_id: String,
    /// Number of rules in this version.
    pub version_rules_num: i64,
    /// Parent dictionary identifier.
    pub pronunciation_dictionary_id: String,
    /// Dictionary name.
    pub dictionary_name: String,
    /// Version name.
    pub version_name: String,
    /// Permission level on this resource.
    pub permission_on_resource: serde_json::Value,
    /// User ID of the creator.
    pub created_by: String,
    /// Unix timestamp of creation.
    pub creation_time_unix: i64,
    /// Unix timestamp when archived, if applicable.
    #[serde(default)]
    pub archived_time_unix: Option<i64>,
}

// ---------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------

/// Request body for adding rules to a pronunciation dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddPronunciationRulesRequest {
    /// Rules to add (can be alias or phoneme rules, serialized as JSON).
    pub rules: Vec<serde_json::Value>,
}

/// Request body for removing rules from a pronunciation dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemovePronunciationRulesRequest {
    /// The rule strings to remove.
    pub rule_strings: Vec<String>,
}

/// Request body for updating a pronunciation dictionary (PATCH).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdatePronunciationDictionaryRequest {
    /// Whether to archive/unarchive the dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// New name for the dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Locator for a specific pronunciation dictionary version (request).
///
/// Used when attaching dictionaries to projects or other resources.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PronunciationDictionaryLocatorRequest {
    /// The ID of the pronunciation dictionary.
    pub pronunciation_dictionary_id: String,
    /// Optional version ID. If omitted, the latest version is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

/// Response from listing pronunciation dictionaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetPronunciationDictionariesResponse {
    /// List of pronunciation dictionary metadata entries.
    pub pronunciation_dictionaries: Vec<PronunciationDictionaryMetadata>,
    /// Cursor for the next page of results.
    #[serde(default)]
    pub next_cursor: Option<String>,
    /// Whether more results are available.
    pub has_more: bool,
}

/// Response from creating a pronunciation dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatePronunciationDictionaryResponse {
    /// Status string, typically `"ok"`.
    pub status: String,
}

/// Response from adding a pronunciation dictionary (from file or rules).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddPronunciationDictionaryResponse {
    /// ID of the created dictionary.
    pub id: String,
    /// Name of the created dictionary.
    pub name: String,
    /// User ID of the creator.
    pub created_by: String,
    /// Unix timestamp of creation.
    pub creation_time_unix: i64,
    /// ID of the created version.
    pub version_id: String,
    /// Number of rules in the version.
    pub version_rules_num: i64,
    /// Optional description.
    #[serde(default)]
    pub description: Option<String>,
    /// Permission on this resource.
    pub permission_on_resource: serde_json::Value,
}

/// Response after modifying rules (add/remove).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PronunciationDictionaryRulesResponse {
    /// Dictionary identifier.
    pub id: String,
    /// Version ID after the modification.
    pub version_id: String,
    /// Number of rules in the new version.
    pub version_rules_num: i64,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    #[test]
    fn alias_rule_request_serialize() {
        let rule = PronunciationAliasRuleRequest {
            string_to_replace: "ElevenLabs".into(),
            rule_type: "alias".into(),
            alias: "Eleven Labs".into(),
        };
        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains(r#""type":"alias""#));
        assert!(json.contains(r#""string_to_replace":"ElevenLabs""#));
    }

    #[test]
    fn alias_rule_response_deserialize() {
        let json = r#"{
            "string_to_replace": "ElevenLabs",
            "type": "alias",
            "alias": "Eleven Labs"
        }"#;
        let rule: PronunciationAliasRuleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(rule.string_to_replace, "ElevenLabs");
        assert_eq!(rule.alias, "Eleven Labs");
    }

    #[test]
    fn phoneme_rule_request_serialize() {
        let rule = PronunciationPhonemeRuleRequest {
            string_to_replace: "tomato".into(),
            rule_type: "phoneme".into(),
            phoneme: "təˈmeɪtoʊ".into(),
            alphabet: "ipa".into(),
        };
        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains(r#""type":"phoneme""#));
        assert!(json.contains(r#""alphabet":"ipa""#));
    }

    #[test]
    fn phoneme_rule_response_deserialize() {
        let json = r#"{
            "string_to_replace": "tomato",
            "type": "phoneme",
            "phoneme": "təˈmeɪtoʊ",
            "alphabet": "ipa"
        }"#;
        let rule: PronunciationPhonemeRuleResponse = serde_json::from_str(json).unwrap();
        assert_eq!(rule.phoneme, "təˈmeɪtoʊ");
    }

    #[test]
    fn dictionary_metadata_deserialize() {
        let json = r#"{
            "id": "dict1",
            "latest_version_id": "v1",
            "latest_version_rules_num": 5,
            "name": "My Dictionary",
            "permission_on_resource": "admin",
            "created_by": "user1",
            "creation_time_unix": 1700000000,
            "description": "Test dictionary"
        }"#;
        let meta: PronunciationDictionaryMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(meta.id, "dict1");
        assert_eq!(meta.latest_version_rules_num, 5);
        assert_eq!(meta.description, Some("Test dictionary".into()));
    }

    #[test]
    fn dictionary_version_deserialize() {
        let json = r#"{
            "version_id": "v1",
            "version_rules_num": 10,
            "pronunciation_dictionary_id": "dict1",
            "dictionary_name": "My Dict",
            "version_name": "v1.0",
            "permission_on_resource": "editor",
            "created_by": "user1",
            "creation_time_unix": 1700000000
        }"#;
        let ver: PronunciationDictionaryVersion = serde_json::from_str(json).unwrap();
        assert_eq!(ver.version_id, "v1");
        assert_eq!(ver.version_rules_num, 10);
    }

    #[test]
    fn get_dictionaries_response_deserialize() {
        let json = r#"{
            "pronunciation_dictionaries": [
                {
                    "id": "dict1",
                    "latest_version_id": "v1",
                    "latest_version_rules_num": 3,
                    "name": "Dict One",
                    "permission_on_resource": "admin",
                    "created_by": "user1",
                    "creation_time_unix": 1700000000
                }
            ],
            "has_more": false,
            "next_cursor": null
        }"#;
        let resp: GetPronunciationDictionariesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.pronunciation_dictionaries.len(), 1);
        assert!(!resp.has_more);
    }

    #[test]
    fn add_dictionary_response_deserialize() {
        let json = r#"{
            "id": "dict1",
            "name": "New Dict",
            "created_by": "user1",
            "creation_time_unix": 1700000000,
            "version_id": "v1",
            "version_rules_num": 2,
            "permission_on_resource": "admin"
        }"#;
        let resp: AddPronunciationDictionaryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, "dict1");
        assert_eq!(resp.version_rules_num, 2);
    }

    #[test]
    fn rules_response_deserialize() {
        let json = r#"{
            "id": "dict1",
            "version_id": "v2",
            "version_rules_num": 7
        }"#;
        let resp: PronunciationDictionaryRulesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.version_rules_num, 7);
    }

    #[test]
    fn update_request_serialize() {
        let req = UpdatePronunciationDictionaryRequest { archived: Some(true), name: None };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"archived\":true"));
        assert!(!json.contains("name"));
    }

    #[test]
    fn locator_request_serialize() {
        let req = PronunciationDictionaryLocatorRequest {
            pronunciation_dictionary_id: "pd1".into(),
            version_id: Some("v1".into()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"pronunciation_dictionary_id\":\"pd1\""));
        assert!(json.contains("\"version_id\":\"v1\""));
    }

    #[test]
    fn remove_rules_request_serialize() {
        let req =
            RemovePronunciationRulesRequest { rule_strings: vec!["word1".into(), "word2".into()] };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"rule_strings\""));
    }
}
