//! Share session modal for initiating shared sessions.

use session_sharing_protocol::common::SessionId;
use cuteui::{AppContext, Element, Entity, View, ViewContext};

/// Modal for sharing a terminal session.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct ShareSessionModal {
    is_open: bool,
    session_id: Option<SessionId>,
}

impl ShareSessionModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self::default()
    }

    pub fn open(&mut self, _ctx: &mut ViewContext<Self>) {
        self.is_open = true;
    }

    pub fn close(&mut self, _ctx: &mut ViewContext<Self>) {
        self.is_open = false;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

#[derive(Debug, Clone)]
pub enum ShareSessionModalEvent {
    Close,
    StartSharing {
        scrollback_type: crate::terminal::shared_session::SharedSessionScrollbackType,
        source: crate::terminal::shared_session::SharedSessionSource,
    },
    Upgrade,
}

impl Entity for ShareSessionModal {
    type Event = ShareSessionModalEvent;
}

impl View for ShareSessionModal {
    fn ui_name() -> &'static str {
        "ShareSessionModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        // Placeholder implementation - actual rendering would be done here
        Box::new(cuteui::elements::Empty::new())
    }
}
