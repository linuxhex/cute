//! Stub for ssh_remote_server_failed_banner after remote_server removal.

use warp_core::SessionId;
use warpui::{AppContext, Entity, View, TypedActionView};

use crate::remote_server::transport::UserFacingError;

/// Stub SshRemoteServerFailedBanner.
pub struct SshRemoteServerFailedBanner {
    session_id: SessionId,
}

/// Stub event enum.
#[derive(Clone, Debug)]
pub enum SshRemoteServerFailedBannerEvent {
    Dismissed,
}

/// Stub action for TypedActionView.
#[derive(Clone, Debug)]
pub struct SshRemoteServerFailedBannerAction;

impl SshRemoteServerFailedBanner {
    pub fn new(session_id: SessionId, _error: UserFacingError) -> Self {
        Self { session_id }
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }
}

impl Entity for SshRemoteServerFailedBanner {
    type Event = SshRemoteServerFailedBannerEvent;
}

impl View for SshRemoteServerFailedBanner {
    fn ui_name() -> &'static str {
        "SshRemoteServerFailedBanner"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
        Box::new(warpui::elements::Empty::new())
    }
}

impl TypedActionView for SshRemoteServerFailedBanner {
    type Action = SshRemoteServerFailedBannerAction;
}
