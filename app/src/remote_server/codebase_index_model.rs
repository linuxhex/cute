//! Stub for RemoteCodebaseIndexModel after remote_server removal.

use std::path::PathBuf;
use cute_core::HostId;
use cute_util::remote_path::RemotePath;
use cuteui::{Entity, ModelContext, SingletonEntity};

/// Stub RemoteCodebaseIndexModel.
pub struct RemoteCodebaseIndexModel {
    _phantom: (),
}

impl Default for RemoteCodebaseIndexModel {
    fn default() -> Self {
        Self {
            _phantom: (),
        }
    }
}

/// Stub event enum for RemoteCodebaseIndexModel.
#[derive(Clone, Debug)]
pub enum RemoteCodebaseIndexModelEvent {
    #[allow(dead_code)]
    IndexUpdated,
    #[allow(dead_code)]
    IndexRemoved,
    #[allow(dead_code)]
    SettingsEntriesChanged,
}

impl Entity for RemoteCodebaseIndexModel {
    type Event = RemoteCodebaseIndexModelEvent;
}

impl SingletonEntity for RemoteCodebaseIndexModel {}

/// Stub for agent context codebase info.
#[derive(Clone, Debug)]
pub struct AgentContextCodebase {
    pub name: String,
    pub path: String,
}

impl RemoteCodebaseIndexModel {
    pub fn entries_for_settings(&self) -> Vec<RemoteCodebaseIndexSettingsEntry> {
        Vec::new()
    }

    pub fn codebases_for_agent_context(&self, _host_id: HostId) -> Vec<AgentContextCodebase> {
        Vec::new()
    }

    pub fn active_repo_path(
        &self,
        _session_context: &crate::ai::blocklist::SessionContext,
        _requested_codebase_path: Option<&str>,
    ) -> Option<PathBuf> {
        None
    }

    pub fn active_repo_availability(
        &self,
        _session_context: &crate::ai::blocklist::SessionContext,
        _requested_codebase_path: Option<&str>,
    ) -> RemoteCodebaseSearchAvailability {
        RemoteCodebaseSearchAvailability::NoConnectedHost
    }

    pub fn request_index(
        &mut self,
        _remote_path: RemotePath,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }

    pub fn resync_index(
        &mut self,
        _remote_path: RemotePath,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }

    pub fn drop_index(
        &mut self,
        _remote_path: RemotePath,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }
}

/// Stub RemoteCodebaseIndex.
#[allow(dead_code)]
pub struct RemoteCodebaseIndex {
    #[allow(dead_code)]
    pub repo_path: PathBuf,
    #[allow(dead_code)]
    pub host_id: HostId,
    #[allow(dead_code)]
    pub state: RemoteCodebaseIndexState,
}

/// Stub RemoteCodebaseIndexState.
#[derive(Clone, Debug, Default)]
pub enum RemoteCodebaseIndexState {
    #[default]
    NotEnabled,
    #[allow(dead_code)]
    Unavailable,
    #[allow(dead_code)]
    Disabled,
    #[allow(dead_code)]
    Queued,
    #[allow(dead_code)]
    Indexing,
    #[allow(dead_code)]
    Ready,
    #[allow(dead_code)]
    Stale,
    #[allow(dead_code)]
    Failed,
    #[allow(dead_code)]
    NotIndexed,
    #[allow(dead_code)]
    Indexed,
    #[allow(dead_code)]
    Error,
}

/// Stub RemoteCodebaseIndexStatus.
#[derive(Clone, Debug, Default)]
pub struct RemoteCodebaseIndexStatus {
    #[allow(dead_code)]
    pub repo_path: String,
    #[allow(dead_code)]
    pub state: RemoteCodebaseIndexState,
    #[allow(dead_code)]
    pub progress: f32,
    #[allow(dead_code)]
    pub status: Option<String>,
    #[allow(dead_code)]
    pub progress_total: Option<u32>,
    #[allow(dead_code)]
    pub progress_completed: Option<u32>,
    #[allow(dead_code)]
    pub failure_message: Option<String>,
    #[allow(dead_code)]
    pub last_updated_epoch_millis: Option<u64>,
    #[allow(dead_code)]
    pub root_hash: Option<String>,
}

/// Stub RemoteCodebaseSearchAvailability.
#[derive(Clone, Debug)]
pub enum RemoteCodebaseSearchAvailability {
    #[allow(dead_code)]
    Unavailable { remote_path: RemotePath, message: String },
    #[allow(dead_code)]
    Ready(Box<RemoteCodebaseSearchContext>),
    #[allow(dead_code)]
    NotIndexed { remote_path: RemotePath },
    #[allow(dead_code)]
    Indexing { remote_path: RemotePath },
    #[allow(dead_code)]
    NoConnectedHost,
    #[allow(dead_code)]
    NoActiveRepo,
}

impl Default for RemoteCodebaseSearchAvailability {
    fn default() -> Self {
        Self::NoConnectedHost
    }
}

/// Stub RemoteCodebaseSearchContext.
#[derive(Clone, Debug, Default)]
pub struct RemoteCodebaseSearchContext {
    #[allow(dead_code)]
    pub availability: RemoteCodebaseSearchAvailability,
    #[allow(dead_code)]
    pub root_hash: String,
    #[allow(dead_code)]
    pub remote_path: Option<RemotePath>,
    #[allow(dead_code)]
    pub is_stale: bool,
}

/// Stub RemoteCodebaseIndexSettingsEntry.
#[derive(Clone, Debug, Default)]
pub struct RemoteCodebaseIndexSettingsEntry {
    pub repo_path: String,
    #[allow(dead_code)]
    pub state: RemoteCodebaseIndexState,
    pub remote_path: Option<cute_util::remote_path::RemotePath>,
    pub host_label: Option<String>,
    #[allow(dead_code)]
    pub path: Option<cute_util::remote_path::RemotePath>,
    pub status: RemoteCodebaseIndexStatus,
}
