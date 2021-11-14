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
use super::{Feed, FeedError, FeedResult};
use feed_rs::parser as feed_parser;
use std::io::Read;

/// ## Client
///
/// RSS client. Fetches its sources to retrieve all the required Feeds
#[derive(Default)]
pub struct Client;

impl Client {
    /// ### fetch_source
    ///
    /// Fetch a single source from remote
    pub fn fetch(&self, source: &str) -> FeedResult<Feed> {
        let body = self.get_feed(source)?;
        self.parse_feed(body)
    }

    // -- private

    /// ### get_feed
    ///
    /// Get feed via HTTP GET request
    fn get_feed(&self, source: &str) -> FeedResult<impl Read + Send> {
        Ok(ureq::get(source).call()?.into_reader())
    }

    /// ### parse_feed
    ///
    /// Parse feed from HTTP response
    fn parse_feed<R: Read>(&self, response: R) -> FeedResult<Feed> {
        feed_parser::parse(response)
            .map(Feed::from)
            .map_err(FeedError::from)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_get_source() {
        let client = Client::default();
        assert!(client
            .get_feed(&String::from(
                "https://rss.nytimes.com/services/xml/rss/nyt/World.xml"
            ))
            .is_ok());
    }

    #[test]
    fn should_fail_getting_source() {
        let client = Client::default();
        assert!(client
            .get_feed(&String::from(
                "https://rss.nytimes.com/services/xml/rss/nyt/pippopippopippo.xml"
            ))
            .is_err());
    }

    #[test]
    fn should_fetch_source() {
        let client = Client::default();
        assert!(client
            .fetch("https://rss.nytimes.com/services/xml/rss/nyt/World.xml",)
            .is_ok());
        assert!(client
            .fetch("https://www.lefigaro.fr/rss/figaro_actualites.xml",)
            .is_ok());
    }
}
