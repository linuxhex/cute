//! This module contains model, controller, and view logic for Blocklist AI.
mod action_model;
pub mod agent_view;
pub mod block;
pub mod code_block;
mod context_model;
mod controller;

pub(crate) mod local_agent_task_sync_model;
pub(crate) mod orchestration_event_streamer;
pub(crate) mod orchestration_events;
pub(crate) mod orchestration_topology;
mod passive_suggestions;
pub(crate) mod queued_query;
pub(super) use controller::RequestInput;
pub mod history_model;
pub mod inline_action;
mod input_model;
mod permissions;
mod persistence;
pub mod prompt;
pub mod suggested_agent_mode_workflow_modal;
pub mod suggested_rule_modal;
mod suggestion_chip_view;
pub mod summarization_cancel_dialog;
pub(crate) mod telemetry;
pub mod usage;

pub(crate) mod codebase_index_speedbump_banner;
pub(crate) mod telemetry_banner;
pub(super) mod view_util;

#[cfg_attr(target_family = "wasm", allow(unused_imports))]
pub(crate) use action_model::{
    apply_edits, read_local_file_context, BlocklistAIActionEvent, BlocklistAIActionModel,
    FileReadResult, RequestFileEditsFormatKind, ShellCommandExecutor,
    ShellCommandExecutorEvent, StartAgentExecutor, StartAgentExecutorEvent, StartAgentRequest,
    StartAgentRequestId,
};
#[cfg(any(test, feature = "integration_tests"))]
pub(crate) use block::model::testing::FakeAIBlockModel;
pub(crate) use block::{init, model, AIBlock, AIBlockEvent, RequestedEditResolution};
pub use block::{keyboard_navigable_buttons, toggleable_items};
pub(crate) use context_model::{
    block_context_from_terminal_model, AttachmentType, BlocklistAIContextEvent,
    BlocklistAIContextModel, PendingAttachment, PendingFile, PendingQueryState,
};
pub use controller::input_context::{
    BLOCK_CONTEXT_ATTACHMENT_REGEX, DIFF_HUNK_ATTACHMENT_REGEX, DRIVE_OBJECT_ATTACHMENT_REGEX,
};
pub(crate) use controller::response_stream::ResponseStreamId;
pub(crate) use controller::{
    BlocklistAIController, BlocklistAIControllerEvent, ClientIdentifiers, SessionContext,
    SlashCommandRequest,
};
pub(crate) use history_model::{
    AIQueryHistory, AIQueryHistoryOutputStatus, BlocklistAIHistoryEvent, BlocklistAIHistoryModel,
    ConversationStatusUpdate, FORK_PREFIX, PRE_REWIND_PREFIX,
};
pub(crate) use input_model::{
    BlocklistAIInputEvent, BlocklistAIInputModel, InputConfig, InputType,
    InputTypeAutoDetectionSource,
};
pub(crate) use passive_suggestions::{
    LegacyPassiveSuggestionsEvent, LegacyPassiveSuggestionsModel, MaaPassiveSuggestionsEvent,
    MaaPassiveSuggestionsModel, PassiveSuggestionsModels,
};
pub use permissions::BlocklistAIPermissions;
#[cfg_attr(target_family = "wasm", allow(unused))]
pub(crate) use persistence::PersistedAIInputType;
pub(crate) use persistence::{PersistedAIInput, SerializedBlockListItem};
pub(crate) use queued_query::{
    AutofireAction, QueuedQuery, QueuedQueryEvent, QueuedQueryId, QueuedQueryModel,
    QueuedQueryOrigin,
};
pub use suggestion_chip_view::*;
pub use view_util::error_color;
pub(crate) use view_util::{
    ai_brand_color, ai_indicator_height, format_credits,
    get_ai_block_overflow_menu_element_position_id, get_attached_blocks_chip_element_position_id,
    render_ai_agent_mode_icon, render_ai_follow_up_icon, ATTACH_AS_AGENT_MODE_CONTEXT_TEXT,
    CLAUDE_ORANGE, NEW_AGENT_PANE_LABEL,
};

pub use crate::ai::blocklist::block::{secret_redaction, AIBlockResponseRating, TextLocation};

// ---------------------------------------------------------------------------
// Handoff stubs (OMJF-11111 de-cloudification)
//
// The `handoff` module was physically removed as part of de-cloudification.
// These stub types preserve the API surface for callers that still reference
// handoff types (e.g. `terminal::view::ambient_agent`). Function stubs are
// no-ops that disable the cloud-handoff behavior paths.
// ---------------------------------------------------------------------------

#[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
mod handoff_stubs {
    use std::path::{Path, PathBuf};

    use crate::ai::cloud_environments::{CloudAmbientAgentEnvironment, GithubRepo};
    use crate::server::ids::SyncId;
    use crate::server::server_api::ai::AttachmentInput;

    use super::PendingAttachment;

    #[derive(Debug, Clone, Default)]
    pub(crate) struct HandoffLaunchAttachments {
        pub(crate) request_attachments: Vec<AttachmentInput>,
        pub(crate) display_attachments: Vec<PendingAttachment>,
    }

    #[derive(Debug, Clone)]
    pub(crate) struct PendingCloudLaunch {
        pub(crate) prompt: String,
        pub(crate) attachments: HandoffLaunchAttachments,
    }

    #[derive(Clone, Debug, Default)]
    pub(crate) struct TouchedWorkspace {
        pub repos: Vec<TouchedRepo>,
        pub orphan_files: Vec<PathBuf>,
    }

    #[derive(Clone, Debug)]
    pub(crate) struct TouchedRepo {
        pub git_root: PathBuf,
        pub repo_id: Option<GithubRepo>,
    }

    pub(crate) fn pick_handoff_overlap_env(
        _workspace: &TouchedWorkspace,
        _envs: Vec<CloudAmbientAgentEnvironment>,
    ) -> Option<SyncId> {
        None
    }

    pub(crate) async fn resolve_repo_for_path(_path: &Path) -> Option<TouchedRepo> {
        None
    }
}

#[cfg(all(feature = "local_fs", not(target_family = "wasm")))]
pub(crate) use handoff_stubs::{
    HandoffLaunchAttachments, PendingCloudLaunch, TouchedRepo, TouchedWorkspace,
    pick_handoff_overlap_env, resolve_repo_for_path,
};
