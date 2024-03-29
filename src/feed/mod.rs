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

/// Contains, for a feed source, the list of articles fetched from remote
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feed {
    pub(crate) articles: Vec<Article>,
}

/// identifies a single article in the feed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub title: Option<String>,
    pub authors: Vec<String>,
    pub summary: String,
    pub url: String,
    pub date: Option<DateTime<Local>>,
}

impl Feed {
    /// Get an iterator over articles
    pub fn articles(&self) -> Iter<'_, Article> {
        self.articles.iter()
    }
}

// -- converter

impl From<RssFeed> for Feed {
    fn from(feed: RssFeed) -> Self {
        let mut articles: Vec<Article> = feed.entries.into_iter().map(Article::from).collect();
        articles.sort_by_key(|x| std::cmp::Reverse(x.date));
        Self { articles }
    }
}

impl From<RssEntry> for Article {
    fn from(entry: RssEntry) -> Self {
        let content_or_summary = content_or_summary(&entry);
        Self {
            title: entry
                .title
                .map(|x| str_helpers::strip_html(x.content.as_str())),
            authors: entry.authors.into_iter().map(|x| x.name).collect(),
            summary: content_or_summary,
            url: entry
                .links
                .get(0)
                .map(|x| x.href.clone())
                .unwrap_or(entry.id),
            date: entry
                .published
                .or(entry.updated)
                .map(DateTime::<Local>::from),
        }
    }
}

/// This function returns content if any, otherwise the summary of the article.
/// The reason is that content is USUALLY the entire article, BUT sometimes is not filled, so summary is preferred in these cases
fn content_or_summary(entry: &RssEntry) -> String {
    let content = entry
        .content
        .as_ref()
        .and_then(|x| x.body.as_ref().map(|x| str_helpers::strip_html(x)))
        .unwrap_or_default();
    if content.trim_matches('\n').trim().is_empty() {
        // get summary instead
        entry
            .summary
            .as_ref()
            .map(|x| str_helpers::strip_html(x.content.as_str()))
            .unwrap_or_default()
    } else {
        content
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
