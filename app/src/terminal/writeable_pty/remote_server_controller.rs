//! Stub for remote_server_controller after remote_server removal.

use std::sync::Arc;

use parking_lot::Mutex;
use warpui::{AppContext, ModelContext};

use super::message::Message;
use crate::terminal::ModelEventDispatcher;

/// Stub RemoteServerController.
pub struct RemoteServerController<T> {
    _sender: T,
    _phantom: (),
}

impl<T: Clone + Send + 'static> RemoteServerController<T> {
    pub fn new(sender: T, _ctx: &mut AppContext) -> Self {
        Self {
            _sender: sender,
            _phantom: (),
        }
    }

    pub fn handle_ssh_remote_server_install(&mut self, _session_id: u64, _ctx: &mut AppContext) {
        // No-op
    }

    pub fn handle_ssh_remote_server_skip(&mut self, _session_id: u64, _ctx: &mut AppContext) {
        // No-op
    }
}

/// Creates a RemoteServerController and registers it as a model.
pub fn init_remote_server_controller<T: Clone + Send + 'static>(
    pty_controller: &warpui::ModelHandle<super::pty_controller::PtyController<T>>,
    model_events: &warpui::ModelHandle<ModelEventDispatcher>,
    ctx: &mut AppContext,
) -> warpui::ModelHandle<RemoteServerController<T>> {
    ctx.add_model(|ctx| {
        let sender = pty_controller.read(ctx).sender();
        RemoteServerController::new(sender, ctx)
    })
}

/// Returns a connection label from user and host.
pub fn connection_label_from_user_and_host(user: &str, host: &str) -> String {
    format!("{}@{}", user, host)
}
