//! Types for the ElevenLabs Music endpoints.
//!
//! Covers five music endpoints:
//! - `POST /v1/music/plan` — generate a composition plan
//! - `POST /v1/music` — compose music (returns audio bytes)
//! - `POST /v1/music/detailed` — compose music with detailed metadata
//! - `POST /v1/music/stream` — stream composed music
//! - `POST /v1/music/stem-separation` — separate audio into stems
//!
//! Shared types such as [`MusicPrompt`], [`SongSection`], and [`TimeRange`]
//! appear in both request and response positions and therefore derive both
//! `Serialize` and `Deserialize`.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Shared value types
// ---------------------------------------------------------------------------

/// A time range in milliseconds.
///
/// Used within [`SectionSource`] to specify which portion of a source song
/// to extract or exclude during inpainting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start of the range in milliseconds.
    pub start_ms: i64,
    /// End of the range in milliseconds.
    pub end_ms: i64,
}

/// Source specification for inpainting a song section.
///
/// Points to an existing generated song and a time range within it.
/// Only available to enterprise clients with access to the inpainting API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionSource {
    /// The ID of the source song (found in response headers when generating).
    pub song_id: String,
    /// The time range to extract from the source song.
    pub range: TimeRange,
    /// Ranges within `range` to exclude.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub negative_ranges: Vec<TimeRange>,
}

/// A section within a composition plan.
///
/// Each section defines a named part of the song (e.g. "Verse 1", "Chorus")
/// with its own styles, duration, and lyrics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongSection {
    /// The name of the section (e.g. "Verse 1", "Chorus"). 1–100 characters.
    pub section_name: String,
    /// Styles that *should* be present in this section.
    pub positive_local_styles: Vec<String>,
    /// Styles that should *not* be present in this section.
    pub negative_local_styles: Vec<String>,
    /// Duration of this section in milliseconds (3 000–120 000).
    pub duration_ms: i64,
    /// Lyrics for this section (max 200 chars per line, max 30 lines).
    pub lines: Vec<String>,
    /// Optional source for inpainting (enterprise only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_from: Option<SectionSource>,
}

/// A structured composition plan describing the full song.
///
/// Appears in both requests (as `composition_plan` field) and responses
/// (returned from the plan endpoint and inside [`DetailedMusicResponse`]).
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::{MusicPrompt, SongSection};
///
/// let plan = MusicPrompt {
///     positive_global_styles: vec!["pop".into(), "rock".into()],
///     negative_global_styles: vec!["metal".into()],
///     sections: vec![SongSection {
///         section_name: "Verse 1".into(),
///         positive_local_styles: vec!["acoustic".into()],
///         negative_local_styles: vec![],
///         duration_ms: 15000,
///         lines: vec!["Hello world".into()],
///         source_from: None,
///     }],
/// };
/// assert_eq!(plan.sections.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MusicPrompt {
    /// Styles that should be present throughout the entire song.
    pub positive_global_styles: Vec<String>,
    /// Styles that should *not* be present in the song.
    pub negative_global_styles: Vec<String>,
    /// The ordered sections of the song.
    pub sections: Vec<SongSection>,
}

// ---------------------------------------------------------------------------
// Stem variation
// ---------------------------------------------------------------------------

/// Stem separation variation to use when splitting audio.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StemVariation {
    /// Two stems: vocals + accompaniment.
    #[serde(rename = "two_stems_v1")]
    TwoStemsV1,
    /// Six stems: vocals, drums, bass, guitar, piano, other.
    #[serde(rename = "six_stems_v1")]
    #[default]
    SixStemsV1,
}

// ---------------------------------------------------------------------------
// Response-only types
// ---------------------------------------------------------------------------

/// A word with its timestamp range in the generated song.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WordTimestamp {
    /// The word.
    pub word: String,
    /// Start time in milliseconds.
    pub start_ms: i64,
    /// End time in milliseconds.
    pub end_ms: i64,
}

/// Metadata about a generated song.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongMetadata {
    /// Title of the song.
    pub title: Option<String>,
    /// Description of the song.
    pub description: Option<String>,
    /// Musical genres.
    pub genres: Vec<String>,
    /// Languages used in lyrics.
    pub languages: Vec<String>,
    /// Whether the song contains explicit content.
    pub is_explicit: Option<bool>,
}

/// Detailed response from `POST /v1/music/detailed`.
///
/// Contains the composition plan used, song metadata, and optional word
/// timestamps. The audio data itself is returned in a separate part of the
/// multipart response and is not represented here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetailedMusicResponse {
    /// The composition plan used to generate the song.
    pub composition_plan: MusicPrompt,
    /// Metadata about the generated song.
    pub song_metadata: SongMetadata,
    /// Timestamps for words in the generated song (if lyrics are present).
    pub words_timestamps: Option<Vec<WordTimestamp>>,
}

// ---------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------

/// Request body for `POST /v1/music/plan`.
///
/// Generates a composition plan from a text prompt. The response is a
/// [`MusicPrompt`] that can then be passed to the compose endpoint.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::MusicPlanRequest;
///
/// let req =
///     MusicPlanRequest { prompt: "An upbeat pop song about summer".into(), ..Default::default() };
/// assert_eq!(req.model_id, "music_v1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MusicPlanRequest {
    /// A text prompt describing the desired composition.
    pub prompt: String,

    /// Desired length in milliseconds (3 000–600 000).
    /// If `None`, the model chooses based on the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_length_ms: Option<i64>,

    /// An optional existing composition plan to use as a starting point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_composition_plan: Option<MusicPrompt>,

    /// The model to use.
    pub model_id: String,
}

impl Default for MusicPlanRequest {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            music_length_ms: None,
            source_composition_plan: None,
            model_id: "music_v1".into(),
        }
    }
}

/// Request body for `POST /v1/music`, `POST /v1/music/stream`, and
/// `POST /v1/music/detailed`.
///
/// Generates (or streams) a song. Exactly one of `prompt` or
/// `composition_plan` must be provided.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::MusicComposeRequest;
///
/// let req =
///     MusicComposeRequest { prompt: Some("A mellow jazz piece".into()), ..Default::default() };
/// assert_eq!(req.model_id, "music_v1");
/// assert!(!req.force_instrumental);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MusicComposeRequest {
    /// A simple text prompt to generate a song from.
    /// Cannot be used together with `composition_plan`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// A detailed composition plan to guide generation.
    /// Cannot be used together with `prompt`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composition_plan: Option<MusicPrompt>,

    /// Desired length in milliseconds (3 000–600 000).
    /// Used only with `prompt`. If `None`, the model chooses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_length_ms: Option<i64>,

    /// The model to use.
    pub model_id: String,

    /// Seed for deterministic generation (0–2 147 483 647).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,

    /// If `true`, guarantees no vocals in the output.
    /// Can only be used with `prompt`.
    pub force_instrumental: bool,

    /// Whether section durations in `composition_plan` are strictly enforced.
    /// `true` (default) respects exact durations; `false` allows the model
    /// to adjust for better quality.
    pub respect_sections_durations: bool,

    /// Whether to store the song for later inpainting (enterprise only).
    pub store_for_inpainting: bool,

    /// Whether to sign the generated song with C2PA (MP3 only).
    pub sign_with_c2pa: bool,
}

impl Default for MusicComposeRequest {
    fn default() -> Self {
        Self {
            prompt: None,
            composition_plan: None,
            music_length_ms: None,
            model_id: "music_v1".into(),
            seed: None,
            force_instrumental: false,
            respect_sections_durations: true,
            store_for_inpainting: false,
            sign_with_c2pa: false,
        }
    }
}

/// Request fields for `POST /v1/music/stem-separation`.
///
/// Uses `multipart/form-data`; the audio file itself is provided
/// separately in the service layer.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::MusicStemSeparationRequest;
///
/// let req = MusicStemSeparationRequest::default();
/// assert_eq!(req.stem_variation_id, elevenlabs_sdk::types::StemVariation::SixStemsV1);
/// assert!(!req.sign_with_c2pa);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct MusicStemSeparationRequest {
    /// Which stem separation variant to use.
    pub stem_variation_id: StemVariation,

    /// Whether to sign the output with C2PA (MP3 only).
    pub sign_with_c2pa: bool,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    // -- TimeRange ----------------------------------------------------------

    #[test]
    fn time_range_serde_round_trip() {
        let tr = TimeRange { start_ms: 1000, end_ms: 5000 };
        let json = serde_json::to_string(&tr).unwrap();
        let back: TimeRange = serde_json::from_str(&json).unwrap();
        assert_eq!(tr, back);
    }

    // -- SectionSource ------------------------------------------------------

    #[test]
    fn section_source_serde_round_trip() {
        let ss = SectionSource {
            song_id: "song_abc".into(),
            range: TimeRange { start_ms: 0, end_ms: 10000 },
            negative_ranges: vec![TimeRange { start_ms: 3000, end_ms: 5000 }],
        };
        let json = serde_json::to_string(&ss).unwrap();
        let back: SectionSource = serde_json::from_str(&json).unwrap();
        assert_eq!(ss, back);
    }

    #[test]
    fn section_source_empty_negative_ranges_omitted() {
        let ss = SectionSource {
            song_id: "s1".into(),
            range: TimeRange { start_ms: 0, end_ms: 5000 },
            negative_ranges: vec![],
        };
        let json = serde_json::to_string(&ss).unwrap();
        assert!(!json.contains("negative_ranges"));
    }

    // -- SongSection --------------------------------------------------------

    #[test]
    fn song_section_serde_round_trip() {
        let sec = SongSection {
            section_name: "Chorus".into(),
            positive_local_styles: vec!["upbeat".into()],
            negative_local_styles: vec![],
            duration_ms: 20000,
            lines: vec!["La la la".into()],
            source_from: None,
        };
        let json = serde_json::to_string(&sec).unwrap();
        let back: SongSection = serde_json::from_str(&json).unwrap();
        assert_eq!(sec, back);
    }

    // -- MusicPrompt --------------------------------------------------------

    #[test]
    fn music_prompt_serde_round_trip() {
        let plan = MusicPrompt {
            positive_global_styles: vec!["pop".into(), "rock".into()],
            negative_global_styles: vec!["metal".into()],
            sections: vec![SongSection {
                section_name: "Verse 1".into(),
                positive_local_styles: vec!["acoustic".into()],
                negative_local_styles: vec![],
                duration_ms: 15000,
                lines: vec!["Hello world".into()],
                source_from: None,
            }],
        };
        let json = serde_json::to_string(&plan).unwrap();
        let back: MusicPrompt = serde_json::from_str(&json).unwrap();
        assert_eq!(plan, back);
    }

    // -- StemVariation ------------------------------------------------------

    #[test]
    fn stem_variation_default_is_six() {
        assert_eq!(StemVariation::default(), StemVariation::SixStemsV1);
    }

    #[test]
    fn stem_variation_serde_round_trip() {
        for variant in [StemVariation::TwoStemsV1, StemVariation::SixStemsV1] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: StemVariation = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn stem_variation_serde_names() {
        assert_eq!(serde_json::to_string(&StemVariation::TwoStemsV1).unwrap(), r#""two_stems_v1""#);
        assert_eq!(serde_json::to_string(&StemVariation::SixStemsV1).unwrap(), r#""six_stems_v1""#);
    }

    // -- WordTimestamp ------------------------------------------------------

    #[test]
    fn word_timestamp_deserialization() {
        let json = r#"{"word":"Hello","start_ms":0,"end_ms":500}"#;
        let wt: WordTimestamp = serde_json::from_str(json).unwrap();
        assert_eq!(wt.word, "Hello");
        assert_eq!(wt.start_ms, 0);
        assert_eq!(wt.end_ms, 500);
    }

    // -- SongMetadata -------------------------------------------------------

    #[test]
    fn song_metadata_deserialization() {
        let json = r#"{
            "title": "My Song",
            "description": "A test song",
            "genres": ["pop", "rock"],
            "languages": ["en"],
            "is_explicit": false
        }"#;
        let meta: SongMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(meta.title.as_deref(), Some("My Song"));
        assert_eq!(meta.genres.len(), 2);
        assert_eq!(meta.is_explicit, Some(false));
    }

    #[test]
    fn song_metadata_nullable_fields() {
        let json = r#"{
            "title": null,
            "description": null,
            "genres": [],
            "languages": [],
            "is_explicit": null
        }"#;
        let meta: SongMetadata = serde_json::from_str(json).unwrap();
        assert!(meta.title.is_none());
        assert!(meta.description.is_none());
        assert!(meta.is_explicit.is_none());
    }

    // -- DetailedMusicResponse ---------------------------------------------

    #[test]
    fn detailed_response_deserialization() {
        let json = r#"{
            "composition_plan": {
                "positive_global_styles": ["pop"],
                "negative_global_styles": [],
                "sections": [{
                    "section_name": "Intro",
                    "positive_local_styles": [],
                    "negative_local_styles": [],
                    "duration_ms": 5000,
                    "lines": []
                }]
            },
            "song_metadata": {
                "title": "Test",
                "description": null,
                "genres": ["pop"],
                "languages": ["en"],
                "is_explicit": false
            },
            "words_timestamps": [
                {"word": "la", "start_ms": 0, "end_ms": 500}
            ]
        }"#;
        let resp: DetailedMusicResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.composition_plan.sections.len(), 1);
        assert_eq!(resp.song_metadata.title.as_deref(), Some("Test"));
        let ts = resp.words_timestamps.unwrap();
        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0].word, "la");
    }

    #[test]
    fn detailed_response_null_timestamps() {
        let json = r#"{
            "composition_plan": {
                "positive_global_styles": [],
                "negative_global_styles": [],
                "sections": []
            },
            "song_metadata": {
                "title": null,
                "description": null,
                "genres": [],
                "languages": [],
                "is_explicit": null
            },
            "words_timestamps": null
        }"#;
        let resp: DetailedMusicResponse = serde_json::from_str(json).unwrap();
        assert!(resp.words_timestamps.is_none());
    }

    // -- MusicPlanRequest ---------------------------------------------------

    #[test]
    fn plan_request_default_values() {
        let req = MusicPlanRequest::default();
        assert!(req.prompt.is_empty());
        assert!(req.music_length_ms.is_none());
        assert!(req.source_composition_plan.is_none());
        assert_eq!(req.model_id, "music_v1");
    }

    #[test]
    fn plan_request_minimal_serialization() {
        let req = MusicPlanRequest { prompt: "A chill lo-fi beat".into(), ..Default::default() };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj["prompt"], "A chill lo-fi beat");
        assert_eq!(obj["model_id"], "music_v1");
        assert!(!obj.contains_key("music_length_ms"));
        assert!(!obj.contains_key("source_composition_plan"));
    }

    #[test]
    fn plan_request_full_serialization() {
        let req = MusicPlanRequest {
            prompt: "Epic orchestral".into(),
            music_length_ms: Some(60000),
            source_composition_plan: Some(MusicPrompt {
                positive_global_styles: vec!["orchestral".into()],
                negative_global_styles: vec![],
                sections: vec![],
            }),
            model_id: "music_v1".into(),
        };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["music_length_ms"], 60000);
        assert!(v["source_composition_plan"].is_object());
    }

    // -- MusicComposeRequest ------------------------------------------------

    #[test]
    fn compose_request_default_values() {
        let req = MusicComposeRequest::default();
        assert!(req.prompt.is_none());
        assert!(req.composition_plan.is_none());
        assert!(req.music_length_ms.is_none());
        assert_eq!(req.model_id, "music_v1");
        assert!(req.seed.is_none());
        assert!(!req.force_instrumental);
        assert!(req.respect_sections_durations);
        assert!(!req.store_for_inpainting);
        assert!(!req.sign_with_c2pa);
    }

    #[test]
    fn compose_request_minimal_serialization() {
        let req = MusicComposeRequest { prompt: Some("A happy song".into()), ..Default::default() };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj["prompt"], "A happy song");
        assert_eq!(obj["model_id"], "music_v1");
        assert_eq!(obj["force_instrumental"], false);
        assert_eq!(obj["respect_sections_durations"], true);
        assert!(!obj.contains_key("composition_plan"));
        assert!(!obj.contains_key("music_length_ms"));
        assert!(!obj.contains_key("seed"));
    }

    #[test]
    fn compose_request_with_plan() {
        let req = MusicComposeRequest {
            composition_plan: Some(MusicPrompt {
                positive_global_styles: vec!["jazz".into()],
                negative_global_styles: vec![],
                sections: vec![SongSection {
                    section_name: "Intro".into(),
                    positive_local_styles: vec![],
                    negative_local_styles: vec![],
                    duration_ms: 5000,
                    lines: vec![],
                    source_from: None,
                }],
            }),
            seed: Some(42),
            ..Default::default()
        };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(v["composition_plan"].is_object());
        assert_eq!(v["seed"], 42);
    }

    // -- MusicStemSeparationRequest -----------------------------------------

    #[test]
    fn stem_separation_request_default_values() {
        let req = MusicStemSeparationRequest::default();
        assert_eq!(req.stem_variation_id, StemVariation::SixStemsV1);
        assert!(!req.sign_with_c2pa);
    }

    #[test]
    fn stem_separation_request_serialization() {
        let req = MusicStemSeparationRequest {
            stem_variation_id: StemVariation::TwoStemsV1,
            sign_with_c2pa: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["stem_variation_id"], "two_stems_v1");
        assert_eq!(v["sign_with_c2pa"], true);
    }
}
