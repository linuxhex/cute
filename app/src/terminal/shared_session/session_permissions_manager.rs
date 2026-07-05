//! Session permissions manager for shared sessions.

use cuteui::{Entity, ModelContext, SingletonEntity};
use session_sharing_protocol::common::{Role, SessionId};

/// Events emitted by the SessionPermissionsManager.
#[derive(Debug, Clone)]
pub enum SessionPermissionsManagerEvent {
    PermissionsUpdated,
}

/// Manages permissions for shared sessions.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct SessionPermissionsManager {
    sessions: Vec<()>,
}

impl SessionPermissionsManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn updated_guests(
        &mut self,
        _ctx: &mut ModelContext<Self>,
        _session_id: SessionId,
        _guests: Vec<session_sharing_protocol::common::ParticipantInfo>,
        _pending_guests: Vec<session_sharing_protocol::common::PendingParticipantInfo>,
    ) {
        // TODO: Implement proper guest tracking
    }

    pub fn updated_link_permissions(
        &mut self,
        _ctx: &mut ModelContext<Self>,
        _session_id: SessionId,
        _role: Role,
    ) {
        // TODO: Implement proper link permissions tracking
    }

    pub fn updated_team_permissions(
        &mut self,
        _ctx: &mut ModelContext<Self>,
        _session_id: SessionId,
        _team_acl: session_sharing_protocol::sharer::TeamAcl,
    ) {
        // TODO: Implement proper team permissions tracking
    }
}

impl Entity for SessionPermissionsManager {
    type Event = SessionPermissionsManagerEvent;
}

impl SingletonEntity for SessionPermissionsManager {}
