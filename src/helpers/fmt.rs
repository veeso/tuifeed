//! # Fmt
//!
//! Tuifeed formatting helpers

use chrono::{DateTime, Local};

/// Format datetime according to provided format
pub fn format_datetime(datetime: DateTime<Local>, fmt: &str) -> String {
    datetime.format(fmt).to_string()
}

#[cfg(test)]
mod test {

    use std::ops::Add;
    use std::time::{Duration, SystemTime};

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_format_datetime() {
        let datetime: DateTime<Local> = SystemTime::UNIX_EPOCH
            .add(Duration::from_secs(36000))
            .into();
        assert_eq!(format_datetime(datetime, "%Y-%m-%d"), "1970-01-01");
    }
}
