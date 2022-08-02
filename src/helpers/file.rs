//! # File
//!
//! File helpers

use std::fs::File;
use std::io::{Error as IoError, Read, Write};
use std::path::Path;

/// Open file at `p` for read
pub fn open_file_read(p: &Path) -> Result<impl Read, IoError> {
    File::open(p)
}

/// Write `content` to file located at `p`
pub fn write_file(p: &Path, content: &str) -> Result<(), IoError> {
    File::create(p)?.write_all(content.as_bytes())
}

#[cfg(test)]
mod test {

    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn should_write_file() {
        let temp = NamedTempFile::new().ok().unwrap();
        assert!(write_file(temp.path(), "Hello world!\n").is_ok());
    }

    #[test]
    fn should_open_file_for_read() {
        let temp = NamedTempFile::new().ok().unwrap();
        assert!(write_file(temp.path(), "Hello world!\n").is_ok());
        assert!(open_file_read(temp.path()).is_ok());
    }
}
