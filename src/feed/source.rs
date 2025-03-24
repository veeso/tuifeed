use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

const FILE_PREFIX: &str = "file://";

/// A source for a feed
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeedSource {
    File(PathBuf),
    Http(String),
}

impl fmt::Display for FeedSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeedSource::File(p) => write!(f, "{FILE_PREFIX}{}", p.display()),
            FeedSource::Http(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for FeedSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("http://") || s.starts_with("https://") {
            return Ok(FeedSource::Http(s.to_string()));
        }

        if s.starts_with(FILE_PREFIX) {
            let p = PathBuf::from(&s.trim_start_matches(FILE_PREFIX));
            if !p.is_absolute() {
                return Err(format!("File path must be absolute: {p:?}",));
            }

            return Ok(FeedSource::File(p));
        }

        Err(format!("Invalid source: {s}",))
    }
}

impl From<String> for FeedSource {
    fn from(s: String) -> Self {
        FeedSource::from_str(s.as_str()).unwrap()
    }
}

impl Serialize for FeedSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'a> Deserialize<'a> for FeedSource {
    fn deserialize<D>(deserializer: D) -> Result<FeedSource, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        FeedSource::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_parse_http_source() {
        let source = "http://example.com/feed.xml";
        let expected = FeedSource::Http(source.to_string());
        assert_eq!(FeedSource::from_str(source), Ok(expected));
    }

    #[test]
    fn test_should_parse_https_source() {
        let source = "https://example.com/feed.xml";
        let expected = FeedSource::Http(source.to_string());
        assert_eq!(FeedSource::from_str(source), Ok(expected));
    }

    #[test]
    fn test_should_parse_file_source() {
        let source = "file:///path/to/feed.xml";
        let expected = FeedSource::File(PathBuf::from("/path/to/feed.xml"));
        assert_eq!(FeedSource::from_str(source), Ok(expected));
    }

    #[test]
    fn test_should_fail_parsing_invalid_source() {
        let source = "ftp://example.com/feed.xml";
        let expected = "Invalid source: ftp://example.com/feed.xml";
        assert_eq!(FeedSource::from_str(source), Err(expected.to_string()));
    }

    #[test]
    fn test_should_fail_parsing_relative_path() {
        let source = "file://path/to/feed.xml";
        assert!(FeedSource::from_str(source).is_err());
    }
}
