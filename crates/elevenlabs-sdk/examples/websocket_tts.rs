//! Real-time TTS via WebSocket.
//!
//! Sends text chunks through the input-streaming WebSocket endpoint and
//! decodes the base64 audio responses into a file.
//!
//! Usage:
//!
//! ```sh
//! ELEVENLABS_API_KEY=... cargo run -p elevenlabs-sdk --example websocket_tts
//! ```

use std::{fs::File, io::Write};

use elevenlabs_sdk::{ClientConfig, ElevenLabsClient, TtsWebSocket, TtsWsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build client from the ELEVENLABS_API_KEY environment variable.
    let config = ClientConfig::from_env()?;
    // Verify the API key is valid by constructing the client (not strictly
    // required for WebSocket, but good practice).
    let _client = ElevenLabsClient::new(config.clone())?;

    let ws_config = TtsWsConfig {
        voice_id: "21m00Tcm4TlvDq8ikWAM".into(),
        model_id: "eleven_turbo_v2".into(),
        voice_settings: None,
        generation_config: None,
        output_format: None,
    };

    println!("Connecting to TTS WebSocket...");
    let mut ws = TtsWebSocket::connect(&config, &ws_config).await?;

    // Send text in chunks to demonstrate incremental streaming.
    let chunks = ["Hello! ", "This is a real-time ", "text-to-speech demo ", "using WebSockets."];

    for chunk in &chunks {
        println!("  Sending: {chunk:?}");
        ws.send_text(chunk).await?;
    }

    // Flush to ensure all buffered text is synthesised.
    println!("Flushing...");
    ws.flush().await?;

    let output_path = "output_ws.mp3";
    let mut file = File::create(output_path)?;
    let mut total_bytes: usize = 0;

    // Receive audio responses until the final marker.
    while let Some(resp) = ws.recv().await? {
        if let Some(ref audio_b64) = resp.audio {
            use base64::Engine;
            let decoded = base64::engine::general_purpose::STANDARD.decode(audio_b64)?;
            file.write_all(&decoded)?;
            total_bytes += decoded.len();
            println!("  Received audio chunk: {} bytes", decoded.len());
        }

        if resp.is_final == Some(true) {
            println!("Received final marker.");
            break;
        }
    }

    // Close the WebSocket cleanly.
    ws.close().await?;

    println!("Done! Saved {total_bytes} bytes to {output_path}");

    Ok(())
}
