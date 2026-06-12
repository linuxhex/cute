//! Stub for ssh_remote_server_failed_banner after remote_server removal.

use warpui::{AppContext, Entity, View, ViewContext};

/// Stub SshRemoteServerFailedBanner.
pub struct SshRemoteServerFailedBanner {
    _phantom: (),
}

/// Stub event enum.
#[derive(Clone, Debug)]
pub enum SshRemoteServerFailedBannerEvent {
    Dismissed,
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
