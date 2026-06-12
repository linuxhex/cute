//! Stub for ssh_remote_server_choice_view after remote_server removal.

use warpui::{AppContext, Entity, View, ViewContext};

/// Stub SshRemoteServerChoiceView.
pub struct SshRemoteServerChoiceView {
    _phantom: (),
}

/// Stub event enum.
#[derive(Clone, Debug)]
pub enum SshRemoteServerChoiceViewEvent {
    Dismissed,
}

impl Entity for SshRemoteServerChoiceView {
    type Event = SshRemoteServerChoiceViewEvent;
}

impl View for SshRemoteServerChoiceView {
    fn ui_name() -> &'static str {
        "SshRemoteServerChoiceView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
        Box::new(warpui::elements::Empty::new())
    }
}
