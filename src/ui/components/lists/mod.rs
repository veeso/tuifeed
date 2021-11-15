//! # Lists
//!
//! Components related to the lists area

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
mod feed_list;

pub use feed_list::{
    FEED_LIST_PROP_ITEMS, FEED_STATE_ERROR, FEED_STATE_LOADING, FEED_STATE_SUCCESS,
};

use crate::ui::lib::FlatFeedState;

use super::Msg;

use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

#[derive(MockComponent)]
pub struct FeedList {
    component: feed_list::FeedList,
}

impl FeedList {
    pub fn new(sources: Vec<(String, FlatFeedState)>) -> Self {
        Self {
            component: feed_list::FeedList::new(sources),
        }
    }
}

impl Component<Msg, NoUserEvent> for FeedList {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd_result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::FeedListBlur),
            _ => return None,
        };
        if let CmdResult::Changed(State::One(StateValue::Usize(index))) = cmd_result {
            Some(Msg::FeedChanged(index))
        } else {
            Some(Msg::None)
        }
    }
}

#[derive(MockComponent)]
pub struct ArticleList {
    component: List,
}

impl ArticleList {
    pub fn new(articles: &[String]) -> Self {
        Self {
            component: List::default()
                .highlighted_color(Color::LightCyan)
                .highlighted_str("➤ ")
                .rewind(true)
                .scroll(true)
                .step(4)
                .title("Articles", Alignment::Center)
                .borders(
                    Borders::default()
                        .color(Color::LightCyan)
                        .modifiers(BorderType::Rounded),
                )
                .rows(
                    articles
                        .iter()
                        .map(|x| vec![TextSpan::from(x.as_str())])
                        .collect(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ArticleList {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd_result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::ArticleListBlur),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => return Some(Msg::OpenArticle),
            _ => return None,
        };
        if let CmdResult::Changed(State::One(StateValue::Usize(index))) = cmd_result {
            Some(Msg::ArticleChanged(index))
        } else {
            Some(Msg::None)
        }
    }
}
