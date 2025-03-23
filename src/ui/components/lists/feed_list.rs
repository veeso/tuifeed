//! # Feed list
//!
//! Mock component to implement the feed list

use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, BorderType, Borders, Color, Style, TextModifiers, TextSpan,
};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::text::{Line, Span};
use tuirealm::ratatui::widgets::{List as TuiList, ListDirection, ListItem, ListState};
use tuirealm::{Frame, MockComponent, State};

use crate::ui::lib::FlatFeedState;

const SEQUENCE: [char; 8] = ['⣾', '⣽', '⣻', '⢿', '⡿', '⣟', '⣯', '⣷'];
pub const FEED_LIST_PROP_ITEMS: &str = "items";

pub const FEED_STATE_ERROR: u8 = 1;
pub const FEED_STATE_LOADING: u8 = 2;
pub const FEED_STATE_SUCCESS: u8 = 0;

#[derive(Default)]
struct OwnStates {
    step: usize,
}

impl OwnStates {
    pub fn step(&mut self) -> char {
        let ch = SEQUENCE.get(self.step).cloned().unwrap_or(' ');
        // Incr step
        if self.step + 1 >= SEQUENCE.len() {
            self.step = 0;
        } else {
            self.step += 1;
        }
        ch
    }
}

/// A list which prepends the fetch state for each source for the feed
pub struct FeedList {
    list: List,
    items: Vec<(String, FlatFeedState)>,
    states: OwnStates,
}

impl FeedList {
    pub fn new(items: Vec<(String, FlatFeedState)>) -> Self {
        Self {
            list: List::default()
                .highlighted_color(Color::LightBlue)
                .highlighted_str("➤ ")
                .rewind(true)
                .scroll(true)
                .step(4)
                .title("Feed", Alignment::Center)
                .borders(
                    Borders::default()
                        .color(Color::LightBlue)
                        .modifiers(BorderType::Rounded),
                )
                .rows((0..items.len()).map(|_| vec![TextSpan::new("")]).collect()),
            items,
            states: OwnStates::default(),
        }
    }

    fn feed_state_to_span(state: &FlatFeedState, loading_step: char) -> Span {
        match *state {
            FlatFeedState::Success => Span::from("  "),
            FlatFeedState::Loading => Span::from(format!("{} ", loading_step)),
            FlatFeedState::Error => Span::styled(
                "✘ ",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(TextModifiers::BOLD),
            ),
        }
    }
}

impl MockComponent for FeedList {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let focus = self
            .query(Attribute::Focus)
            .unwrap_or(AttrValue::Flag(false))
            .unwrap_flag();
        let div = tui_realm_stdlib::utils::get_block(
            Borders::default()
                .color(Color::LightBlue)
                .modifiers(BorderType::Rounded),
            Some(("Feed".to_string(), Alignment::Center)),
            focus,
            None,
        );
        let step = self.states.step();
        // Make list entries
        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .map(|(name, state)| {
                ListItem::new(Line::default().spans(vec![
                    Self::feed_state_to_span(state, step),
                    Span::from(name.as_str()),
                ]))
            })
            .collect();
        let (fg, bg): (Color, Color) = match focus {
            true => (Color::Reset, Color::LightBlue),
            false => (Color::LightBlue, Color::Reset),
        };
        // Make list
        let list = TuiList::new(list_items)
            .block(div)
            .direction(ListDirection::TopToBottom)
            .highlight_style(Style::default().fg(fg).bg(bg))
            .highlight_symbol("➤ ");
        let mut state: ListState = ListState::default();
        state.select(Some(self.list.states.list_index));
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.list.query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if matches!(attr, Attribute::Custom(FEED_LIST_PROP_ITEMS)) {
            let (name, state) = value.unwrap_payload().unwrap_tup2();
            let name = name.unwrap_str();
            let state = state.unwrap_u8();
            let state = match state {
                FEED_STATE_ERROR => FlatFeedState::Error,
                FEED_STATE_LOADING => FlatFeedState::Loading,
                FEED_STATE_SUCCESS => FlatFeedState::Success,
                _ => panic!("Invalid state {}", state),
            };
            for (i_name, i_state) in self.items.iter_mut() {
                if i_name == &name {
                    *i_state = state;
                    break;
                }
            }
        } else {
            self.list.attr(attr, value)
        }
    }

    fn state(&self) -> State {
        self.list.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.list.perform(cmd)
    }
}
