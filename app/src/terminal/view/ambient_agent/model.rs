use cuteui::{AppContext, Entity, EntityId, ModelContext};
use std::sync::Arc;

use crate::ai::agent::conversation::AIConversationId;
use crate::server::server_api::ai::AttachmentInput;
use crate::server::ids::SyncId;
use crate::terminal::cli_agent::CLIAgent;
use cute_cli::agent::Harness;
use cute_core::features::FeatureFlag;
use super::PendingHandoff;

/// Status of the ambient agent run.
#[derive(Debug, Clone)]
pub enum Status {
    /// The user is composing their ambient agent prompt.
    Composing,
    /// The agent is running.
    AgentRunning,
    /// The agent failed.
    Failed { error_message: String },
    /// The agent was cancelled.
    Cancelled,
    /// Waiting for the session to be ready.
    WaitingForSession,
}

/// Model to track the state of an ambient agent run.
#[allow(dead_code)]
pub struct AmbientAgentViewModel {
    status: Status,

    /// The terminal view this model is part of.
    terminal_view_id: EntityId,

    /// The local conversation associated with this ambient agent run, if any.
    conversation_id: Option<AIConversationId>,

    /// The execution harness for this ambient agent run.
    harness: Harness,

    /// Whether the harness command has started.
    harness_command_started: bool,

    /// The environment ID associated with this run, if any.
    environment_id: Option<SyncId>,

    /// The pending handoff state, if any.
    pending_handoff: Option<PendingHandoff>,

    /// UI state for the ambient agent view.
    pub ui_state: AmbientAgentUIState,
}

/// UI state for the ambient agent view.
pub struct AmbientAgentUIState {
    /// The selected text in the error screen, if any.
    pub error_selected_text: Arc<parking_lot::RwLock<String>>,
}

impl Default for AmbientAgentUIState {
    fn default() -> Self {
        Self {
            error_selected_text: Arc::new(parking_lot::RwLock::new(String::new())),
        }
    }
}

impl AmbientAgentViewModel {
    pub fn new(terminal_view_id: EntityId, _ctx: &mut ModelContext<Self>) -> Self {
        Self {
            status: Status::Composing,
            terminal_view_id,
            conversation_id: None,
            harness: Harness::default(),
            harness_command_started: false,
            environment_id: None,
            pending_handoff: None,
            ui_state: AmbientAgentUIState::default(),
        }
    }

    /// Whether or not this terminal session is for an ambient agent.
    pub fn is_ambient_agent(&self) -> bool {
        true
    }

    /// Whether or not this terminal session is currently setting up an ambient agent run.
    pub fn is_configuring_ambient_agent(&self) -> bool {
        matches!(self.status, Status::Composing)
    }

    /// Whether or not the ambient agent failed to spawn.
    pub fn is_failed(&self) -> bool {
        matches!(self.status, Status::Failed { .. })
    }

    /// Whether or not the ambient agent was cancelled.
    pub fn is_cancelled(&self) -> bool {
        matches!(self.status, Status::Cancelled)
    }

    /// Whether or not the ambient agent is currently running.
    pub fn is_agent_running(&self) -> bool {
        matches!(self.status, Status::AgentRunning)
    }

    /// Whether or not the ambient agent is waiting for the session to be ready.
    pub fn is_waiting_for_session(&self) -> bool {
        matches!(self.status, Status::WaitingForSession)
    }

    /// Whether or not we should show a status footer (error or cancelled).
    pub fn should_show_status_footer(&self) -> bool {
        self.is_failed() || self.is_cancelled()
    }

    /// Returns the error message if the agent is in a failed state.
    pub fn error_message(&self) -> Option<&str> {
        match &self.status {
            Status::Failed { error_message } => Some(error_message),
            _ => None,
        }
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Gets the local conversation ID associated with this ambient agent run.
    pub fn conversation_id(&self) -> Option<&AIConversationId> {
        self.conversation_id.as_ref()
    }

    /// Sets the local conversation ID associated with this ambient agent run.
    pub fn set_conversation_id(&mut self, id: Option<AIConversationId>) {
        self.conversation_id = id;
    }

    /// Enter the running state.
    pub fn enter_running(&mut self, ctx: &mut ModelContext<Self>) {
        self.status = Status::AgentRunning;
        ctx.emit(AmbientAgentViewModelEvent::AgentRunning);
    }

    /// Enter the failed state.
    pub fn enter_failed(&mut self, error_message: String, ctx: &mut ModelContext<Self>) {
        self.status = Status::Failed {
            error_message: error_message.clone(),
        };
        ctx.emit(AmbientAgentViewModelEvent::Failed { error_message });
    }

    /// Enter the cancelled state.
    pub fn enter_cancelled(&mut self, ctx: &mut ModelContext<Self>) {
        self.status = Status::Cancelled;
        ctx.emit(AmbientAgentViewModelEvent::Cancelled);
    }

    /// Reset state so a view can compose a new task.
    pub fn reset_for_new_prompt(&mut self, ctx: &mut ModelContext<Self>) {
        self.status = Status::Composing;
        self.conversation_id = None;
        self.harness = Harness::default();
        self.harness_command_started = false;
        ctx.emit(AmbientAgentViewModelEvent::Reset);
    }

    /// True when the run is configured to use a non-Oz execution harness and the
    /// required feature flags are enabled.
    pub fn is_third_party_harness(&self) -> bool {
        FeatureFlag::AgentHarness.is_enabled() && self.harness != Harness::Oz
    }

    /// Whether the harness command has started.
    pub fn harness_command_started(&self) -> bool {
        self.harness_command_started
    }

    /// Mark that the harness command has started.
    pub fn mark_harness_command_started(&mut self, ctx: &mut ModelContext<Self>) {
        if self.harness_command_started {
            return;
        }
        self.harness_command_started = true;
        ctx.emit(AmbientAgentViewModelEvent::HarnessCommandStarted);
    }

    /// Set the harness for this run.
    pub fn set_harness(&mut self, harness: Harness) {
        self.harness = harness;
    }

    /// Set the environment ID for this run.
    pub fn set_environment_id(&mut self, environment_id: Option<SyncId>, _ctx: &mut ModelContext<Self>) {
        self.environment_id = environment_id;
    }

    /// Get the selected harness for this run.
    pub fn selected_harness(&self) -> Harness {
        self.harness
    }

    /// Returns the CLI agent corresponding to the selected harness, if it's a third-party harness.
    /// Returns None for Harness::Oz (Warp's built-in harness) or when the feature flag is disabled.
    pub fn selected_third_party_cli_agent(&self) -> Option<CLIAgent> {
        if self.is_third_party_harness() {
            CLIAgent::from_harness(self.harness)
        } else {
            None
        }
    }

    /// Whether this pane is in local-to-cloud handoff mode.
    pub fn is_local_to_cloud_handoff(&self) -> bool {
        self.pending_handoff.is_some()
    }

    /// Get the selected environment ID for this run.
    pub fn selected_environment_id(&self) -> Option<&SyncId> {
        self.environment_id.as_ref()
    }

    /// Stub method for blocks_cloud_followups.
    pub fn blocks_cloud_followups(&self) -> bool {
        false
    }

    /// Stub method for task_id.
    pub fn task_id(&self) -> Option<crate::ai::ambient_agent_types::AmbientAgentTaskId> {
        None
    }

    /// Stub method for is_in_setup.
    pub fn is_in_setup(&self) -> bool {
        false
    }

    /// Stub method for cancel_task.
    pub fn cancel_task(&mut self, _ctx: &mut ModelContext<Self>) {
        // Stub implementation - no-op for local version
    }

    /// Stub method for agent_progress.
    pub fn agent_progress(&self) -> Option<f32> {
        None
    }

    /// Stub method for is_handoff_ready_to_submit.
    pub fn is_handoff_ready_to_submit(&self) -> bool {
        true
    }

    /// Stub method for submit_handoff.
    pub fn submit_handoff(
        &mut self,
        _prompt: String,
        _attachments: Vec<AttachmentInput>,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Stub method for set_pending_handoff.
    pub fn set_pending_handoff(&mut self, pending: Option<PendingHandoff>) {
        self.pending_handoff = pending;
    }

    /// Stub method for spawn_agent.
    pub fn spawn_agent(
        &mut self,
        _prompt: String,
        _attachments: Vec<AttachmentInput>,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Stub method for is_ready_for_cloud_followup_prompt.
    pub fn is_ready_for_cloud_followup_prompt(&self) -> bool {
        false
    }

    /// Stub method for selected_harness_model_id.
    pub fn selected_harness_model_id(&self) -> Option<&str> {
        None
    }

    /// Stub method for selected_harness_reasoning_level.
    pub fn selected_harness_reasoning_level(&self) -> Option<&str> {
        None
    }

    /// Stub method for set_harness_model_selection.
    pub fn set_harness_model_selection(
        &mut self,
        _model_id: Option<String>,
        _reasoning_level: Option<String>,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }
}


/// Events emitted by the ambient agent view model.
#[derive(Debug, Clone)]
pub enum AmbientAgentViewModelEvent {
    /// The user has entered the composing state (typing their prompt).
    EnteredComposingState,
    /// The user has entered the setup state.
    EnteredSetupState,
    /// The ambient agent is running.
    AgentRunning,
    /// The ambient agent failed.
    Failed { error_message: String },
    /// The ambient agent was cancelled.
    Cancelled,
    /// The state was reset for a new prompt.
    Reset,
    /// The harness command has started.
    HarnessCommandStarted,
    /// The agent has been dispatched to the server.
    DispatchedAgent,
    /// The environment has been selected.
    EnvironmentSelected,
    /// The pending handoff state has changed.
    PendingHandoffChanged,
    /// The execution session is ready.
    ExecutionSessionReady,
    /// The session is ready.
    SessionReady,
    /// A follow-up has been dispatched.
    FollowupDispatched,
    /// GitHub authentication is needed.
    NeedsGithubAuth,
    /// Progress has been updated.
    ProgressUpdated,
    /// A harness has been selected.
    HarnessSelected,
    /// A harness model has been selected.
    HarnessModelSelected,
    /// The run lifecycle has changed.
    RunLifecycleChanged,
    /// Handoff snapshot upload failed.
    HandoffSnapshotUploadFailed { error_message: String },
}

pub(crate) fn should_disable_snapshot(_ctx: &AppContext) -> bool {
    true
}

impl Entity for AmbientAgentViewModel {
    type Event = AmbientAgentViewModelEvent;
}
