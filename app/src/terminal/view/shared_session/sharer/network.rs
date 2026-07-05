//! Network model for sharer in shared sessions.

use std::sync::Arc;
use parking_lot::FairMutex;

use cuteui::{Entity, ModelContext, SingletonEntity};
use session_sharing_protocol::common::{
    ActivePrompt, ControlAction, ControlActionFailureReason,
    CommandExecutionFailureReason, ParticipantId, ParticipantList, ParticipantPresenceUpdate,
    Role, RoleRequestId, SessionId, WindowSize,
    UniversalDeveloperInputContextUpdate, WriteToPtyFailureReason, AgentPromptFailureReason,
    UniversalDeveloperInputContext,
};
use session_sharing_protocol::sharer::{
    FailedToInitializeSessionReason, Lifetime, SessionEndedReason, TeamAccessLevelUpdateResponse,
    LinkAccessLevelUpdateResponse, AddGuestsResponse, RemoveGuestResponse,
    UpdatePendingUserRoleResponse,
};

use crate::terminal::model::BlockId;
use crate::terminal::shared_session::{
    SharedSessionScrollbackType, SharedSessionSource, SharedSessionActionSource,
};
use crate::terminal::TerminalModel;
use crate::editor::{CrdtOperation, ReplicaId};

/// Stub types for request IDs that may not exist in session_sharing_protocol
pub type RequestId = u64;
pub type WriteToPtyRequestId = u64;
pub type AgentPromptRequestId = u64;

/// Stub type for agent prompt request
#[derive(Debug, Clone, Default)]
pub struct AgentPromptRequest {
    pub prompt: String,
    pub server_conversation_token: Option<session_sharing_protocol::common::ServerConversationToken>,
    pub attachments: Vec<session_sharing_protocol::common::AgentAttachment>,
}

/// Events emitted by the Network model.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum NetworkEvent {
    SharedSessionCreatedSuccessfully {
        session_id: Box<SessionId>,
        sharer_id: ParticipantId,
        sharer_firebase_uid: Box<String>,
    },
    FailedToCreateSharedSession {
        reason: FailedToInitializeSessionReason,
        cause: Option<String>,
    },
    SessionTerminated {
        reason: SessionEndedReason,
    },
    Reconnecting,
    ReconnectedSuccessfully,
    FailedToReconnect,
    ControlActionRequested {
        participant_id: ParticipantId,
        request_id: RequestId,
        action: ControlAction,
    },
    ParticipantListUpdated(Box<ParticipantList>),
    ParticipantPresenceUpdated(Box<ParticipantPresenceUpdate>),
    RoleRequested {
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
        role: Box<Role>,
    },
    RoleRequestCancelled {
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
    },
    ParticipantRoleChanged {
        participant_id: ParticipantId,
        role: Box<Role>,
    },
    InputUpdated {
        block_id: BlockId,
        operations: Vec<CrdtOperation>,
    },
    CommandExecutionRequested {
        id: RequestId,
        participant_id: ParticipantId,
        block_id: BlockId,
        command: String,
    },
    WriteToPtyRequested {
        id: WriteToPtyRequestId,
        participant_id: ParticipantId,
        bytes: Vec<u8>,
    },
    AgentPromptRequested {
        id: AgentPromptRequestId,
        participant_id: ParticipantId,
        request: AgentPromptRequest,
    },
    LinkAccessLevelUpdateResponse {
        response: LinkAccessLevelUpdateResponse,
    },
    TeamAccessLevelUpdateResponse {
        response: TeamAccessLevelUpdateResponse,
    },
    AddGuestsResponse {
        response: AddGuestsResponse,
    },
    RemoveGuestResponse {
        response: RemoveGuestResponse,
    },
    UpdatePendingUserRoleResponse {
        response: UpdatePendingUserRoleResponse,
    },
    ViewerTerminalSizeReported {
        window_size: WindowSize,
    },
    UniversalDeveloperInputContextUpdated(Box<UniversalDeveloperInputContextUpdate>),
}

/// Network model that handles shared session connections for the sharer.
#[derive(Debug, Clone, Default)]
pub struct Network {
    is_connected: bool,
    max_session_size: usize,
}

impl Network {
    #[cfg(any(test, feature = "integration_tests"))]
    pub fn new_for_test(
        _model: Arc<FairMutex<TerminalModel>>,
        _events_rx: async_channel::Receiver<NetworkEvent>,
        _scrollback_type: SharedSessionScrollbackType,
        _active_prompt: ActivePrompt,
        _selection: session_sharing_protocol::common::Selection,
        _input_replica_id: u64,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self::default()
    }

    #[cfg(not(any(test, feature = "integration_tests")))]
    pub fn new(
        _model: Arc<FairMutex<TerminalModel>>,
        _events_rx: async_channel::Receiver<NetworkEvent>,
        _scrollback_type: SharedSessionScrollbackType,
        _active_prompt: ActivePrompt,
        _selection: session_sharing_protocol::common::Selection,
        _input_replica_id: ReplicaId,
        _terminal_view_id: cuteui::EntityId,
        _universal_developer_input_context: UniversalDeveloperInputContext,
        _lifetime: Lifetime,
        _source: SharedSessionSource,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self::default()
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub fn connect(&mut self) {
        self.is_connected = true;
    }

    pub fn disconnect(&mut self) {
        self.is_connected = false;
    }

    pub fn max_session_size(&self) -> usize {
        self.max_session_size
    }

    pub fn end_session(&mut self, _reason: SessionEndedReason) {
        self.disconnect();
    }

    // Stub methods for sending updates
    pub fn send_active_prompt_update_if_changed(&mut self, _prompt: ActivePrompt) {}
    pub fn send_universal_developer_input_context_update(&mut self, _update: UniversalDeveloperInputContextUpdate) {}
    pub fn send_input_update<'a>(&mut self, _block_id: BlockId, _operations: impl Iterator<Item = &'a CrdtOperation>) {}
    pub fn send_presence_selection_if_changed(&mut self, _selection: session_sharing_protocol::common::Selection) {}
    pub fn send_role_update(&mut self, _participant_id: ParticipantId, _role: Role) {}
    pub fn send_user_role_update(&mut self, _user_uid: String, _role: Role) {}
    pub fn send_pending_user_role_update(&mut self, _email: String, _role: Role) {}
    pub fn send_add_guests(&mut self, _emails: Vec<String>, _role: Role) {}
    pub fn send_remove_guest(&mut self, _user_uid: String) {}
    pub fn send_remove_pending_guest(&mut self, _email: String) {}
    pub fn send_make_all_participants_readers(&mut self, _reason: SharedSessionActionSource) {}
    pub fn send_role_request_response(&mut self, _participant_id: ParticipantId, _role_request_id: RoleRequestId, _response: session_sharing_protocol::common::RoleRequestResponse) {}
    pub fn send_link_permission_update(&mut self, _role: Role) {}
    pub fn send_team_permission_update(&mut self, _role: Role, _team_uid: String) {}
    pub fn send_control_action_rejection(&mut self, _participant_id: ParticipantId, _request_id: RequestId, _reason: ControlActionFailureReason) {}
    pub fn send_command_execution_rejection(&mut self, _id: RequestId, _participant_id: ParticipantId, _reason: CommandExecutionFailureReason) {}
    pub fn send_write_to_pty_rejection(&mut self, _id: WriteToPtyRequestId, _reason: WriteToPtyFailureReason) {}
    pub fn send_agent_prompt_rejection(&mut self, _id: AgentPromptRequestId, _participant_id: ParticipantId, _reason: AgentPromptFailureReason) {}
}

impl Entity for Network {
    type Event = NetworkEvent;
}

impl SingletonEntity for Network {}
