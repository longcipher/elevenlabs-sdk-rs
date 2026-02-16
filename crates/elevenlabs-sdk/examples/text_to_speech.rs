//! Basic text-to-speech example.
//!
//! Converts a text string to speech and saves the audio to a file.
//!
//! Usage:
//!
//! ```sh
//! ELEVENLABS_API_KEY=... cargo run -p elevenlabs-sdk --example text_to_speech
//! ```

use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, types::TextToSpeechRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build client from the ELEVENLABS_API_KEY environment variable.
    let config = ClientConfig::from_env()?;
    let client = ElevenLabsClient::new(config)?;

    // Use a well-known default voice (Rachel).
    let voice_id = "21m00Tcm4TlvDq8ikWAM";

    let request = TextToSpeechRequest::new("Hello! This is a test of the ElevenLabs Rust SDK.");

    println!("Converting text to speech with voice {voice_id}...");

    let audio = client.text_to_speech().convert(voice_id, &request, None, None).await?;

    let output_path = "output.mp3";
    std::fs::write(output_path, &audio)?;

    println!("Saved {} bytes of audio to {output_path}", audio.len());

    Ok(())
}
