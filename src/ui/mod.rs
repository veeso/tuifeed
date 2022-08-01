//! # Ui
//!
//! Ui for tuifeed

mod components;
mod lib;

use crate::config::Config;
use components::{ErrorPopup, GlobalListener};
use lib::{FeedClient, FeedState, FlatFeedState, Kiosk};

use std::time::Duration;
use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    props::{PropPayload, PropValue},
    terminal::TerminalBridge,
    Application, AttrValue, Attribute, EventListenerCfg, NoUserEvent, PollStrategy, Sub, SubClause,
    SubEventClause,
};

const FORCED_REDRAW_INTERVAL: Duration = Duration::from_millis(50);

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
}

/// Messages produced by components
#[derive(Debug, PartialEq)]
pub enum Msg {
    ArticleBlur,
    ArticleChanged(usize),
    ArticleListBlur,
    CloseErrorPopup,
    CloseQuitPopup,
    FeedChanged(usize),
    FeedListBlur,
    FetchSource,
    FetchAllSources,
    GoReadArticle,
    OpenArticle,
    Quit,
    ShowQuitPopup,
    None,
}

pub struct Ui {
    application: Application<Id, Msg, NoUserEvent>,
    config: Config,
    kiosk: Kiosk,
}

impl Ui {
    /// Init a new `Ui`
    pub fn init(config: Config, ticks: u64) -> Self {
        Self {
            application: Self::init_application(),
            config,
            kiosk: Kiosk::default(),
        }
    }
}
