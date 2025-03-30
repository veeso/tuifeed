use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::feed::Article;

/// Result type for [`History`]
pub type HistoryResult<T> = Result<T, HistoryError>;

#[derive(Debug, Error)]
pub enum HistoryError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("No cache directory found on your system")]
    NoCacheDir,
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// History of read articles
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct History {
    #[serde(skip)]
    path: PathBuf,
    sources: HashMap<String, SourceHistory>,
}

impl History {
    /// Load [`History`] from disk
    pub fn load(path: &Path) -> HistoryResult<Self> {
        if !path.exists() {
            return Ok(History {
                path: path.to_path_buf(),
                ..Default::default()
            });
        }

        let mut reader = File::open(path)?;
        let mut history: History = serde_json::from_reader(&mut reader).unwrap_or_default();

        // save path
        history.path = path.to_path_buf();

        Ok(history)
    }

    /// Save [`History`] to disk
    pub fn save(&self) -> HistoryResult<()> {
        let mut writer = File::create(&self.path)?;
        serde_json::to_writer(&mut writer, self)?;

        Ok(())
    }

    /// Init path for history file
    pub fn default_path() -> HistoryResult<PathBuf> {
        let path = dirs::cache_dir()
            .or(dirs::config_dir())
            .ok_or(HistoryError::NoCacheDir)?
            .join("tuifeed");

        // init dir
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        Ok(path.join("history.json"))
    }

    /// Remove all articles from the history of a source, if not in the provided
    /// [`Article`]s list.
    pub fn filter_articles(&mut self, source: &str, articles: &[&Article]) {
        let source = self
            .sources
            .entry(source.to_string())
            .or_insert_with(|| SourceHistory {
                feed: HashMap::new(),
            });

        // remove all articles not in the provided list
        source
            .feed
            .retain(|article_id, _| articles.iter().any(|a| &a.id == article_id));
    }

    /// Insert an article into the history
    pub fn insert(&mut self, source: &str, article: &Article) {
        let source = self
            .sources
            .entry(source.to_string())
            .or_insert_with(|| SourceHistory {
                feed: HashMap::new(),
            });

        let article_time = article
            .date
            .map(|date| date.into())
            .unwrap_or_else(SystemTime::now);

        source
            .feed
            .entry(article.id.clone())
            .or_insert_with(|| ArticleHistory {
                timestamp: timestamp(article_time),
                last_viewed: None,
            });
    }

    /// set an article as read
    pub fn read(&mut self, source: &str, article: &Article) {
        // get entry
        let source = self
            .sources
            .entry(source.to_string())
            .or_insert_with(|| SourceHistory {
                feed: HashMap::new(),
            });

        let article_time = article
            .date
            .map(|date| date.into())
            .unwrap_or_else(SystemTime::now);

        // update article
        source
            .feed
            .entry(article.id.clone())
            .or_insert_with(|| ArticleHistory {
                timestamp: timestamp(article_time),
                last_viewed: None,
            })
            .last_viewed = Some(now());
    }

    /// set all articles from a source as read
    pub fn read_source(&mut self, source: &str) {
        // get entry
        let source = self
            .sources
            .entry(source.to_string())
            .or_insert_with(|| SourceHistory {
                feed: HashMap::new(),
            });

        // update all articles
        for article in source.feed.values_mut() {
            article.last_viewed = Some(now());
        }
    }

    /// set all articles as read
    pub fn read_all(&mut self) {
        // update all articles
        for source in self.sources.values_mut() {
            for article in source.feed.values_mut() {
                article.last_viewed = Some(now());
            }
        }
    }

    /// Returns whether the article has been read
    pub fn is_article_read(&self, source: &str, article: &Article) -> bool {
        self.sources
            .get(source)
            .and_then(|source| source.feed.get(&article.id))
            .map(|article| article.is_read())
            .unwrap_or_default()
    }

    /// Returns whether all articles from a source have been read.
    pub fn is_source_read(&self, source: &str) -> bool {
        self.sources
            .get(source)
            .map(|source| source.feed.values().all(|article| article.is_read()))
            .unwrap_or_default()
    }
}

fn now() -> u64 {
    timestamp(SystemTime::now())
}

fn timestamp(t: SystemTime) -> u64 {
    t.duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SourceHistory {
    feed: HashMap<String, ArticleHistory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArticleHistory {
    timestamp: u64,
    last_viewed: Option<u64>,
}

impl ArticleHistory {
    /// Returns whether the article has been read
    fn is_read(&self) -> bool {
        self.last_viewed
            .map(|t| t >= self.timestamp)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test {

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_should_work_with_history() {
        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        let mut history = History::load(path).expect("load history");
        assert!(history.sources.is_empty());

        let source = "figaro".to_string();
        let article = Article {
            id: "1".to_string(),
            title: Some("title".to_string()),
            authors: vec![],
            date: None,
            summary: String::default(),
            url: "http://example.com".to_string(),
        };

        // mark read
        history.read(&source, &article);
        assert!(history.is_article_read(&source, &article));

        // unread source
        let unread_source = "nytimes".to_string();
        assert!(!history.is_article_read(&unread_source, &article));

        // unread article
        let unread_article = Article {
            id: "2".to_string(),
            title: Some("title".to_string()),
            authors: vec![],
            date: None,
            summary: String::default(),
            url: "http://example.com".to_string(),
        };
        assert!(!history.is_article_read(&source, &unread_article));

        history.save().expect("save history");
    }

    #[test]
    fn test_should_insert_article() {
        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        let mut history = History::load(path).expect("load history");
        assert!(history.sources.is_empty());

        let source = "figaro".to_string();
        let article = Article {
            id: "1".to_string(),
            title: Some("title".to_string()),
            authors: vec![],
            date: None,
            summary: String::default(),
            url: "http://example.com".to_string(),
        };

        history.insert(&source, &article);
        assert!(!history.is_article_read(&source, &article));

        history.save().expect("save history");
    }

    #[test]
    fn test_should_filter_articles() {
        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        let mut history = History::load(path).expect("load history");
        assert!(history.sources.is_empty());

        let source = "figaro".to_string();
        let article = Article {
            id: "1".to_string(),
            title: Some("title".to_string()),
            authors: vec![],
            date: None,
            summary: String::default(),
            url: "http://example.com".to_string(),
        };

        let article2 = Article {
            id: "2".to_string(),
            title: Some("title".to_string()),
            authors: vec![],
            date: None,
            summary: String::default(),
            url: "http://example.com".to_string(),
        };

        history.insert(&source, &article);
        history.insert(&source, &article2);

        // filter articles
        history.filter_articles(&source, &[&article]);
        assert!(history.sources.get(&source).is_some());
        assert!(
            history
                .sources
                .get(&source)
                .unwrap()
                .feed
                .contains_key(&article.id)
        );
        assert!(
            !history
                .sources
                .get(&source)
                .unwrap()
                .feed
                .contains_key(&article2.id)
        );
    }

    #[test]
    fn test_should_mark_source_as_read() {
        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        let mut history = History::load(path).expect("load history");
        assert!(history.sources.is_empty());

        let source = "figaro".to_string();
        let article = Article {
            id: "1".to_string(),
            title: Some("title".to_string()),
            authors: vec![],
            date: None,
            summary: String::default(),
            url: "http://example.com".to_string(),
        };

        history.insert(&source, &article);
        assert!(!history.is_source_read(&source));

        let source2 = "lemonde".to_string();
        let article = Article {
            id: "1".to_string(),
            title: Some("title".to_string()),
            authors: vec![],
            date: None,
            summary: String::default(),
            url: "http://example.com".to_string(),
        };

        history.insert(&source2, &article);
        assert!(!history.is_source_read(&source2));

        history.read_source(&source);
        assert!(history.is_source_read(&source));
        assert!(!history.is_source_read(&source2));
        assert!(history.is_article_read(&source, &article));
    }

    #[test]
    fn test_should_read_all_sources() {
        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        let mut history = History::load(path).expect("load history");
        assert!(history.sources.is_empty());

        let source = "figaro".to_string();
        let article = Article {
            id: "1".to_string(),
            title: Some("title".to_string()),
            authors: vec![],
            date: None,
            summary: String::default(),
            url: "http://example.com".to_string(),
        };

        history.insert(&source, &article);
        assert!(!history.is_source_read(&source));

        let source2 = "lemonde".to_string();
        let article = Article {
            id: "1".to_string(),
            title: Some("title".to_string()),
            authors: vec![],
            date: None,
            summary: String::default(),
            url: "http://example.com".to_string(),
        };

        history.insert(&source2, &article);
        assert!(!history.is_source_read(&source2));

        history.read_all();
        assert!(history.is_source_read(&source));
        assert!(history.is_source_read(&source2));
        assert!(history.is_article_read(&source, &article));
    }
}
