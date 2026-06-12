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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct InteractionSource;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToggleBlockFilterSource;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentModeRewindEntrypoint;

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptSuggestionFallbackReason;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotebookTelemetryMetadata;

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudObjectTelemetryMetadata;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelemetryCloudObjectType;

// Additional stub types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentModeCitation;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AICommandSearchEntrypoint;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentModeAutoDetectionFalsePositivePayload;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandXRayTrigger;

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
        reason: &'static str,
    },
    // Autoupdate relaunch attempt
    AutoupdateRelaunchAttempt {
        version: String,
    },
    // Page up/down in editor
    PageUpDownInEditorPressed {
        direction: &'static str,
    },
}

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
