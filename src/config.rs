//! # Config
//!
//! Configuration types for tuifeed

pub mod serializer;

use std::collections::HashMap;

use serde::Deserialize;

use crate::feed::FeedSource;

/// tuifeed configuration
#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    /// Article title configuration
    #[serde(rename = "article-title")]
    pub article_title: Option<ArticleTitleConfig>,
    /// Association between source name and url
    pub sources: HashMap<String, FeedSource>,
}

/// article title configuration
#[derive(Deserialize, Clone, Copy, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ArticleTitleConfig {
    pub show_timestamp: bool,
    pub show_author: bool,
}
