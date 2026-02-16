//! Output formatting helpers for the CLI.

use serde::Serialize;

/// Controls how CLI output is rendered.
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub(crate) enum OutputFormat {
    /// Compact JSON (single line).
    Json,
    /// Pretty-printed JSON (indented).
    #[default]
    Pretty,
}

/// Print a serialisable value to stdout in the requested format.
///
/// # Errors
///
/// Returns an error if JSON serialisation fails.
pub(crate) fn print_json<T: Serialize>(value: &T, format: OutputFormat) -> eyre::Result<()> {
    let output = match format {
        OutputFormat::Json => serde_json::to_string(value)?,
        OutputFormat::Pretty => serde_json::to_string_pretty(value)?,
    };
    println!("{output}");
    Ok(())
}
