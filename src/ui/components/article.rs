//! # Article
//!
//! Components related to the article area

use super::Msg;
use crate::helpers::fmt as fmt_helpers;
use crate::helpers::strings as str_helpers;

use chrono::{DateTime, Local};
use tui_realm_stdlib::{Label, Paragraph, Textarea};
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{
    Alignment, BorderSides, BorderType, Borders, Color, TextModifiers, TextSpan,
};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

#[derive(MockComponent)]
pub struct ArticleTitle {
    component: Paragraph,
}

impl ArticleTitle {
    pub fn new(title: &str) -> Self {
        Self {
            component: Paragraph::default()
                .borders(Borders::default().sides(BorderSides::empty()))
                .foreground(Color::LightYellow)
                .modifiers(TextModifiers::BOLD)
                .text(&[TextSpan::from(title)]),
        }
    }
}

impl Component<Msg, NoUserEvent> for ArticleTitle {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct ArticleDate {
    component: Label,
}

impl ArticleDate {
    pub fn new(datetime: Option<DateTime<Local>>) -> Self {
        Self {
            component: Label::default()
                .foreground(Color::LightGreen)
                .modifiers(TextModifiers::BOLD | TextModifiers::ITALIC)
                .text(
                    datetime
                        .map(|x| fmt_helpers::format_datetime(x, "%A %d %B %Y, %H:%M"))
                        .unwrap_or_default(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ArticleDate {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct ArticleAuthors {
    component: Label,
}

impl ArticleAuthors {
    pub fn new(authors: &[String]) -> Self {
        Self {
            component: Label::default()
                .foreground(Color::LightGreen)
                .modifiers(TextModifiers::BOLD | TextModifiers::ITALIC)
                .text(authors.join(", ")),
        }
    }
}

impl Component<Msg, NoUserEvent> for ArticleAuthors {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct ArticleLink {
    component: Label,
}

impl ArticleLink {
    pub fn new(url: &str) -> Self {
        Self {
            component: Label::default().modifiers(TextModifiers::BOLD).text(url),
        }
    }
}

impl Component<Msg, NoUserEvent> for ArticleLink {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct ArticleSummary {
    component: Textarea,
}

impl ArticleSummary {
    pub fn new(summary: &str) -> Self {
        Self {
            component: Textarea::default()
                .borders(
                    Borders::default()
                        .color(Color::LightCyan)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Reset)
                .title("Summary", Alignment::Left)
                .step(4)
                .highlighted_str("• ")
                .text_rows(Self::make_summary_rows(summary).as_slice()),
        }
    }

    /// ### make_summary_rows
    ///
    /// Make summary rows
    fn make_summary_rows(summary: &str) -> Vec<TextSpan> {
        let summary =
            str_helpers::replace_multiple_newlines(summary.trim_matches('\n').trim(), "\n");
        // Split summary by newline
        summary.split('\n').map(TextSpan::from).collect()
    }
}

impl Component<Msg, NoUserEvent> for ArticleSummary {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Scroll(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => {
                // Scroll twice
                self.perform(Cmd::Scroll(Direction::Down));
                self.perform(Cmd::Scroll(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => {
                // Scroll twice
                self.perform(Cmd::Scroll(Direction::Up));
                self.perform(Cmd::Scroll(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => Some(Msg::ArticleBlur),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::OpenArticle),
            _ => None,
        }
    }
}
