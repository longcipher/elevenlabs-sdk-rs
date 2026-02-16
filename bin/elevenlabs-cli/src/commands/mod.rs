//! CLI subcommand implementations.
//!
//! Each subcommand gets its own module. Modules are added here as commands are
//! implemented.

pub(crate) mod agents;
pub(crate) mod audio_isolation;
pub(crate) mod audio_native;
pub(crate) mod dubbing;
pub(crate) mod forced_alignment;
pub(crate) mod history;
pub(crate) mod models;
pub(crate) mod music;
pub(crate) mod pvc_voices;
pub(crate) mod single_use_token;
pub(crate) mod sound_generation;
pub(crate) mod speech_to_speech;
pub(crate) mod speech_to_text;
pub(crate) mod studio;
pub(crate) mod text_to_dialogue;
pub(crate) mod text_to_voice;
pub(crate) mod tts;
pub(crate) mod user;
pub(crate) mod voice_generation;
pub(crate) mod voices;
pub(crate) mod workspace;
pub(crate) mod ws;
