//! # Result
//!
//! Exposes error and result types for Feed

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
use feed_rs::parser::ParseFeedError;
use thiserror::Error;
use ureq::Error as RequestError;

/// ## FeedResult
///
/// A result returned by the feed client
pub type FeedResult<T> = Result<T, FeedError>;

/// ## FeedError
///
/// Describes a feed error
#[derive(Debug, Error, PartialEq)]
pub enum FeedError {
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("HTTP request failed: {0}")]
    Http(String),
    #[error("I/O error: {0}")]
    Io(String),
}

impl From<RequestError> for FeedError {
    fn from(e: RequestError) -> Self {
        FeedError::Http(e.to_string())
    }
}

impl From<std::io::Error> for FeedError {
    fn from(e: std::io::Error) -> Self {
        FeedError::Io(e.to_string())
    }
}

impl From<ParseFeedError> for FeedError {
    fn from(e: ParseFeedError) -> Self {
        FeedError::Parse(e.to_string())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use feed_rs::parser::ParseErrorKind;
    use pretty_assertions::assert_eq;
    use ureq::Response;

    #[test]
    fn should_convert_request_error() {
        assert_eq!(
            FeedError::from(RequestError::Status(
                404,
                Response::new(404, "not found", "").ok().unwrap()
            )),
            FeedError::Http(String::from(": status code 404"))
        );
    }

    #[test]
    fn should_convert_io_error() {
        assert_eq!(
            FeedError::from(std::io::Error::from(std::io::ErrorKind::NotFound)),
            FeedError::Io(String::from("entity not found"))
        );
    }

    #[test]
    fn should_convert_parse_error() {
        assert_eq!(
            FeedError::from(ParseFeedError::ParseError(ParseErrorKind::NoFeedRoot)),
            FeedError::Parse(String::from("unable to parse feed: no root element"))
        );
    }
}
