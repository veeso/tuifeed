//! # Kiosk
//!
//! kiosk entity, a collector for feed sources

use std::collections::HashMap;

use crate::feed::{Feed, FeedError};

/// Describes the current feed holder.
/// It contains different sources, each one with its own feed fetch state
#[derive(Debug, Default)]
pub struct Kiosk {
    /// Association between Source name and Feed
    feed: HashMap<String, FeedState>,
}

/// Describes the current feed state for a source.
#[derive(Debug, Eq, PartialEq)]
pub enum FeedState {
    /// Feed loaded with success
    Success(Feed),
    /// Failed to fetch / parse feed
    Error(FeedError),
    /// Loading feed
    Loading,
}

/// Describes the current feed state without containing the object
#[derive(Debug, PartialEq, Eq)]
pub enum FlatFeedState {
    /// Feed loaded with success
    Success,
    /// Failed to fetch / parse feed
    Error,
    /// Loading feed
    Loading,
}

impl Kiosk {
    /// Insert a feed into kiosk
    pub fn insert_feed<S: AsRef<str>>(&mut self, source: S, state: FeedState) {
        self.feed.insert(source.as_ref().to_string(), state);
    }

    /// Returns the list of sources associated to their feed list
    pub fn get_state(&self) -> Vec<(String, FlatFeedState)> {
        self.feed
            .iter()
            .map(|(name, state)| (name.to_string(), FlatFeedState::from(state)))
            .collect()
    }

    /// Get current feed state
    pub fn get_feed_state(&self, source: &str) -> Option<&FeedState> {
        self.feed.get(source)
    }

    /// Get feed from kiosk.
    /// Feed is returned only if source exists and if the current feed state is `Success`
    pub fn get_feed(&self, source: &str) -> Option<&Feed> {
        if let Some(FeedState::Success(feed)) = self.get_feed_state(source) {
            Some(feed)
        } else {
            None
        }
    }

    /// Get sources in kiosk
    pub fn sources(&self) -> Vec<&String> {
        self.feed.keys().collect()
    }
}

impl From<&FeedState> for FlatFeedState {
    fn from(f: &FeedState) -> Self {
        match *f {
            FeedState::Error(_) => Self::Error,
            FeedState::Loading => Self::Loading,
            FeedState::Success(_) => Self::Success,
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_create_kiosk() {
        let kiosk = Kiosk::default();
        assert!(kiosk.feed.is_empty());
    }

    #[test]
    fn should_insert_feed_into_kiosk() {
        let mut kiosk = Kiosk::default();
        kiosk.insert_feed(
            "lefigaro",
            FeedState::Success(Feed {
                articles: Vec::default(),
            }),
        );
        assert_eq!(kiosk.feed.len(), 1);
    }

    #[test]
    fn should_get_feed_from_kiosk() {
        let mut kiosk = Kiosk::default();
        kiosk.insert_feed(
            "lefigaro",
            FeedState::Success(Feed {
                articles: Vec::default(),
            }),
        );
        assert!(kiosk.get_feed("lefigaro").is_some());
        assert!(kiosk.get_feed("foobar").is_none());
    }

    #[test]
    fn should_get_feed_state_from_kiosk() {
        let mut kiosk = Kiosk::default();
        kiosk.insert_feed(
            "lefigaro",
            FeedState::Error(FeedError::Parse(String::from("parse error"))),
        );
        assert_eq!(
            kiosk.get_feed_state("lefigaro").unwrap(),
            &FeedState::Error(FeedError::Parse(String::from("parse error")))
        );
    }

    #[test]
    fn should_get_sources_from_kiosk() {
        let mut kiosk = Kiosk::default();
        kiosk.insert_feed(
            "lefigaro",
            FeedState::Success(Feed {
                articles: Vec::default(),
            }),
        );
        assert_eq!(kiosk.sources(), vec![&String::from("lefigaro")]);
    }

    #[test]
    fn should_return_kiosk_state() {
        let mut kiosk = Kiosk::default();
        kiosk.insert_feed(
            "lefigaro",
            FeedState::Success(Feed {
                articles: Vec::default(),
            }),
        );
        kiosk.insert_feed(
            "nytimes",
            FeedState::Error(FeedError::Parse(String::from("parse error"))),
        );
    }
}
