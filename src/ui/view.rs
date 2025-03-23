use std::time::{Duration, Instant};

use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{PropPayload, PropValue};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::ratatui::widgets::Clear;
use tuirealm::{
    Application, AttrValue, Attribute, EventListenerCfg, NoUserEvent, Sub, SubClause,
    SubEventClause,
};

use super::components::*;
use super::{FlatFeedState, Id, Kiosk, Msg, Ui};
use crate::config::Config;
use crate::feed::{Article, Feed};
use crate::helpers::{strings as str_helpers, ui as ui_helpers};

static mut SUMMARY_WIDTH: usize = 0;
const CROSSTERM_MAX_POLL: usize = 10;

/// Article view components
struct ArticleView<'a> {
    authors: ArticleAuthors,
    date: ArticleDate,
    link: ArticleLink,
    summary: ArticleSummary<'a>,
    title: ArticleTitle,
}

impl From<&Article> for ArticleView<'_> {
    fn from(article: &Article) -> Self {
        unsafe {
            Self {
                authors: ArticleAuthors::new(article.authors.as_ref()),
                date: ArticleDate::new(article.date),
                link: ArticleLink::new(article.url.as_str()),
                summary: ArticleSummary::new(article.summary.as_str(), SUMMARY_WIDTH),
                title: ArticleTitle::new(article.title.as_deref().unwrap_or("")),
            }
        }
    }
}

impl Ui {
    /// Initialize application
    pub(super) fn init_application(
        kiosk: &Kiosk,
        ticks: Duration,
    ) -> Application<Id, Msg, NoUserEvent> {
        let mut app = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(ticks, CROSSTERM_MAX_POLL)
                .poll_timeout(ticks),
        );
        assert!(
            app.mount(Id::FeedList, Box::new(Self::get_feed_list(kiosk)), vec![])
                .is_ok()
        );
        assert!(
            app.mount(Id::ArticleList, Box::new(ArticleList::new(&[])), vec![])
                .is_ok()
        );
        assert!(
            app.mount(
                Id::GlobalListener,
                Box::new(GlobalListener::default()),
                Self::subs(),
            )
            .is_ok()
        );
        assert!(app.active(&Id::FeedList).is_ok());

        app
    }

    pub(super) fn view(&mut self) {
        self.last_redraw = Instant::now();
        self.redraw = false;
        assert!(
            self.terminal
                .raw_mut()
                .draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .margin(1)
                        .constraints(
                            [
                                Constraint::Percentage(50), // Lists
                                Constraint::Percentage(50), // Article
                            ]
                            .as_ref(),
                        )
                        .split(f.area());

                    // Render layout only if kiosk has been initialized
                    // -- list
                    let list_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .horizontal_margin(2)
                        .constraints(
                            [Constraint::Percentage(30), Constraint::Percentage(70)].as_ref(),
                        )
                        .split(chunks[0]);
                    self.application.view(&Id::FeedList, f, list_chunks[0]);
                    self.application.view(&Id::ArticleList, f, list_chunks[1]);
                    // -- article
                    let article_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Length(3), // Title
                                Constraint::Length(1), // Authors + date
                                Constraint::Min(6),    // Summary
                                Constraint::Length(1), // Link
                            ]
                            .as_ref(),
                        )
                        .split(chunks[1]);
                    let second_article_row = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(60), Constraint::Percentage(40)].as_ref(),
                        )
                        .split(article_chunks[1]);
                    self.application
                        .view(&Id::ArticleTitle, f, article_chunks[0]);
                    self.application
                        .view(&Id::ArticleAuthors, f, second_article_row[0]);
                    self.application
                        .view(&Id::ArticleDate, f, second_article_row[1]);
                    // Update summary width
                    unsafe {
                        SUMMARY_WIDTH = (article_chunks[2].width as usize).saturating_sub(4);
                    }
                    self.application
                        .view(&Id::ArticleSummary, f, article_chunks[2]);
                    self.application
                        .view(&Id::ArticleLink, f, article_chunks[3]);
                    // -- popups
                    if self.application.mounted(&Id::QuitPopup) {
                        let popup = ui_helpers::draw_area_in(f.area(), 30, 10);
                        f.render_widget(Clear, popup);
                        self.application.view(&Id::QuitPopup, f, popup);
                    } else if self.application.mounted(&Id::ErrorPopup) {
                        let popup = ui_helpers::draw_area_in(f.area(), 50, 15);
                        f.render_widget(Clear, popup);
                        self.application.view(&Id::ErrorPopup, f, popup);
                    }
                })
                .is_ok()
        );
    }

    // -- components

    /// Mount error and give focus to it
    pub(super) fn mount_error_popup(&mut self, err: impl ToString) {
        assert!(
            self.application
                .remount(
                    Id::ErrorPopup,
                    Box::new(ErrorPopup::new(err.to_string())),
                    vec![]
                )
                .is_ok()
        );
        assert!(self.application.active(&Id::ErrorPopup).is_ok());
    }

    pub(super) fn umount_error_popup(&mut self) {
        let _ = self.application.umount(&Id::ErrorPopup);
    }

    /// Mount quit popup
    pub(super) fn mount_quit_popup(&mut self) {
        assert!(
            self.application
                .remount(Id::QuitPopup, Box::new(QuitPopup::default()), vec![])
                .is_ok()
        );
        assert!(self.application.active(&Id::QuitPopup).is_ok());
    }

    pub(super) fn umount_quit_popup(&mut self) {
        let _ = self.application.umount(&Id::QuitPopup);
    }

    /// Returns whether article list is empty
    pub(super) fn is_article_list_empty(&self) -> bool {
        self.application
            .query(&Id::ArticleList, Attribute::Content)
            .ok()
            .flatten()
            .map(|x| x.unwrap_table().is_empty())
            .unwrap_or(true)
    }

    /// Update feed list item
    pub(super) fn update_feed_list_item(&mut self, name: &str, state: FlatFeedState) {
        // Update item
        let state = match state {
            FlatFeedState::Error => lists::FEED_STATE_ERROR,
            FlatFeedState::Loading => lists::FEED_STATE_LOADING,
            FlatFeedState::Success => lists::FEED_STATE_SUCCESS,
        };
        let prop_value = AttrValue::Payload(PropPayload::Tup2((
            PropValue::Str(name.to_string()),
            PropValue::U8(state),
        )));
        assert!(
            self.application
                .attr(
                    &Id::FeedList,
                    Attribute::Custom(lists::FEED_LIST_PROP_ITEMS),
                    prop_value
                )
                .is_ok()
        );
    }

    /// Initialize article list entries and article.
    /// This function should be called only if article list is empty
    pub(super) fn init_article(&mut self, config: &Config) {
        let Some(source) = self.sorted_sources().first().cloned() else {
            return;
        };
        let Some(feed) = self.kiosk.get_feed(source.as_str()) else {
            return;
        };

        assert!(
            self.application
                .remount(
                    Id::ArticleList,
                    Box::new(self.get_article_list(config, feed, self.max_article_name_len())),
                    vec![]
                )
                .is_ok()
        );
        // Mount first article
        if let Some(article) = feed.articles().next() {
            let ArticleView {
                authors,
                date,
                link,
                summary,
                title,
            } = article.into();
            assert!(
                self.application
                    .remount(Id::ArticleAuthors, Box::new(authors), vec![])
                    .is_ok()
            );
            assert!(
                self.application
                    .remount(Id::ArticleDate, Box::new(date), vec![])
                    .is_ok()
            );
            assert!(
                self.application
                    .remount(Id::ArticleLink, Box::new(link), vec![])
                    .is_ok()
            );
            assert!(
                self.application
                    .remount(Id::ArticleSummary, Box::new(summary), vec![])
                    .is_ok()
            );
            assert!(
                self.application
                    .remount(Id::ArticleTitle, Box::new(title), vec![])
                    .is_ok()
            );
        }
    }

    /// Update the current article list
    pub(super) fn get_article_list(
        &self,
        config: &Config,
        feed: &Feed,
        max_title_len: usize,
    ) -> ArticleList {
        let articles: Vec<String> = feed
            .articles()
            .map(|x| Self::fmt_article_title_in_list(config, x, max_title_len))
            .collect();
        ArticleList::new(articles.as_slice())
    }

    /// Format article title in articles list
    fn fmt_article_title_in_list(
        config: &Config,
        article: &Article,
        max_title_len: usize,
    ) -> String {
        let article_title_config = config.article_title.unwrap_or_default();
        let mut title = String::new();
        // article date
        if article_title_config.show_timestamp && article.date.is_some() {
            title.push_str(format!("{} ", article.date.unwrap().to_rfc3339()).as_str());
        }
        if article_title_config.show_author && !article.authors.is_empty() {
            title.push_str(format!("({}) ", article.authors[0]).as_str());
        }
        let max_title_len = max_title_len.saturating_sub(title.len());
        if let Some(article_title) = article.title.as_deref() {
            title.push_str(&str_helpers::elide_string_at(article_title, max_title_len))
        }
        title
    }

    /// Get feed list component
    fn get_feed_list(kiosk: &Kiosk) -> FeedList {
        let mut sources = kiosk.get_state();
        sources.sort_by(|a, b| a.0.cmp(&b.0));
        FeedList::new(sources)
    }

    /// global listener subs
    pub(super) fn subs() -> Vec<Sub<Id, NoUserEvent>> {
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
                    modifiers: KeyModifiers::CONTROL,
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

    /// Get terminal width. If it fails to collect width, returns 65535
    fn terminal_width(&self) -> usize {
        self.terminal
            .raw()
            .size()
            .map(|x| x.width as usize)
            .unwrap_or(u16::MAX as usize)
    }

    pub(super) fn max_article_name_len(&self) -> usize {
        (self.terminal_width() / 2) - 9 // 50 % - margin - 1
    }

    /// Update article into view by index
    pub(super) fn update_article(&mut self, article: usize) {
        if let Some(feed) = self.get_selected_feed() {
            if let Some(article) = feed.articles().nth(article) {
                let ArticleView {
                    authors,
                    date,
                    link,
                    summary,
                    title,
                } = article.into();
                assert!(
                    self.application
                        .remount(Id::ArticleAuthors, Box::new(authors), vec![])
                        .is_ok()
                );
                assert!(
                    self.application
                        .remount(Id::ArticleDate, Box::new(date), vec![])
                        .is_ok()
                );
                assert!(
                    self.application
                        .remount(Id::ArticleLink, Box::new(link), vec![])
                        .is_ok()
                );
                assert!(
                    self.application
                        .remount(Id::ArticleSummary, Box::new(summary), vec![])
                        .is_ok()
                );
                assert!(
                    self.application
                        .remount(Id::ArticleTitle, Box::new(title), vec![])
                        .is_ok()
                );
            }
        }
    }
}
