//! # Ui
//!
//! Ui for tuifeed

mod components;
mod lib;
mod view;

use std::time::{Duration, Instant};

use lib::{FeedClient, FeedState, FlatFeedState, History, Kiosk};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalBridge};
use tuirealm::{
    Application, AttrValue, Attribute, NoUserEvent, PollStrategy, State, StateValue, Update,
};

use crate::config::Config;
use crate::feed::{Feed, FeedSource};
use crate::helpers::open as open_helpers;

const FORCED_REDRAW_INTERVAL: Duration = Duration::from_millis(50);

/// identifiers for components
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ArticleAuthors,
    ArticleDate,
    ArticleLink,
    ArticleList,
    ArticleSummary,
    ArticleTitle,
    ErrorPopup,
    FeedList,
    GlobalListener,
    QuitPopup,
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
    FetchAllSources,
    FetchSource,
    MarkSourceAsRead,
    MarkAllSourcesAsRead,
    GoReadArticle,
    OpenArticle,
    Quit,
    ShowQuitPopup,
    /// No-op
    None,
}

/// tuifeed ui application
pub struct Ui {
    application: Application<Id, Msg, NoUserEvent>,
    client: FeedClient,
    config: Config,
    history: History,
    kiosk: Kiosk,
    last_redraw: Instant,
    redraw: bool,
    terminal: TerminalBridge<CrosstermTerminalAdapter>,
}

impl Ui {
    /// Init a new [`Ui`] instance
    pub fn init(config: Config, ticks: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let mut terminal = TerminalBridge::init_crossterm()?;
        let _ = terminal.disable_mouse_capture();

        let history_path = History::default_path()?;
        let history = History::load(&history_path)?;

        let mut kiosk = Kiosk::default();
        for name in config.sources.keys() {
            kiosk.insert_feed(name, FeedState::Loading);
        }
        Ok(Self {
            application: Self::init_application(&kiosk, Duration::from_millis(ticks)),
            client: FeedClient::default(),
            config,
            history,
            kiosk,
            last_redraw: Instant::now(),
            redraw: true,
            terminal,
        })
    }

    /// run the ui
    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
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

        // save history
        self.history.save()?;

        Ok(())
    }

    /// Fetch all sources and update Ui
    fn fetch_all_sources(&mut self) {
        // Fetch sources
        let sources: Vec<_> = self
            .config
            .sources
            .iter()
            .map(|(name, uri)| (name.clone(), uri.clone()))
            .collect();
        for (name, source) in sources.into_iter() {
            self.fetch_source(name.as_str(), source);
        }
    }

    /// Start a worker to fetch sources
    fn fetch_source(&mut self, name: &str, source: FeedSource) {
        self.client.fetch(name, &source);
        // Mark source as Loading
        self.update_source(name, FeedState::Loading);
        self.update_feed_list_item(
            name,
            FlatFeedState::Loading,
            self.history.is_source_read(name),
        );
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

            // if feed is [`FeedState::Success`] update history
            if let FeedState::Success(feed) = &state {
                // filter articlesd
                let articles: Vec<_> = feed.articles().collect();
                self.history.filter_articles(name.as_str(), &articles);
                for article in articles {
                    self.history.insert(&feed.name, article);
                }
            }

            // Update source
            let flat_state = FlatFeedState::from(&state);
            self.update_source(name.as_str(), state);
            // Update feed list and initialize article
            self.update_feed_list_item(
                name.as_str(),
                flat_state,
                self.history.is_source_read(&name),
            );
            let selected_feed = self.get_selected_feed();
            if self.is_article_list_empty() && selected_feed.is_some() {
                let article_list = self.get_article_list(
                    &self.config,
                    selected_feed.expect("selected feed cannot be none"),
                    &self.history,
                    self.max_article_name_len(),
                    None,
                );
                self.init_article(article_list);
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
        let feed = self.get_selected_feed_name()?;
        self.kiosk.get_feed(feed.as_str())
    }

    /// Get currently selected feed name
    fn get_selected_feed_name(&self) -> Option<String> {
        let State::One(StateValue::Usize(feed)) = self.application.state(&Id::FeedList).ok()?
        else {
            return None;
        };

        self.sorted_sources().get(feed).cloned().cloned()
    }

    /// mark article as viewed in history
    fn mark_viewed_article(&mut self, index: usize) {
        let Some(feed_name) = self.get_selected_feed_name() else {
            return;
        };
        let Some(feed) = self.get_selected_feed().cloned() else {
            return;
        };
        let Some(article) = feed.articles().nth(index).cloned() else {
            return;
        };
        let was_read = self.history.is_article_read(feed_name.as_str(), &article);
        if !was_read {
            self.history.read(feed_name.as_str(), &article);
            // update view
            self.reload_article_list(&feed, Some(index));
        }
        // if the entire source is read, reload source list
        if self.history.is_source_read(&feed_name) {
            let state = self
                .kiosk
                .get_feed_state(&feed_name)
                .map(FlatFeedState::from)
                .unwrap_or(FlatFeedState::Success);
            self.update_feed_list_item(&feed_name, state, true);
        }
    }

    /// Mark a source as read
    fn mark_source_as_read(&mut self, name: &str) {
        self.history.read_source(name);
        let selected_line = self.application.state(&Id::FeedList).ok();
        let selected_line = match selected_line {
            Some(State::One(StateValue::Usize(line))) => Some(line),
            _ => None,
        };
        let Some(feed) = self.get_selected_feed().cloned() else {
            return;
        };
        self.reload_article_list(&feed, selected_line);

        let state = self
            .kiosk
            .get_feed_state(name)
            .map(FlatFeedState::from)
            .unwrap_or(FlatFeedState::Success);
        self.update_feed_list_item(&feed.name, state, true);
    }

    /// Mark all sources as read
    fn mark_all_sources_as_read(&mut self) {
        self.history.read_all();

        let selected_line = self.application.state(&Id::FeedList).ok();
        let selected_line = match selected_line {
            Some(State::One(StateValue::Usize(line))) => Some(line),
            _ => None,
        };
        let Some(feed) = self.get_selected_feed().cloned() else {
            return;
        };
        self.reload_article_list(&feed, selected_line);

        let sources_with_states = self.kiosk.get_state();
        for (feed_name, state) in sources_with_states {
            self.update_feed_list_item(&feed_name, state, true);
        }
    }

    fn reload_article_list(&mut self, feed: &Feed, selected_line: Option<usize>) {
        let articles = self.get_article_list(
            &self.config,
            feed,
            &self.history,
            self.max_article_name_len(),
            selected_line,
        );
        assert!(
            self.application
                .remount(Id::ArticleList, Box::new(articles), vec![])
                .is_ok()
        );
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
                self.mark_viewed_article(article);
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
                let feed = self.sorted_sources().get(feed).cloned()?;
                let feed = self.kiosk.get_feed(feed.as_str()).cloned()?;
                // mark first article as read
                self.mark_viewed_article(0);
                // Update feed list item
                self.reload_article_list(&feed, None);
                // Then load the first article of feed
                self.update_article(0);

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
                        self.fetch_source(name.as_str(), uri)
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
            Msg::MarkAllSourcesAsRead => {
                self.mark_all_sources_as_read();
                None
            }
            Msg::MarkSourceAsRead => {
                if let Some(name) = self.get_selected_feed_name() {
                    self.mark_source_as_read(name.as_str());
                }
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

impl Drop for Ui {
    fn drop(&mut self) {
        let _ = self.terminal.restore();
    }
}
