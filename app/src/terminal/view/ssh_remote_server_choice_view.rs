//! Stub for ssh_remote_server_choice_view after remote_server removal.

use cute_core::SessionId;
use cuteui::{AppContext, Entity, View, ViewContext, TypedActionView};

/// Stub SshRemoteServerChoiceView.
pub struct SshRemoteServerChoiceView {
    session_id: SessionId,
}

/// Stub event enum.
#[derive(Clone, Debug)]
pub enum SshRemoteServerChoiceViewEvent {
    Dismissed,
    Install,
    Skip,
    OpenWarpifySettings,
}

/// Stub action for TypedActionView.
#[derive(Clone, Debug)]
pub struct SshRemoteServerChoiceViewAction;

impl SshRemoteServerChoiceView {
    pub fn new(session_id: SessionId, _ctx: &ViewContext<Self>) -> Self {
        Self { session_id }
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }
}

impl Entity for SshRemoteServerChoiceView {
    type Event = SshRemoteServerChoiceViewEvent;
}

impl View for SshRemoteServerChoiceView {
    fn ui_name() -> &'static str {
        "SshRemoteServerChoiceView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn cuteui::Element> {
        Box::new(cuteui::elements::Empty::new())
    }
}

impl TypedActionView for SshRemoteServerChoiceView {
    type Action = SshRemoteServerChoiceViewAction;
}
