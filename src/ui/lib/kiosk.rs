//! # Kiosk
//!
//! kiosk entity, a collector for feed sources

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
use crate::feed::{Feed, FeedError};

use std::collections::HashMap;

/// ## Kiosk
///
/// Describes the current feed holder.
/// It contains different sources, each one with its own feed fetch state
#[derive(Debug, Default)]
pub struct Kiosk {
    /// Association between Source name and Feed
    feed: HashMap<String, FeedState>,
}

/// ## FeedState
///
/// Describes the current feed state for a source.
#[derive(Debug, PartialEq)]
pub enum FeedState {
    /// Feed loaded with success
    Success(Feed),
    /// Failed to fetch / parse feed
    Error(FeedError),
    /// Loading feed
    Loading,
    /// Not fetched yet
    None,
}

impl Kiosk {
    /// ### insert_feed
    ///
    /// Insert a feed into kiosk
    pub fn insert_feed<S: AsRef<str>>(&mut self, source: S, state: FeedState) {
        self.feed.insert(source.as_ref().to_string(), state);
    }

    /// ### get_feed_state
    ///
    /// Get current feed state
    pub fn get_feed_state(&self, source: &str) -> Option<&FeedState> {
        self.feed.get(source)
    }

    /// ### get_feed
    ///
    /// Get feed from kiosk.
    /// Feed is returned only if source exists and if the current feed state is `Success`
    pub fn get_feed(&self, source: &str) -> Option<&Feed> {
        if let Some(FeedState::Success(feed)) = self.get_feed_state(source) {
            Some(feed)
        } else {
            None
        }
    }

    /// ### sources
    ///
    /// Get sources in kiosk
    pub fn sources(&self) -> Vec<&String> {
        self.feed.keys().into_iter().collect()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

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
}
