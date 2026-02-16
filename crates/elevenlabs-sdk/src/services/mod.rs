//! Service modules providing typed access to ElevenLabs API endpoints.
//!
//! Each service groups related endpoints (e.g., text-to-speech, voices) and
//! is accessed via a corresponding method on [`crate::client::ElevenLabsClient`].

pub mod agents;
pub mod audio_isolation;
pub mod audio_native;
pub mod dubbing;
pub mod forced_alignment;
pub mod history;
pub mod models;
pub mod music;
pub mod pvc_voices;
pub mod single_use_token;
pub mod sound_generation;
pub mod speech_to_speech;
pub mod speech_to_text;
pub mod studio;
pub mod text_to_dialogue;
pub mod text_to_speech;
pub mod text_to_voice;
pub mod user;
pub mod voice_generation;
pub mod voices;
pub mod workspace;

pub use agents::AgentsService;
pub use audio_isolation::AudioIsolationService;
pub use audio_native::AudioNativeService;
pub use dubbing::DubbingService;
pub use forced_alignment::ForcedAlignmentService;
pub use history::HistoryService;
pub use models::ModelsService;
pub use music::MusicService;
pub use pvc_voices::PvcVoicesService;
pub use single_use_token::SingleUseTokenService;
pub use sound_generation::SoundGenerationService;
pub use speech_to_speech::SpeechToSpeechService;
pub use speech_to_text::SpeechToTextService;
pub use studio::StudioService;
pub use text_to_dialogue::TextToDialogueService;
pub use text_to_speech::TextToSpeechService;
pub use text_to_voice::TextToVoiceService;
pub use user::UserService;
pub use voice_generation::VoiceGenerationService;
pub use voices::VoicesService;
pub use workspace::WorkspaceService;
