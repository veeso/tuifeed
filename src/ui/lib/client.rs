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
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
pub struct FeedClient {
    /// Indicates whether the worker should keep running
    running: Arc<RwLock<bool>>,
    /// Receiver. The worker send a message with the result of the fetch process
    recv: mpsc::Receiver<(String, FeedResult<Feed>)>,
    /// Sender. Used to request to the worker to fetch a source (SourceName, Uri)
    send: mpsc::Sender<(String, String)>,
    /// Join handle
    thread: Option<JoinHandle<()>>,
}

impl FeedClient {
    /// ### start
    ///
    /// Start feed client
    pub fn start() -> Self {
        let (source_send, source_recv) = mpsc::channel();
        let (feed_send, feed_recv) = mpsc::channel();
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        // Start thread
        let thread = thread::spawn(move || {
            Worker::new(source_recv, feed_send, running_t).run();
        });
        Self {
            running,
            recv: feed_recv,
            send: source_send,
            thread: Some(thread),
        }
    }

    /// ### stop
    ///
    /// Stop event listener
    pub fn stop(&mut self) {
        {
            // NOTE: keep these brackets to drop running after block
            if let Ok(running) = self.running.write() {
                *running = false;
            }
        }
        // Join thread
        let _ = self.thread.take().map(|x| x.join());
    }

    /// ### fetch
    ///
    /// Fetch source.
    /// Panics if fails to send request
    pub fn fetch(&self, name: &str, uri: &str) {
        if let Err(err) = self.send.send((name.to_string(), uri.to_string())) {
            panic!("Runtime error (fetch): {}", err);
        }
    }

    /// ### poll
    ///
    /// Poll receiver for fetch results.
    /// Panics if fails to poll
    pub fn poll(&self) -> Option<(String, FeedResult<Feed>)> {
        match self.recv.recv_timeout(Duration::from_millis(10)) {
            Ok(msg) => Some(msg),
            Err(mpsc::RecvTimeoutError::Timeout) => None,
            Err(err) => panic!("Runtime error (poll): {}", err),
        }
    }
}

impl Drop for FeedClient {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

// -- worker

/// ## Worker
///
/// Worker thread which fetches async the feed sources
pub struct Worker {
    client: Client,
    running: Arc<RwLock<bool>>,
    recv: mpsc::Receiver<(String, String)>,
    send: mpsc::Sender<(String, FeedResult<Feed>)>,
}

impl Worker {
    /// ### new
    ///
    /// Instantiates a new `Worker`+
    pub fn new(
        recv: mpsc::Receiver<(String, String)>,
        send: mpsc::Sender<(String, FeedResult<Feed>)>,
        running: Arc<RwLock<bool>>,
    ) -> Self {
        Self {
            client: Client::default(),
            running,
            recv,
            send,
        }
    }

    /// ### run
    ///
    /// Main loop for Worker
    pub fn run(&mut self) {
        loop {
            if !self.running() {
                break;
            }
        }
    }

    // -- private

    /// ### process_fetch_requests
    ///
    /// Process incoming fetch requests.
    /// Panics if fails to poll
    fn process_fetch_requests(&mut self) {
        loop {
            // Get source to fetch
            let (name, uri) = match self.recv.recv_timeout(Duration::from_millis(10)) {
                Ok((name, uri)) => (name, uri),
                Err(mpsc::RecvTimeoutError::Timeout) => break,
                Err(err) => panic!("Runtime error (process_fetch_requests): {}", err),
            };
            // Fetch source
            self.fetch_source(name, uri);
        }
    }

    /// fetch_source
    ///
    /// Fetch source
    fn fetch_source(&mut self, name: String, uri: String) {
        if let Err(err) = self.send.send((name, self.client.fetch(uri.as_str()))) {
            panic!("Runtime error (fetch_source): {}", err);
        }
    }

    /// ### running
    ///
    /// Returns whether should keep running
    fn running(&self) -> bool {
        if let Ok(lock) = self.running.read() {
            return *lock;
        }
        true
    }
}
