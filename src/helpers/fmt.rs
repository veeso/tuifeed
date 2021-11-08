//! # Fmt
//!
//! Tuifeed formatting helpers

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
use chrono::{DateTime, Local};

/// ### format_datetime
///
/// Format datetime according to provided format
pub fn format_datetime(datetime: DateTime<Local>, fmt: &str) -> String {
    datetime.format(fmt).to_string()
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::{
        ops::Add,
        time::{Duration, SystemTime},
    };

    #[test]
    fn should_format_datetime() {
        let datetime: DateTime<Local> = SystemTime::from(SystemTime::UNIX_EPOCH)
            .add(Duration::from_secs(36000))
            .into();
        assert_eq!(format_datetime(datetime, "%Y-%m-%d"), "1970-01-01");
    }
}
