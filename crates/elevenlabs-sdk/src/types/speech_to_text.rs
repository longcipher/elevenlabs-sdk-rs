//! Types for the ElevenLabs Speech-to-Text (STT) endpoints.
//!
//! Covers three STT endpoints:
//! - `POST /v1/speech-to-text` — transcribe audio
//! - `GET /v1/speech-to-text/transcripts/{transcription_id}` — retrieve transcript
//! - `DELETE /v1/speech-to-text/transcripts/{transcription_id}` — delete transcript
//!
//! The transcription endpoint accepts multipart/form-data with an audio file
//! (or a cloud storage URL) and configuration fields. It returns either a
//! single-channel or multichannel transcript, or a webhook acknowledgement.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// STT model identifier.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeechToTextModelId {
    /// Scribe v1 model.
    ScribeV1,
    /// Scribe v2 model (default).
    #[default]
    ScribeV2,
}

/// Granularity of timestamps returned in the transcription.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimestampsGranularity {
    /// No timestamps.
    None,
    /// Word-level timestamps (default).
    #[default]
    Word,
    /// Character-level timestamps within each word.
    Character,
}

/// Format of the input audio file sent to speech-to-text.
///
/// Using `PcmS16le16` (16-bit PCM, 16 kHz, mono, little-endian) avoids
/// server-side decoding and reduces latency.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeechToTextFileFormat {
    /// 16-bit PCM at 16 kHz, mono, little-endian.
    #[serde(rename = "pcm_s16le_16")]
    PcmS16le16,
    /// Any other encoded audio format.
    #[default]
    Other,
}

/// Export format for additional transcript representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    /// SubRip subtitle format.
    Srt,
    /// Plain text.
    Txt,
    /// Microsoft Word document.
    Docx,
    /// PDF document.
    Pdf,
    /// HTML document.
    Html,
    /// Segmented JSON.
    SegmentedJson,
}

/// Type classification for a transcribed word or sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WordType {
    /// A spoken word.
    Word,
    /// Whitespace or punctuation spacing.
    Spacing,
    /// A non-word sound event (e.g. laughter, footsteps).
    AudioEvent,
}

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// Options for an additional export format alongside the main transcript.
///
/// Up to 10 additional formats can be requested per transcription.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::{ExportFormat, ExportOptions};
///
/// let opts = ExportOptions {
///     format: ExportFormat::Srt,
///     include_speakers: Some(true),
///     include_timestamps: Some(true),
///     max_characters_per_line: Some(42),
///     segment_on_silence_longer_than_s: None,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ExportOptions {
    /// Target export format.
    pub format: ExportFormat,

    /// Whether to include speaker labels in the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_speakers: Option<bool>,

    /// Whether to include timestamps in the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_timestamps: Option<bool>,

    /// Maximum characters per line (applies to SRT and TXT formats).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_characters_per_line: Option<u32>,

    /// Segment on silence gaps longer than this many seconds
    /// (applies to DOCX, PDF, HTML, and segmented-JSON formats).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_on_silence_longer_than_s: Option<f64>,
}

/// Configuration fields for `POST /v1/speech-to-text`.
///
/// The endpoint uses `multipart/form-data`. This struct captures every
/// non-file field; the audio file is provided separately when building
/// the multipart request in the service layer.
///
/// Either `file` (via multipart) or `cloud_storage_url` must be provided,
/// but not both. That constraint is enforced at the service layer.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::{SpeechToTextModelId, SpeechToTextRequest};
///
/// let req = SpeechToTextRequest {
///     model_id: SpeechToTextModelId::ScribeV2,
///     ..SpeechToTextRequest::default()
/// };
/// assert!(req.language_code.is_none());
/// assert!(req.tag_audio_events);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpeechToTextRequest {
    /// The STT model to use for transcription.
    pub model_id: SpeechToTextModelId,

    /// ISO-639-1 or ISO-639-3 language code of the audio. When `None`, the
    /// language is detected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// Whether to tag audio events like `(laughter)` or `(footsteps)`.
    pub tag_audio_events: bool,

    /// Maximum number of speakers in the audio (1–32). When `None`, the
    /// model uses its maximum supported speaker count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_speakers: Option<u32>,

    /// Granularity of timestamps in the transcription.
    pub timestamps_granularity: TimestampsGranularity,

    /// Whether to annotate which speaker is currently talking.
    pub diarize: bool,

    /// Threshold for speaker diarization (0.1–0.4). Higher values produce
    /// fewer predicted speakers. Only valid when `diarize` is `true` and
    /// `num_speakers` is `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarization_threshold: Option<f64>,

    /// Additional export formats for the transcript (max 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_formats: Option<Vec<ExportOptions>>,

    /// Format of the input audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<SpeechToTextFileFormat>,

    /// HTTPS URL of the audio file to transcribe (alternative to uploading
    /// a file). Must be less than 2 GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_storage_url: Option<String>,

    /// When `true`, the transcript is delivered asynchronously via webhook
    /// and the request returns early with a [`SpeechToTextWebhookResponse`].
    pub webhook: bool,

    /// Specific webhook ID to send the result to. Only valid when
    /// `webhook` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,

    /// Controls randomness of the transcription output (0.0–2.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Seed for deterministic generation (0–2 147 483 647).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,

    /// Whether each audio channel holds a single speaker (max 5 channels).
    /// Each word in the response will include a `channel_index`.
    pub use_multi_channel: bool,

    /// Metadata included in the webhook response (JSON string or object,
    /// max 16 KB, depth ≤ 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_metadata: Option<String>,

    /// Entity types to detect in the transcript (e.g. `"all"`, `"pii"`).
    /// Incurs additional cost when enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_detection: Option<Vec<String>>,

    /// Key terms to bias the transcription towards (max 100 terms,
    /// each ≤ 50 chars, ≤ 5 words). Incurs additional cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyterms: Option<Vec<String>>,
}

impl Default for SpeechToTextRequest {
    fn default() -> Self {
        Self {
            model_id: SpeechToTextModelId::default(),
            language_code: None,
            tag_audio_events: true,
            num_speakers: None,
            timestamps_granularity: TimestampsGranularity::default(),
            diarize: false,
            diarization_threshold: None,
            additional_formats: None,
            file_format: None,
            cloud_storage_url: None,
            webhook: false,
            webhook_id: None,
            temperature: None,
            seed: None,
            use_multi_channel: false,
            webhook_metadata: None,
            entity_detection: None,
            keyterms: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

/// Character-level timing information within a word.
///
/// Returned when `timestamps_granularity` is set to `Character`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeechToTextCharacter {
    /// The transcribed character.
    pub text: String,
    /// Start time in seconds.
    pub start: Option<f64>,
    /// End time in seconds.
    pub end: Option<f64>,
}

/// Word-level detail in a speech-to-text transcription.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeechToTextWord {
    /// The transcribed word, spacing, or audio event text.
    pub text: String,
    /// Start time of the word in seconds.
    pub start: Option<f64>,
    /// End time of the word in seconds.
    pub end: Option<f64>,
    /// Classification of this element (word, spacing, or audio event).
    #[serde(rename = "type")]
    pub word_type: WordType,
    /// Unique identifier of the speaker. Present when diarization is enabled.
    pub speaker_id: Option<String>,
    /// Log probability of this word. Range: `(-∞, 0]`.
    pub logprob: f64,
    /// Character-level timing breakdown. Present when
    /// `timestamps_granularity` is `Character`.
    pub characters: Option<Vec<SpeechToTextCharacter>>,
}

/// An entity detected in the transcript text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectedEntity {
    /// The text identified as an entity.
    pub text: String,
    /// The entity type (e.g. `"credit_card"`, `"email_address"`,
    /// `"person_name"`).
    pub entity_type: String,
    /// Start character position in the transcript text.
    pub start_char: u64,
    /// End character position in the transcript text.
    pub end_char: u64,
}

/// An additional format representation of the transcript.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdditionalFormatResponse {
    /// The format that was requested (e.g. `"srt"`, `"txt"`).
    pub requested_format: String,
    /// File extension for this format.
    pub file_extension: String,
    /// MIME content type.
    pub content_type: String,
    /// Whether `content` is base64-encoded.
    pub is_base64_encoded: bool,
    /// The transcript content in the requested format.
    pub content: String,
}

/// Single-channel (or per-channel) transcription result.
///
/// Returned by `POST /v1/speech-to-text` for single-channel audio and also
/// used as elements inside [`MultichannelSpeechToTextResponse::transcripts`].
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::SpeechToTextChunkResponse;
///
/// let json = r#"{
///     "language_code": "eng",
///     "language_probability": 0.98,
///     "text": "Hello world!",
///     "words": [
///         {"text": "Hello", "start": 0.0, "end": 0.5, "type": "word", "logprob": -0.124},
///         {"text": " ", "start": 0.5, "end": 0.5, "type": "spacing", "logprob": 0.0},
///         {"text": "world!", "start": 0.5, "end": 1.2, "type": "word", "logprob": -0.089}
///     ]
/// }"#;
/// let resp: SpeechToTextChunkResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(resp.text, "Hello world!");
/// assert_eq!(resp.words.len(), 3);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeechToTextChunkResponse {
    /// Detected language code (e.g. `"eng"` for English).
    pub language_code: String,
    /// Confidence score of the language detection (0.0–1.0).
    pub language_probability: f64,
    /// Raw text of the transcription.
    pub text: String,
    /// Word-level details with timing information.
    pub words: Vec<SpeechToTextWord>,
    /// Channel index (for multichannel audio via `use_multi_channel`).
    pub channel_index: Option<u32>,
    /// Additional format representations of the transcript.
    pub additional_formats: Option<Vec<Option<AdditionalFormatResponse>>>,
    /// The transcription ID.
    pub transcription_id: Option<String>,
    /// Detected entities in the transcript text.
    pub entities: Option<Vec<DetectedEntity>>,
}

/// Multichannel transcription result.
///
/// Returned by `POST /v1/speech-to-text` when `use_multi_channel` is `true`.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::MultichannelSpeechToTextResponse;
///
/// let json = r#"{
///     "transcripts": [{
///         "language_code": "eng",
///         "language_probability": 0.98,
///         "text": "Hello from channel one.",
///         "words": [
///             {"text": "Hello", "start": 0.0, "end": 0.5, "type": "word", "logprob": -0.124}
///         ]
///     }]
/// }"#;
/// let resp: MultichannelSpeechToTextResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(resp.transcripts.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultichannelSpeechToTextResponse {
    /// One transcript per audio channel.
    pub transcripts: Vec<SpeechToTextChunkResponse>,
    /// The transcription ID.
    pub transcription_id: Option<String>,
}

/// Webhook acknowledgement returned when `webhook` is set to `true`.
///
/// The actual transcript is delivered asynchronously to the configured
/// webhook endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeechToTextWebhookResponse {
    /// Acknowledgement message.
    pub message: String,
    /// Request ID for tracking.
    pub request_id: String,
    /// The transcription ID.
    pub transcription_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    // -- SpeechToTextModelId -------------------------------------------------

    #[test]
    fn model_id_default_is_scribe_v2() {
        assert_eq!(SpeechToTextModelId::default(), SpeechToTextModelId::ScribeV2);
    }

    #[test]
    fn model_id_serde_round_trip() {
        for variant in [SpeechToTextModelId::ScribeV1, SpeechToTextModelId::ScribeV2] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: SpeechToTextModelId = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn model_id_serde_names() {
        assert_eq!(
            serde_json::to_string(&SpeechToTextModelId::ScribeV1).unwrap(),
            r#""scribe_v1""#
        );
        assert_eq!(
            serde_json::to_string(&SpeechToTextModelId::ScribeV2).unwrap(),
            r#""scribe_v2""#
        );
    }

    // -- TimestampsGranularity -----------------------------------------------

    #[test]
    fn timestamps_granularity_default_is_word() {
        assert_eq!(TimestampsGranularity::default(), TimestampsGranularity::Word);
    }

    #[test]
    fn timestamps_granularity_serde_round_trip() {
        for variant in [
            TimestampsGranularity::None,
            TimestampsGranularity::Word,
            TimestampsGranularity::Character,
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: TimestampsGranularity = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn timestamps_granularity_serde_names() {
        assert_eq!(serde_json::to_string(&TimestampsGranularity::None).unwrap(), r#""none""#);
        assert_eq!(serde_json::to_string(&TimestampsGranularity::Word).unwrap(), r#""word""#);
        assert_eq!(
            serde_json::to_string(&TimestampsGranularity::Character).unwrap(),
            r#""character""#
        );
    }

    // -- SpeechToTextFileFormat -----------------------------------------------

    #[test]
    fn stt_file_format_default_is_other() {
        assert_eq!(SpeechToTextFileFormat::default(), SpeechToTextFileFormat::Other);
    }

    #[test]
    fn stt_file_format_serde_round_trip() {
        for variant in [SpeechToTextFileFormat::PcmS16le16, SpeechToTextFileFormat::Other] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: SpeechToTextFileFormat = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    // -- ExportFormat --------------------------------------------------------

    #[test]
    fn export_format_serde_round_trip() {
        for variant in [
            ExportFormat::Srt,
            ExportFormat::Txt,
            ExportFormat::Docx,
            ExportFormat::Pdf,
            ExportFormat::Html,
            ExportFormat::SegmentedJson,
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: ExportFormat = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn export_format_serde_names() {
        assert_eq!(serde_json::to_string(&ExportFormat::Srt).unwrap(), r#""srt""#);
        assert_eq!(
            serde_json::to_string(&ExportFormat::SegmentedJson).unwrap(),
            r#""segmented_json""#
        );
    }

    // -- WordType ------------------------------------------------------------

    #[test]
    fn word_type_serde_round_trip() {
        for variant in [WordType::Word, WordType::Spacing, WordType::AudioEvent] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: WordType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn word_type_serde_names() {
        assert_eq!(serde_json::to_string(&WordType::Word).unwrap(), r#""word""#);
        assert_eq!(serde_json::to_string(&WordType::Spacing).unwrap(), r#""spacing""#);
        assert_eq!(serde_json::to_string(&WordType::AudioEvent).unwrap(), r#""audio_event""#);
    }

    // -- SpeechToTextRequest -------------------------------------------------

    #[test]
    fn stt_request_default_values() {
        let req = SpeechToTextRequest::default();
        assert_eq!(req.model_id, SpeechToTextModelId::ScribeV2);
        assert!(req.tag_audio_events);
        assert!(!req.diarize);
        assert!(!req.webhook);
        assert!(!req.use_multi_channel);
        assert!(req.language_code.is_none());
        assert!(req.num_speakers.is_none());
        assert!(req.keyterms.is_none());
    }

    #[test]
    fn stt_request_minimal_serialization() {
        let req = SpeechToTextRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = v.as_object().unwrap();
        // Non-optional fields should be present.
        assert_eq!(obj["model_id"], "scribe_v2");
        assert_eq!(obj["tag_audio_events"], true);
        assert_eq!(obj["diarize"], false);
        assert_eq!(obj["webhook"], false);
        assert_eq!(obj["use_multi_channel"], false);
        assert_eq!(obj["timestamps_granularity"], "word");
        // Optional fields should be absent.
        assert!(!obj.contains_key("language_code"));
        assert!(!obj.contains_key("num_speakers"));
        assert!(!obj.contains_key("cloud_storage_url"));
        assert!(!obj.contains_key("seed"));
    }

    #[test]
    fn stt_request_full_serialization() {
        let req = SpeechToTextRequest {
            model_id: SpeechToTextModelId::ScribeV1,
            language_code: Some("en".into()),
            tag_audio_events: false,
            num_speakers: Some(3),
            timestamps_granularity: TimestampsGranularity::Character,
            diarize: true,
            diarization_threshold: Some(0.25),
            additional_formats: Some(vec![ExportOptions {
                format: ExportFormat::Srt,
                include_speakers: Some(true),
                include_timestamps: Some(true),
                max_characters_per_line: Some(42),
                segment_on_silence_longer_than_s: None,
            }]),
            file_format: Some(SpeechToTextFileFormat::PcmS16le16),
            cloud_storage_url: Some("https://example.com/audio.mp3".into()),
            webhook: true,
            webhook_id: Some("wh_123".into()),
            temperature: Some(0.5),
            seed: Some(12345),
            use_multi_channel: true,
            webhook_metadata: Some(r#"{"user_id":"123"}"#.into()),
            entity_detection: Some(vec!["pii".into()]),
            keyterms: Some(vec!["hello".into(), "world".into()]),
        };
        let json = serde_json::to_string_pretty(&req).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["model_id"], "scribe_v1");
        assert_eq!(v["language_code"], "en");
        assert_eq!(v["num_speakers"], 3);
        assert_eq!(v["timestamps_granularity"], "character");
        assert_eq!(v["diarize"], true);
        assert_eq!(v["webhook"], true);
        assert_eq!(v["seed"], 12345);
        assert_eq!(v["use_multi_channel"], true);
        // Verify additional_formats array.
        let fmts = v["additional_formats"].as_array().unwrap();
        assert_eq!(fmts.len(), 1);
        assert_eq!(fmts[0]["format"], "srt");
    }

    // -- ExportOptions -------------------------------------------------------

    #[test]
    fn export_options_serialization() {
        let opts = ExportOptions {
            format: ExportFormat::Txt,
            include_speakers: Some(false),
            include_timestamps: None,
            max_characters_per_line: Some(100),
            segment_on_silence_longer_than_s: None,
        };
        let json = serde_json::to_string(&opts).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj["format"], "txt");
        assert_eq!(obj["include_speakers"], false);
        assert!(!obj.contains_key("include_timestamps"));
        assert_eq!(obj["max_characters_per_line"], 100);
        assert!(!obj.contains_key("segment_on_silence_longer_than_s"));
    }

    // -- SpeechToTextCharacter -----------------------------------------------

    #[test]
    fn stt_character_deserialize() {
        let json = r#"{"text": "H", "start": 0.0, "end": 0.1}"#;
        let ch: SpeechToTextCharacter = serde_json::from_str(json).unwrap();
        assert_eq!(ch.text, "H");
        assert!((ch.start.unwrap() - 0.0).abs() < f64::EPSILON);
        assert!((ch.end.unwrap() - 0.1).abs() < f64::EPSILON);
    }

    #[test]
    fn stt_character_null_times() {
        let json = r#"{"text": "H", "start": null, "end": null}"#;
        let ch: SpeechToTextCharacter = serde_json::from_str(json).unwrap();
        assert!(ch.start.is_none());
        assert!(ch.end.is_none());
    }

    #[test]
    fn stt_character_missing_times() {
        let json = r#"{"text": "H"}"#;
        let ch: SpeechToTextCharacter = serde_json::from_str(json).unwrap();
        assert!(ch.start.is_none());
        assert!(ch.end.is_none());
    }

    // -- SpeechToTextWord ----------------------------------------------------

    #[test]
    fn stt_word_deserialize() {
        let json = r#"{
            "text": "Hello",
            "start": 0.0,
            "end": 0.5,
            "type": "word",
            "speaker_id": "speaker_1",
            "logprob": -0.124
        }"#;
        let word: SpeechToTextWord = serde_json::from_str(json).unwrap();
        assert_eq!(word.text, "Hello");
        assert!((word.start.unwrap() - 0.0).abs() < f64::EPSILON);
        assert!((word.end.unwrap() - 0.5).abs() < f64::EPSILON);
        assert_eq!(word.word_type, WordType::Word);
        assert_eq!(word.speaker_id.as_deref(), Some("speaker_1"));
        assert!((word.logprob - (-0.124)).abs() < f64::EPSILON);
        assert!(word.characters.is_none());
    }

    #[test]
    fn stt_word_with_characters() {
        let json = r#"{
            "text": "Hi",
            "start": 0.0,
            "end": 0.3,
            "type": "word",
            "logprob": -0.1,
            "characters": [
                {"text": "H", "start": 0.0, "end": 0.15},
                {"text": "i", "start": 0.15, "end": 0.3}
            ]
        }"#;
        let word: SpeechToTextWord = serde_json::from_str(json).unwrap();
        let chars = word.characters.unwrap();
        assert_eq!(chars.len(), 2);
        assert_eq!(chars[0].text, "H");
        assert_eq!(chars[1].text, "i");
    }

    #[test]
    fn stt_word_spacing() {
        let json = r#"{"text": " ", "start": 0.5, "end": 0.5, "type": "spacing", "logprob": 0.0}"#;
        let word: SpeechToTextWord = serde_json::from_str(json).unwrap();
        assert_eq!(word.word_type, WordType::Spacing);
    }

    #[test]
    fn stt_word_audio_event() {
        let json = r#"{"text": "(laughter)", "start": 1.0, "end": 2.5, "type": "audio_event", "logprob": -0.5}"#;
        let word: SpeechToTextWord = serde_json::from_str(json).unwrap();
        assert_eq!(word.word_type, WordType::AudioEvent);
        assert_eq!(word.text, "(laughter)");
    }

    // -- DetectedEntity ------------------------------------------------------

    #[test]
    fn detected_entity_deserialize() {
        let json = r#"{
            "text": "John Doe",
            "entity_type": "person_name",
            "start_char": 10,
            "end_char": 18
        }"#;
        let entity: DetectedEntity = serde_json::from_str(json).unwrap();
        assert_eq!(entity.text, "John Doe");
        assert_eq!(entity.entity_type, "person_name");
        assert_eq!(entity.start_char, 10);
        assert_eq!(entity.end_char, 18);
    }

    // -- AdditionalFormatResponse --------------------------------------------

    #[test]
    fn additional_format_response_deserialize() {
        let json = r#"{
            "requested_format": "srt",
            "file_extension": "srt",
            "content_type": "text/srt",
            "is_base64_encoded": false,
            "content": "1\n00:00:00,000 --> 00:00:01,000\nHello world!\n"
        }"#;
        let resp: AdditionalFormatResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.requested_format, "srt");
        assert!(!resp.is_base64_encoded);
        assert!(resp.content.contains("Hello world!"));
    }

    // -- SpeechToTextChunkResponse -------------------------------------------

    #[test]
    fn stt_chunk_response_minimal() {
        let json = r#"{
            "language_code": "eng",
            "language_probability": 0.98,
            "text": "Hello world!",
            "words": [
                {"text": "Hello", "start": 0.0, "end": 0.5, "type": "word", "logprob": -0.124},
                {"text": " ", "start": 0.5, "end": 0.5, "type": "spacing", "logprob": 0.0},
                {"text": "world!", "start": 0.5, "end": 1.2, "type": "word", "logprob": -0.089}
            ]
        }"#;
        let resp: SpeechToTextChunkResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.language_code, "eng");
        assert!((resp.language_probability - 0.98).abs() < f64::EPSILON);
        assert_eq!(resp.text, "Hello world!");
        assert_eq!(resp.words.len(), 3);
        assert!(resp.channel_index.is_none());
        assert!(resp.additional_formats.is_none());
        assert!(resp.transcription_id.is_none());
        assert!(resp.entities.is_none());
    }

    #[test]
    fn stt_chunk_response_with_all_optional_fields() {
        let json = r#"{
            "language_code": "eng",
            "language_probability": 0.95,
            "text": "Hello",
            "words": [
                {"text": "Hello", "start": 0.0, "end": 0.5, "type": "word", "logprob": -0.1}
            ],
            "channel_index": 0,
            "additional_formats": [
                {
                    "requested_format": "txt",
                    "file_extension": "txt",
                    "content_type": "text/plain",
                    "is_base64_encoded": false,
                    "content": "Hello"
                }
            ],
            "transcription_id": "tx_abc123",
            "entities": [
                {
                    "text": "Hello",
                    "entity_type": "greeting",
                    "start_char": 0,
                    "end_char": 5
                }
            ]
        }"#;
        let resp: SpeechToTextChunkResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.channel_index, Some(0));
        assert_eq!(resp.transcription_id.as_deref(), Some("tx_abc123"));
        let fmts = resp.additional_formats.unwrap();
        assert_eq!(fmts.len(), 1);
        let entities = resp.entities.unwrap();
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].entity_type, "greeting");
    }

    // -- MultichannelSpeechToTextResponse ------------------------------------

    #[test]
    fn multichannel_response_deserialize() {
        let json = r#"{
            "transcripts": [
                {
                    "language_code": "eng",
                    "language_probability": 0.98,
                    "text": "Hello from channel one.",
                    "words": [
                        {"text": "Hello", "start": 0.0, "end": 0.5, "type": "word", "logprob": -0.124}
                    ]
                },
                {
                    "language_code": "eng",
                    "language_probability": 0.97,
                    "text": "Greetings from channel two.",
                    "words": [
                        {"text": "Greetings", "start": 0.1, "end": 0.7, "type": "word", "logprob": -0.156}
                    ]
                }
            ],
            "transcription_id": "tx_multi_123"
        }"#;
        let resp: MultichannelSpeechToTextResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.transcripts.len(), 2);
        assert_eq!(resp.transcripts[0].text, "Hello from channel one.");
        assert_eq!(resp.transcripts[1].text, "Greetings from channel two.");
        assert_eq!(resp.transcription_id.as_deref(), Some("tx_multi_123"));
    }

    #[test]
    fn multichannel_response_no_transcription_id() {
        let json = r#"{
            "transcripts": [
                {
                    "language_code": "eng",
                    "language_probability": 0.99,
                    "text": "Test",
                    "words": [
                        {"text": "Test", "start": 0.0, "end": 0.3, "type": "word", "logprob": -0.05}
                    ]
                }
            ]
        }"#;
        let resp: MultichannelSpeechToTextResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.transcripts.len(), 1);
        assert!(resp.transcription_id.is_none());
    }

    // -- SpeechToTextWebhookResponse -----------------------------------------

    #[test]
    fn webhook_response_deserialize() {
        let json = r#"{
            "message": "Request accepted. Transcription result will be sent to the webhook endpoint.",
            "request_id": "1234567890"
        }"#;
        let resp: SpeechToTextWebhookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            resp.message,
            "Request accepted. Transcription result will be sent to the webhook endpoint."
        );
        assert_eq!(resp.request_id, "1234567890");
        assert!(resp.transcription_id.is_none());
    }

    #[test]
    fn webhook_response_with_transcription_id() {
        let json = r#"{
            "message": "Accepted",
            "request_id": "req_abc",
            "transcription_id": "tx_xyz"
        }"#;
        let resp: SpeechToTextWebhookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.transcription_id.as_deref(), Some("tx_xyz"));
    }
}
