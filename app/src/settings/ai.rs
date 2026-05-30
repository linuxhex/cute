//! Stub implementation for AI settings
//! This provides minimal types to satisfy compilation without actual AI functionality

use serde::{Deserialize, Serialize};
use cuteui::{Entity, ModelContext, SingletonEntity};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AISettings {
    pub enabled: bool,
}

impl AISettings {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self::default()
    }

    pub fn is_any_ai_enabled(&self, _ctx: &cuteui::AppContext) -> bool {
        false
    }
}

impl Entity for AISettings {
    type Event = AISettingsChangedEvent;
}

impl SingletonEntity for AISettings {}

#[derive(Debug, Clone)]
pub struct AISettingsChangedEvent {
    pub settings: AISettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum DefaultSessionMode {
    #[default]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentModeCommandExecutionPredicate {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloudPreferencesSettings {
    pub enabled: bool,
}

pub mod cloud_preferences {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct CloudPreferences {
        pub enabled: bool,
    }
}

#[derive(Debug, Clone)]
pub struct FocusedTerminalInfo {
    pub id: String,
}
