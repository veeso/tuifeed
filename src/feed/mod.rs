//! # Feed
//!
//! feed types to work with RSS/Atom.
//! It is based on feed-rs: <https://docs.rs/feed-rs/1.0.0/feed_rs/>

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
// -- modules
mod client;
mod result;

// -- export
pub use client::Client;
pub use result::{FeedError, FeedResult};
// -- deps
use chrono::{DateTime, Local};
use feed_rs::model::{Entry as RssEntry, Feed as RssFeed};
use std::collections::HashMap;
use std::slice::Iter;

/// ## Kiosk
///
/// Describes the current feed holder.
/// It contains different sources, each one with its own feed
#[derive(Debug, Default)]
pub struct Kiosk {
    /// Association between Source name and Feed
    feed: HashMap<String, Feed>,
}

/// ## Feed
///
/// Contains, for a feed source, the list of articles fetched from remote
#[derive(Debug)]
pub struct Feed {
    title: Option<String>,
    articles: Vec<Article>,
}

/// ## Article
///
/// identifies a single article in the feed
#[derive(Debug)]
pub struct Article {
    pub title: Option<String>,
    pub authors: Vec<String>,
    pub summary: String,
    pub url: String,
    pub date: Option<DateTime<Local>>,
}

// -- impl

impl Kiosk {
    /// ### insert_feed
    ///
    /// Insert a feed into kiosk
    pub fn insert_feed<S: AsRef<str>>(&mut self, source: S, feed: Feed) {
        self.feed.insert(source.as_ref().to_string(), feed);
    }

    /// ### get_feed
    ///
    /// Get feed from kiosk
    pub fn get_feed(&self, source: &str) -> Option<&Feed> {
        self.feed.get(source)
    }

    /// ### sources
    ///
    /// Get sources in kiosk
    pub fn sources(&self) -> Vec<&String> {
        self.feed.keys().into_iter().collect()
    }
}

impl Feed {
    /// ### title
    ///
    /// Get a reference to title
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// ### articles
    ///
    /// Get an iterator over articles
    pub fn articles(&self) -> Iter<'_, Article> {
        self.articles.iter()
    }
}

// -- converter

impl From<RssFeed> for Feed {
    fn from(feed: RssFeed) -> Self {
        Self {
            title: feed.title.map(|x| x.content),
            articles: feed.entries.into_iter().map(Article::from).collect(),
        }
    }
}

impl From<RssEntry> for Article {
    fn from(entry: RssEntry) -> Self {
        Self {
            title: entry.title.map(|x| x.content),
            authors: entry.authors.into_iter().map(|x| x.name).collect(),
            summary: entry.summary.map(|x| x.content).unwrap_or_default(),
            url: entry
                .content
                .map(|x| x.src.map(|x| x.href))
                .flatten()
                .unwrap_or(entry.id),
            date: entry.updated.map(DateTime::<Local>::from),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use feed_rs::model::FeedType;
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
            Feed {
                title: None,
                articles: Vec::default(),
            },
        );
        assert_eq!(kiosk.feed.len(), 1);
    }

    #[test]
    fn should_get_feed_from_kiosk() {
        let mut kiosk = Kiosk::default();
        kiosk.insert_feed(
            "lefigaro",
            Feed {
                title: None,
                articles: Vec::default(),
            },
        );
        assert!(kiosk.get_feed("lefigaro").is_some());
        assert!(kiosk.get_feed("foobar").is_none());
    }

    #[test]
    fn should_get_sources_from_kiosk() {
        let mut kiosk = Kiosk::default();
        kiosk.insert_feed(
            "lefigaro",
            Feed {
                title: None,
                articles: Vec::default(),
            },
        );
        assert_eq!(kiosk.sources(), vec![&String::from("lefigaro")]);
    }

    #[test]
    fn should_get_feed_attributes() {
        let feed = Feed {
            title: Some(String::from("foo")),
            articles: Vec::default(),
        };
        assert_eq!(feed.title().unwrap(), "foo");
        assert!(feed.articles.is_empty());
    }

    #[test]
    fn should_convert_entry_into_article() {
        let entry = RssEntry::default();
        let article = Article::from(entry);
        assert!(article.authors.is_empty());
        assert_eq!(article.date, None);
        assert_eq!(article.summary, String::new());
        assert_eq!(article.title, None);
        assert_eq!(article.url, String::new());
    }

    #[test]
    fn should_convert_rssfeed_into_feed() {
        let feed = RssFeed {
            feed_type: FeedType::Atom,
            id: String::from("pippo"),
            contributors: Vec::new(),
            title: None,
            updated: None,
            authors: Vec::new(),
            description: None,
            links: Vec::new(),
            categories: Vec::new(),
            generator: None,
            icon: None,
            language: None,
            logo: None,
            published: None,
            rating: None,
            rights: None,
            ttl: None,
            entries: vec![RssEntry::default(), RssEntry::default()],
        };
        let feed = Feed::from(feed);
        assert_eq!(feed.title.is_none(), true);
        assert_eq!(feed.articles.len(), 2);
    }
}