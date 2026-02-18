//! ElevenLabs CLI — command-line interface for the ElevenLabs API.
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod cli;
mod commands;
mod context;
mod output;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose {
        tracing::level_filters::LevelFilter::DEBUG
    } else {
        tracing::level_filters::LevelFilter::INFO
    };
    tracing_subscriber::fmt().with_max_level(filter).with_writer(std::io::stderr).init();

    tracing::debug!(?cli, "parsed CLI arguments");

    match &cli.command {
        Some(cmd) => match cmd {
            cli::Commands::Tts(args) => commands::tts::execute(args, &cli).await?,
            cli::Commands::Voices(args) => commands::voices::execute(args, &cli).await?,
            cli::Commands::Models(args) => commands::models::execute(args, &cli).await?,
            cli::Commands::User(args) => commands::user::execute(args, &cli).await?,
            cli::Commands::Workspace(args) => commands::workspace::execute(args, &cli).await?,
            cli::Commands::Agents(args) => commands::agents::execute(args, &cli).await?,
            cli::Commands::AudioIsolation(args) => {
                commands::audio_isolation::execute(args, &cli).await?;
            }
            cli::Commands::AudioNative(args) => {
                commands::audio_native::execute(args, &cli).await?;
            }
            cli::Commands::Dubbing(args) => commands::dubbing::execute(args, &cli).await?,
            cli::Commands::ForcedAlignment(args) => {
                commands::forced_alignment::execute(args, &cli).await?;
            }
            cli::Commands::History(args) => commands::history::execute(args, &cli).await?,
            cli::Commands::Music(args) => commands::music::execute(args, &cli).await?,
            cli::Commands::PvcVoices(args) => commands::pvc_voices::execute(args, &cli).await?,
            cli::Commands::SingleUseToken(args) => {
                commands::single_use_token::execute(args, &cli).await?;
            }
            cli::Commands::SoundGeneration(args) => {
                commands::sound_generation::execute(args, &cli).await?;
            }
            cli::Commands::SpeechToSpeech(args) => {
                commands::speech_to_speech::execute(args, &cli).await?;
            }
            cli::Commands::SpeechToText(args) => {
                commands::speech_to_text::execute(args, &cli).await?;
            }
            cli::Commands::Studio(args) => commands::studio::execute(args, &cli).await?,
            cli::Commands::TextToDialogue(args) => {
                commands::text_to_dialogue::execute(args, &cli).await?;
            }
            cli::Commands::TextToVoice(args) => {
                commands::text_to_voice::execute(args, &cli).await?;
            }
            cli::Commands::VoiceGeneration(args) => {
                commands::voice_generation::execute(args, &cli).await?;
            }
            cli::Commands::Ws(args) => commands::ws::execute(args, &cli).await?,
        },
        None => {
            eprintln!("elevenlabs-bin-cli — use --help for usage information");
        }
    }

    Ok(())
}
