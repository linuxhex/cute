//! Cloud preferences syncer.
//!
//! Note: Cloud-specific logic has been removed. Simplified stub.

use std::path::PathBuf;
use std::sync::Arc;

use cuteui::{Entity, ModelContext, SingletonEntity};

use super::cloud_preferences::{CloudPreferencesSettings, CloudPreferencesSettingsChangedEvent};
use super::manager::SettingsEvent;
use crate::auth::auth_state::AuthState;
use crate::server::ids::ClientId;
use crate::settings::manager::SettingsManager;

/// Provides client ids for creating cloud preferences.
pub trait ClientIdProvider {
    fn next_client_id(&self) -> ClientId;
}

struct DefaultClientIdProvider;
impl ClientIdProvider for DefaultClientIdProvider {
    fn next_client_id(&self) -> ClientId {
        ClientId::new()
    }
}

/// Key used to persist the hash of the settings file content.
/// Simplified: local version has no settings sync
// pub(super) const SETTINGS_FILE_LAST_SYNCED_HASH_KEY: &str = "SettingsFileLastSyncedHash";

/// Constructs the cloud preferences syncer (simplified stub).
pub fn initialize_cloud_preferences_syncer(
    _toml_file_path: PathBuf,
    _startup_toml_parse_error: Option<&str>,
    ctx: &mut ModelContext<CloudPreferencesSyncer>,
) -> CloudPreferencesSyncer {
    CloudPreferencesSyncer::new(false, PathBuf::new(), ctx)
}

/// Handles syncing CloudPreferences (simplified stub).
pub struct CloudPreferencesSyncer {
    #[allow(dead_code)]
    client_id_provider: Arc<dyn ClientIdProvider>,
    has_completed_initial_load: bool,
    force_local_wins_on_startup: bool,
    #[allow(dead_code)]
    toml_file_path: PathBuf,
}

/// Event fired by the CloudPreferencesSyncer.
#[derive(Debug)]
pub enum CloudPreferencesSyncerEvent {
    InitialLoadCompleted,
    Updated { key: String, value: String },
}

/// Whether to force the cloud to match the local settings.
#[derive(Debug)]
pub enum ForceCloudToMatchLocal {
    Yes,
    No,
}

impl CloudPreferencesSyncer {
    #[cfg(test)]
    pub fn new_for_test(
        ctx: &mut ModelContext<Self>,
        client_id_provider: Arc<dyn ClientIdProvider>,
    ) -> Self {
        Self::new_internal(ctx, client_id_provider, PathBuf::new())
    }

    pub fn new(
        force_local_wins_on_startup: bool,
        toml_file_path: PathBuf,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        let mut me = Self::new_internal(ctx, Arc::new(DefaultClientIdProvider), toml_file_path);
        me.force_local_wins_on_startup = force_local_wins_on_startup;
        me
    }

    fn new_internal(
        ctx: &mut ModelContext<Self>,
        client_id_provider: Arc<dyn ClientIdProvider>,
        toml_file_path: PathBuf,
    ) -> Self {
        // Subscribe to settings events (simplified)
        ctx.subscribe_to_model(
            &SettingsManager::handle(ctx),
            |_, event, _| match event {
                SettingsEvent::LocalPreferencesUpdated { .. } => {
                    // Simplified: no cloud sync
                }
            },
        );
        ctx.subscribe_to_model(
            &CloudPreferencesSettings::handle(ctx),
            |_, event, _| match event {
                CloudPreferencesSettingsChangedEvent::IsSettingsSyncEnabled { .. } => {
                    // Simplified: no cloud sync
                }
            },
        );

        Self {
            client_id_provider,
            has_completed_initial_load: false,
            force_local_wins_on_startup: false,
            toml_file_path,
        }
        .emit_initial_load_if_local(ctx)
    }

    /// 本地模式无云端同步，启动时立即通知订阅方。
    fn emit_initial_load_if_local(mut self, ctx: &mut ModelContext<Self>) -> Self {
        {
            self.has_completed_initial_load = true;
            ctx.emit(CloudPreferencesSyncerEvent::InitialLoadCompleted);
        }
        self
    }

    /// Handler for when the user has been fetched (simplified stub).
    pub fn handle_user_fetched(
        &mut self,
        _auth_state: Arc<AuthState>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.has_completed_initial_load = false;
        self.force_local_wins_on_startup = false;
        // Simplified: just mark initial load complete
        if !self.has_completed_initial_load {
            self.has_completed_initial_load = true;
            ctx.emit(CloudPreferencesSyncerEvent::InitialLoadCompleted);
        }
    }

    /// Performs a settings sync (simplified stub).
    pub fn sync(
        &self,
        _force_cloud_to_match_local: ForceCloudToMatchLocal,
        ctx: &mut ModelContext<Self>,
    ) {
        // Simplified: just emit initial load completed
        ctx.emit(CloudPreferencesSyncerEvent::InitialLoadCompleted);
    }

    /// Syncs the local preferences with the given storage keys to the cloud (simplified stub).
    #[allow(dead_code)]
    pub fn maybe_sync_local_prefs_to_cloud(
        &mut self,
        _keys_to_sync: Vec<String>,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Simplified: no cloud sync
    }
}

impl Entity for CloudPreferencesSyncer {
    type Event = CloudPreferencesSyncerEvent;
}

impl SingletonEntity for CloudPreferencesSyncer {}

