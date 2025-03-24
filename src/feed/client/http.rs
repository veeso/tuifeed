use std::io::Read;

use super::Fetch;
use crate::feed::FeedResult;

pub struct HttpClient;

impl Fetch for HttpClient {
    type Source = String;

    fn fetch(&self, source: &Self::Source) -> FeedResult<impl Read> {
        Ok(ureq::get(source).call()?.into_body().into_reader())
    }
}
