//! Cloud preferences settings.
//!
//! Note: Cloud-specific logic has been removed. Simplified stub.

pub use crate::cloud_stub_types::models::{CloudPreference, CloudPreferenceModel};
use settings::macros::define_settings_group;
use settings::{RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};

define_settings_group!(CloudPreferencesSettings, settings: [
   settings_sync_enabled: IsSettingsSyncEnabled {
       type: bool,
       default: false,
       supported_platforms: SupportedPlatforms::ALL,
       sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::No),
       private: false,
       toml_path: "account.is_settings_sync_enabled",
       description: "Whether settings are synced across devices via the cloud.",
   },
]);

// Note: StringModel and JsonModel impls for Preference are in cloud_stub_types/models/preference.rs
