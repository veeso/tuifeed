//! # Components
//!
//! Components for tuifeed

use super::Msg;

mod article;
pub mod lists;
mod popups;

pub use article::{ArticleAuthors, ArticleDate, ArticleLink, ArticleSummary, ArticleTitle};
pub use lists::{ArticleList, FeedList};
pub use popups::{ErrorPopup, QuitPopup};

use tui_realm_stdlib::Phantom;
use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    Component, Event, MockComponent, NoUserEvent,
};

#[derive(Default, MockComponent)]
pub struct GlobalListener {
    component: Phantom,
}

impl Component<Msg, NoUserEvent> for GlobalListener {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Msg::ShowQuitPopup),
            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::FetchAllSources),
            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                ..
            }) => Some(Msg::FetchSource),
            _ => None,
        }
    }
}
