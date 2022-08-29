//! # Ui
//!
//! Ui for tuifeed

mod components;
mod lib;
mod view;

use crate::config::Config;
use crate::feed::Feed;
use crate::helpers::open as open_helpers;

use lib::{FeedClient, FeedState, FlatFeedState, Kiosk};

use std::time::{Duration, Instant};
use tuirealm::{
    terminal::TerminalBridge, Application, AttrValue, Attribute, NoUserEvent, PollStrategy, State,
    StateValue, Update,
};

const FORCED_REDRAW_INTERVAL: Duration = Duration::from_millis(50);

/// identifiers for components
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    GlobalListener,
    FeedList,
    ArticleList,
    ArticleTitle,
    ArticleDate,
    ArticleAuthors,
    ArticleSummary,
    ArticleLink,
    QuitPopup,
    ErrorPopup,
}

/// Messages produced by components
#[derive(Debug, PartialEq, Eq)]
pub enum Msg {
    ArticleBlur,
    ArticleChanged(usize),
    ArticleListBlur,
    CloseErrorPopup,
    CloseQuitPopup,
    FeedChanged(usize),
    FeedListBlur,
    FetchSource,
    FetchAllSources,
    GoReadArticle,
    OpenArticle,
    Quit,
    ShowQuitPopup,
    None,
}

/// tuifeed ui application
pub struct Ui {
    application: Application<Id, Msg, NoUserEvent>,
    client: FeedClient,
    config: Config,
    kiosk: Kiosk,
    last_redraw: Instant,
    redraw: bool,
    terminal: TerminalBridge,
}

impl Ui {
    /// Init a new `Ui`
    pub fn init(config: Config, ticks: u64) -> Self {
        let mut kiosk = Kiosk::default();
        for name in config.sources.keys() {
            kiosk.insert_feed(name, FeedState::Loading);
        }
        Self {
            application: Self::init_application(&kiosk, Duration::from_millis(ticks)),
            client: FeedClient::default(),
            config,
            kiosk,
            last_redraw: Instant::now(),
            redraw: true,
            terminal: TerminalBridge::new().expect("could not setup terminal"),
        }
    }

    /// run ui
    pub fn run(mut self) {
        self.init_terminal();
        // Fetch sources once
        self.fetch_all_sources();
        let mut quit = false;
        // Main loop
        while !quit {
            // poll and update
            match self.application.tick(PollStrategy::UpTo(3)) {
                Ok(messages) if messages.is_empty() => {}
                Ok(messages) => {
                    self.redraw = true;
                    for msg in messages.into_iter() {
                        if let Some(Msg::Quit) = self.update(Some(msg)) {
                            quit = true;
                            break;
                        }
                    }
                }
                Err(err) => {
                    self.mount_error_popup(format!("Application error: {}", err));
                }
            }
            // Poll fetched sources
            self.poll_fetched_sources();
            // Check whether to force redraw
            self.check_force_redraw();
            // View
            if self.redraw {
                self.view();
            }
        }
        self.finalize_terminal();
    }

    /// Fetch all sources and update Ui
    #[allow(clippy::needless_collect)]
    fn fetch_all_sources(&mut self) {
        // Fetch sources
        let sources: Vec<(String, String)> = self
            .config
            .sources
            .iter()
            .map(|(name, uri)| (name.clone(), uri.clone()))
            .collect();
        for (name, uri) in sources.into_iter() {
            self.fetch_source(name.as_str(), uri.as_str());
        }
    }

    /// Start a worker to fetch sources
    fn fetch_source(&mut self, name: &str, uri: &str) {
        self.client.fetch(name, uri);
        // Mark source as Loading
        self.update_source(name, FeedState::Loading);
        self.update_feed_list_item(name, FlatFeedState::Loading);
        // Force redraw
        self.redraw = true;
    }

    /// Update source in kiosk
    fn update_source(&mut self, name: &str, state: FeedState) {
        self.kiosk.insert_feed(name, state);
    }

    /// Get result for all fetched sources
    fn poll_fetched_sources(&mut self) {
        if let Some((name, result)) = self.client.poll() {
            // Adapt state
            let state = match result {
                Ok(feed) => FeedState::Success(feed),
                Err(err) => {
                    // Mount error and return err
                    self.mount_error_popup(format!(r#"Could not fetch feed "{}": {}"#, name, err));
                    FeedState::Error(err)
                }
            };
            // Update source
            let flat_state = FlatFeedState::from(&state);
            self.update_source(name.as_str(), state);
            // Update feed list and initialize article
            self.update_feed_list_item(name.as_str(), flat_state);
            if self.is_article_list_empty() {
                let config = self.config.clone();
                self.init_article(&config);
            }
            // Force redraw
            self.redraw = true;
        }
    }

    /// Check whether should force redraw
    fn check_force_redraw(&mut self) {
        // If source are loading and at least 100ms has elapsed since last redraw...
        if self.client.running() && self.since_last_redraw() >= FORCED_REDRAW_INTERVAL {
            self.redraw = true;
        }
    }

    /// return last redraw
    fn since_last_redraw(&self) -> Duration {
        self.last_redraw.elapsed()
    }

    /// Get sorted sources from kiosk
    fn sorted_sources(&self) -> Vec<&String> {
        let mut sources = self.kiosk.sources();
        sources.sort();
        sources
    }

    /// Get currently selected feed
    fn get_selected_feed(&self) -> Option<&Feed> {
        if let Some(feed) = self.get_selected_feed_name() {
            Some(self.kiosk.get_feed(feed.as_str()).unwrap())
        } else {
            None
        }
    }

    /// Get currently selected feed name
    fn get_selected_feed_name(&self) -> Option<String> {
        if let State::One(StateValue::Usize(feed)) =
            self.application.state(&Id::FeedList).ok().unwrap()
        {
            Some((*self.sorted_sources().get(feed).unwrap()).clone())
        } else {
            None
        }
    }
}

impl Update<Msg> for Ui {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        match msg.unwrap_or(Msg::None) {
            Msg::ArticleBlur => {
                assert!(self.application.active(&Id::ArticleList).is_ok());
                None
            }
            Msg::ArticleChanged(article) => {
                self.update_article(article);
                None
            }
            Msg::ArticleListBlur => {
                assert!(self.application.active(&Id::FeedList).is_ok());
                None
            }
            Msg::CloseErrorPopup => {
                self.umount_error_popup();
                None
            }
            Msg::CloseQuitPopup => {
                self.umount_quit_popup();
                None
            }
            Msg::FeedChanged(feed) => {
                let feed = &(*self.sorted_sources().get(feed).unwrap()).clone();
                if let Some(feed) = self.kiosk.get_feed(feed.as_str()) {
                    let articles =
                        self.get_article_list(&self.config, feed, self.max_article_name_len());
                    assert!(self
                        .application
                        .remount(Id::ArticleList, Box::new(articles), vec![])
                        .is_ok());
                    // Then load the first article of feed
                    self.update_article(0);
                }
                None
            }
            Msg::FeedListBlur => {
                assert!(self.application.active(&Id::ArticleList).is_ok());
                None
            }
            Msg::FetchSource => {
                if let Some(name) = self.get_selected_feed_name() {
                    let uri = self.config.sources.get(&name).cloned();
                    if let Some(uri) = uri {
                        self.fetch_source(name.as_str(), uri.as_str())
                    }
                }
                None
            }
            Msg::FetchAllSources => {
                self.fetch_all_sources();
                None
            }
            Msg::GoReadArticle => {
                let _ = self.application.active(&Id::ArticleSummary);
                None
            }
            Msg::OpenArticle => {
                if let Ok(Some(AttrValue::String(url))) =
                    self.application.query(&Id::ArticleLink, Attribute::Text)
                {
                    if let Err(err) = open_helpers::open_link(url.as_str()) {
                        self.mount_error_popup(err);
                    }
                }
                None
            }
            Msg::Quit => Some(Msg::Quit),
            Msg::ShowQuitPopup => {
                self.mount_quit_popup();
                None
            }
            Msg::None => None,
        }
    }
}
