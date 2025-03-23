//! # Client
//!
//! Async feed client

use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

use crate::feed::{Client, Feed, FeedResult};

#[derive(Debug, Default)]
pub struct FeedClient {
    /// Join handle
    workers: Vec<WorkerThread>,
}

impl FeedClient {
    /// Fetch source.
    pub fn fetch(&mut self, name: &str, uri: &str) {
        self.workers.push(WorkerThread::start(name, uri));
    }

    /// Poll worker threads
    pub fn poll(&mut self) -> Option<(String, FeedResult<Feed>)> {
        let mut i = 0;
        while i < self.workers.len() {
            // if worker at `i` is joinable, join and return
            if self.workers[i].is_joinable() {
                let worker = self.workers.remove(i);
                // Join and return
                return Some(worker.join());
            }
            i += 1;
        }
        None
    }

    /// Returns whether client has running workers
    pub fn running(&self) -> bool {
        !self.workers.is_empty()
    }
}

impl Drop for FeedClient {
    fn drop(&mut self) {
        // Wait for all
        let mut i = 0;
        while i < self.workers.len() {
            let worker = self.workers.remove(i);
            let _ = worker.join();
            i += 1;
        }
    }
}

// -- worker thread

/// Thread holder for worker
#[derive(Debug)]
struct WorkerThread(Arc<RwLock<bool>>, JoinHandle<(String, FeedResult<Feed>)>);

impl WorkerThread {
    /// Start a new worker thread
    pub fn start(name: &str, uri: &str) -> Self {
        let completed = Arc::new(RwLock::new(false));
        let completed_t = Arc::clone(&completed);
        let name = name.to_string();
        let uri = uri.to_string();
        let thread = thread::spawn(|| Worker::new(completed_t, name, uri).run());
        Self(completed, thread)
    }

    /// Returns whether thread is joinable
    pub fn is_joinable(&self) -> bool {
        if let Ok(lock) = self.0.read() {
            return *lock;
        }
        true
    }

    /// Join thread and consume worker.
    /// Returns thread product
    pub fn join(self) -> (String, FeedResult<Feed>) {
        self.1.join().ok().unwrap()
    }
}

// -- worker

/// Worker thread which fetches async the feed sources
pub struct Worker {
    completed: Arc<RwLock<bool>>,
    name: String,
    uri: String,
}

impl Worker {
    pub fn new(completed: Arc<RwLock<bool>>, name: String, uri: String) -> Self {
        Self {
            completed,
            name,
            uri,
        }
    }

    /// Run function for worker
    pub fn run(&mut self) -> (String, FeedResult<Feed>) {
        // Set running to false
        self.stop();
        // Return to handle
        (self.name.clone(), Client.fetch(self.uri.as_str()))
    }

    fn stop(&mut self) {
        // NOTE: keep these brackets to drop running after block
        if let Ok(mut completed) = self.completed.write() {
            *completed = true;
        }
    }
}

#[cfg(test)]
mod test {

    use std::thread::sleep;
    use std::time::{Duration, Instant};

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_run_worker() {
        let mut client = FeedClient::default();
        assert_eq!(client.running(), false);
        // Start worker
        client.fetch(
            "Le Figaro",
            "https://www.lefigaro.fr/rss/figaro_actualites.xml",
        );
        assert_eq!(client.running(), true);
        // Wait up to 10 seconds before failing
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(10) {
            if let Some((name, result)) = client.poll() {
                assert_eq!(name.as_str(), "Le Figaro");
                assert!(result.is_ok());
                assert_eq!(client.running(), false);
                return;
            }
            sleep(Duration::from_millis(500));
        }
        panic!("Failed to fetch source")
    }
}
