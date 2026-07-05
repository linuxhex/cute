//! Role change modal for shared sessions.

use session_sharing_protocol::common::{ParticipantId, Role, RoleRequestId};
use cuteui::{AppContext, Element, Entity, View, ViewContext};

/// Source for opening the role change modal.
#[derive(Debug, Clone)]
pub enum RoleChangeOpenSource {
    ViewerRequest {
        role: Role,
    },
    SharerResponse {
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
        role: Role,
    },
    SharerGrant {
        participant_id: ParticipantId,
    },
}

/// Source for closing the role change modal.
#[derive(Debug, Clone, Copy)]
pub enum RoleChangeCloseSource {
    ViewerRequest,
    SharerResponse,
    SharerGrant,
}

/// Modal for changing participant roles in a shared session.
#[derive(Debug, Clone, Default)]
pub struct RoleChangeModal {
    is_open: bool,
}

impl RoleChangeModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self::default()
    }

    pub fn open_for_viewer_request(&mut self, _role: Role, _ctx: &mut ViewContext<Self>) {
        self.is_open = true;
    }

    pub fn open_for_sharer_response(
        &mut self,
        _participant_id: ParticipantId,
        _role_request_id: RoleRequestId,
        _role: Role,
        _ctx: &mut ViewContext<Self>,
    ) {
        self.is_open = true;
    }

    pub fn open_for_sharer_grant(&mut self, _participant_id: ParticipantId, _ctx: &mut ViewContext<Self>) {
        self.is_open = true;
    }

    pub fn close_for_viewer_request(&mut self, _ctx: &mut ViewContext<Self>) {
        self.is_open = false;
    }

    pub fn close_for_sharer_response(&mut self, _ctx: &mut ViewContext<Self>) {
        self.is_open = false;
    }

    pub fn close_for_sharer_grant(&mut self, _ctx: &mut ViewContext<Self>) {
        self.is_open = false;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

#[derive(Debug, Clone)]
pub enum RoleChangeModalEvent {
    CancelRequest {
        role_request_id: RoleRequestId,
    },
    ApproveRequest {
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
        role: Role,
    },
    DenyRequest {
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
    },
    Close {
        source: RoleChangeCloseSource,
    },
    CancelGrant,
    GrantRole {
        participant_id: ParticipantId,
        role: Role,
    },
}

impl Entity for RoleChangeModal {
    type Event = RoleChangeModalEvent;
}

impl View for RoleChangeModal {
    fn ui_name() -> &'static str {
        "RoleChangeModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        // Placeholder implementation - actual rendering would be done here
        Box::new(cuteui::elements::Empty::new())
    }
}
