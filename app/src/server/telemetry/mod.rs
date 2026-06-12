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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnonymousUserSignupEntrypoint;

impl AnonymousUserSignupEntrypoint {
    pub const SignUpButton: Self = Self;
    pub const SignUpAIPrompt: Self = Self;
    pub const LoginGatedFeature: Self = Self;
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodeContextDestination;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PtySpawnMode;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageProtocol;

impl ImageProtocol {
    pub const Kitty: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct InteractionSource;

impl InteractionSource {
    pub const Keybinding: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToggleBlockFilterSource;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentModeRewindEntrypoint;

impl AgentModeRewindEntrypoint {
    pub const ContextMenu: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CLIAgentType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct DownloadSource;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloseTarget;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenedWarpAISource;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptChoice;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginChipTelemetryKind;

impl PluginChipTelemetryKind {
    pub const Install: Self = Self;
    pub const Update: Self = Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptSuggestionFallbackReason;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotebookTelemetryMetadata;

impl NotebookTelemetryMetadata {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddTabWithShellSource;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MCPTemplateInstallationSource;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockLatencyInfo;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnvVarTelemetryMetadata;

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowTelemetryMetadata;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudObjectTelemetryMetadata {
    pub team_uid: Option<String>,
    pub space: Option<String>,
    pub object_uid: Option<String>,
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelemetryCloudObjectType;

// Additional stub types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentModeCitation;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AICommandSearchEntrypoint;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkOpenMethod;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationAgentVariant;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptSuggestionViewType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveAsWorkflowModalSource;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CpuUsageStats;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MemoryUsageStats;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BlockMemoryUsageStats;

// Stub enum for TelemetryEvent
#[derive(Debug, Clone, Serialize, Deserialize, EnumDiscriminants, EnumIter)]
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
    pub struct AppTelemetryContextProvider;

    impl AppTelemetryContextProvider {
        pub fn new_context_provider() -> Self {
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
