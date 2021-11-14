//! # Client
//!
//! Async feed client

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
use crate::feed::{Client, Feed, FeedResult};
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

#[derive(Debug, Default)]
pub struct FeedClient {
    /// Join handle
    workers: Vec<WorkerThread>,
}

impl FeedClient {
    /// ### fetch
    ///
    /// Fetch source.
    /// Panics if fails to send request
    pub fn fetch(&mut self, name: &str, uri: &str) {
        self.workers.push(WorkerThread::start(name, uri));
    }

    /// ### poll
    ///
    /// Poll receiver for fetch results.
    /// Panics if fails to poll
    pub fn poll(&mut self) -> Option<(String, FeedResult<Feed>)> {
        // FIXME: use drain_filter when stable
        let mut i = 0;
        while i < self.workers.len() {
            // if worker at `i` is joinable, join and return
            if self.workers[i].is_joinable() {
                let mut worker = self.workers.remove(i);
                // Join and return
                return Some(worker.join());
            }
            i += 1;
        }
        None
    }
}

impl Drop for FeedClient {
    fn drop(&mut self) {
        // Wait for all
        for worker in self.workers.into_iter() {
            let _ = worker.join();
        }
    }
}

// -- worker thread

/// ## WorkerThread
///
/// Thread holder for worker
#[derive(Debug)]
struct WorkerThread(Arc<RwLock<bool>>, JoinHandle<(String, FeedResult<Feed>)>);

impl WorkerThread {
    /// ### start
    ///
    /// Start a new worker thread
    pub fn start(name: &str, uri: &str) -> Self {
        let completed = Arc::new(RwLock::new(false));
        let completed_t = Arc::clone(&completed);
        let thread =
            thread::spawn(|| Worker::new(completed_t, name.to_string(), uri.to_string()).run());
        Self(completed, thread)
    }

    /// ### is_joinable
    ///
    /// Returns whether thread is joinable
    pub fn is_joinable(&self) -> bool {
        if let Ok(lock) = self.0.read() {
            return *lock;
        }
        true
    }

    /// ### join
    ///
    /// Join thread and consume worker.
    /// Returns thread product
    pub fn join(self) -> (String, FeedResult<Feed>) {
        self.1.join().ok().unwrap()
    }
}

// -- worker

/// ## Worker
///
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

    /// ### run
    ///
    /// Run function for worker
    pub fn run(&mut self) -> (String, FeedResult<Feed>) {
        let fetch_result = Client::default().fetch(self.uri.as_str());
        // Set running to false
        self.stop();
        // Return to handle
        (self.name, Client::default().fetch(self.uri.as_str()))
    }

    fn stop(&mut self) {
        // NOTE: keep these brackets to drop running after block
        if let Ok(completed) = self.completed.write() {
            *completed = true;
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    // TODO: impl tests
}
