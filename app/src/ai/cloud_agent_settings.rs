//! Settings for cloud agent functionality.
//!
//! This module contains user-specific settings for cloud agent features,
//! such as remembering the last selected environment.
//!
//! Note: Cloud-specific logic has been removed. Only settings storage remains
//! for local agent use.

use std::collections::HashMap;

use settings::macros::define_settings_group;
use settings::{Setting as _, SupportedPlatforms, SyncToCloud};
use cute_cli::agent::Harness;

use crate::server::ids::SyncId;

#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(description = "Selected third-party harness model.")]
pub struct HarnessModelSelection {
    pub model_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_level: Option<String>,
}

define_settings_group!(CloudAgentSettings, settings: [
    last_selected_environment_id: LastSelectedEnvironmentId {
        type: Option<SyncId>,
        default: None,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    harness_auth_ftux_completed: HarnessAuthFtuxCompleted {
        type: HashMap<String, bool>,
        default: HashMap::new(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    last_selected_harness: LastSelectedHarness {
        type: Option<String>,
        default: None,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    last_selected_host: LastSelectedHost {
        type: Option<String>,
        default: None,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    last_selected_harness_model: LastSelectedHarnessModel {
        type: HashMap<String, HarnessModelSelection>,
        default: HashMap::new(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    last_selected_auth_secret: LastSelectedAuthSecret {
        type: HashMap<String, String>,
        default: HashMap::new(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    },
    // Per-harness record of whether the user explicitly chose "Inherit
    // key from environment" in the orchestration auth secret picker.
    // Distinct from "never picked anything" (entry absent) so the plan
    // card's Inherit choice survives across the RunAgents handoff.
    inherit_auth_secret_harnesses: InheritAuthSecretHarnesses {
        type: HashMap<String, bool>,
        default: HashMap::new(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        private: true,
    }
]);

impl CloudAgentSettings {
    #[allow(dead_code)]
    pub fn is_harness_auth_ftux_completed(&self, harness: Harness) -> bool {
        self.harness_auth_ftux_completed
            .value()
            .get(harness.config_name())
            .copied()
            .unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn mark_harness_auth_ftux_completed(
        &mut self,
        harness: Harness,
        ctx: &mut cuteui::ModelContext<Self>,
    ) {
        let mut map = self.harness_auth_ftux_completed.value().clone();
        map.insert(harness.config_name().to_string(), true);
        let _ = self.harness_auth_ftux_completed.set_value(map, ctx);
    }

    /// Persists (or clears) the harness model selection for the given harness.
    #[allow(dead_code)]
    pub fn persist_harness_model_selection(
        &mut self,
        harness: Harness,
        model_id: &str,
        reasoning_level: Option<String>,
        ctx: &mut cuteui::ModelContext<Self>,
    ) {
        let mut map = self.last_selected_harness_model.value().clone();
        if model_id.is_empty() {
            map.remove(harness.config_name());
        } else {
            map.insert(
                harness.config_name().to_string(),
                HarnessModelSelection {
                    model_id: model_id.to_string(),
                    reasoning_level,
                },
            );
        }
        let _ = self.last_selected_harness_model.set_value(map, ctx);
    }
}
