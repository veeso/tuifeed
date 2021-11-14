//! # Article
//!
//! Components related to the article area

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
use super::Msg;
use crate::helpers::fmt as fmt_helpers;

use chrono::{DateTime, Local};
use tui_realm_stdlib::{Label, Paragraph};
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
    component: Paragraph,
}

impl ArticleSummary {
    pub fn new(summary: &str) -> Self {
        Self {
            component: Paragraph::default()
                .borders(
                    Borders::default()
                        .color(Color::LightCyan)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Reset)
                .title("Summary", Alignment::Left)
                .text(
                    summary
                        .split('\n')
                        .map(TextSpan::from)
                        .collect::<Vec<TextSpan>>()
                        .as_slice(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ArticleSummary {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
