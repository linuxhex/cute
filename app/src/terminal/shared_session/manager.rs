//! Manager for shared sessions.

use cuteui::{Entity, EntityId, ModelContext, SingletonEntity, WeakViewHandle, WindowId};

use crate::terminal::view::TerminalView;

/// Events emitted by the Manager.
#[derive(Debug, Clone)]
pub enum ManagerEvent {
    JoinedSession {
        session_id: session_sharing_protocol::common::SessionId,
        view_id: EntityId,
    },
}

/// Manages all shared sessions in the application.
#[derive(Debug, Clone, Default)]
pub struct Manager {
    sessions: Vec<()>,
}

impl Manager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_session(&mut self, _session_id: session_sharing_protocol::common::SessionId) {
    }

    pub fn unregister_session(&mut self, _session_id: &session_sharing_protocol::common::SessionId) {
    }

    pub fn has_session(&self, _session_id: &session_sharing_protocol::common::SessionId) -> bool {
        false
    }

    pub fn started_share(
        &mut self,
        _view: WeakViewHandle<TerminalView>,
        _session_id: session_sharing_protocol::common::SessionId,
        _window_id: WindowId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // TODO: Implement proper share tracking
    }

    pub fn stopped_share(
        &mut self,
        _view_id: EntityId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // TODO: Implement proper share tracking
    }

    pub fn share_failed(
        &mut self,
        _window_id: WindowId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // TODO: Implement proper failure handling
    }
}

impl Entity for Manager {
    type Event = ManagerEvent;
}

impl SingletonEntity for Manager {}
