//! # Client
//!
//! RSS/Atom client

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
use super::Kiosk;

use thiserror::Error;

/// ## FeedResult
///
/// A result returned by the feed client
pub type FeedResult<T> = Result<T, FeedError>;

/// ## FeedError
///
/// Describes a feed error
#[derive(Debug, Error)]
pub enum FeedError {
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("HTTP request failed: {0}")]
    Http(String),
}

/// ## Client
///
/// RSS client. Fetches its sources to retrieve all the required Feeds
pub struct Client {
    sources: Vec<String>,
}

impl Client {
    /// ### new
    ///
    /// Setup a new Feed client.
    pub fn new(sources: &[String]) -> Self {
        Self {
            sources: sources.to_vec(),
        }
    }

    /// ### fetch
    ///
    /// Fetch feed with the current configuration
    pub fn fetch(&self) -> FeedResult<Kiosk> {
        todo!()
    }
}
