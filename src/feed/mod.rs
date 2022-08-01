//! # Feed
//!
//! feed types to work with RSS/Atom.
//! It is based on feed-rs: <https://docs.rs/feed-rs/1.0.0/feed_rs/>

// -- modules
mod client;
mod result;

use crate::helpers::strings as str_helpers;

// -- export
pub use client::Client;
pub use result::{FeedError, FeedResult};
// -- deps
use chrono::{DateTime, Local};
use feed_rs::model::{Entry as RssEntry, Feed as RssFeed};
use std::slice::Iter;

/// ## Feed
///
/// Contains, for a feed source, the list of articles fetched from remote
#[derive(Debug, Clone, PartialEq)]
pub struct Feed {
    pub(crate) articles: Vec<Article>,
}

/// ## Article
///
/// identifies a single article in the feed
#[derive(Debug, Clone, PartialEq)]
pub struct Article {
    pub title: Option<String>,
    pub authors: Vec<String>,
    pub summary: String,
    pub url: String,
    pub date: Option<DateTime<Local>>,
}

impl Feed {
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
            articles: feed.entries.into_iter().map(Article::from).collect(),
        }
    }
}

impl From<RssEntry> for Article {
    fn from(entry: RssEntry) -> Self {
        Self {
            title: entry
                .title
                .map(|x| str_helpers::strip_html(x.content.as_str())),
            authors: entry.authors.into_iter().map(|x| x.name).collect(),
            summary: entry
                .summary
                .map(|x| str_helpers::strip_html(x.content.as_str()))
                .unwrap_or_default(),
            url: entry
                .links
                .get(0)
                .map(|x| x.href.clone())
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
    fn should_get_feed_attributes() {
        let feed = Feed {
            articles: Vec::default(),
        };
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
        assert_eq!(feed.articles.len(), 2);
    }
}
