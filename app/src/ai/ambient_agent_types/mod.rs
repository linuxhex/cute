use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::{NonNilUuid, Uuid};

use crate::ai::agent::conversation::{AIConversation, ConversationStatus};
use crate::ai::agent::{
    AIAgentOutputStatus, CancellationReason, FinishedAIAgentOutput, RenderableAIError,
};

#[derive(Debug, thiserror::Error)]
#[error("Invalid task ID: {0}")]
pub struct ParseAmbientAgentTaskIdError(#[from] uuid::Error);

/// A globally unique ID for an ambient agent task.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmbientAgentTaskId(NonNilUuid);

impl Default for AmbientAgentTaskId {
    fn default() -> Self {
        Self(NonNilUuid::try_from(Uuid::nil()).unwrap_or_else(|_| NonNilUuid::try_from(Uuid::new_v4()).unwrap()))
    }
}

impl Display for AmbientAgentTaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for AmbientAgentTaskId {
    type Err = ParseAmbientAgentTaskIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::try_parse(s)?;
        Ok(Self(NonNilUuid::try_from(uuid)?))
    }
}

impl From<AmbientAgentTaskId> for cynic::Id {
    fn from(id: AmbientAgentTaskId) -> Self {
        Self::new(id.to_string())
    }
}

/// High-level outcome of an ambient agent conversation.
#[derive(Clone, Debug)]
pub enum AmbientConversationStatus {
    Success,
    Error {
        error: RenderableAIError,
    },
    #[allow(dead_code)]
    Cancelled {
        reason: CancellationReason,
    },
    #[allow(dead_code)]
    Blocked {
        blocked_action: String,
    },
}

/// Derive an [`AmbientConversationStatus`] from the given conversation, if it has
/// reached a terminal state that we care about for ambient agents.
pub fn conversation_output_status_from_conversation(
    conversation: &AIConversation,
) -> Option<AmbientConversationStatus> {
    if let ConversationStatus::Blocked { blocked_action } = conversation.status() {
        return Some(AmbientConversationStatus::Blocked {
            blocked_action: blocked_action.clone(),
        });
    }
    if let ConversationStatus::Error = conversation.status() {
        if let Some(error_message) = conversation.status_error_message() {
            return Some(AmbientConversationStatus::Error {
                error: RenderableAIError::Other {
                    error_message: error_message.to_string(),
                    will_attempt_resume: false,
                    waiting_for_network: false,
                },
            });
        }
    }

    if let Some(last_exchange) = conversation.root_task_exchanges().last() {
        if let AIAgentOutputStatus::Finished { finished_output } = &last_exchange.output_status {
            let status = match finished_output {
                FinishedAIAgentOutput::Cancelled { output: _, reason } => {
                    AmbientConversationStatus::Cancelled { reason: *reason }
                }
                FinishedAIAgentOutput::Error { output: _, error } => {
                    AmbientConversationStatus::Error {
                        error: error.clone(),
                    }
                }
                FinishedAIAgentOutput::Success { output: _ } => AmbientConversationStatus::Success,
            };
            return Some(status);
        }
    }

    None
}

// Re-export the real runtime config types that already live in
// `cloud_object::models::scheduled_ambient_agent`.
pub use crate::cloud_object::models::AgentConfigSnapshot;

/// Source that initiated an ambient agent run (stub).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentSource {
    GitHubAction,
    Cli,
    CloudMode,
    Interactive,
    Linear,
    Slack,
    ScheduledAgent,
    WebApp,
    AgentWebhook,
}

impl AgentSource {
    /// Returns a display name for this source (stub).
    pub fn display_name(&self) -> &'static str {
        match self {
            AgentSource::GitHubAction => "GitHub Action",
            AgentSource::Cli => "CLI",
            AgentSource::CloudMode => "Cloud Mode",
            AgentSource::Interactive => "Interactive",
            AgentSource::Linear => "Linear",
            AgentSource::Slack => "Slack",
            AgentSource::ScheduledAgent => "Scheduled Agent",
            AgentSource::WebApp => "Web App",
            AgentSource::AgentWebhook => "Agent Webhook",
        }
    }

    /// Returns a string representation for this source (stub).
    pub fn as_str(&self) -> &'static str {
        match self {
            AgentSource::GitHubAction => "github_action",
            AgentSource::Cli => "cli",
            AgentSource::CloudMode => "cloud_mode",
            AgentSource::Interactive => "interactive",
            AgentSource::Linear => "linear",
            AgentSource::Slack => "slack",
            AgentSource::ScheduledAgent => "scheduled_agent",
            AgentSource::WebApp => "web_app",
            AgentSource::AgentWebhook => "agent_webhook",
        }
    }
}

/// Lifecycle state of an ambient agent task (stub).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AmbientAgentTaskState {
    #[default]
    Queued,
    Pending,
    Claimed,
    InProgress,
    Succeeded,
    Failed,
    Error,
    Blocked,
    Cancelled,
    Unknown,
}

impl AmbientAgentTaskState {
    /// Returns true for states that represent a failure outcome (stub).
    pub fn is_failure_like(&self) -> bool {
        matches!(self, Self::Failed | Self::Error | Self::Unknown)
    }

    /// Returns a string representation for use as a query parameter (stub).
    pub fn as_query_param(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Pending => "pending",
            Self::Claimed => "claimed",
            Self::InProgress => "in_progress",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Error => "error",
            Self::Blocked => "blocked",
            Self::Cancelled => "cancelled",
            Self::Unknown => "unknown",
        }
    }
}

/// Live session state of an ambient agent task (stub).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmbientAgentLiveSessionState {
    Attachable { session_id: String },
    ActiveUnattachable,
    Inactive,
}

/// An ambient agent task (stub).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmbientAgentTask {
    pub task_id: AmbientAgentTaskId,
    pub state: AmbientAgentTaskState,
    pub agent_config_snapshot: Option<AgentConfigSnapshot>,
    pub last_event_sequence: Option<u64>,
    pub parent_run_id: Option<String>,
    pub conversation_id: Option<crate::ai::agent::conversation::AIConversationId>,
    pub creator: task::TaskPrincipalInfo,
    pub executor: Option<task::TaskPrincipalInfo>,
    pub title: String,
    pub prompt: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub source: AgentSource,
    pub artifacts: Vec<task::TaskAttachment>,
    pub status_message: Option<TaskStatusMessage>,
    pub session_id: Option<String>,
    pub is_sandbox_running: bool,
    pub children: Vec<AmbientAgentTaskId>,
}

impl Default for AmbientAgentTask {
    fn default() -> Self {
        Self {
            task_id: AmbientAgentTaskId::default(),
            state: AmbientAgentTaskState::default(),
            agent_config_snapshot: None,
            last_event_sequence: None,
            parent_run_id: None,
            conversation_id: None,
            creator: task::TaskPrincipalInfo::default(),
            executor: None,
            title: String::new(),
            prompt: String::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            source: AgentSource::Interactive,
            artifacts: Vec::new(),
            status_message: None,
            session_id: None,
            is_sandbox_running: false,
            children: Vec::new(),
        }
    }
}

impl AmbientAgentTask {
    /// Returns the active live session state for this task, if any (stub).
    pub fn active_live_session_state(&self) -> Option<AmbientAgentLiveSessionState> {
        None
    }

    /// Returns the run ID for this task.
    pub fn run_id(&self) -> AmbientAgentTaskId {
        self.task_id
    }

    /// Returns the conversation ID for this task.
    pub fn conversation_id(&self) -> Option<crate::ai::agent::conversation::AIConversationId> {
        self.conversation_id
    }

    /// Returns the display name of the creator (stub).
    pub fn creator_display_name(&self) -> Option<String> {
        self.creator.display_name.clone()
    }

    /// Returns the display name of the executor, if set (stub).
    pub fn executor_display_name(&self) -> Option<String> {
        self.executor.as_ref().and_then(|e| e.display_name.clone())
    }

    /// Returns the active run execution for this task (stub).
    pub fn active_run_execution(&self) -> Option<AmbientAgentRunExecution> {
        None
    }

    /// Returns the run time for this task (stub).
    pub fn run_time(&self) -> Option<std::time::Duration> {
        None
    }

    /// Returns the active execution session ID for this task (stub).
    pub fn active_execution_session_id(&self) -> Option<String> {
        None
    }

    /// Returns whether this task has an active execution (stub).
    pub fn has_active_execution(&self) -> bool {
        false
    }

    /// Returns the credits used for this task (stub).
    pub fn credits_used(&self) -> Option<f64> {
        None
    }

    /// Stub method for blocks_cloud_followups.
    pub fn blocks_cloud_followups(&self) -> bool {
        false
    }
}

/// Active run execution state (stub).
#[derive(Debug, Clone, Default)]
pub struct AmbientAgentRunExecution {
    /// The session ID for this execution.
    pub session_id: Option<String>,
    /// The session link for joining this execution.
    pub session_link: Option<String>,
}

/// Error code for task status messages.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskErrorCode {
    #[default]
    EnvironmentSetupFailed,
}

/// Status message associated with a task state change (stub).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskStatusMessage {
    pub message: String,
    #[serde(rename = "errorCode")]
    pub error_code: Option<TaskErrorCode>,
}

impl TaskStatusMessage {
    /// Returns true if this status message represents an environment setup failure.
    pub fn is_environment_setup_failure(&self) -> bool {
        matches!(self.error_code, Some(TaskErrorCode::EnvironmentSetupFailed))
    }
}

/// Cancel an ambient agent task and surface a toast notification (stub).
pub fn cancel_task_with_toast<Ctx>(_task_id: AmbientAgentTaskId, _ctx: &mut Ctx) {}

/// Cancel an ambient agent task without surfacing a toast notification (stub).
pub fn cancel_task_silently<Ctx>(_task_id: AmbientAgentTaskId, _ctx: &mut Ctx) {}

/// `task` submodule: ambient agent task runtime types (stubs + re-exports).
pub mod task {
    use serde::{Deserialize, Serialize};

    // Re-export the real harness config types.
    pub use crate::cloud_object::models::{
        HarnessAuthSecretsConfig, HarnessConfig, HarnessModelConfig,
    };

    // Re-export the ambient agent task type defined at the parent level.
    pub use super::AmbientAgentTask;

    /// Input for uploading a task attachment (stub).
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct AttachmentInput {
        pub file_name: String,
        pub mime_type: String,
        pub data: String,
    }

    /// A downloaded task attachment (stub).
    #[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
    pub struct TaskAttachment {
        pub file_id: String,
        pub filename: String,
        pub download_url: String,
        pub mime_type: String,
    }

    /// Principal info for a task (stub).
    #[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct TaskPrincipalInfo {
        pub display_name: Option<String>,
        pub uid: String,
        pub creator_type: String,
    }

    /// Normalize an orchestrator agent name (stub). Returns `None`.
    pub fn normalize_orchestrator_agent_name(_name: &str) -> Option<String> {
        None
    }
}

/// `spawn` submodule: ambient agent task spawning/streaming (stubs).
pub mod spawn {
    use super::{AmbientAgentTask, AmbientAgentTaskId, AmbientAgentTaskState, TaskStatusMessage};

    /// Polling duration for ambient agent task status (stub).
    pub const TASK_STATUS_POLLING_DURATION: std::time::Duration =
        std::time::Duration::from_secs(300);

    /// Events emitted while spawning/monitoring an ambient agent task (stub).
    #[derive(Debug, Clone)]
    pub enum AmbientAgentEvent {
        TaskSpawned { task_id: AmbientAgentTaskId },
        AtCapacity,
        StateChanged {
            state: AmbientAgentTaskState,
            status_message: Option<TaskStatusMessage>,
        },
        SessionStarted { session_join_info: SessionJoinInfo },
        TimedOut,
    }

    /// Info required to join a live ambient agent session (stub).
    #[derive(Debug, Clone, Default)]
    pub struct SessionJoinInfo {
        pub session_link: String,
    }

    impl SessionJoinInfo {
        /// Derive join info from a task, if a joinable session is present (stub).
        pub fn from_task(_task: &AmbientAgentTask) -> Option<Self> {
            None
        }
    }

    /// Spawn an ambient agent task and stream lifecycle events (stub).
    ///
    /// Returns an empty stream so callers that poll it terminate immediately.
    pub fn spawn_task<Req, Client>(
        _request: Req,
        _client: Client,
        _timeout: Option<std::time::Duration>,
    ) -> futures::stream::Empty<Result<AmbientAgentEvent, anyhow::Error>> {
        futures::stream::empty()
    }
}

/// `scheduled` submodule: scheduled ambient agent management (stubs + re-exports).
pub mod scheduled {
    use futures;
    use cuteui::{Entity, SingletonEntity};
    use cute_server_client::cloud_object::Owner;

    // Re-export the real scheduled-agent types.
    pub use crate::cloud_object::models::{CloudScheduledAmbientAgent, ScheduledAmbientAgent};

    /// Manager for scheduled ambient agents (stub).
    #[derive(Debug, Clone, Default)]
    pub struct ScheduledAgentManager;

    impl Entity for ScheduledAgentManager {
        type Event = ();
    }

    impl SingletonEntity for ScheduledAgentManager {}

    impl ScheduledAgentManager {
        pub fn new(_ctx: &mut cuteui::ModelContext<Self>) -> Self {
            Self
        }

        pub fn create_schedule(
            &self,
            _config: ScheduledAmbientAgent,
            _owner: Owner,
        ) -> impl std::future::Future<Output = anyhow::Result<crate::server::ids::SyncId>> {
            futures::future::ok(crate::server::ids::SyncId::ClientId(
                crate::server::ids::ClientId::new(),
            ))
        }

        pub fn pause_schedule(
            &self,
            _schedule_id: crate::server::ids::SyncId,
        ) -> impl std::future::Future<Output = anyhow::Result<()>> {
            futures::future::ok(())
        }

        pub fn unpause_schedule(
            &self,
            _schedule_id: crate::server::ids::SyncId,
        ) -> impl std::future::Future<Output = anyhow::Result<()>> {
            futures::future::ok(())
        }

        pub fn update_schedule(
            &self,
            _schedule_id: crate::server::ids::SyncId,
            _params: UpdateScheduleParams,
        ) -> impl std::future::Future<Output = anyhow::Result<()>> {
            futures::future::ok(())
        }

        pub fn delete_schedule(
            &self,
            _schedule_id: crate::server::ids::SyncId,
        ) -> impl std::future::Future<Output = anyhow::Result<()>> {
            futures::future::ok(())
        }

        pub fn list_schedules(&self) -> Vec<CloudScheduledAmbientAgent> {
            Vec::new()
        }

        pub fn fetch_schedule_history(
            &self,
            _schedule_id: crate::server::ids::SyncId,
        ) -> impl std::future::Future<Output = anyhow::Result<Option<cute_graphql::queries::get_scheduled_agent_history::ScheduledAgentHistory>>> {
            futures::future::ok(None)
        }
    }

    /// Parameters for updating a schedule (stub).
    #[derive(Debug, Clone, Default)]
    pub struct UpdateScheduleParams {
        pub name: Option<String>,
        pub cron: Option<String>,
        pub model_id: Option<String>,
        pub environment_id: Option<String>,
        pub base_prompt: Option<String>,
        pub prompt: Option<String>,
        pub mcp_servers_upsert: Option<serde_json::Map<String, serde_json::Value>>,
        pub remove_mcp_server_names: Vec<String>,
        pub skill_spec: Option<Option<String>>,
        pub worker_host: Option<String>,
    }
}

/// `telemetry` submodule: ambient agent telemetry types (stubs).
pub mod telemetry {
    /// Entry point for a cloud handoff flow (stub).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum HandoffEntryPoint {
        FooterChip,
        #[default]
        Ampersand,
        SlashCommand,
    }
}

/// `github_auth_url` submodule: GitHub OAuth URL builders (stubs).
pub mod github_auth_url {
    /// Where the GitHub auth flow should redirect after completion (stub).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum GithubAuthRedirectTarget {
        #[default]
        SettingsEnvironments,
        FocusCloudMode,
    }

    /// Source initiating GitHub auth (stub).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum AuthSource {
        #[default]
        Settings,
        CloudSetup,
    }

    pub fn settings_environments_auth_url_with_next(_base_auth_url: &str) -> String {
        String::new()
    }

    pub fn cloud_setup_auth_url_with_next(_base_auth_url: &str) -> String {
        String::new()
    }

    pub fn auth_url_with_next(
        _base_auth_url: &str,
        _target: GithubAuthRedirectTarget,
        _auth_source: AuthSource,
    ) -> String {
        String::new()
    }

    pub fn build_auth_url_with_next(
        _base_auth_url: &str,
        _target: GithubAuthRedirectTarget,
        _scheme: &str,
        _auth_source: AuthSource,
    ) -> String {
        String::new()
    }
}

/// `github_auth_notifier` submodule: GitHub auth completion notifier (stubs).
pub mod github_auth_notifier {
    use cuteui::{Entity, SingletonEntity};

    /// Events emitted by the GitHub auth notifier (stub).
    #[derive(Debug, Clone)]
    pub enum GitHubAuthEvent {
        AuthCompleted,
    }

    /// Notifier model for GitHub auth completion (stub).
    #[derive(Debug, Clone, Default)]
    pub struct GitHubAuthNotifier;

    impl Entity for GitHubAuthNotifier {
        type Event = GitHubAuthEvent;
    }

    impl SingletonEntity for GitHubAuthNotifier {}

    impl GitHubAuthNotifier {
        pub fn new() -> Self {
            Self
        }
    }
}
