//! The Grimoire (File System I/O)
//!
//! A robust, memory-safe interface for ingesting physical MagickScript (.ms) 
//! files from the storage layer. Ensures entropic anomalies like UTF-8 BOMs 
//! are stripped before passing the payload to the Lexer.

use std::fs;
use std::io;

/// Loads a physical MagickScript from disk into an immutable memory string.
/// Degrades gracefully if the script does not exist in the physical plane.
pub fn load_script(file_path: &str) -> Result<String, io::Error> {
    let raw_payload = fs::read_to_string(file_path)?;

    // Eradicate entropic noise: Strip the UTF-8 Byte Order Mark (BOM) if present.
    let sanitized_payload = if let Some(stripped) = raw_payload.strip_prefix('\u{FEFF}') {
        stripped.to_string()
    } else {
        raw_payload
    };

    Ok(sanitized_payload)
}
