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
use chrono::{DateTime, Local};
use std::collections::HashMap;

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
#[derive(Debug, Default)]
pub struct Feed {
    title: Option<String>,
    articles: Vec<Article>,
}

/// ## Article
///
/// identifies a single article in the feed
#[derive(Debug)]
pub struct Article {
    title: Option<String>,
    authors: Vec<String>,
    summary: String,
    url: String,
    date: DateTime<Local>,
}
