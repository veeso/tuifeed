use std::io::Read;
use std::path::PathBuf;

use super::Fetch;
use crate::feed::FeedResult;

pub struct FileClient;

impl Fetch for FileClient {
    type Source = PathBuf;

    fn fetch(&self, source: &Self::Source) -> FeedResult<impl Read> {
        Ok(std::fs::File::open(source)?)
    }
}
