//! Streaming TTS example.
//!
//! Converts text to speech using the streaming endpoint, writing audio chunks
//! to a file as they arrive.
//!
//! Usage:
//!
//! ```sh
//! ELEVENLABS_API_KEY=... cargo run -p elevenlabs-sdk --example streaming
//! ```

use std::{fs::File, io::Write};

use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, types::TextToSpeechRequest};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build client from the ELEVENLABS_API_KEY environment variable.
    let config = ClientConfig::from_env()?;
    let client = ElevenLabsClient::new(config)?;

    let voice_id = "21m00Tcm4TlvDq8ikWAM";

    let request = TextToSpeechRequest::new(
        "This is a streaming example. The audio is received in chunks \
         and written to a file incrementally.",
    );

    println!("Starting streaming TTS with voice {voice_id}...");

    let tts = client.text_to_speech();
    let mut stream = tts.convert_stream(voice_id, &request, None, None).await?;

    let output_path = "output_stream.mp3";
    let mut file = File::create(output_path)?;
    let mut total_bytes: usize = 0;
    let mut chunk_count: usize = 0;

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk)?;
        total_bytes += chunk.len();
        chunk_count += 1;
        println!("  Received chunk {chunk_count}: {} bytes", chunk.len());
    }

    println!("\nDone! Saved {total_bytes} bytes ({chunk_count} chunks) to {output_path}");

    Ok(())
}
