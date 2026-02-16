//! Top-level CLI argument definitions.

use clap::{Parser, Subcommand};

use crate::{
    commands::{
        agents, audio_isolation, audio_native, dubbing, forced_alignment, history, models, music,
        pvc_voices, single_use_token, sound_generation, speech_to_speech, speech_to_text, studio,
        text_to_dialogue, text_to_voice, tts, user, voice_generation, voices, workspace, ws,
    },
    output::OutputFormat,
};

/// ElevenLabs CLI — interact with the ElevenLabs API from the command line.
#[derive(Debug, Parser)]
#[command(name = "elevenlabs", version, about, long_about = None)]
pub(crate) struct Cli {
    /// ElevenLabs API key (can also be set via environment variable).
    #[arg(long, env = "ELEVENLABS_API_KEY", hide_env_values = true, global = true)]
    pub api_key: Option<String>,

    /// Base URL for the ElevenLabs API.
    #[arg(long, env = "ELEVENLABS_BASE_URL", global = true)]
    pub base_url: Option<String>,

    /// Output format.
    #[arg(long, default_value = "pretty", global = true)]
    pub format: OutputFormat,

    /// Enable verbose (debug) logging.
    #[arg(long, short, global = true)]
    pub verbose: bool,

    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available CLI subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Text-to-speech operations.
    Tts(tts::TtsArgs),

    /// List and manage voices.
    Voices(voices::VoicesArgs),

    /// List available models.
    Models(models::ModelsArgs),

    /// User account information.
    User(user::UserArgs),

    /// Workspace management.
    Workspace(workspace::WorkspaceArgs),

    /// Conversational AI agents.
    Agents(agents::AgentsArgs),

    /// Isolate audio from background noise.
    AudioIsolation(audio_isolation::AudioIsolationArgs),

    /// Audio native project operations.
    AudioNative(audio_native::AudioNativeArgs),

    /// Dubbing operations.
    Dubbing(dubbing::DubbingArgs),

    /// Forced alignment operations.
    ForcedAlignment(forced_alignment::ForcedAlignmentArgs),

    /// History of generated audio.
    History(history::HistoryArgs),

    /// Music generation.
    Music(music::MusicArgs),

    /// Professional voice cloning.
    PvcVoices(pvc_voices::PvcVoicesArgs),

    /// Single-use token management.
    SingleUseToken(single_use_token::SingleUseTokenArgs),

    /// Sound effect generation.
    SoundGeneration(sound_generation::SoundGenerationArgs),

    /// Speech-to-speech conversion.
    SpeechToSpeech(speech_to_speech::SpeechToSpeechArgs),

    /// Speech-to-text transcription.
    SpeechToText(speech_to_text::SpeechToTextArgs),

    /// Studio project management.
    Studio(studio::StudioArgs),

    /// Text-to-dialogue conversion.
    TextToDialogue(text_to_dialogue::TextToDialogueArgs),

    /// Text-to-voice generation.
    TextToVoice(text_to_voice::TextToVoiceArgs),

    /// Voice generation.
    VoiceGeneration(voice_generation::VoiceGenerationArgs),

    /// WebSocket operations (TTS streaming, Conversational AI).
    Ws(ws::WsArgs),
}
