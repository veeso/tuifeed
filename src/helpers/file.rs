//! # File
//!
//! File helpers

/**
 * MIT License
 *
 * tuifeed - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::fs::File;
use std::io::{Error as IoError, Read, Write};
use std::path::Path;

/// ### open_file_read
///
/// Open file at `p` for read
pub fn open_file_read(p: &Path) -> Result<impl Read, IoError> {
    File::open(p)
}

/// ### write_file
///
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
