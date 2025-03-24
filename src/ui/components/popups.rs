//! # Popups
//!
//! Popups components

use tui_realm_stdlib::{Paragraph, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextModifiers, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

use super::Msg;

#[derive(MockComponent)]
pub struct QuitPopup {
    component: Radio,
}

impl Default for QuitPopup {
    fn default() -> Self {
        Self {
            component: Radio::default()
                .foreground(Color::Yellow)
                .borders(
                    Borders::default()
                        .color(Color::Yellow)
                        .modifiers(BorderType::Rounded),
                )
                .title("Are sure you want to quit?", Alignment::Center)
                .rewind(true)
                .choices(&["Yes", "No"])
                .value(0),
        }
    }
}

impl Component<Msg, NoUserEvent> for QuitPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd_result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => self.perform(Cmd::Submit),
            _ => return None,
        };
        if matches!(
            cmd_result,
            CmdResult::Submit(State::One(StateValue::Usize(0)))
        ) {
            Some(Msg::Quit)
        } else if matches!(
            cmd_result,
            CmdResult::Submit(State::One(StateValue::Usize(1)))
        ) {
            Some(Msg::CloseQuitPopup)
        } else {
            Some(Msg::None)
        }
    }
}

#[derive(MockComponent)]
pub struct ErrorPopup {
    component: Paragraph,
}

impl ErrorPopup {
    pub fn new<S: AsRef<str>>(msg: S) -> Self {
        Self {
            component: Paragraph::default()
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Red)
                .modifiers(TextModifiers::BOLD)
                .alignment(Alignment::Center)
                .text(vec![TextSpan::from(msg.as_ref().to_string())].as_slice()),
        }
    }
}

impl Component<Msg, NoUserEvent> for ErrorPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter | Key::Esc,
                ..
            }) => Some(Msg::CloseErrorPopup),
            _ => None,
        }
    }
}
