//! List available voices example.
//!
//! Fetches and prints all voices accessible with the configured API key.
//!
//! Usage:
//!
//! ```sh
//! ELEVENLABS_API_KEY=... cargo run -p elevenlabs-sdk --example voices
//! ```

use elevenlabs_sdk::{ClientConfig, ElevenLabsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build client from the ELEVENLABS_API_KEY environment variable.
    let config = ClientConfig::from_env()?;
    let client = ElevenLabsClient::new(config)?;

    println!("Fetching available voices...\n");

    let response = client.voices().list(None).await?;

    println!("Found {} voices:\n", response.voices.len());

    for voice in &response.voices {
        println!("  Name:     {}", voice.name);
        println!("  ID:       {}", voice.voice_id);
        println!("  Category: {:?}", voice.category);
        if let Some(ref desc) = voice.description {
            println!("  Desc:     {desc}");
        }
        println!();
    }

    Ok(())
}
