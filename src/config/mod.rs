//! # Config
//!
//! Configuration types for tuifeed

pub mod serializer;

use serde::Deserialize;
use std::collections::HashMap;

/// tuifeed configuration
#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    /// Article title configuration
    #[serde(rename = "article-title")]
    pub article_title: Option<ArticleTitleConfig>,
    /// Association between source name and url
    pub sources: HashMap<String, String>,
}

/// article title configuration
#[derive(Deserialize, Clone, Copy, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ArticleTitleConfig {
    pub show_timestamp: bool,
    pub show_author: bool,
}
