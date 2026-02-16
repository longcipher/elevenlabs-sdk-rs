//! Common types shared across multiple ElevenLabs API groups.
//!
//! Includes voice settings, output format enums, model metadata,
//! language descriptors, and cursor-based pagination helpers.

use std::fmt;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Voice Settings
// ---------------------------------------------------------------------------

/// Voice generation settings controlling stability, similarity, style, and speed.
///
/// Used by text-to-speech, speech-to-speech, and voice management endpoints.
/// All fields are optional and carry sensible defaults on the server side when
/// omitted.
///
/// # Example
///
/// ```
/// use elevenlabs_sdk::types::VoiceSettings;
///
/// let settings = VoiceSettings {
///     stability: Some(0.5),
///     similarity_boost: Some(0.75),
///     style: Some(0.0),
///     use_speaker_boost: Some(true),
///     speed: Some(1.0),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSettings {
    /// Determines how stable the voice is and the randomness between each
    /// generation. Lower values introduce broader emotional range. Higher
    /// values can result in a monotonous voice. Range: `0.0..=1.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<f64>,

    /// Determines how closely the AI should adhere to the original voice when
    /// attempting to replicate it. Range: `0.0..=1.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_boost: Option<f64>,

    /// Amplifies the style of the original speaker. Consumes additional
    /// compute and may increase latency if set to anything other than `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<f64>,

    /// Boosts similarity to the original speaker at the cost of slightly
    /// higher latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_boost: Option<bool>,

    /// Adjusts the speed of the voice. `1.0` is normal speed; values below
    /// `1.0` slow down, above `1.0` speed up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self {
            stability: Some(0.5),
            similarity_boost: Some(0.75),
            style: Some(0.0),
            use_speaker_boost: Some(true),
            speed: Some(1.0),
        }
    }
}

// ---------------------------------------------------------------------------
// Output Format
// ---------------------------------------------------------------------------

/// Audio output format for text-to-speech and speech-to-speech endpoints.
///
/// Encoded as `codec_sampleRate_bitrate`. For example, `mp3_44100_128`
/// represents MP3 at 44.1 kHz sample rate and 128 kbps bitrate.
///
/// Some formats require higher subscription tiers:
/// - MP3 192 kbps requires **Creator** tier or above.
/// - PCM/WAV at 44.1 kHz requires **Pro** tier or above.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[expect(
    non_camel_case_types,
    reason = "variant names mirror the wire format codec_sampleRate_bitrate"
)]
pub enum OutputFormat {
    // -- MP3 --
    /// MP3 at 22.05 kHz, 32 kbps.
    #[serde(rename = "mp3_22050_32")]
    Mp3_22050_32,
    /// MP3 at 24 kHz, 48 kbps.
    #[serde(rename = "mp3_24000_48")]
    Mp3_24000_48,
    /// MP3 at 44.1 kHz, 32 kbps.
    #[serde(rename = "mp3_44100_32")]
    Mp3_44100_32,
    /// MP3 at 44.1 kHz, 64 kbps.
    #[serde(rename = "mp3_44100_64")]
    Mp3_44100_64,
    /// MP3 at 44.1 kHz, 96 kbps.
    #[serde(rename = "mp3_44100_96")]
    Mp3_44100_96,
    /// MP3 at 44.1 kHz, 128 kbps (default).
    #[serde(rename = "mp3_44100_128")]
    #[default]
    Mp3_44100_128,
    /// MP3 at 44.1 kHz, 192 kbps. Requires Creator tier or above.
    #[serde(rename = "mp3_44100_192")]
    Mp3_44100_192,

    // -- PCM (raw, headerless) --
    /// PCM at 8 kHz.
    #[serde(rename = "pcm_8000")]
    Pcm_8000,
    /// PCM at 16 kHz.
    #[serde(rename = "pcm_16000")]
    Pcm_16000,
    /// PCM at 22.05 kHz.
    #[serde(rename = "pcm_22050")]
    Pcm_22050,
    /// PCM at 24 kHz.
    #[serde(rename = "pcm_24000")]
    Pcm_24000,
    /// PCM at 32 kHz.
    #[serde(rename = "pcm_32000")]
    Pcm_32000,
    /// PCM at 44.1 kHz. Requires Pro tier or above.
    #[serde(rename = "pcm_44100")]
    Pcm_44100,
    /// PCM at 48 kHz.
    #[serde(rename = "pcm_48000")]
    Pcm_48000,

    // -- WAV --
    /// WAV at 8 kHz.
    #[serde(rename = "wav_8000")]
    Wav_8000,
    /// WAV at 16 kHz.
    #[serde(rename = "wav_16000")]
    Wav_16000,
    /// WAV at 22.05 kHz.
    #[serde(rename = "wav_22050")]
    Wav_22050,
    /// WAV at 24 kHz.
    #[serde(rename = "wav_24000")]
    Wav_24000,
    /// WAV at 32 kHz.
    #[serde(rename = "wav_32000")]
    Wav_32000,
    /// WAV at 44.1 kHz. Requires Pro tier or above.
    #[serde(rename = "wav_44100")]
    Wav_44100,
    /// WAV at 48 kHz.
    #[serde(rename = "wav_48000")]
    Wav_48000,

    // -- μ-law --
    /// μ-law at 8 kHz. Commonly used for Twilio audio inputs.
    #[serde(rename = "ulaw_8000")]
    Ulaw_8000,

    // -- A-law --
    /// A-law at 8 kHz.
    #[serde(rename = "alaw_8000")]
    Alaw_8000,

    // -- Opus --
    /// Opus at 48 kHz, 32 kbps.
    #[serde(rename = "opus_48000_32")]
    Opus_48000_32,
    /// Opus at 48 kHz, 64 kbps.
    #[serde(rename = "opus_48000_64")]
    Opus_48000_64,
    /// Opus at 48 kHz, 96 kbps.
    #[serde(rename = "opus_48000_96")]
    Opus_48000_96,
    /// Opus at 48 kHz, 128 kbps.
    #[serde(rename = "opus_48000_128")]
    Opus_48000_128,
    /// Opus at 48 kHz, 192 kbps.
    #[serde(rename = "opus_48000_192")]
    Opus_48000_192,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Serialize to JSON string, strip the surrounding quotes.
        match self {
            Self::Mp3_22050_32 => f.write_str("mp3_22050_32"),
            Self::Mp3_24000_48 => f.write_str("mp3_24000_48"),
            Self::Mp3_44100_32 => f.write_str("mp3_44100_32"),
            Self::Mp3_44100_64 => f.write_str("mp3_44100_64"),
            Self::Mp3_44100_96 => f.write_str("mp3_44100_96"),
            Self::Mp3_44100_128 => f.write_str("mp3_44100_128"),
            Self::Mp3_44100_192 => f.write_str("mp3_44100_192"),
            Self::Pcm_8000 => f.write_str("pcm_8000"),
            Self::Pcm_16000 => f.write_str("pcm_16000"),
            Self::Pcm_22050 => f.write_str("pcm_22050"),
            Self::Pcm_24000 => f.write_str("pcm_24000"),
            Self::Pcm_32000 => f.write_str("pcm_32000"),
            Self::Pcm_44100 => f.write_str("pcm_44100"),
            Self::Pcm_48000 => f.write_str("pcm_48000"),
            Self::Wav_8000 => f.write_str("wav_8000"),
            Self::Wav_16000 => f.write_str("wav_16000"),
            Self::Wav_22050 => f.write_str("wav_22050"),
            Self::Wav_24000 => f.write_str("wav_24000"),
            Self::Wav_32000 => f.write_str("wav_32000"),
            Self::Wav_44100 => f.write_str("wav_44100"),
            Self::Wav_48000 => f.write_str("wav_48000"),
            Self::Ulaw_8000 => f.write_str("ulaw_8000"),
            Self::Alaw_8000 => f.write_str("alaw_8000"),
            Self::Opus_48000_32 => f.write_str("opus_48000_32"),
            Self::Opus_48000_64 => f.write_str("opus_48000_64"),
            Self::Opus_48000_96 => f.write_str("opus_48000_96"),
            Self::Opus_48000_128 => f.write_str("opus_48000_128"),
            Self::Opus_48000_192 => f.write_str("opus_48000_192"),
        }
    }
}

// ---------------------------------------------------------------------------
// Language
// ---------------------------------------------------------------------------

/// A language supported by an ElevenLabs model.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Language {
    /// ISO-style language identifier (e.g. `"en"`, `"es"`).
    pub language_id: String,
    /// Human-readable language name (e.g. `"English"`).
    pub name: String,
}

// ---------------------------------------------------------------------------
// Model Rates
// ---------------------------------------------------------------------------

/// Billing rates for a model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelRates {
    /// Multiplier applied to the base character cost.
    pub character_cost_multiplier: f64,
}

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// Metadata for an ElevenLabs model.
///
/// Returned by the `GET /v1/models` endpoint. Each model describes its
/// capabilities (TTS, voice conversion, style, speaker boost) and the
/// languages it supports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Model {
    /// Unique model identifier (e.g. `"eleven_multilingual_v2"`).
    pub model_id: String,
    /// Human-readable model name.
    pub name: String,
    /// Whether the model can be finetuned with custom voice data.
    pub can_be_finetuned: bool,
    /// Whether the model supports text-to-speech.
    pub can_do_text_to_speech: bool,
    /// Whether the model supports voice conversion (speech-to-speech).
    pub can_do_voice_conversion: bool,
    /// Whether the model supports the style parameter.
    pub can_use_style: bool,
    /// Whether the model supports the speaker boost parameter.
    pub can_use_speaker_boost: bool,
    /// Whether the model serves professional voices.
    pub serves_pro_voices: bool,
    /// Cost factor relative to the base model pricing.
    pub token_cost_factor: f64,
    /// Human-readable description of the model.
    pub description: String,
    /// Whether the model requires alpha access to use.
    pub requires_alpha_access: bool,
    /// Maximum characters per request for free-tier users.
    pub max_characters_request_free_user: i64,
    /// Maximum characters per request for subscribed users.
    pub max_characters_request_subscribed_user: i64,
    /// Absolute maximum text length that can be sent in a single request.
    pub maximum_text_length_per_request: i64,
    /// Languages this model supports.
    pub languages: Vec<Language>,
    /// Billing rates for this model.
    pub model_rates: ModelRates,
    /// Concurrency group this model belongs to.
    pub concurrency_group: String,
}

// ---------------------------------------------------------------------------
// Subscription
// ---------------------------------------------------------------------------

/// Status of a user's subscription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    /// Trial period.
    Trialing,
    /// Active paid subscription.
    Active,
    /// Payment incomplete.
    Incomplete,
    /// Payment past due.
    PastDue,
    /// Free tier.
    Free,
    /// Free tier, disabled.
    FreeDisabled,
}

/// Billing period for a subscription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BillingPeriod {
    /// Monthly billing.
    #[serde(rename = "monthly_period")]
    Monthly,
    /// Quarterly (3-month) billing.
    #[serde(rename = "3_month_period")]
    ThreeMonth,
    /// Semi-annual (6-month) billing.
    #[serde(rename = "6_month_period")]
    SixMonth,
    /// Annual billing.
    #[serde(rename = "annual_period")]
    Annual,
}

/// Subscription currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Currency {
    /// US dollars.
    Usd,
    /// Euros.
    Eur,
    /// Indian rupees.
    Inr,
}

/// User subscription details returned by the API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subscription {
    /// Subscription tier name (e.g. `"trial"`, `"creator"`).
    pub tier: String,
    /// Number of characters used in the current billing period.
    pub character_count: i64,
    /// Maximum characters allowed in the current billing period.
    pub character_limit: i64,
    /// Maximum additional characters the limit can be extended by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_character_limit_extension: Option<i64>,
    /// Whether the user can extend their character limit.
    pub can_extend_character_limit: bool,
    /// Whether the user is allowed to extend their character limit.
    pub allowed_to_extend_character_limit: bool,
    /// Unix timestamp of next character count reset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_character_count_reset_unix: Option<i64>,
    /// Number of voice slots in use.
    pub voice_slots_used: i64,
    /// Number of professional voice slots in use.
    pub professional_voice_slots_used: i64,
    /// Maximum number of voice slots allowed.
    pub voice_limit: i64,
    /// Maximum voice add/edit operations allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_voice_add_edits: Option<i64>,
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
    /// Currency of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Current subscription status.
    pub status: SubscriptionStatus,
    /// Billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<BillingPeriod>,
    /// Character refresh period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_refresh_period: Option<BillingPeriod>,
}

// ---------------------------------------------------------------------------
// Voice Category
// ---------------------------------------------------------------------------

/// Category of a voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceCategory {
    /// AI-generated voice.
    Generated,
    /// Cloned from audio samples.
    Cloned,
    /// Pre-made voice from the ElevenLabs library.
    Premade,
    /// Professional voice clone.
    Professional,
    /// Celebrity / famous voice.
    Famous,
    /// High-quality voice.
    HighQuality,
}

// ---------------------------------------------------------------------------
// Safety Control
// ---------------------------------------------------------------------------

/// Safety control level applied to a voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SafetyControl {
    /// No safety restrictions.
    None,
    /// Voice is banned.
    Ban,
    /// Captcha required before use.
    Captcha,
    /// Banned for enterprise users.
    EnterpriseBan,
    /// Captcha required for enterprise users.
    EnterpriseCaptcha,
}

// ---------------------------------------------------------------------------
// Verified Voice Language
// ---------------------------------------------------------------------------

/// A language that has been verified for a voice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedVoiceLanguage {
    /// ISO language code (e.g. `"en"`).
    pub language: String,
    /// Model ID this verification applies to.
    pub model_id: String,
    /// Optional accent descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    /// Optional locale descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Optional preview audio URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
}

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

/// Cursor-based pagination parameters common across list endpoints.
///
/// Most ElevenLabs list endpoints accept a `page_size` (max items) and an
/// opaque `cursor` string obtained from a previous response.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPageParams {
    /// Maximum number of items to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// Opaque cursor returned from a previous response for fetching the
    /// next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Pagination metadata returned alongside paged results.
///
/// Typical usage: check `has_more`; if true, pass `next_cursor` into the
/// next request's [`CursorPageParams::cursor`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageInfo {
    /// Whether additional pages are available.
    pub has_more: bool,
    /// Opaque cursor to fetch the next page, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

// ---------------------------------------------------------------------------
// Type aliases for readability
// ---------------------------------------------------------------------------

/// Opaque voice identifier.
pub type VoiceId = String;

/// Opaque model identifier (e.g. `"eleven_multilingual_v2"`).
pub type ModelId = String;

/// ISO language code (e.g. `"en"`, `"ja"`).
pub type LanguageCode = String;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests use unwrap")]
mod tests {
    use super::*;

    /// Round-trip: serialize then deserialize, assert equality.
    fn round_trip<T>(value: &T)
    where
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + fmt::Debug,
    {
        let json = serde_json::to_string(value).unwrap();
        let back: T = serde_json::from_str(&json).unwrap();
        assert_eq!(*value, back, "round-trip failed for {json}");
    }

    // -- VoiceSettings -------------------------------------------------------

    #[test]
    fn voice_settings_round_trip() {
        let settings = VoiceSettings {
            stability: Some(0.5),
            similarity_boost: Some(0.75),
            style: Some(0.0),
            use_speaker_boost: Some(true),
            speed: Some(1.0),
        };
        round_trip(&settings);
    }

    #[test]
    fn voice_settings_default_round_trip() {
        round_trip(&VoiceSettings::default());
    }

    #[test]
    fn voice_settings_omits_none_fields() {
        let settings = VoiceSettings {
            stability: Some(0.5),
            similarity_boost: None,
            style: None,
            use_speaker_boost: None,
            speed: None,
        };
        let json = serde_json::to_string(&settings).unwrap();
        assert!(!json.contains("similarity_boost"));
        assert!(!json.contains("style"));
        assert!(!json.contains("use_speaker_boost"));
        assert!(!json.contains("speed"));
    }

    #[test]
    fn voice_settings_deserialize_from_api_example() {
        let json = r#"{
            "stability": 1.0,
            "similarity_boost": 1.0,
            "style": 0.0,
            "use_speaker_boost": true,
            "speed": 1.0
        }"#;
        let settings: VoiceSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.stability, Some(1.0));
        assert_eq!(settings.use_speaker_boost, Some(true));
    }

    // -- OutputFormat --------------------------------------------------------

    #[test]
    fn output_format_round_trip_all_variants() {
        let variants = [
            OutputFormat::Mp3_22050_32,
            OutputFormat::Mp3_24000_48,
            OutputFormat::Mp3_44100_32,
            OutputFormat::Mp3_44100_64,
            OutputFormat::Mp3_44100_96,
            OutputFormat::Mp3_44100_128,
            OutputFormat::Mp3_44100_192,
            OutputFormat::Pcm_8000,
            OutputFormat::Pcm_16000,
            OutputFormat::Pcm_22050,
            OutputFormat::Pcm_24000,
            OutputFormat::Pcm_32000,
            OutputFormat::Pcm_44100,
            OutputFormat::Pcm_48000,
            OutputFormat::Wav_8000,
            OutputFormat::Wav_16000,
            OutputFormat::Wav_22050,
            OutputFormat::Wav_24000,
            OutputFormat::Wav_32000,
            OutputFormat::Wav_44100,
            OutputFormat::Wav_48000,
            OutputFormat::Ulaw_8000,
            OutputFormat::Alaw_8000,
            OutputFormat::Opus_48000_32,
            OutputFormat::Opus_48000_64,
            OutputFormat::Opus_48000_96,
            OutputFormat::Opus_48000_128,
            OutputFormat::Opus_48000_192,
        ];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn output_format_deserialize_from_string() {
        let val: OutputFormat = serde_json::from_str(r#""mp3_44100_128""#).unwrap();
        assert_eq!(val, OutputFormat::Mp3_44100_128);
    }

    #[test]
    fn output_format_default() {
        assert_eq!(OutputFormat::default(), OutputFormat::Mp3_44100_128);
    }

    #[test]
    fn output_format_display() {
        assert_eq!(OutputFormat::Mp3_44100_128.to_string(), "mp3_44100_128");
        assert_eq!(OutputFormat::Ulaw_8000.to_string(), "ulaw_8000");
        assert_eq!(OutputFormat::Opus_48000_192.to_string(), "opus_48000_192");
    }

    // -- Language ------------------------------------------------------------

    #[test]
    fn language_round_trip() {
        let lang = Language { language_id: "en".into(), name: "English".into() };
        round_trip(&lang);
    }

    // -- Model ---------------------------------------------------------------

    #[test]
    fn model_deserialize_from_api_example() {
        let json = r#"{
            "model_id": "eleven_multilingual_v2",
            "name": "Eleven Multilingual v2",
            "can_be_finetuned": true,
            "can_do_text_to_speech": true,
            "can_do_voice_conversion": true,
            "can_use_style": true,
            "can_use_speaker_boost": true,
            "serves_pro_voices": false,
            "token_cost_factor": 1.0,
            "description": "Our state of the art multilingual speech synthesis model.",
            "requires_alpha_access": false,
            "max_characters_request_free_user": 2500,
            "max_characters_request_subscribed_user": 5000,
            "maximum_text_length_per_request": 1000000,
            "languages": [
                { "language_id": "en", "name": "English" }
            ],
            "model_rates": { "character_cost_multiplier": 1.0 },
            "concurrency_group": "standard_eleven_multilingual_v2"
        }"#;
        let model: Model = serde_json::from_str(json).unwrap();
        assert_eq!(model.model_id, "eleven_multilingual_v2");
        assert_eq!(model.languages.len(), 1);
        assert!(model.can_do_text_to_speech);
        round_trip(&model);
    }

    // -- Subscription --------------------------------------------------------

    #[test]
    fn subscription_status_round_trip() {
        let variants = [
            SubscriptionStatus::Trialing,
            SubscriptionStatus::Active,
            SubscriptionStatus::Incomplete,
            SubscriptionStatus::PastDue,
            SubscriptionStatus::Free,
            SubscriptionStatus::FreeDisabled,
        ];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn subscription_status_serde_names() {
        let json = serde_json::to_string(&SubscriptionStatus::PastDue).unwrap();
        assert_eq!(json, r#""past_due""#);
        let json = serde_json::to_string(&SubscriptionStatus::FreeDisabled).unwrap();
        assert_eq!(json, r#""free_disabled""#);
    }

    #[test]
    fn billing_period_round_trip() {
        let variants = [
            BillingPeriod::Monthly,
            BillingPeriod::ThreeMonth,
            BillingPeriod::SixMonth,
            BillingPeriod::Annual,
        ];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn billing_period_serde_names() {
        let json = serde_json::to_string(&BillingPeriod::ThreeMonth).unwrap();
        assert_eq!(json, r#""3_month_period""#);
    }

    #[test]
    fn subscription_deserialize_from_api_example() {
        let json = r#"{
            "tier": "trial",
            "character_count": 17231,
            "character_limit": 100000,
            "max_character_limit_extension": 10000,
            "can_extend_character_limit": false,
            "allowed_to_extend_character_limit": false,
            "voice_slots_used": 1,
            "professional_voice_slots_used": 0,
            "voice_limit": 120,
            "max_voice_add_edits": 230,
            "voice_add_edit_counter": 212,
            "professional_voice_limit": 1,
            "can_extend_voice_limit": false,
            "can_use_instant_voice_cloning": true,
            "can_use_professional_voice_cloning": true,
            "currency": "usd",
            "status": "free",
            "billing_period": "monthly_period",
            "character_refresh_period": "monthly_period"
        }"#;
        let sub: Subscription = serde_json::from_str(json).unwrap();
        assert_eq!(sub.tier, "trial");
        assert_eq!(sub.status, SubscriptionStatus::Free);
        assert_eq!(sub.billing_period, Some(BillingPeriod::Monthly));
        round_trip(&sub);
    }

    // -- VoiceCategory -------------------------------------------------------

    #[test]
    fn voice_category_round_trip() {
        let variants = [
            VoiceCategory::Generated,
            VoiceCategory::Cloned,
            VoiceCategory::Premade,
            VoiceCategory::Professional,
            VoiceCategory::Famous,
            VoiceCategory::HighQuality,
        ];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn voice_category_serde_names() {
        let json = serde_json::to_string(&VoiceCategory::HighQuality).unwrap();
        assert_eq!(json, r#""high_quality""#);
    }

    // -- SafetyControl -------------------------------------------------------

    #[test]
    fn safety_control_round_trip() {
        let variants = [
            SafetyControl::None,
            SafetyControl::Ban,
            SafetyControl::Captcha,
            SafetyControl::EnterpriseBan,
            SafetyControl::EnterpriseCaptcha,
        ];
        for v in &variants {
            round_trip(v);
        }
    }

    #[test]
    fn safety_control_serde_names() {
        let json = serde_json::to_string(&SafetyControl::EnterpriseBan).unwrap();
        assert_eq!(json, r#""ENTERPRISE_BAN""#);
    }

    // -- VerifiedVoiceLanguage -----------------------------------------------

    #[test]
    fn verified_voice_language_round_trip() {
        let vvl = VerifiedVoiceLanguage {
            language: "en".into(),
            model_id: "eleven_turbo_v2_5".into(),
            accent: Some("American".into()),
            locale: None,
            preview_url: None,
        };
        round_trip(&vvl);
    }

    // -- CursorPageParams ----------------------------------------------------

    #[test]
    fn cursor_page_params_round_trip() {
        let params = CursorPageParams { page_size: Some(30), cursor: Some("abc123".into()) };
        round_trip(&params);
    }

    #[test]
    fn cursor_page_params_default_is_empty() {
        let params = CursorPageParams::default();
        let json = serde_json::to_string(&params).unwrap();
        assert_eq!(json, "{}");
    }

    // -- PageInfo ------------------------------------------------------------

    #[test]
    fn page_info_round_trip() {
        let info = PageInfo { has_more: true, next_cursor: Some("next123".into()) };
        round_trip(&info);

        let info_done = PageInfo { has_more: false, next_cursor: None };
        round_trip(&info_done);
    }
}
