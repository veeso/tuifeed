//! # Lists
//!
//! Components related to the lists area

mod feed_list;

pub use feed_list::{
    FEED_LIST_PROP_ITEMS, FEED_STATE_ERROR, FEED_STATE_LOADING, FEED_STATE_SUCCESS,
};
use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

use super::Msg;
use crate::ui::lib::FlatFeedState;

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
            Event::Keyboard(KeyEvent {
                code: Key::Tab | Key::Right,
                ..
            }) => return Some(Msg::FeedListBlur),
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
    pub fn new(articles: &[(String, bool)], selected_line: Option<usize>) -> Self {
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
                        .map(|(title, read)| {
                            let text = match read {
                                true => TextSpan::from(title.as_str()),
                                false => TextSpan::from(title.as_str()).reversed(),
                            };
                            vec![text]
                        })
                        .collect(),
                )
                .selected_line(selected_line.unwrap_or_default()),
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
            Event::Keyboard(KeyEvent {
                code: Key::Tab | Key::Left,
                ..
            }) => return Some(Msg::ArticleListBlur),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => return Some(Msg::GoReadArticle),
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
