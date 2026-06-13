//! Telemetry module stub - telemetry functionality has been removed.
//! This module provides stub types to maintain compatibility.

pub mod secret_redaction;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumIter};

// Stub types for compatibility
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelemetrySpace;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaunchConfigUiLocation;

impl LaunchConfigUiLocation {
    pub const AppMenu: Self = Self;
    pub const CommandPalette: Self = Self;
    pub const Uri: Self = Self;
    pub const TabMenu: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnonymousUserSignupEntrypoint;

impl AnonymousUserSignupEntrypoint {
    pub const SignUpButton: Self = Self;
    pub const SignUpAIPrompt: Self = Self;
    pub const LoginGatedFeature: Self = Self;
    pub const HitDriveObjectLimit: Self = Self;
    pub const Unknown: Self = Self;
    pub const RenotificationBlock: Self = Self;
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
    pub const AgentManagementView: Self = Self;
    pub const NewPaneBinding: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodeContextDestination;

impl CodeContextDestination {
    pub const RichInput: Self = Self;
    pub const Pty: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PtySpawnMode;

impl PtySpawnMode {
    pub const FallbackToDirect: Self = Self;
    pub const Direct: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageProtocol;

impl ImageProtocol {
    pub const Kitty: Self = Self;
    pub const ITerm: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct InteractionSource;

impl InteractionSource {
    pub const Keybinding: Self = Self;
    pub const Button: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToggleBlockFilterSource;

impl ToggleBlockFilterSource {
    pub const Binding: Self = Self;
    pub const ContextMenu: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentModeRewindEntrypoint;

impl AgentModeRewindEntrypoint {
    pub const ContextMenu: Self = Self;
    pub const Button: Self = Self;
    pub const SlashCommand: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CLIAgentType;

impl CLIAgentType {
    pub const Claude: Self = Self;
    pub const Gemini: Self = Self;
    pub const Codex: Self = Self;
    pub const Amp: Self = Self;
    pub const Droid: Self = Self;
    pub const OpenCode: Self = Self;
    pub const Copilot: Self = Self;
    pub const Pi: Self = Self;
    pub const Cursor: Self = Self;
    pub const Auggie: Self = Self;
    pub const Goose: Self = Self;
    pub const Hermes: Self = Self;
    pub const Qoder: Self = Self;
    pub const Unknown: Self = Self;
    pub const Vibe: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct DownloadSource;

impl DownloadSource {
    pub const Homebrew: Self = Self;
    pub const Website: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloseTarget;

impl CloseTarget {
    pub const Pane: Self = Self;
    pub const Tab: Self = Self;
    pub const Window: Self = Self;
    pub const App: Self = Self;
    pub const EditorTab: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenedWarpAISource;

impl OpenedWarpAISource {
    pub const FromAICommandSearch: Self = Self;
    pub const HelpWithBlock: Self = Self;
    pub const HelpWithTextSelection: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptChoice;

impl PromptChoice {
    pub const PS1: Self = Self;
    pub const Default: Self = Self;
    pub const Custom: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginChipTelemetryKind;

impl PluginChipTelemetryKind {
    pub const Install: Self = Self;
    pub const Update: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptSuggestionFallbackReason;

impl PromptSuggestionFallbackReason {
    pub const NoReadFilesPermission: Self = Self;
    pub const SSHRemoteSession: Self = Self;
    pub const MissingFile: Self = Self;
    pub const FailedToRetrieveFile: Self = Self;
    pub const FileTooManyLines: Self = Self;
    pub const FileTooManyBytes: Self = Self;
    pub const FailedToSendAIRequest: Self = Self;
    pub const AIQueryTimeout: Self = Self;
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
    pub const CommandPalette: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MCPTemplateInstallationSource;

impl MCPTemplateInstallationSource {
    pub const Shared: Self = Self;
    pub const Local: Self = Self;
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MCPServerTelemetryMetadata;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MCPTemplateCreationSource;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MCPServerTelemetryTransportType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrappingInfo;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SlowBootstrapInfo;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppStartupInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudObjectTelemetryMetadata {
    pub team_uid: Option<String>,
    pub space: Option<String>,
    pub object_uid: Option<String>,
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelemetryCloudObjectType;

impl TelemetryCloudObjectType {
    pub const PromptTemplate: Self = Self;
    pub const Workflow: Self = Self;
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
    pub const ShortHandTrigger: Self = Self;
    pub const Keybinding: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkOpenMethod;

impl LinkOpenMethod {
    pub const CmdClick: Self = Self;
    pub const MiddleClick: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationAgentVariant;

impl NotificationAgentVariant {
    pub const CLIAgent: Self = Self;
    pub const Oz: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptSuggestionViewType;

impl PromptSuggestionViewType {
    pub const AgentView: Self = Self;
    pub const TerminalView: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveAsWorkflowModalSource;

impl SaveAsWorkflowModalSource {
    pub const Block: Self = Self;
    pub const Input: Self = Self;
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

impl warp_core::telemetry::TelemetryEvent for TelemetryEvent {
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

    fn enablement_state(&self) -> warp_core::telemetry::EnablementState {
        warp_core::telemetry::EnablementState::Always
    }

    fn payload(&self) -> Option<serde_json::Value> {
        None
    }

    fn contains_ugc(&self) -> bool {
        false
    }

    fn event_descs() -> impl Iterator<Item = Box<dyn warp_core::telemetry::TelemetryEventDesc>> {
        warp_core::telemetry::enum_events::<Self>()
    }
}

impl warp_core::telemetry::TelemetryEventDesc for TelemetryEventDiscriminants {
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

    fn enablement_state(&self) -> warp_core::telemetry::EnablementState {
        warp_core::telemetry::EnablementState::Always
    }
}

warp_core::register_telemetry_event!(TelemetryEvent);

pub mod context_provider {
    //! Context provider stub
    use warpui::{Entity, SingletonEntity};

    pub struct AppTelemetryContextProvider;

    impl Entity for AppTelemetryContextProvider {
        type Event = ();
    }

    impl SingletonEntity for AppTelemetryContextProvider {}

    impl AppTelemetryContextProvider {
        pub fn new_context_provider(_ctx: &mut warpui::AppContext) -> Self {
            Self
        }
    }
}

pub fn telemetry_context() -> serde_json::Value {
    serde_json::json!({})
}

pub fn clear_event_queue() {
    // No-op
}
