//! # Open
//!
//! Open helpers

use open;
use std::path::Path;

/// ### open_text_file
///
/// Open text file
pub fn open_text_file(p: &Path) -> Result<(), String> {
    if !p.exists() {
        return Err(format!("{}: No such file or directory", p.display()));
    }
    open::that(p).map_err(|e| e.to_string())
}

/// ### open_link
///
/// Open link
pub fn open_link(link: &str) -> Result<(), String> {
    open::that(link).map_err(|e| e.to_string())
}
