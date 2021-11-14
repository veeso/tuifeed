//! # Ui
//!
//! Ui for tuifeed

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
mod components;
mod model;

use components::{ErrorPopup, GlobalListener, WaitPopup};
use model::Model;

use crate::config::Config;

use std::time::Duration;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent, KeyModifiers},
    terminal::TerminalBridge,
    Application, EventListenerCfg, NoUserEvent, Sub, SubClause, SubEventClause,
};

/// ## Id
///
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
    WaitPopup,
}

/// ## Msg
///
/// Messages produced by components
#[derive(Debug, PartialEq)]
pub enum Msg {
    ArticleChanged(usize),
    ArticleListBlur,
    CloseApp,
    CloseErrorPopup,
    CloseQuitPopup,
    FeedChanged(usize),
    FeedListBlur,
    FetchSources,
    OpenArticle,
    ShowQuitPopup,
    None,
}

/// ## Task
///
/// A task requested by the model in the Update routine, to be performed by the ui
#[derive(Debug, Clone, PartialEq)]
pub enum Task {
    FetchSources,
    ShowError(String),
}

pub struct Ui {
    model: Model,
    app: Application<Id, Msg, NoUserEvent>,
}

impl Ui {
    /// ### new
    ///
    /// Instantiates a new Ui
    pub fn new(config: Config, tick: u64) -> Self {
        let model = Model::new(config, Self::init_terminal());
        let app = Self::init_application(&model, tick);
        Self { model, app }
    }

    /// ### run
    ///
    /// Main loop for Ui thread
    pub fn run(&mut self) {
        self.model.init_terminal();
        // Fetch sources once
        self.fetch_sources();
        // Main loop
        while !self.model.quit() {
            if let Err(err) = self.app.tick(&mut self.model, PollStrategy::UpTo(3)) {
                self.mount_error_popup(format!("Application error: {}", err));
            }
            self.run_tasks();
            self.model.view(&mut self.app);
        }
        self.model.finalize_terminal();
    }

    // -- private

    /// ### run_tasks
    ///
    /// Run model tasks
    fn run_tasks(&mut self) {
        for task in self.model.get_tasks().into_iter() {
            match task {
                Task::FetchSources => self.fetch_sources(),
                Task::ShowError(err) => self.mount_error_popup(err),
            }
        }
    }

    /// ### fetch_sources
    ///
    /// Fetch sources and update Ui
    fn fetch_sources(&mut self) {
        self.mount_wait_popup(Some("Downloading feed. Please, wait…"));
        // Force redraw
        self.model.force_redraw();
        // Render once
        self.model.view(&mut self.app);
        // Force redraw for next cycle
        self.model.force_redraw();
        // Fetch sources
        let result = self.model.fetch_sources();
        // Umount wait popup
        assert!(self.app.umount(&Id::WaitPopup).is_ok());
        assert!(self.app.active(&Id::FeedList).is_ok());
        // If result is error, let's show the error message
        if let Err(err) = result {
            self.mount_error_popup(format!("Could not fetch feed: {}", err));
        } else {
            assert!(self
                .app
                .remount(Id::FeedList, Box::new(self.model.get_feed_list()), vec![])
                .is_ok());
            if let Some(source) = self.model.sorted_sources().get(0) {
                let feed = self.model.kiosk().get_feed(source.as_str()).unwrap();
                assert!(self
                    .app
                    .remount(
                        Id::ArticleList,
                        Box::new(Model::get_article_list(
                            feed,
                            self.model.max_article_name_len()
                        )),
                        vec![]
                    )
                    .is_ok());
                // Mount first article
                if let Some(article) = feed.articles().next() {
                    let (authors, date, link, summary, title) = Model::get_article_view(article);
                    assert!(self
                        .app
                        .remount(Id::ArticleAuthors, Box::new(authors), vec![])
                        .is_ok());
                    assert!(self
                        .app
                        .remount(Id::ArticleDate, Box::new(date), vec![])
                        .is_ok());
                    assert!(self
                        .app
                        .remount(Id::ArticleLink, Box::new(link), vec![])
                        .is_ok());
                    assert!(self
                        .app
                        .remount(Id::ArticleSummary, Box::new(summary), vec![])
                        .is_ok());
                    assert!(self
                        .app
                        .remount(Id::ArticleTitle, Box::new(title), vec![])
                        .is_ok());
                }
            }
        }
    }

    /// ### mount_error_popup
    ///
    /// Mount error and give focus to it
    fn mount_error_popup(&mut self, err: impl ToString) {
        assert!(self
            .app
            .remount(
                Id::ErrorPopup,
                Box::new(ErrorPopup::new(err.to_string())),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::ErrorPopup).is_ok());
    }

    /// ### mount_wait_popup
    ///
    /// Mount wait popup
    fn mount_wait_popup(&mut self, msg: Option<&str>) {
        // Remount wait
        assert!(self
            .app
            .remount(Id::WaitPopup, Box::new(WaitPopup::new(msg)), vec![])
            .is_ok());
        // Active wait
        assert!(self.app.active(&Id::WaitPopup).is_ok());
    }

    /// ### init_terminal
    ///
    /// Initialize terminal.
    /// Panics if it fails to connect the terminal bridge
    fn init_terminal() -> TerminalBridge {
        TerminalBridge::new().expect("Could not create terminal bridge")
    }

    /// ### init_application
    ///
    /// Initialize application.
    /// Panics if it fails
    fn init_application(model: &Model, tick: u64) -> Application<Id, Msg, NoUserEvent> {
        let mut app = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(tick))
                .poll_timeout(Duration::from_millis(tick)),
        );
        assert!(app
            .mount(Id::FeedList, Box::new(model.get_feed_list()), vec![])
            .is_ok());
        assert!(app
            .mount(
                Id::ArticleList,
                Box::new(Model::get_empty_article_list()),
                vec![]
            )
            .is_ok());
        assert!(app
            .mount(
                Id::GlobalListener,
                Box::new(GlobalListener::default()),
                Self::subs(),
            )
            .is_ok());
        assert!(app.active(&Id::FeedList).is_ok());
        app
    }

    /// ### subs
    ///
    /// Get application subscriptions
    fn subs() -> Vec<Sub<NoUserEvent>> {
        vec![
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Esc,
                    modifiers: KeyModifiers::NONE,
                }),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('r'),
                    modifiers: KeyModifiers::NONE,
                }),
                SubClause::Always,
            ),
        ]
    }
}
