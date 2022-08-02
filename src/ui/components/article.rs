//! # Article
//!
//! Components related to the article area

use super::Msg;
use crate::helpers::fmt as fmt_helpers;
use crate::helpers::strings as str_helpers;

use chrono::{DateTime, Local};
use tui_realm_stdlib::{Label, Paragraph};
use tui_realm_textarea::TextArea;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{
    Alignment, BorderSides, BorderType, Borders, Color, Style, TextModifiers, TextSpan,
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

pub struct ArticleSummary<'a> {
    component: TextArea<'a>,
}

impl<'a> MockComponent for ArticleSummary<'a> {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        self.component.view(frame, area);
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        self.component.attr(attr, value)
    }

    fn state(&self) -> tuirealm::State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> tuirealm::command::CmdResult {
        self.component.perform(cmd)
    }
}

impl<'a> ArticleSummary<'a> {
    pub fn new(summary: &str, width: usize) -> Self {
        Self {
            component: TextArea::new(Self::make_summary_rows(summary, width))
                .borders(
                    Borders::default()
                        .color(Color::LightCyan)
                        .modifiers(BorderType::Rounded),
                )
                .title("Summary", Alignment::Left)
                .cursor_style(Style::default())
                .scroll_step(4),
        }
    }

    /// Make summary rows
    fn make_summary_rows(summary: &str, width: usize) -> Vec<String> {
        let summary =
            str_helpers::replace_multiple_newlines(summary.trim_matches('\n').trim(), "\n");
        // Split summary by newline
        let mut lines = Vec::new();
        let mut summary_it = summary.chars();
        let mut acc = Vec::new();
        loop {
            match summary_it.next() {
                None => {
                    // End of summary
                    // push acc to lines
                    lines.push(acc.iter().collect());
                    break;
                }
                Some('\n') => {
                    lines.push(acc.iter().collect());
                    acc.clear();
                }
                Some(ch) => {
                    acc.push(ch);
                }
            }
            // if width has exceeded, push and clear
            if acc.len() >= width {
                lines.push(acc.iter().collect());
                acc.clear();
            }
        }
        lines
    }
}

impl<'a> Component<Msg, NoUserEvent> for ArticleSummary<'a> {
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
