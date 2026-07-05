use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::color;
use crate::cute_terminal;

/// Role for a shared session participant
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Role {
    Reader,
    Executor,
    /// Executor, and can change ACLs of others
    Full,
}

impl Role {
    /// Returns true if this role has execution permissions.
    pub fn can_execute(&self) -> bool {
        matches!(self, Role::Executor | Role::Full)
    }

    /// Downgrades `Full` to `Executor` for clients that don't support the Full role.
    pub fn downgrade_full(&mut self) {
        if *self == Role::Full {
            *self = Role::Executor;
        }
    }
}

impl Default for Role {
    fn default() -> Self {
        Self::Reader
    }
}

/// Profile data for a participant
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProfileData {
    pub firebase_uid: String,
    pub display_name: String,
    pub photo_url: Option<String>,
    pub email: Option<String>,
}

/// An ID for a shared session participant
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ParticipantId(String);

impl ParticipantId {
    pub fn new() -> ParticipantId {
        ParticipantId(Uuid::new_v4().to_string())
    }
}

impl Default for ParticipantId {
    fn default() -> Self {
        ParticipantId::new()
    }
}

impl std::fmt::Display for ParticipantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

/// An ID for a role request
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct RoleRequestId(String);

impl RoleRequestId {
    pub fn new() -> RoleRequestId {
        RoleRequestId(Uuid::new_v4().to_string())
    }
}

impl Default for RoleRequestId {
    fn default() -> Self {
        RoleRequestId::new()
    }
}

impl std::fmt::Display for RoleRequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

/// An ID for a shared session
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SessionId(String);

impl SessionId {
    pub fn new() -> SessionId {
        SessionId(Uuid::new_v4().to_string())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        SessionId::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::str::FromStr for SessionId {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SessionId(s.to_string()))
    }
}

/// Stub type for ResponseEvent
pub struct ResponseEvent {
    // TODO: Add fields
}

/// Stub type for PendingParticipantInfo
pub struct PendingParticipantInfo {
    // TODO: Add fields
}

/// Participant information
#[derive(Clone, Debug, Default)]
pub struct ParticipantInfo {
    pub id: ParticipantId,
    pub profile_data: ProfileData,
    pub selection: Selection,
    pub color: color::ColorU,
    pub role: Option<Role>,
    pub info: ProfileData,
    pub input_replica_id: String,
}

impl ParticipantInfo {
    pub fn id(&self) -> &ParticipantId {
        &self.id
    }

    pub fn input_replica_id(&self) -> &String {
        &self.input_replica_id
    }

    pub fn get_selected_block_index_for_avatar(&self, _block_list: &cute_terminal::model::BlockList) -> Option<usize> {
        None
    }
}

/// Participant list for a shared session
#[derive(Clone, Debug, Default)]
pub struct ParticipantList {
    pub sharer: Box<ParticipantInfo>,
    pub viewers: Vec<Box<ParticipantInfo>>,
    pub guests: Vec<GuestInfo>,
    pub pending_guests: Vec<PendingGuestInfo>,
    pub present_viewers: Vec<ParticipantInfo>,
}

/// Guest information
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GuestInfo {
    pub user_uid: String,
    pub role: Role,
}

/// Pending guest information
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PendingGuestInfo {
    pub email: String,
    pub role: Role,
}

/// Stub type for BlockId
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub struct BlockId(String);

impl std::fmt::Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl From<String> for BlockId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<BlockId> for String {
    fn from(value: BlockId) -> Self {
        value.0
    }
}

/// Stub type for BufferId
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub struct BufferId(String);

impl std::fmt::Display for BufferId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl From<String> for BufferId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<BufferId> for String {
    fn from(value: BufferId) -> Self {
        value.0
    }
}

impl From<BufferId> for BlockId {
    fn from(value: BufferId) -> Self {
        value.0.into()
    }
}

impl From<BlockId> for BufferId {
    fn from(value: BlockId) -> Self {
        value.0.into()
    }
}

/// Selection type for shared sessions
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Selection {
    #[default]
    None,
    Blocks {
        block_ids: Vec<BlockId>,
    },
    BlockText {
        block_id: BlockId,
        start: Point,
        end: Point,
        is_reversed: bool,
    },
    AltScreenText {
        start: Point,
        end: Point,
        is_reversed: bool,
    },
}

/// Point in a terminal
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

/// Window size
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct WindowSize {
    pub rows: usize,
    pub columns: usize,
}

/// Active prompt type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActivePrompt {
    PS1,
    WarpPrompt(String),
}

/// Input mode
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Terminal,
    Agent,
}

impl InputMode {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Input type
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum InputType {
    Text,
    AgentPrompt,
    Shell,
    AI,
}

/// Agent attachment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentAttachment {
    /// Reference to a terminal block
    BlockReference {
        block_id: String,
    },
    /// Plain text attachment
    PlainText {
        content: String,
    },
    /// Reference to an uploaded file
    FileReference {
        attachment_id: String,
        file_name: String,
    },
}

/// Server conversation token
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct ServerConversationToken(String);

impl std::fmt::Display for ServerConversationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl std::str::FromStr for ServerConversationToken {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ServerConversationToken(s.to_string()))
    }
}

impl ServerConversationToken {
    /// Creates a ServerConversationToken from a UUID string.
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        ServerConversationToken(uuid.to_string())
    }

    /// Returns the token as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Participant presence update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticipantPresenceUpdate {
    pub participant_id: ParticipantId,
    // TODO: Add other fields
}

/// Role request response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RoleRequestResponse {
    Approved { new_role: Role },
    Rejected { reason: RoleRequestRejectedReason },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RoleRequestRejectedReason {
    RejectedBySharer,
}

/// Agent prompt failure reason
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentPromptFailureReason {
    InsufficientPermissions,
    InvalidConversation,
}

/// CLI agent session state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CLIAgentSessionState {
    Active {
        cli_agent: String,
        is_rich_input_open: bool,
    },
    Inactive,
}

/// Command execution failure reason
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CommandExecutionFailureReason {
    StaleBuffer,
    InsufficientPermissions,
}

/// Control action for shared sessions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ControlAction {
    CancelConversation {
        server_conversation_token: ServerConversationToken,
    },
}

/// Control action failure reason
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ControlActionFailureReason {
    InsufficientPermissions,
}

/// Selected agent model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedAgentModel {
    pub model_id: String,
}

impl SelectedAgentModel {
    pub fn new(model_id: String) -> Self {
        Self { model_id }
    }
}

/// Universal developer input context update
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UniversalDeveloperInputContextUpdate {
    pub selected_model: Option<SelectedAgentModel>,
    pub input_mode: Option<InputMode>,
    pub auto_approve_agent_actions: Option<bool>,
    pub selected_conversation: Option<SelectedConversation>,
    pub long_running_command_agent_interaction_state: Option<LongRunningCommandAgentInteractionState>,
    pub cli_agent_session: Option<CLIAgentSessionState>,
}

/// Write to PTY failure reason
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WriteToPtyFailureReason {
    StaleBuffer,
    InsufficientPermissions,
}

/// Long running command agent interaction state
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LongRunningCommandAgentInteractionState {
    InControl,
    TaggedIn,
    NotInteracting,
}

/// Selected conversation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedConversation {
    pub conversation_token: ServerConversationToken,
    pub is_forked: bool,
}

/// Universal developer input context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniversalDeveloperInputContext {
    pub input_mode: Option<InputMode>,
    pub selected_conversation: Option<SelectedConversation>,
    pub auto_approve_agent_actions: Option<bool>,
    pub selected_model: Option<SelectedAgentModel>,
    pub long_running_command_agent_interaction_state: Option<LongRunningCommandAgentInteractionState>,
    pub cli_agent_session: CLIAgentSessionState,
}

/// Stub type for AICommandMetadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AICommandMetadata {
    // TODO: Add fields
}

/// Stub type for OrderedTerminalEventType
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OrderedTerminalEventType {
    AgentResponseEvent {
        response_initiator: ParticipantId,
        response_event: String,
        forked_from_conversation_token: Option<String>,
    },
    AgentConversationReplayStarted,
    AgentConversationReplayEnded,
    CommandExecutionStarted {
        command: String,
    },
    CommandExecutionFinished {
        command: String,
        exit_code: i32,
    },
    Resize {
        width: u16,
        height: u16,
    },
    PtyBytesRead {
        data: Vec<u8>,
    },
}
