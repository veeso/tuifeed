//! # Config
//!
//! Configuration types for tuifeed

pub mod serializer;

use serde::Deserialize;
use std::collections::HashMap;

/// tuifeed configuration
#[derive(Deserialize, Debug, Default)]
pub struct Config {
    /// Association between source name and url
    pub sources: HashMap<String, String>,
}
