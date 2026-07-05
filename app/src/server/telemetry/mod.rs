//! Telemetry module stub - telemetry functionality has been removed.
//! This module provides stub types to maintain compatibility.
#![allow(dead_code)]

pub mod secret_redaction;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumIter};

// Stub types for compatibility
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaunchConfigUiLocation;

impl LaunchConfigUiLocation {
    pub const APP_MENU: Self = Self;
    pub const COMMAND_PALETTE: Self = Self;
    pub const URI: Self = Self;
    pub const TAB_MENU: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnonymousUserSignupEntrypoint;

impl AnonymousUserSignupEntrypoint {
    pub const SIGN_UP_BUTTON: Self = Self;
    pub const SIGN_UP_AI_PROMPT: Self = Self;
    pub const LOGIN_GATED_FEATURE: Self = Self;
    pub const HIT_DRIVE_OBJECT_LIMIT: Self = Self;
    pub const UNKNOWN: Self = Self;
    pub const RENOTIFICATION_BLOCK: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaletteSource {
    LogOutModal,
    QuitModal,
    IntegrationTest,
    Keybinding,
    CtrlTab { shift_pressed_initially: bool },
    TitleBarSearchBar,
    ConversationManager,
    WarpDrive,
    ContextChip,
    AgentTip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentModeEntrypoint;

impl AgentModeEntrypoint {
    pub const AGENT_MANAGEMENT_VIEW: Self = Self;
    pub const NEW_PANE_BINDING: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodeContextDestination;

impl CodeContextDestination {
    pub const RICH_INPUT: Self = Self;
    pub const PTY: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PtySpawnMode;

impl PtySpawnMode {
    pub const FALLBACK_TO_DIRECT: Self = Self;
    pub const DIRECT: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageProtocol;

impl ImageProtocol {
    pub const KITTY: Self = Self;
    pub const ITERM: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct InteractionSource;

impl InteractionSource {
    pub const KEYBINDING: Self = Self;
    pub const BUTTON: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToggleBlockFilterSource;

impl ToggleBlockFilterSource {
    pub const BINDING: Self = Self;
    pub const CONTEXT_MENU: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentModeRewindEntrypoint;

impl AgentModeRewindEntrypoint {
    pub const CONTEXT_MENU: Self = Self;
    pub const BUTTON: Self = Self;
    pub const SLASH_COMMAND: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CLIAgentType;

impl CLIAgentType {
    pub const CLAUDE: Self = Self;
    pub const GEMINI: Self = Self;
    pub const CODEX: Self = Self;
    pub const AMP: Self = Self;
    pub const DROID: Self = Self;
    pub const OPEN_CODE: Self = Self;
    pub const COPILOT: Self = Self;
    pub const PI: Self = Self;
    pub const CURSOR: Self = Self;
    pub const AUGGIE: Self = Self;
    pub const GOOSE: Self = Self;
    pub const HERMES: Self = Self;
    pub const QODER: Self = Self;
    pub const TRAE: Self = Self;
    pub const UNKNOWN: Self = Self;
    pub const VIBE: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct DownloadSource;

impl DownloadSource {
    pub const HOMEBREW: Self = Self;
    pub const WEBSITE: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloseTarget;

impl CloseTarget {
    pub const PANE: Self = Self;
    pub const TAB: Self = Self;
    pub const WINDOW: Self = Self;
    pub const APP: Self = Self;
    pub const EDITOR_TAB: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenedWarpAISource;

impl OpenedWarpAISource {
    pub const FROM_AI_COMMAND_SEARCH: Self = Self;
    pub const HELP_WITH_BLOCK: Self = Self;
    pub const HELP_WITH_TEXT_SELECTION: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptChoice;

impl PromptChoice {
    pub const PS1: Self = Self;
    pub const DEFAULT: Self = Self;
    pub const CUSTOM: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginChipTelemetryKind;

impl PluginChipTelemetryKind {
    pub const INSTALL: Self = Self;
    pub const UPDATE: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptSuggestionFallbackReason;

impl PromptSuggestionFallbackReason {
    pub const NO_READ_FILES_PERMISSION: Self = Self;
    pub const SSH_REMOTE_SESSION: Self = Self;
    pub const MISSING_FILE: Self = Self;
    pub const FAILED_TO_RETRIEVE_FILE: Self = Self;
    pub const FILE_TOO_MANY_LINES: Self = Self;
    pub const FILE_TOO_MANY_BYTES: Self = Self;
    pub const FAILED_TO_SEND_AI_REQUEST: Self = Self;
    pub const AI_QUERY_TIMEOUT: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotebookTelemetryMetadata;

impl NotebookTelemetryMetadata {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddTabWithShellSource;

impl AddTabWithShellSource {
    pub const COMMAND_PALETTE: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MCPTemplateInstallationSource;

impl MCPTemplateInstallationSource {
    pub const SHARED: Self = Self;
    pub const LOCAL: Self = Self;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BlockLatencyInfo {
    pub command: Option<String>,
    pub shell: Option<String>,
    pub is_ssh: bool,
    pub execution_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct EnvVarTelemetryMetadata {
    pub object_id: Option<String>,
    pub team_uid: Option<String>,
    pub space: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct WorkflowTelemetryMetadata {
    pub workflow_source: Option<String>,
    pub workflow_categories: Option<Vec<String>>,
    pub workflow_selection_source: Option<String>,
    pub workflow_id: Option<String>,
    pub workflow_space: Option<String>,
    pub enum_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudObjectTelemetryMetadata {
    pub team_uid: Option<String>,
    pub space: Option<String>,
    pub object_uid: Option<String>,
    pub object_type: Option<String>,
}

// Additional stub types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentModeCitation {
    WarpDriveObject {
        object_type: String,
        uid: String,
    },
    WarpDocs {
        page: String,
    },
    WebPage {
        url: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AICommandSearchEntrypoint;

impl AICommandSearchEntrypoint {
    pub const SHORT_HAND_TRIGGER: Self = Self;
    pub const KEYBINDING: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkOpenMethod;

impl LinkOpenMethod {
    pub const CMD_CLICK: Self = Self;
    pub const MIDDLE_CLICK: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationAgentVariant;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptSuggestionViewType;

impl PromptSuggestionViewType {
    pub const AGENT_VIEW: Self = Self;
    pub const TERMINAL_VIEW: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveAsWorkflowModalSource;

impl SaveAsWorkflowModalSource {
    pub const BLOCK: Self = Self;
    pub const INPUT: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub struct CpuUsageStats {
    pub num_cpus: u32,
    pub max_usage: f32,
    pub avg_usage: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MemoryUsageStats {
    pub total_application_usage_bytes: u64,
    pub total_blocks: u64,
    pub total_lines: u64,
    pub active_block_stats: BlockMemoryUsageStats,
    pub inactive_5m_stats: BlockMemoryUsageStats,
    pub inactive_1h_stats: BlockMemoryUsageStats,
    pub inactive_24h_stats: BlockMemoryUsageStats,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BlockMemoryUsageStats {
    pub num_blocks: u64,
    pub num_lines: u64,
    pub estimated_memory_usage_bytes: u64,
}

// Stub enum for TelemetryEvent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Serialize, Deserialize))]
pub enum CommandXRayTrigger {
    Keystroke,
    Hover,
}

// Stub enum for TelemetryEvent
#[derive(Debug, Clone, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Serialize, Deserialize))]
pub enum AgentModeAutoDetectionFalsePositivePayload {
    InternalDogfoodUsers {
        input_text: String,
    },
    ExternalUsers,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
#[strum_discriminants(derive(Serialize, Deserialize, EnumIter))]
pub enum TelemetryEvent {
    // Stub variant
    Stub,
    // Features page action
    FeaturesPageAction {
        action: String,
        value: String,
    },
    // Palette search events
    PaletteSearchResultAccepted {
        result_type: &'static str,
        filter: &'static str,
        buffer_length: usize,
    },
    PaletteSearchExited {
        filter: &'static str,
        buffer_length: usize,
    },
    // Toggle restore session
    ToggleRestoreSession(bool),
    // Show subshell banner
    ShowSubshellBanner,
    // SSH tmux warpify banner
    SshTmuxWarpifyBannerDisplayed,
    // Baseline command latency
    BaselineCommandLatency(BlockLatencyInfo),
    // Session abandoned before bootstrap
    SessionAbandonedBeforeBootstrap {
        pending_shell: bool,
        has_pending_ssh_session: bool,
        was_ever_visible: bool,
        duration_since_start: std::time::Duration,
    },
    // Autoupdate relaunch attempt
    AutoupdateRelaunchAttempt {
        version: String,
    },
    // Page up/down in editor
    PageUpDownInEditorPressed {
        is_empty_editor: bool,
        is_down: bool,
    },
}

impl cute_core::telemetry::TelemetryEvent for TelemetryEvent {
    fn name(&self) -> &'static str {
        match self {
            TelemetryEvent::Stub => "Stub",
            TelemetryEvent::FeaturesPageAction { .. } => "FeaturesPageAction",
            TelemetryEvent::PaletteSearchResultAccepted { .. } => "PaletteSearchResultAccepted",
            TelemetryEvent::PaletteSearchExited { .. } => "PaletteSearchExited",
            TelemetryEvent::ToggleRestoreSession(_) => "ToggleRestoreSession",
            TelemetryEvent::ShowSubshellBanner => "ShowSubshellBanner",
            TelemetryEvent::SshTmuxWarpifyBannerDisplayed => "SshTmuxWarpifyBannerDisplayed",
            TelemetryEvent::BaselineCommandLatency(_) => "BaselineCommandLatency",
            TelemetryEvent::SessionAbandonedBeforeBootstrap { .. } => "SessionAbandonedBeforeBootstrap",
            TelemetryEvent::AutoupdateRelaunchAttempt { .. } => "AutoupdateRelaunchAttempt",
            TelemetryEvent::PageUpDownInEditorPressed { .. } => "PageUpDownInEditorPressed",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            TelemetryEvent::Stub => "Stub event",
            TelemetryEvent::FeaturesPageAction { .. } => "Features page action",
            TelemetryEvent::PaletteSearchResultAccepted { .. } => "Palette search result accepted",
            TelemetryEvent::PaletteSearchExited { .. } => "Palette search exited",
            TelemetryEvent::ToggleRestoreSession(_) => "Toggle restore session",
            TelemetryEvent::ShowSubshellBanner => "Show subshell banner",
            TelemetryEvent::SshTmuxWarpifyBannerDisplayed => "SSH tmux warpify banner displayed",
            TelemetryEvent::BaselineCommandLatency(_) => "Baseline command latency",
            TelemetryEvent::SessionAbandonedBeforeBootstrap { .. } => "Session abandoned before bootstrap",
            TelemetryEvent::AutoupdateRelaunchAttempt { .. } => "Autoupdate relaunch attempt",
            TelemetryEvent::PageUpDownInEditorPressed { .. } => "Page up/down in editor pressed",
        }
    }

    fn enablement_state(&self) -> cute_core::telemetry::EnablementState {
        cute_core::telemetry::EnablementState::Always
    }

    fn payload(&self) -> Option<serde_json::Value> {
        None
    }

    fn contains_ugc(&self) -> bool {
        false
    }

    fn event_descs() -> impl Iterator<Item = Box<dyn cute_core::telemetry::TelemetryEventDesc>> {
        cute_core::telemetry::enum_events::<Self>()
    }
}

impl cute_core::telemetry::TelemetryEventDesc for TelemetryEventDiscriminants {
    fn name(&self) -> &'static str {
        match self {
            TelemetryEventDiscriminants::Stub => "Stub",
            TelemetryEventDiscriminants::FeaturesPageAction => "FeaturesPageAction",
            TelemetryEventDiscriminants::PaletteSearchResultAccepted => "PaletteSearchResultAccepted",
            TelemetryEventDiscriminants::PaletteSearchExited => "PaletteSearchExited",
            TelemetryEventDiscriminants::ToggleRestoreSession => "ToggleRestoreSession",
            TelemetryEventDiscriminants::ShowSubshellBanner => "ShowSubshellBanner",
            TelemetryEventDiscriminants::SshTmuxWarpifyBannerDisplayed => "SshTmuxWarpifyBannerDisplayed",
            TelemetryEventDiscriminants::BaselineCommandLatency => "BaselineCommandLatency",
            TelemetryEventDiscriminants::SessionAbandonedBeforeBootstrap => "SessionAbandonedBeforeBootstrap",
            TelemetryEventDiscriminants::AutoupdateRelaunchAttempt => "AutoupdateRelaunchAttempt",
            TelemetryEventDiscriminants::PageUpDownInEditorPressed => "PageUpDownInEditorPressed",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            TelemetryEventDiscriminants::Stub => "Stub event",
            TelemetryEventDiscriminants::FeaturesPageAction => "Features page action",
            TelemetryEventDiscriminants::PaletteSearchResultAccepted => "Palette search result accepted",
            TelemetryEventDiscriminants::PaletteSearchExited => "Palette search exited",
            TelemetryEventDiscriminants::ToggleRestoreSession => "Toggle restore session",
            TelemetryEventDiscriminants::ShowSubshellBanner => "Show subshell banner",
            TelemetryEventDiscriminants::SshTmuxWarpifyBannerDisplayed => "SSH tmux warpify banner displayed",
            TelemetryEventDiscriminants::BaselineCommandLatency => "Baseline command latency",
            TelemetryEventDiscriminants::SessionAbandonedBeforeBootstrap => "Session abandoned before bootstrap",
            TelemetryEventDiscriminants::AutoupdateRelaunchAttempt => "Autoupdate relaunch attempt",
            TelemetryEventDiscriminants::PageUpDownInEditorPressed => "Page up/down in editor pressed",
        }
    }

    fn enablement_state(&self) -> cute_core::telemetry::EnablementState {
        cute_core::telemetry::EnablementState::Always
    }
}

cute_core::register_telemetry_event!(TelemetryEvent);

pub mod context_provider {
    //! Context provider for telemetry
    use cute_core::telemetry::TelemetryContextProvider;
    use cuteui::AppContext;

    pub struct AppTelemetryContextProvider;

    impl TelemetryContextProvider for AppTelemetryContextProvider {
        fn user_id(&self, _ctx: &AppContext) -> Option<String> {
            None
        }

        fn anonymous_id(&self, _ctx: &AppContext) -> String {
            "cute-local".to_string()
        }
    }

    impl AppTelemetryContextProvider {
        pub fn new_context_provider(_ctx: &mut AppContext) -> Box<dyn TelemetryContextProvider> {
            Box::new(Self)
        }
    }
}

pub fn telemetry_context() -> serde_json::Value {
    serde_json::json!({})
}
